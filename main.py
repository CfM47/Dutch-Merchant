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
    
    # Paired t-test (since we have paired samples from the same instances)
    t_stat, p_value = stats.ttest_rel(bf_profits, rl_profits)
    
    # Effect size (Cohen's d for paired samples)
    differences = bf_profits - rl_profits
    cohens_d = np.mean(differences) / np.std(differences, ddof=1)
    
    # Wilcoxon signed-rank test (non-parametric alternative)
    wilcoxon_stat, wilcoxon_p = stats.wilcoxon(bf_profits, rl_profits)
    
    # One-sample t-test on ratio (testing if mean ratio significantly differs from 1.0)
    ratio_t_stat, ratio_p_value = stats.ttest_1samp(ratios, 1.0)
    
    # Confidence interval for ratio mean
    ratio_mean = np.mean(ratios)
    ratio_std = np.std(ratios, ddof=1)
    ratio_se = ratio_std / np.sqrt(len(ratios))
    ratio_ci = stats.t.interval(0.95, len(ratios)-1, loc=ratio_mean, scale=ratio_se)
    
    return {
        "bf_mean": np.mean(bf_profits),
        "bf_std": np.std(bf_profits, ddof=1),
        "rl_mean": np.mean(rl_profits),
        "rl_std": np.std(rl_profits, ddof=1),
        "mean_diff": np.mean(differences),
        "t_statistic": t_stat,
        "p_value": p_value,
        "cohens_d": cohens_d,
        "wilcoxon_stat": wilcoxon_stat,
        "wilcoxon_p": wilcoxon_p,
        "ratio_mean": ratio_mean,
        "ratio_std": ratio_std,
        "ratio_t_stat": ratio_t_stat,
        "ratio_p_value": ratio_p_value,
        "ratio_ci": ratio_ci,
        "ratio_median": np.median(ratios),
        "win_rate": np.sum(rl_profits >= bf_profits) / len(results),
        "n_samples": len(results)
    }

def generate_statistical_report(results, stats_dict, config):
    """Generate a detailed statistical report."""
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    report = f"""# Statistical Test Report: Brute Force vs RL Solver
Generated on: {now}

## Experiment Configuration
- **Number of Cases**: {stats_dict['n_samples']}
- **Ports Range**: {config.n_ports_range}
- **Goods Range**: {config.n_goods_range}
- **Time Limit Range**: {config.time_limit_range}
- **RL Hyperparameters**: 50 epochs, 5 episodes/epoch

## Descriptive Statistics

| Metric | Brute Force | RL Solver |
| :--- | :--- | :--- |
| **Mean Profit** | {stats_dict['bf_mean']:.2f} | {stats_dict['rl_mean']:.2f} |
| **Std Dev** | {stats_dict['bf_std']:.2f} | {stats_dict['rl_std']:.2f} |
| **Mean Difference (BF - RL)** | {stats_dict['mean_diff']:.2f} | - |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | {stats_dict['ratio_mean']:.4f} ({stats_dict['ratio_mean']*100:.2f}%) |
| **Median Ratio** | {stats_dict['ratio_median']:.4f} ({stats_dict['ratio_median']*100:.2f}%) |
| **Std Dev** | {stats_dict['ratio_std']:.4f} |
| **95% Confidence Interval** | [{stats_dict['ratio_ci'][0]:.4f}, {stats_dict['ratio_ci'][1]:.4f}] |
| **RL Win/Tie Rate** | {stats_dict['win_rate']:.2%} |

## Statistical Tests

### 1. Paired t-test (BF vs RL)
**Null Hypothesis**: There is no difference between BF and RL mean profits.

- **t-statistic**: {stats_dict['t_statistic']:.4f}
- **p-value**: {stats_dict['p_value']:.6f}
- **Result**: {'Reject H0' if stats_dict['p_value'] < 0.05 else 'Fail to reject H0'} at α = 0.05
- **Interpretation**: {'The difference is statistically significant' if stats_dict['p_value'] < 0.05 else 'No significant difference detected'}

### 2. Effect Size (Cohen's d)
- **Cohen's d**: {stats_dict['cohens_d']:.4f}
- **Interpretation**: {
    'Negligible effect' if abs(stats_dict['cohens_d']) < 0.2 else
    'Small effect' if abs(stats_dict['cohens_d']) < 0.5 else
    'Medium effect' if abs(stats_dict['cohens_d']) < 0.8 else
    'Large effect'
}

### 3. Wilcoxon Signed-Rank Test (Non-parametric)
- **Test Statistic**: {stats_dict['wilcoxon_stat']:.4f}
- **p-value**: {stats_dict['wilcoxon_p']:.6f}
- **Result**: {'Reject H0' if stats_dict['wilcoxon_p'] < 0.05 else 'Fail to reject H0'} at α = 0.05

### 4. One-Sample t-test on Ratio
**Null Hypothesis**: Mean ratio equals 1.0 (RL performs equally to BF).

- **t-statistic**: {stats_dict['ratio_t_stat']:.4f}
- **p-value**: {stats_dict['ratio_p_value']:.6f}
- **Result**: {'Reject H0' if stats_dict['ratio_p_value'] < 0.05 else 'Fail to reject H0'} at α = 0.05
- **Interpretation**: {'RL performance differs significantly from optimal' if stats_dict['ratio_p_value'] < 0.05 else 'RL performance is not significantly different from optimal'}

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves {stats_dict['ratio_mean']*100:.2f}% of the brute-force optimal solution.

2. **Consistency**: The 95% confidence interval for the ratio is [{stats_dict['ratio_ci'][0]*100:.2f}%, {stats_dict['ratio_ci'][1]*100:.2f}%], indicating {"high" if stats_dict['ratio_std'] < 0.1 else "moderate" if stats_dict['ratio_std'] < 0.2 else "variable"} consistency.

3. **Practical Significance**: The RL solver wins or ties in {stats_dict['win_rate']*100:.2f}% of cases, demonstrating {'strong' if stats_dict['win_rate'] > 0.8 else 'good' if stats_dict['win_rate'] > 0.5 else 'limited'} competitive performance.

4. **Statistical Significance**: {'The paired t-test confirms a significant difference between methods (p < 0.05).' if stats_dict['p_value'] < 0.05 else 'No significant difference was found between the methods (p ≥ 0.05).'}

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
    print(f"Sample Size: {stats_dict['n_samples']}")
    print(f"Mean BF Profit: {stats_dict['bf_mean']:.2f}")
    print(f"Mean RL Profit: {stats_dict['rl_mean']:.2f}")
    print(f"Mean Ratio (RL/BF): {stats_dict['ratio_mean']:.4f} ({stats_dict['ratio_mean']*100:.2f}%)")
    print(f"95% CI for Ratio: [{stats_dict['ratio_ci'][0]:.4f}, {stats_dict['ratio_ci'][1]:.4f}]")
    print(f"Paired t-test p-value: {stats_dict['p_value']:.6f}")
    print(f"Cohen's d: {stats_dict['cohens_d']:.4f}")
    print(f"RL Win/Tie Rate: {stats_dict['win_rate']:.2%}")
    print("="*60)

def main():
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

    # Run 5 cases for verification
    results = run_experiment(500, config)
    
    # Perform statistical tests
    stats_dict = perform_statistical_tests(results)
    
    # Generate report
    generate_statistical_report(results, stats_dict, config)
    
    # Save raw results
    results_path = Path("experiment_results.json")
    with open(results_path, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"Raw results saved to {results_path.absolute()}")

if __name__ == "__main__":
    main()