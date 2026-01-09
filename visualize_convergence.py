
import os
import json
import matplotlib.pyplot as plt
import numpy as np

def plot_convergence(log_dir: str):
    if not os.path.exists(log_dir):
        print(f"Directory {log_dir} does not exist.")
        return

    log_files = [f for f in os.listdir(log_dir) if f.endswith('.json')]
    
    if not log_files:
        print(f"No .json log files found in {log_dir}")
        return

    for log_file in log_files:
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
        plt.plot(epochs, avg_rewards, label='Average Reward', marker='o')
        plt.plot(epochs, max_rewards, label='Max Reward', linestyle='--', marker='x')
        
        # Plot Brute Force Optimal if available
        if "bf_profit" in extra_data:
            bf_profit = extra_data["bf_profit"]
            plt.axhline(y=bf_profit, color='r', linestyle='-', label=f'Brute Force ({bf_profit:.2f})')
        
        plt.title(f'Convergence Analysis - {log_file}')
        plt.xlabel('Epoch')
        plt.ylabel('Reward')
        plt.legend()
        plt.grid(True)
        
        plot_path = os.path.join(log_dir, log_file.replace('.json', '.png'))
        plt.savefig(plot_path)
        print(f"Saved plot to {plot_path}")
        plt.close()

def main():
    log_dir = "results/logs"
    print(f"Plotting results from {log_dir}...")
    plot_convergence(log_dir)
    print("Done!")

if __name__ == "__main__":
    main()
