#[cfg(test)]
mod tests {
    use super::super::profit_calculator::LpProfitCalculator;
    use crate::evaluator::path_evaluator::PathEvaluator;
    use crate::model::instance::{Instance, PortId};

    #[test]
    fn test_calculate_best_profit_simple_case() {
        // 2 ports, 1 good
        let instance = Instance {
            n_ports: 2,
            n_goods: 1,
            travel_time: vec![vec![0.0, 1.0], vec![1.0, 0.0]],
            travel_cost: 1.0,
            weight: vec![1.0],
            buy_price: vec![vec![2.0], vec![0.0]], // Only port 0 sells
            sell_price: vec![vec![0.0], vec![5.0]], // Only port 1 buys
            buy_cap: vec![vec![10.0], vec![0.0]],
            sell_cap: vec![vec![0.0], vec![10.0]],
            visit_cost: vec![1.0, 0.0],
            start_port: 0,
            capacity: 10.0,
            time_limit: 100.0,
            initial_capital: 20.0,
        };
        // Route: port 0 -> port 1 -> port 0
        let route: Vec<PortId> = vec![0, 1, 0];
        let calculator = LpProfitCalculator;
        let (profit, _decisions) = calculator.calculate_best_profit(&instance, &route);
        // Buy 10 at port 0 for 2 each, sell 10 at port 1 for 5 each
        // Initial: 20, buy: -20, sell: +50, final: 50
        assert!(
            (profit - 47.0).abs() < 1e-6,
            "Expected profit 48.0, got {}",
            profit
        );
    }

    #[test]
    fn test_calculate_best_profit_liquidity_and_multi_stop() {
        // 4 ports, 2 goods
        // Good 0: Cheap (Weight 1.0, Buy 10, Sell 20)
        // Good 1: Luxury (Weight 2.0, Buy 200, Sell 1000)
        let instance = Instance {
            n_ports: 4,
            n_goods: 2,
            travel_time: vec![
                vec![0.0, 1.0, 1.0, 1.0],
                vec![1.0, 0.0, 1.0, 1.0],
                vec![1.0, 1.0, 0.0, 1.0],
                vec![1.0, 1.0, 1.0, 0.0],
            ],
            travel_cost: 0.0,
            weight: vec![1.0, 2.0],
            buy_price: vec![
                vec![10.0, 200.0], // Port 0: Luxury too expensive for initial 100
                vec![0.0, 0.0],
                vec![0.0, 200.0], // Port 2: Buy Luxury here after making money
                vec![0.0, 0.0],
            ],
            sell_price: vec![
                vec![0.0, 0.0],
                vec![20.0, 0.0], // Port 1: Sell Cheap good to get cash
                vec![0.0, 0.0],
                vec![0.0, 1000.0], // Port 3: Big payout
            ],
            buy_cap: vec![
                vec![10.0, 5.0],
                vec![0.0, 0.0],
                vec![0.0, 5.0],
                vec![0.0, 0.0],
            ],
            sell_cap: vec![
                vec![0.0, 0.0],
                vec![10.0, 0.0],
                vec![0.0, 0.0],
                vec![0.0, 5.0],
            ],
            visit_cost: vec![0.0, 0.0, 0.0, 0.0],
            start_port: 0,
            capacity: 10.0,
            time_limit: 100.0,
            initial_capital: 10105.0, // Enough for 10 units of Good 0 + visit cost
        };

        // Route: 0 -> 1 -> 2 -> 3
        let route: Vec<PortId> = vec![0, 1, 2, 3];
        let evaluator = LpProfitCalculator;
        let (profit, _) = evaluator.calculate_best_profit(&instance, &route);

        /* Step-by-Step Logic:
            1. Port 0: Spend 100 to buy 10 units of Good 0, spend 1000 to buy 5 units of Good 1. Capital left: -995+10000.
            2. Port 1: Sell 10 units of Good 0 for 200. New Capital: -795+10000.
            3. Port 2: does nothing. Capital: -795+10000.
            4. Port 3: Sell 5 units of Good 1 for 5000. New Capital: 4205+10000.
        */

        assert!(
            (profit - 14205.0).abs() < 1e-6,
            "Expected profit 14205.0, got {}",
            profit
        );
    }
}
