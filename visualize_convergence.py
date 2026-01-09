
import os
import json
import random
import matplotlib.pyplot as plt
import numpy as np
from typing import List

from rl_model.solver import Solver
from rl_model.schemas import Instance

def generate_random_instance(
    n_ports: int = 5,
    n_goods: int = 2,
    seed: int = 42
) -> Instance:
    random.seed(seed)
    np.random.seed(seed)
    
    # Generate random symmetric travel times
    coords = np.random.rand(n_ports, 2) * 100
    travel_time = np.zeros((n_ports, n_ports))
    for i in range(n_ports):
        for j in range(i + 1, n_ports):
            dist = np.linalg.norm(coords[i] - coords[j]) / 10.0 # Scale down
            travel_time[i][j] = dist
            travel_time[j][i] = dist
            
    travel_time = travel_time.tolist()
    
    travel_cost = 1.0
    weight = np.random.uniform(0.5, 2.0, n_goods).tolist()
    
    # Goods mechanics
    # To make it interesting, prices should allow arbitrage
    base_prices = np.random.uniform(10, 50, n_goods)
    buy_price = []
    sell_price = []
    
    for _ in range(n_ports):
        # Variations per port
        p_buy = [p * np.random.uniform(0.8, 1.2) for p in base_prices]
        p_sell = [p * np.random.uniform(0.7, 1.3) for p in base_prices] # Sometimes sell higher, sometimes lower
        buy_price.append(p_buy)
        sell_price.append(p_sell)
        
    buy_cap = np.random.uniform(5, 20, (n_ports, n_goods)).tolist()
    sell_cap = np.random.uniform(5, 20, (n_ports, n_goods)).tolist()
    
    visit_cost = np.random.uniform(0, 2.0, n_ports).tolist()
    
    return Instance(
        n_ports=n_ports,
        n_goods=n_goods,
        travel_time=travel_time,
        travel_cost=travel_cost,
        weight=weight,
        buy_price=buy_price,
        sell_price=sell_price,
        buy_cap=buy_cap,
        sell_cap=sell_cap,
        visit_cost=visit_cost,
        start_port=0,
        capacity=20.0,
        time_limit=30.0,
        initial_capital=100.0,
    )

def plot_convergence(log_dir: str):
    log_files = [f for f in os.listdir(log_dir) if f.endswith('.json')]
    
    for log_file in log_files:
        path = os.path.join(log_dir, log_file)
        with open(path, 'r') as f:
            history = json.load(f)
            
        # Organize data
        epochs = []
        avg_rewards = []
        max_rewards = []
        
        current_epoch = -1
        epoch_r = []
        
        # history is flat list of episodes
        # We need to aggregate by epoch
        
        epoch_data = {}
        
        for entry in history:
            ep = entry['epoch']
            r = entry['reward']
            if ep not in epoch_data:
                epoch_data[ep] = []
            epoch_data[ep].append(r)
            
        sorted_epochs = sorted(epoch_data.keys())
        epochs = sorted_epochs
        avg_rewards = [np.mean(epoch_data[e]) for e in sorted_epochs]
        max_rewards = [np.max(epoch_data[e]) for e in sorted_epochs]
        
        # Plot
        plt.figure(figsize=(10, 6))
        plt.plot(epochs, avg_rewards, label='Average Reward', marker='o')
        plt.plot(epochs, max_rewards, label='Max Reward', linestyle='--', marker='x')
        
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
    log_dir = "results/convergence_logs"
    os.makedirs(log_dir, exist_ok=True)
    
    # Define test instances
    instances_config = [
        ("small", 5, 2, 42),
        ("medium", 10, 4, 123),
        ("large", 15, 5, 999),
    ]
    
    solver = Solver(
        embedding_dim=64,
        hidden_dim=128,
        num_epochs=30,
        episodes_per_epoch=10,
        learning_rate=1e-3,
        verbose=True
    )
    
    for name, n_p, n_g, seed in instances_config:
        print(f"\nProcessing instance: {name}")
        instance = generate_random_instance(n_p, n_g, seed)
        
        log_path = os.path.join(log_dir, f"{name}_convergence.json")
        solver.solve(instance, log_path=log_path)
        
    print("\nStarting plotting...")
    plot_convergence(log_dir)
    print("Done!")

if __name__ == "__main__":
    main()
