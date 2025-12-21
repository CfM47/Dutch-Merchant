use crate::evaluator::path_evaluator::PathEvaluator;
use crate::model::instance::{Instance, PortId};
use minilp::{ComparisonOp, OptimizationDirection, Problem};

/// Calculates the best profit given an instance and a vector of nodes (ports).
/// Uses Linear Programming (LP) to solve the relaxed continuous version.
pub struct LpProfitCalculator;

impl PathEvaluator for LpProfitCalculator {
    fn name(&self) -> &'static str {
        "LpProfitCalculator"
    }

    /// # Arguments
    /// * `instance` - The problem instance
    /// * `nodes` - A vector of port IDs representing the route
    ///
    /// # Returns
    /// A tuple containing:
    /// * The best profit as a float (optimal value from LP relaxation)
    /// * The decisions (quantities bought/sold) for each port and good
    fn calculate_best_profit(self, instance: &Instance, nodes: &[PortId]) -> (f64, Vec<Vec<f64>>) {
        let r = nodes.len();
        let m = instance.n_goods;
        
        // Initialize decisions with zeros
        // decisions[j][k] > 0 means buy, < 0 means sell
        let mut decisions: Vec<Vec<f64>> = vec![vec![0.0; m]; r];

        if nodes.is_empty() {
            return (instance.initial_capital, decisions);
        }

        // Create LP problem - we want to maximize final capital
        let mut problem = Problem::new(OptimizationDirection::Maximize);

        // Variables:
        // q_buy[j][m] = amount bought of good m at port j (q_{j,m}^+)
        // q_sell[j][m] = amount sold of good m at port j (q_{j,m}^-)
        // I[j][m] = inventory of good m after visiting port j
        // f[j] = capital after visiting port j

        // Create variables for each port in the route
        let mut q_buy: Vec<Vec<_>> = Vec::with_capacity(r);
        let mut q_sell: Vec<Vec<_>> = Vec::with_capacity(r);
        let mut inventory: Vec<Vec<_>> = Vec::with_capacity(r);
        let mut capital: Vec<_> = Vec::with_capacity(r);

        for j in 0..r {
            let port = nodes[j];
            let mut q_buy_j = Vec::with_capacity(m);
            let mut q_sell_j = Vec::with_capacity(m);
            let mut inv_j = Vec::with_capacity(m);

            for good in 0..m {
                // q_buy[j][m]: 0 <= q_buy <= buy_cap[port][good]
                let buy_cap = instance.buy_cap[port][good];
                let qb = problem.add_var(0.0, (0.0, buy_cap));
                q_buy_j.push(qb);

                // q_sell[j][m]: 0 <= q_sell <= sell_cap[port][good]
                let sell_cap = instance.sell_cap[port][good];
                let qs = problem.add_var(0.0, (0.0, sell_cap));
                q_sell_j.push(qs);

                // Inventory variable: I[j][m] >= 0
                let inv = problem.add_var(0.0, (0.0, f64::INFINITY));
                inv_j.push(inv);
            }

            q_buy.push(q_buy_j);
            q_sell.push(q_sell_j);
            inventory.push(inv_j);

            // Capital variable: f[j] >= 0 (we want to maximize f[r-1])
            // The last capital variable has coefficient 1.0 in objective
            let obj_coef = if j == r - 1 { 1.0 } else { 0.0 };
            let cap = problem.add_var(obj_coef, (0.0, f64::INFINITY));
            capital.push(cap);
        }

        // Add constraints
        for j in 0..r {
            let port = nodes[j];

            // Inventory constraints: I[j][m] = I[j-1][m] + q_buy[j][m] - q_sell[j][m]
            // Rewritten as: I[j][m] - q_buy[j][m] + q_sell[j][m] = I[j-1][m]
            for good in 0..m {
                let prev_inv = if j == 0 { 0.0 } else { 0.0 }; // Initial inventory is 0

                if j == 0 {
                    // I[0][m] - q_buy[0][m] + q_sell[0][m] = 0
                    // Also: q_sell[0][m] <= 0 (can't sell what we don't have initially)
                    problem.add_constraint(
                        [(inventory[j][good], 1.0), (q_buy[j][good], -1.0), (q_sell[j][good], 1.0)],
                        ComparisonOp::Eq,
                        prev_inv,
                    );
                } else {
                    // I[j][m] - I[j-1][m] - q_buy[j][m] + q_sell[j][m] = 0
                    problem.add_constraint(
                        [
                            (inventory[j][good], 1.0),
                            (inventory[j - 1][good], -1.0),
                            (q_buy[j][good], -1.0),
                            (q_sell[j][good], 1.0),
                        ],
                        ComparisonOp::Eq,
                        0.0,
                    );
                }
            }

            // Boat capacity constraint: sum_m I[j][m] * w[m] <= B
            let capacity_terms: Vec<_> = (0..m)
                .map(|good| (inventory[j][good], instance.weight[good]))
                .collect();
            problem.add_constraint(capacity_terms, ComparisonOp::Le, instance.capacity);

            // Capital constraint:
            // f[j] = f[j-1] - sum_m (p_buy[j][m] * q_buy[j][m] - p_sell[j][m] * q_sell[j][m]) - S(port)
            // Rewritten as:
            // f[j] + sum_m (p_buy[j][m] * q_buy[j][m]) - sum_m (p_sell[j][m] * q_sell[j][m]) = f[j-1] - S(port)

            let mut capital_terms: Vec<(minilp::Variable, f64)> = Vec::new();
            capital_terms.push((capital[j], 1.0));

            for good in 0..m {
                capital_terms.push((q_buy[j][good], instance.buy_price[port][good]));
                capital_terms.push((q_sell[j][good], -instance.sell_price[port][good]));
            }

            let prev_capital = if j == 0 {
                instance.initial_capital
            } else {
                0.0
            };

            if j == 0 {
                // f[0] + sum(...) = f_0 - S(port)
                problem.add_constraint(
                    capital_terms,
                    ComparisonOp::Eq,
                    prev_capital - instance.visit_cost[port],
                );
            } else {
                // f[j] - f[j-1] + sum(...) = -S(port)
                capital_terms.push((capital[j - 1], -1.0));
                problem.add_constraint(capital_terms, ComparisonOp::Eq, -instance.visit_cost[port]);
            }
        }

        // Constraint: can only sell what we have in inventory
        // q_sell[j][m] <= I[j-1][m] (for j > 0)
        // For j = 0: q_sell[0][m] = 0 (can't sell, no initial inventory)
        for good in 0..m {
            // At first port, we can't sell anything (no initial inventory)
            problem.add_constraint([(q_sell[0][good], 1.0)], ComparisonOp::Eq, 0.0);
        }

        for j in 1..r {
            for good in 0..m {
                // q_sell[j][m] <= I[j-1][m]
                // Rewritten as: q_sell[j][m] - I[j-1][m] <= 0
                problem.add_constraint(
                    [(q_sell[j][good], 1.0), (inventory[j - 1][good], -1.0)],
                    ComparisonOp::Le,
                    0.0,
                );
            }
        }

        // Solve the LP
        match problem.solve() {
            Ok(solution) => {
                let profit = solution.objective();
                
                // Extract decisions
                for j in 0..r {
                    for good in 0..m {
                        // Get the value of buy and sell variables from the solution
                        // Since minilp solution uses variable indices, we need to map back if necessary.
                        // However, minilp variables carry their index.
                        
                        // Note: minilp 0.2 Solution doesn't have a direct "get value by variable handle" 
                        // if we lost the handle, but we kept them in q_buy/q_sell.
                        // Depending on minilp API, we might need to access by index.
                        
                        let buy_val = solution[q_buy[j][good]];
                        let sell_val = solution[q_sell[j][good]];
                        
                        decisions[j][good] = buy_val - sell_val;
                    }
                }
                
                (profit, decisions)
            },
            Err(_) => (instance.initial_capital, decisions), // If infeasible, return initial capital and zero decisions
        }
    }
}
