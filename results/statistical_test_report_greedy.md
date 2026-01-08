# Statistical Test Report: Brute Force vs RL Solver
Generated on: 2025-12-22 11:35:20

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
| **Mean Profit** | 11766.01 | 11681.60 |
| **Std Dev** | 4145.03 | 4077.11 |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | 0.9939 (99.39%) |
| **Median Ratio** | 0.9999 (99.99%) |
| **RL Win/Tie Rate** | 43.00% |

## Statistical Test

### Paired t-test (BF vs Scaled RL)
**Null Hypothesis**: There is no difference between BF and scaled RL mean profits.

- **t-statistic**: 1.0768
- **p-value**: 0.290460
- **Result**: Fail to reject H0 at α = 0.05
- **Interpretation**: No significant difference detected

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves 99.39% of the brute-force optimal solution.

2. **Practical Significance**: The RL solver wins or ties in 43.00% of cases, demonstrating limited competitive performance.

3. **Statistical Significance**: No significant difference was found between the methods (p ≥ 0.05).

## Recommendations

Based on the statistical analysis, the RL solver shows promise as an approximate solution method for this problem class.
