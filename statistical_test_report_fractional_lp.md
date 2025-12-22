# Statistical Test Report: Brute Force vs RL Solver
Generated on: 2025-12-22 10:32:26

## Experiment Configuration
- **Number of Cases**: 500
- **Sampled Cases for Test**: 30
- **Ports Range**: (5, 9)
- **Goods Range**: (3, 6)
- **Time Limit Range**: (80.0, 150.0)
- **RL Hyperparameters**: 50 epochs, 5 episodes/epoch

## Descriptive Statistics

| Metric | Brute Force | RL Solver |
| :--- | :--- | :--- |
| **Mean Profit** | 7465.78 | 7236.25 |
| **Std Dev** | 2677.01 | 2536.36 |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | 0.9729 (97.29%) |
| **Median Ratio** | 0.9908 (99.08%) |
| **RL Win/Tie Rate** | 46.20% |

## Statistical Test

### Paired t-test (BF vs Scaled RL)
**Null Hypothesis**: There is no difference between BF and scaled RL mean profits.

- **t-statistic**: 0.7967
- **p-value**: 0.432084
- **Result**: Fail to reject H0 at α = 0.05
- **Interpretation**: No significant difference detected

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves 97.29% of the brute-force optimal solution.

2. **Practical Significance**: The RL solver wins or ties in 46.20% of cases, demonstrating limited competitive performance.

3. **Statistical Significance**: No significant difference was found between the methods (p ≥ 0.05).

## Recommendations

Based on the statistical analysis, the RL solver shows promise as an approximate solution method for this problem class.
