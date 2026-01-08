# Statistical Test Report: Brute Force vs RL Solver
Generated on: 2025-12-22 11:26:12

## Experiment Configuration
- **Number of Cases**: 100
- **Sampled Cases for Test**: 30
- **Ports Range**: (5, 9)
- **Goods Range**: (3, 6)
- **Time Limit Range**: (80.0, 150.0)
- **RL Hyperparameters**: 50 epochs, 5 episodes/epoch

## Descriptive Statistics

| Metric | Brute Force | RL Solver |
| :--- | :--- | :--- |
| **Mean Profit** | 16496.51 | 16217.40 |
| **Std Dev** | 7624.85 | 7382.15 |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | 0.9862 (98.62%) |
| **Median Ratio** | 1.0000 (100.00%) |
| **RL Win/Tie Rate** | 48.00% |

## Statistical Test

### Paired t-test (BF vs Scaled RL)
**Null Hypothesis**: There is no difference between BF and scaled RL mean profits.

- **t-statistic**: 0.0178
- **p-value**: 0.985899
- **Result**: Fail to reject H0 at α = 0.05
- **Interpretation**: No significant difference detected

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves 98.62% of the brute-force optimal solution.

2. **Practical Significance**: The RL solver wins or ties in 48.00% of cases, demonstrating limited competitive performance.

3. **Statistical Significance**: No significant difference was found between the methods (p ≥ 0.05).

## Recommendations

Based on the statistical analysis, the RL solver shows promise as an approximate solution method for this problem class.
