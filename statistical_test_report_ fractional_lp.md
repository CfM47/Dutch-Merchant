# Statistical Test Report: Brute Force vs RL Solver
Generated on: 2025-12-22 10:01:28

## Experiment Configuration
- **Number of Cases**: 500
- **Ports Range**: (5, 9)
- **Goods Range**: (3, 6)
- **Time Limit Range**: (80.0, 150.0)
- **RL Hyperparameters**: 50 epochs, 5 episodes/epoch

## Descriptive Statistics

| Metric | Brute Force | RL Solver |
| :--- | :--- | :--- |
| **Mean Profit** | 7465.78 | 7236.25 |
| **Std Dev** | 2677.01 | 2536.36 |
| **Mean Difference (BF - RL)** | 229.53 | - |

## Ratio Analysis (RL/BF)

| Metric | Value |
| :--- | :--- |
| **Mean Ratio** | 0.9729 (97.29%) |
| **Median Ratio** | 0.9908 (99.08%) |
| **Std Dev** | 0.0365 |
| **95% Confidence Interval** | [0.9697, 0.9761] |
| **RL Win/Tie Rate** | 46.20% |

## Statistical Tests

### 1. Paired t-test (BF vs RL)
**Null Hypothesis**: There is no difference between BF and RL mean profits.

- **t-statistic**: 15.5401
- **p-value**: 0.000000
- **Result**: Reject H0 at α = 0.05
- **Interpretation**: The difference is statistically significant

### 2. Effect Size (Cohen's d)
- **Cohen's d**: 0.6950
- **Interpretation**: Medium effect

### 3. Wilcoxon Signed-Rank Test (Non-parametric)
- **Test Statistic**: 0.0000
- **p-value**: 0.000000
- **Result**: Reject H0 at α = 0.05

### 4. One-Sample t-test on Ratio
**Null Hypothesis**: Mean ratio equals 1.0 (RL performs equally to BF).

- **t-statistic**: -16.6364
- **p-value**: 0.000000
- **Result**: Reject H0 at α = 0.05
- **Interpretation**: RL performance differs significantly from optimal

## Conclusions

1. **Performance Gap**: On average, the RL solver achieves 97.29% of the brute-force optimal solution.

2. **Consistency**: The 95% confidence interval for the ratio is [96.97%, 97.61%], indicating high consistency.

3. **Practical Significance**: The RL solver wins or ties in 46.20% of cases, demonstrating limited competitive performance.

4. **Statistical Significance**: The paired t-test confirms a significant difference between methods (p < 0.05).

## Recommendations

Based on the statistical analysis, the RL solver shows promise as an approximate solution method for this problem class.
