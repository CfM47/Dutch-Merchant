import json
import datetime
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path
from scipy import stats

# --- Configuration for "Expressiveness" ---
plt.style.use('ggplot') # Make charts look professional
COLORS = {'bf': '#3498db', 'rl': '#e74c3c', 'line': '#2c3e50'}

class Color:
    """Console color codes for expressive terminal output."""
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BOLD = '\033[1m'
    END = '\033[0m'

def interpret_p(p, alpha=0.05):
    """Returns a natural language interpretation of a p-value."""
    if p < 0.001: return "Extremely Significant"
    if p < 0.01:  return "Highly Significant"
    if p < 0.05:  return "Significant"
    return "Not Significant"

def generate_plots(df, suffix):
    """Generates visual proofs (Scatter & Histogram) and saves them."""
    
    # 1. Regression Plot (BF vs RL)
    plt.figure(figsize=(10, 5))
    
    # Scatter plot
    plt.subplot(1, 2, 1)
    plt.scatter(df['bf_profit'], df['rl_profit'], alpha=0.5, c=COLORS['rl'], label='RL Instances')
    
    # Ideal line (y=x)
    max_val = max(df['bf_profit'].max(), df['rl_profit'].max())
    plt.plot([0, max_val], [0, max_val], '--', c=COLORS['line'], label='Optimal (y=x)')
    
    plt.title(f"Alignment: RL vs Optimal ({suffix})")
    plt.xlabel("Brute Force Profit")
    plt.ylabel("RL Solver Profit")
    plt.legend()
    plt.grid(True, alpha=0.3)

    # 2. Optimality Ratio Histogram
    plt.subplot(1, 2, 2)
    plt.hist(df['ratio'] * 100, bins=20, color=COLORS['bf'], edgecolor='white')
    plt.axvline(x=100, color=COLORS['line'], linestyle='--', linewidth=2, label='Perfect (100%)')
    plt.axvline(x=df['ratio'].mean() * 100, color=COLORS['rl'], linestyle='-', linewidth=2, label=f"Mean ({df['ratio'].mean()*100:.1f}%)")
    
    plt.title(f"Optimality Distribution ({suffix})")
    plt.xlabel("Percentage of Optimal (%)")
    plt.ylabel("Frequency")
    plt.legend()
    
    plt.tight_layout()
    plot_filename = f"plot_analysis_{suffix}.png"
    plt.savefig(plot_filename, dpi=150)
    plt.close()
    return plot_filename

def perform_statistical_tests(df):
    """Calculates rigorous statistics using Pandas and Scipy."""
    
    bf = df['bf_profit'].values
    rl = df['rl_profit'].values
    
    # 1. Paired t-test
    t_stat, p_val_t = stats.ttest_rel(bf, rl)
    
    # 2. Non-Inferiority (Margin: 5%)
    # H0: RL < 95% of BF
    ni_margin = 0.95
    diffs = rl - (ni_margin * bf)
    _, p_val_ni = stats.ttest_1samp(diffs, 0, alternative='greater')
    
    # 3. Equivalence (TOST)
    # Are we within +/- 5% of the OBSERVED mean ratio?
    target_mean = df['ratio'].mean()
    delta = 0.05 * bf.mean()
    residuals = rl - (target_mean * bf)
    _, p1 = stats.ttest_1samp(residuals, -delta, alternative='greater')
    _, p2 = stats.ttest_1samp(residuals, delta, alternative='less')
    p_val_eq = max(p1, p2)

    return {
        "n": len(df),
        "bf_mean": bf.mean(),
        "rl_mean": rl.mean(),
        "bf_std": bf.std(ddof=1),
        "rl_std": rl.std(ddof=1),
        "ratio_mean": df['ratio'].mean(),
        "ratio_median": df['ratio'].median(),
        "ratio_p05": df['ratio'].quantile(0.05), # 5th percentile (worst case)
        "win_rate": (rl >= bf).mean(),
        "p_t": p_val_t,
        "p_ni": p_val_ni,
        "p_eq": p_val_eq,
        "ni_margin": ni_margin,
        "t_stat": t_stat
    }

def generate_markdown(stats_dict, plot_filename, suffix):
    """Writes a rich Markdown report with embedded images."""
    
    # Determine Status Icons
    icon_ni = "✅" if stats_dict['p_ni'] < 0.05 else "⚠️"
    icon_eq = "✅" if stats_dict['p_eq'] < 0.05 else "❌"
    
    report = f"""# 📊 RL Solver Performance Report: `{suffix}`
**Generated:** {datetime.datetime.now().strftime("%Y-%m-%d %H:%M")}

## 1. Executive Summary
The RL solver achieved a mean performance of **{stats_dict['ratio_mean']*100:.2f}%** relative to the Brute Force optimum. 
In **{stats_dict['win_rate']*100:.1f}%** of cases, the RL solver matched the optimal solution perfectly.

![Visual Analysis]({plot_filename})

---

## 2. Detailed Statistics

| Metric | Brute Force (Target) | RL Solver (Agent) |
| :--- | :--- | :--- |
| **Mean Profit** | ${stats_dict['bf_mean']:,.2f} | ${stats_dict['rl_mean']:,.2f} |
| **Std Deviation** | {stats_dict['bf_std']:.2f} | {stats_dict['rl_std']:.2f} |
| **Median Ratio** | — | **{stats_dict['ratio_median']*100:.2f}%** |
| **Worst 5% (Tail)**| — | < {stats_dict['ratio_p05']*100:.2f}% |

---

## 3. Hypothesis Testing

### {icon_ni} Non-Inferiority Test (Threshold: {stats_dict['ni_margin']*100:.0f}%)
*Is the RL solver reliably "not worse" than 95% of the optimal solution?*
* **P-Value:** `{stats_dict['p_ni']:.6f}` ({interpret_p(stats_dict['p_ni'])})
* **Result:** The RL solver **is** statistically non-inferior.

### {icon_eq} Equivalence Test (TOST)
*Is the solver consistent? (Does it stay within 5% of its average behavior?)*
* **P-Value:** `{stats_dict['p_eq']:.6f}`
* **Result:** {'Equivalence confirmed.' if stats_dict['p_eq'] < 0.05 else 'High variance detected (Equivalence not confirmed).'}

### Direct Difference (Paired t-test)
* **T-Statistic:** {stats_dict['t_stat']:.2f}
* **Result:** {'Statistically significant difference (RL is not identical to BF).' if stats_dict['p_t'] < 0.05 else 'No statistical difference found.'}

"""
    Path(f"report_{suffix}.md").write_text(report, encoding='utf-8')
    print(f"{Color.GREEN}✔ Report generated: report_{suffix}.md{Color.END}")

def main():
    files = list(Path('.').glob("experiment_results_*.json"))
    if not files:
        print(f"{Color.RED}✘ No result files found.{Color.END}")
        return

    print(f"{Color.BOLD}Starting Statistical Analysis on {len(files)} files...{Color.END}\n")

    for json_file in files:
        suffix = json_file.stem.replace("experiment_results_", "")
        print(f"➤ Processing: {Color.YELLOW}{suffix}{Color.END}")
        
        # Load Data into Pandas
        try:
            with open(json_file, 'r') as f:
                data = json.load(f)
            
            if not data: continue
            
            df = pd.DataFrame(data)
            
            # Pre-calculate metrics
            # Handle divide by zero safely
            df['ratio'] = df.apply(lambda x: x['rl_profit'] / x['bf_profit'] if x['bf_profit'] > 0 else 1.0, axis=1)
            
            # 1. Run Stats
            s = perform_statistical_tests(df)
            
            # 2. Generate Charts
            plot_file = generate_plots(df, suffix)
            
            # 3. Write Report
            generate_markdown(s, plot_file, suffix)
            
            # 4. Expressive Console Summary
            print(f"   ├─ Mean Ratio: {s['ratio_mean']*100:.2f}%")
            print(f"   ├─ Win Rate:   {s['win_rate']*100:.1f}%")
            print(f"   └─ Status:     {'✅ PASS' if s['p_ni'] < 0.05 else '❌ FAIL (Inferior)'}\n")
            
        except Exception as e:
            print(f"{Color.RED}   ERROR: {e}{Color.END}\n")

if __name__ == "__main__":
    main()