
import os
import json
import matplotlib.pyplot as plt
import numpy as np

def plot_convergence(log_dir: str, output_dir: str = None):
    if output_dir is None:
        output_dir = log_dir

    if not os.path.exists(log_dir):
        print(f"Directory {log_dir} does not exist.")
        return

    log_files = [f for f in os.listdir(log_dir) if f.endswith('.json')]
    log_files.sort()
    log_files = log_files[-9:] + log_files[:-9]
    
    if not log_files:
        print(f"No .json log files found in {log_dir}")
        return

    for i, log_file in enumerate(log_files):
        path = os.path.join(log_dir, log_file)
        try:
            with open(path, 'r') as f:
                data = json.load(f)
                
            # Handle new format vs old format
            if isinstance(data, dict) and "history" in data:
                history = data["history"]
                extra_data = data.get("extra_data", {})
            else:
                history = data
                extra_data = {}
                
        except json.JSONDecodeError:
            print(f"Error reading {log_file}, skipping.")
            continue
            
        # Organize data
        epoch_data = {}
        
        for entry in history:
            ep = entry.get('epoch')
            r = entry.get('reward')
            if ep is None or r is None:
                continue
                
            if ep not in epoch_data:
                epoch_data[ep] = []
            epoch_data[ep].append(r)
            
        if not epoch_data:
            print(f"No valid data in {log_file}, skipping.")
            continue

        sorted_epochs = sorted(epoch_data.keys())
        epochs = sorted_epochs
        avg_rewards = [np.mean(epoch_data[e]) for e in sorted_epochs]
        max_rewards = [np.max(epoch_data[e]) for e in sorted_epochs]
        
        # Plot
        plt.figure(figsize=(10, 6))
        plt.rcParams.update({
            "font.family": "serif",
        })

        plt.plot(epochs, avg_rewards, label='Recompensa Promedio', linewidth=1.5)
        plt.plot(epochs, max_rewards, label='Recompensa máxima', linestyle='--', linewidth=1.2)
        
        # Plot Brute Force Optimal if available
        if "bf_profit" in extra_data:
            bf_profit = extra_data["bf_profit"]
            plt.axhline(y=bf_profit,  linestyle=':', label=f'Solución Optima (BF): ({bf_profit:.2f})')
        
        plt.title('Análisis de Convergencia')
        plt.xlabel('Época')
        plt.ylabel('Recompensa')
        plt.legend(frameon=False, loc="best")
        plt.grid(True, which="both", linestyle=":", linewidth=0.5)

        plot_path = os.path.join(output_dir, log_file.replace('.json', '.eps'))
        plt.savefig(plot_path, format='eps')
        print(f"Saved plot in eps to {plot_path}")

        plt.close()

def main():
    log_dir = "results/logs"
    output_dir = "report"
    print(f"Plotting results from {log_dir}...")
    plot_convergence(log_dir, output_dir)
    print("Done!")

if __name__ == "__main__":
    main()
