"""
Solver class for the Dutch Merchant problem using RL.
"""


from typing import List, Optional
import torch
import torch.optim as optim
import numpy as np
from collections import deque
import json
import os

from .schemas import Instance
from .agent import PolicyGradientAgent
from .scoring import RouteScorer


class Solver:
    """
    Solver for the Dutch Merchant problem using Policy Gradient RL.
    """

    def __init__(
        self,
        checkpoint_path: Optional[str] = None,
        device: str = "cpu",
        embedding_dim: int = 512,
        hidden_dim: int = 2024,
        max_steps: int = 10,
        # Training hyperparameters
        num_epochs: int = 100,
        episodes_per_epoch: int = 10,
        learning_rate: float = 1e-4,
        baseline_decay: float = 0.99,
        # Exploration hyperparameters
        start_temp: float = 40.0,
        end_temp: float = 0.1,
        verbose: bool = True,
        # Memory hyperparameters
        memory_size: int = 4,
        replay_frequency: int = 4,
    ):
        """
        Initialize the Solver.

        Args:
            checkpoint_path: Path to a saved model checkpoint.
            device: Device to run the model on ('cpu' or 'cuda').
            embedding_dim: Embedding dimension for the agent.
            hidden_dim: Hidden dimension for the agent.
            max_steps: Maximum steps in the solution.
            num_epochs: Number of training epochs per solve call.
            episodes_per_epoch: Number of episodes to collect per epoch.
            learning_rate: Learning rate for the optimizer.
            baseline_decay: Decay rate for the reward baseline.
            start_temp: Initial temperature for exploration.
            end_temp: Final temperature for exploitation.
            verbose: Whether to print training progress.
            memory_size: Number of top solutions to keep in memory.
            replay_frequency: Frequency (in epochs) to replay memory.
        """
        self.checkpoint_path = checkpoint_path
        self.device = device
        self.embedding_dim = embedding_dim
        self.hidden_dim = hidden_dim
        self.max_steps = max_steps
        self.num_epochs = num_epochs
        self.episodes_per_epoch = episodes_per_epoch
        self.learning_rate = learning_rate
        self.baseline_decay = baseline_decay
        self.start_temp = start_temp
        self.end_temp = end_temp
        self.verbose = verbose
        self.memory_size = memory_size
        self.replay_frequency = replay_frequency
        
        self.agent: Optional[PolicyGradientAgent] = None

    def solve(self, instance: Instance, log_path: Optional[str] = None, extra_data: Optional[dict] = None) -> List[int]:
        """
        Solve a given problem instance using the RL agent.
        
        This method will:
        1. Initialize/Encode the instance.
        2. Train the agent on this specific instance.
        3. Return the best solution found (or a final greedy solution).

        Args:
            instance: The problem instance to solve.
            log_path: Optional path to save training history (rewards and solutions) as a JSON file.
            extra_data: Optional dictionary to save along with the logs (e.g. brute force optimal).

        Returns:
            A list of port IDs representing the solution path.
        """
        # Initialize agent if not created or if dimensions change
        if (
            self.agent is None
            or self.agent.n_ports != instance.n_ports
            or self.agent.n_goods != instance.n_goods
        ):
            self.agent = PolicyGradientAgent(
                n_ports=instance.n_ports,
                n_goods=instance.n_goods,
                embedding_dim=self.embedding_dim,
                hidden_dim=self.hidden_dim,
                max_steps=self.max_steps,
            ).to(self.device)

            if self.checkpoint_path:
                try:
                    checkpoint = torch.load(
                        self.checkpoint_path, map_location=self.device
                    )
                    # Handle both full checkpoint dict and just state_dict
                    if isinstance(checkpoint, dict) and "model_state_dict" in checkpoint:
                        self.agent.load_state_dict(checkpoint["model_state_dict"])
                    else:
                        self.agent.load_state_dict(checkpoint)
                except Exception as e:
                    print(f"Warning: Failed to load checkpoint from {self.checkpoint_path}: {e}")
                    print("Using initialized weights.")

        
        # --- Training Loop ---
        optimizer = optim.Adam(self.agent.parameters(), lr=self.learning_rate)
        scorer = RouteScorer(instance)
        baseline = 0.0
        
        # We can track the best solution found during training
        best_reward = float('-inf')
        best_solution: List[int] = []
        top_solutions = [] # List of (reward, solution) for memory replay
        
        training_history = []

        if self.verbose:
            print(f"Starting training on instance for {self.num_epochs} epochs...")

        self.agent.train()
        for epoch in range(self.num_epochs):
            # Calculate current temperature (exponential decay)
            if self.num_epochs > 1:
                progress = epoch / (self.num_epochs - 1)
                temp = self.start_temp * ((self.end_temp / self.start_temp) ** progress)
            else:
                temp = self.end_temp

            epoch_rewards = []
            epoch_losses = []
            
            for episode_idx in range(self.episodes_per_epoch):
                # Set the instance for the agent (re-compute features to create fresh graph)
                self.agent.receive_instance(instance)

                # Generate a solution (exploration enabled)
                solution = self.agent.generate_solution(
                    greedy=False,
                    return_log_probs=True,
                    temperature=temp,
                )
                
                # Calculate reward
                reward = scorer.score_route(solution)
                epoch_rewards.append(reward)
                
                # Track best
                if reward > best_reward:
                    best_reward = reward
                    best_solution = solution[:]
                
                # Update memory
                # Only add if not duplicate path (simple check)
                is_duplicate = any(x[1] == solution for x in top_solutions)
                if not is_duplicate:
                    if len(top_solutions) < self.memory_size:
                        top_solutions.append((reward, solution[:]))
                        top_solutions.sort(key=lambda x: x[0], reverse=True)
                    elif reward > top_solutions[-1][0]:
                        top_solutions.pop()
                        top_solutions.append((reward, solution[:]))
                        top_solutions.sort(key=lambda x: x[0], reverse=True)
                
                # Log history if requested
                if log_path:
                    training_history.append({
                        "epoch": epoch,
                        "episode": episode_idx,
                        "reward": float(reward),
                        "solution": [int(x) for x in solution] # ensure json serializable
                    })
                
                # Update model
                loss = self.agent.compute_loss_with_baseline(reward, baseline)
                epoch_losses.append(loss.item())
                
                optimizer.zero_grad()
                loss.backward()
                optimizer.step()
                
                # Update baseline
                baseline = self.baseline_decay * baseline + (1 - self.baseline_decay) * reward
            
            # Replay Memory - reinforce the best solutions found
            if (epoch + 1) % self.replay_frequency == 0 and top_solutions:
                replay_losses = []
                for mem_reward, mem_sol in top_solutions:
                     # Need to re-encode instance to create fresh graph for backprop
                     self.agent.receive_instance(instance)
                     
                     # Re-run to get fresh log_probs for the stored solution
                     self.agent.generate_solution(
                         greedy=False,
                         return_log_probs=True,
                         temperature=1.0,  # Use temperature=1 for stable replay
                         forced_solution=mem_sol
                     )
                     
                     # Skip if no log_probs were saved (solution too short)
                     if not self.agent.saved_log_probs:
                         continue
                     
                     # Update model with stored reward
                     loss = self.agent.compute_loss_with_baseline(mem_reward, baseline)
                     replay_losses.append(loss.item())
                     
                     optimizer.zero_grad()
                     loss.backward()
                     optimizer.step()
                     
                     # Update baseline with replayed reward
                     baseline = self.baseline_decay * baseline + (1 - self.baseline_decay) * mem_reward
                
                if self.verbose and replay_losses:
                    print(f"  [Memory Replay] Replayed {len(replay_losses)} solutions, top reward: {top_solutions[0][0]:.4f}")
            
            if self.verbose and (epoch + 1) % 10 == 0:
                avg_reward = np.mean(epoch_rewards)
                print(f"Epoch {epoch + 1}/{self.num_epochs} | Temp: {temp:.4f} | Avg Reward: {avg_reward:.4f} | Best: {best_reward:.4f}")

        # Save history if requested
        if log_path:
            try:
                # Ensure directory exists
                os.makedirs(os.path.dirname(log_path), exist_ok=True)
                
                output_data = {
                    "history": training_history,
                    "extra_data": extra_data or {}
                }
                
                with open(log_path, 'w') as f:
                    json.dump(output_data, f, indent=2)
                if self.verbose:
                    print(f"Training history saved to {log_path}")
            except Exception as e:
                print(f"Error saving training history: {e}")

        # --- Inference ---
        # We could return best_solution found during training, 
        # or run one final greedy pass with the trained model.
        # Often the greedy pass on the trained model is stable.
        
        self.agent.eval()
        with torch.no_grad():
            final_solution = self.agent.generate_solution(greedy=True)
            final_score = scorer.score_route(final_solution)
        
        if self.verbose:
            print(f"Training complete. Best training score: {best_reward:.4f}")
            
        # Return the better of the two? Or just the greedy one?
        # Usually returning the best seen is safer if training was unstable,
        # but let's stick to the final greedy for now unless it's much worse.
        # Actually, let's return the best one found to be safe.
        if best_reward > final_score:
             return best_solution
        return final_solution
