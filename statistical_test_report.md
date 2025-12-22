# Statistical Test Report: Brute Force vs RL Solver
Generated on: 2025-12-22 00:56:17

## Experiment Configuration
- **Number of Cases**: 300
- **Ports**: 6
- **Goods**: 3
- **Time Limit Range**: (80.0, 150.0)
- **RL Hyperparameters**: 50 epochs, 5 episodes/epoch

## Descriptive Statistics

| Metric | Brute Force | RL Solver |
| :--- | :--- | :--- |
| **Mean Profit** | 5652.53 | 5558.06 |
| **Std Dev** | 1875.30 | 1852.03 |
| **Mean Difference (BF - RL)** | 94.48 | - |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | 0.9835 (98.35%) |
| **Median Ratio** | 1.0000 (100.00%) |
| **Std Dev** | 0.0309 |
| **95% Confidence Interval** | [0.9800, 0.9870] |
| **RL Win/Tie Rate** | 55.00% |

## Statistical Tests

### 1. Paired t-test (BF vs RL)
**Null Hypothesis**: There is no difference between BF and RL mean profits.

- **t-statistic**: 8.9338
- **p-value**: 0.000000
- **Result**: Reject H0 at α = 0.05
- **Interpretation**: The difference is statistically significant

### 2. Effect Size (Cohen's d)
- **Cohen's d**: 0.5158
- **Interpretation**: Medium effect

### 3. Wilcoxon Signed-Rank Test (Non-parametric)
- **Test Statistic**: 0.0000
- **p-value**: 0.000000
- **Result**: Reject H0 at α = 0.05

### 4. One-Sample t-test on Ratio
**Null Hypothesis**: Mean ratio equals 1.0 (RL performs equally to BF).

- **t-statistic**: -9.2266
- **p-value**: 0.000000
- **Result**: Reject H0 at α = 0.05
- **Interpretation**: RL performance differs significantly from optimal

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves 98.35% of the brute-force optimal solution.

2. **Consistency**: The 95% confidence interval for the ratio is [98.00%, 98.70%], indicating high consistency.

3. **Practical Significance**: The RL solver wins or ties in 55.00% of cases, demonstrating good competitive performance.

4. **Statistical Significance**: The paired t-test confirms a significant difference between methods (p < 0.05).

## Recommendations

Based on the statistical analysis, the RL solver shows promise as an approximate solution method for this problem class.
