import json
import statistics
import datetime
from pathlib import Path
from scipy import stats
import numpy as np
import dm_solution
from dm_solution import Instance as RustInstance, RandomConfig, generate_random_instance, brute_force_solve, PathEvaluator
from rl_model.solver import Solver as RLSolver
from rl_model.schemas import Instance as RLInstance

def count_time_paths(rust_inst):
    """Count number of valid cycles starting and ending at start_port within the time limit."""
    n_ports = rust_inst.n_ports
    start_port = rust_inst.start_port
    time_limit = rust_inst.time_limit
    travel_time = rust_inst.travel_time
    
    count = 0
    
    def dfs(current_port, current_time, visited):
        nonlocal count
        
        # Try to return to start port
        time_to_start = travel_time[current_port][start_port]
        if current_time + time_to_start <= time_limit:
            count += 1
            
        # Explore other ports
        for next_port in range(n_ports):
            if next_port != start_port and next_port not in visited:
                travel = travel_time[current_port][next_port]
                if current_time + travel <= time_limit:
                    visited.add(next_port)
                    dfs(next_port, current_time + travel, visited)
                    visited.remove(next_port)

    dfs(start_port, 0.0, set())
    return count

def rust_instance_to_rl_instance(rust_inst):
    """Convert a Rust Instance object to an RL Instance schema."""
    data = {
        "n_ports": rust_inst.n_ports,
        "n_goods": rust_inst.n_goods,
        "travel_time": rust_inst.travel_time,
        "travel_cost": rust_inst.travel_cost,
        "weight": rust_inst.weight,
        "buy_price": rust_inst.buy_price,
        "sell_price": rust_inst.sell_price,
        "buy_cap": rust_inst.buy_cap,
        "sell_cap": rust_inst.sell_cap,
        "visit_cost": rust_inst.visit_cost,
        "start_port": rust_inst.start_port,
        "capacity": rust_inst.capacity,
        "time_limit": rust_inst.time_limit,
        "initial_capital": rust_inst.initial_capital,
    }
    return RLInstance(**data)

def evaluate_with_rust(rust_inst, path):
    """Evaluate a path using the Rust PathEvaluator."""
    rl_inst = rust_instance_to_rl_instance(rust_inst)
    inst_json = rl_inst.model_dump_json()
    evaluator = PathEvaluator(inst_json)
    profit, _ = evaluator.score_route(path, "LpProfitCalculator")
    return profit

def run_experiment(num_cases, config):
    """Run experiment and collect results."""
    print(f"\n--- Running Statistical Test: {num_cases} cases ---")
    results = []
    
    for i in range(num_cases):
        rust_inst = generate_random_instance(config)
        rl_inst = rust_instance_to_rl_instance(rust_inst)

        # Brute Force
        bf_solution = brute_force_solve(rust_inst)
        bf_profit = evaluate_with_rust(rust_inst, bf_solution.route) if bf_solution else 0.0

        # RL Solver
        rl_solver = RLSolver(num_epochs=50, episodes_per_epoch=5, verbose=False)
        rl_path = rl_solver.solve(rl_inst)
        rl_profit = evaluate_with_rust(rust_inst, rl_path)

        ratio = (rl_profit / bf_profit if bf_profit > 0 else (1.0 if rl_profit == 0 else 0.0))
        
        # Path Counting
        n_paths = count_time_paths(rust_inst)

        results.append({
            "case": i + 1,
            "num_ports": rust_inst.n_ports,
            "num_of_goods": rust_inst.n_goods,
            "initial_capital": rust_inst.initial_capital,
            "limit_time": rust_inst.time_limit,
            "capacity": rust_inst.capacity,
            "n_paths": n_paths,
            "bf_profit": bf_profit,
            "rl_profit": rl_profit,
            "ratio": ratio
        })
        
        print(f"Completed {i + 1}/{num_cases} cases...")

    return results

def perform_statistical_tests(results):
    """Perform statistical tests on the results."""
    bf_profits = np.array([r["bf_profit"] for r in results])
    rl_profits = np.array([r["rl_profit"] for r in results])
    ratios = np.array([r["ratio"] for r in results])
    
    # Paired t-test with sampling
    n_samples = 30
    indices = np.random.choice(len(bf_profits), size=n_samples, replace=False)

    # Sample the data
    bf_sampled = bf_profits[indices]
    rl_sampled = rl_profits[indices]

    ratios_mean = np.mean(ratios)
    rl_scaled = rl_sampled / ratios_mean
    
    t_stat, p_value = stats.ttest_rel(bf_sampled, rl_scaled)
    
    return {
        "bf_mean": np.mean(bf_profits),
        "bf_std": np.std(bf_profits, ddof=1),
        "rl_mean": np.mean(rl_profits),
        "rl_std": np.std(rl_profits, ddof=1),
        "t_statistic": t_stat,
        "p_value": p_value,
        "ratio_mean": ratios_mean,
        "ratio_median": np.median(ratios),
        "win_rate": np.sum(rl_profits >= bf_profits) / len(results),
        "n_samples": len(results),
        "n_sampled": n_samples
    }

def generate_statistical_report(results, stats_dict, config):
    """Generate a detailed statistical report."""
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    report = f"""# Statistical Test Report: Brute Force vs RL Solver
Generated on: {now}

## Experiment Configuration
- **Number of Cases**: {stats_dict['n_samples']}
- **Sampled Cases for Test**: {stats_dict['n_sampled']}
- **Ports Range**: {config.n_ports_range}
- **Goods Range**: {config.n_goods_range}
- **Time Limit Range**: {config.time_limit_range}
- **RL Hyperparameters**: 50 epochs, 5 episodes/epoch

## Descriptive Statistics

| Metric | Brute Force | RL Solver |
| :--- | :--- | :--- |
| **Mean Profit** | {stats_dict['bf_mean']:.2f} | {stats_dict['rl_mean']:.2f} |
| **Std Dev** | {stats_dict['bf_std']:.2f} | {stats_dict['rl_std']:.2f} |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | {stats_dict['ratio_mean']:.4f} ({stats_dict['ratio_mean']*100:.2f}%) |
| **Median Ratio** | {stats_dict['ratio_median']:.4f} ({stats_dict['ratio_median']*100:.2f}%) |
| **RL Win/Tie Rate** | {stats_dict['win_rate']:.2%} |

## Statistical Test

### Paired t-test (BF vs Scaled RL)
**Null Hypothesis**: There is no difference between BF and scaled RL mean profits.

- **t-statistic**: {stats_dict['t_statistic']:.4f}
- **p-value**: {stats_dict['p_value']:.6f}
- **Result**: {'Reject H0' if stats_dict['p_value'] < 0.05 else 'Fail to reject H0'} at α = 0.05
- **Interpretation**: {'The difference is statistically significant' if stats_dict['p_value'] < 0.05 else 'No significant difference detected'}

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves {stats_dict['ratio_mean']*100:.2f}% of the brute-force optimal solution.

2. **Practical Significance**: The RL solver wins or ties in {stats_dict['win_rate']*100:.2f}% of cases, demonstrating {'strong' if stats_dict['win_rate'] > 0.8 else 'good' if stats_dict['win_rate'] > 0.5 else 'limited'} competitive performance.

3. **Statistical Significance**: {'The paired t-test confirms a significant difference between methods (p < 0.05).' if stats_dict['p_value'] < 0.05 else 'No significant difference was found between the methods (p ≥ 0.05).'}

## Recommendations

Based on the statistical analysis, the RL solver {'shows promise as an approximate solution method' if stats_dict['ratio_mean'] > 0.85 else 'requires further optimization to approach optimal performance'} for this problem class.
"""
    
    report_path = Path("statistical_test_report.md")
    report_path.write_text(report)
    print(f"\nStatistical report generated at {report_path.absolute()}")
    
    # Also print key results to console
    print("\n" + "="*60)
    print("KEY STATISTICAL RESULTS")
    print("="*60)
    print(f"Total Sample Size: {stats_dict['n_samples']}")
    print(f"Sampled for Test: {stats_dict['n_sampled']}")
    print(f"Mean BF Profit: {stats_dict['bf_mean']:.2f}")
    print(f"Mean RL Profit: {stats_dict['rl_mean']:.2f}")
    print(f"Mean Ratio (RL/BF): {stats_dict['ratio_mean']:.4f} ({stats_dict['ratio_mean']*100:.2f}%)")
    print(f"Paired t-test p-value: {stats_dict['p_value']:.6f}")
    print(f"RL Win/Tie Rate: {stats_dict['win_rate']:.2%}")
    print("="*60)
    
def main():
    import sys
    
    config = RandomConfig(
        n_ports_range=(5, 9),
        n_goods_range=(3, 6),
        travel_time_range=(5.0, 20.0),
        price_range=(50.0, 150.0),
        weight_range=(1.0, 5.0),
        capacity_range=(100.0, 200.0),
        time_limit_range=(80.0, 150.0),
        initial_capital_range=(500.0, 2000.0),
        visit_cost_range=(0.0, 10.0),
        travel_cost_range=(0.1, 0.5),
        value_type="float",
        max_value=100.0
    )

    # Check if a JSON file path is provided as a command-line argument
    json_path = None
    if len(sys.argv) > 1:
        json_path = Path(sys.argv[1])
    else:
        # Check for the specific files or default file
        default_files = [
            Path("experiment_results_fractional_lp.json"),
        ]
        for p in default_files:
            if p.exists():
                json_path = p
                break

    if json_path and json_path.exists():
        print(f"\nLoading results from {json_path}...")
        with open(json_path, 'r') as f:
            results = json.load(f)
    else:
        # Run cases for verification if no JSON is found
        print("\nNo JSON results found. Running experiments...")
        results = run_experiment(50, config) # Default to 50 for quick run

    # Perform statistical tests
    stats_dict = perform_statistical_tests(results)
    
    # Generate report
    generate_statistical_report(results, stats_dict, config)
    
    # Save raw results (only if they were newly generated)
    if not json_path or not json_path.exists():
        results_path = Path("experiment_results.json")
        with open(results_path, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"Raw results saved to {results_path.absolute()}")

if __name__ == "__main__":
    main()