"""
Solver class for the Dutch Merchant problem using RL.
"""


from typing import List, Optional
import torch
import torch.optim as optim
import numpy as np
from collections import deque

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
        embedding_dim: int = 128,
        hidden_dim: int = 256,
        max_steps: int = 50,
        # Training hyperparameters
        num_epochs: int = 1000,
        episodes_per_epoch: int = 10,
        learning_rate: float = 1e-4,
        baseline_decay: float = 0.99,
        # Exploration hyperparameters
        start_temp: float = 200.0,
        end_temp: float = 0.1,
        verbose: bool = True,
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
        
        self.agent: Optional[PolicyGradientAgent] = None

    def solve(self, instance: Instance) -> List[int]:
        """
        Solve a given problem instance using the RL agent.
        
        This method will:
        1. Initialize/Encode the instance.
        2. Train the agent on this specific instance.
        3. Return the best solution found (or a final greedy solution).

        Args:
            instance: The problem instance to solve.

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

        if self.verbose:
            print(f"Starting training on instance for {self.num_epochs} epochs...")

        self.agent.train()
        for epoch in range(self.num_epochs):
            # Calculate current temperature (linear decay)
            if self.num_epochs > 1:
                temp = self.start_temp + (self.end_temp - self.start_temp) * (epoch / (self.num_epochs - 1))
            else:
                temp = self.end_temp

            epoch_rewards = []
            epoch_losses = []
            
            for _ in range(self.episodes_per_epoch):
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
                
                # Update model
                loss = self.agent.compute_loss_with_baseline(reward, baseline)
                epoch_losses.append(loss.item())
                
                optimizer.zero_grad()
                loss.backward()
                optimizer.step()
                
                # Update baseline
                baseline = self.baseline_decay * baseline + (1 - self.baseline_decay) * reward
            
            if self.verbose and (epoch + 1) % 10 == 0:
                avg_reward = np.mean(epoch_rewards)
                print(f"Epoch {epoch + 1}/{self.num_epochs} | Temp: {temp:.4f} | Avg Reward: {avg_reward:.4f} | Best: {best_reward:.4f}")

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
            print(f"Final greedy score: {final_score:.4f}")
            
        # Return the better of the two? Or just the greedy one?
        # Usually returning the best seen is safer if training was unstable,
        # but let's stick to the final greedy for now unless it's much worse.
        # Actually, let's return the best one found to be safe.
        if best_reward > final_score:
             return best_solution
        return final_solution
