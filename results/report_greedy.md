# 📊 RL Solver Performance Report: `greedy`
**Generated:** 2026-01-08 12:17

## 1. Executive Summary
The RL solver achieved a mean performance of **99.39%** relative to the Brute Force optimum. 
In **43.0%** of cases, the RL solver matched the optimal solution perfectly.

![Visual Analysis](plot_analysis_greedy.png)

---

## 2. Detailed Statistics

| Metric | Brute Force (Target) | RL Solver (Agent) |
| :--- | :--- | :--- |
| **Mean Profit** | $11,766.01 | $11,681.60 |
| **Std Deviation** | 4145.03 | 4077.11 |
| **Median Ratio** | — | **99.99%** |
| **Worst 5% (Tail)**| — | < 96.94% |

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
* **T-Statistic:** 4.16
* **Result:** Statistically significant difference (RL is not identical to BF).

