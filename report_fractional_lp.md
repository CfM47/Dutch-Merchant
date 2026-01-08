# 📊 RL Solver Performance Report: `fractional_lp`
**Generated:** 2026-01-08 12:17

## 1. Executive Summary
The RL solver achieved a mean performance of **97.29%** relative to the Brute Force optimum. 
In **46.2%** of cases, the RL solver matched the optimal solution perfectly.

![Visual Analysis](plot_analysis_fractional_lp.png)

---

## 2. Detailed Statistics

| Metric | Brute Force (Target) | RL Solver (Agent) |
| :--- | :--- | :--- |
| **Mean Profit** | $7,465.78 | $7,236.25 |
| **Std Deviation** | 2677.01 | 2536.36 |
| **Median Ratio** | — | **99.08%** |
| **Worst 5% (Tail)**| — | < 89.85% |

---

## 3. Hypothesis Testing

### ✅ Non-Inferiority Test (Threshold: 95%)
*Is the RL solver reliably "not worse" than 95% of the optimal solution?*
* **P-Value:** `0.000000` (Extremely Significant)
* **Result:** The RL solver **is** statistically non-inferior.

### ✅ Equivalence Test (TOST)
*Is the solver consistent? (Does it stay within 5% of its average behavior?)*
* **P-Value:** `0.000000`
* **Result:** Equivalence confirmed.

### Direct Difference (Paired t-test)
* **T-Statistic:** 15.54
* **Result:** Statistically significant difference (RL is not identical to BF).

