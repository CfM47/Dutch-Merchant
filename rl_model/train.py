"""
Training script for the Policy Gradient RL model.
"""

from typing import List
import torch
import torch.optim as optim
from collections import deque
import numpy as np

from schemas import Instance, get_problem_instance
from scoring import RouteScorer
from agent import PolicyGradientAgent


def train(
    instances: List[Instance],
    num_epochs: int = 100,
    episodes_per_epoch: int = 10,
    learning_rate: float = 1e-4,
    baseline_decay: float = 0.99,
    max_steps: int = 50,
    device: str = "cpu",
    verbose: bool = True,
) -> PolicyGradientAgent:
    """
    Train the Policy Gradient agent using REINFORCE.
    
    Args:
        instances: List of Instance objects for training.
        num_epochs: Number of training epochs.
        episodes_per_epoch: Number of episodes per instance per epoch.
        learning_rate: Learning rate for the optimizer.
        baseline_decay: Exponential decay for the running baseline.
        max_steps: Maximum steps in a solution.
        device: Device to train on ('cpu' or 'cuda').
        verbose: Whether to print training progress.
        
    Returns:
        The trained PolicyGradientAgent.
    """
    if not instances:
        raise ValueError("No instances provided for training.")
    
    first_instance = instances[0]
    agent = PolicyGradientAgent(
        n_ports=first_instance.n_ports,
        n_goods=first_instance.n_goods,
        max_steps=max_steps,
    ).to(device)
    
    optimizer = optim.Adam(agent.parameters(), lr=learning_rate)
    baseline = 0.0
    all_rewards = deque(maxlen=100)
    best_reward = float('-inf')
    best_solution = None
    
    for epoch in range(num_epochs):
        epoch_rewards = []
        epoch_losses = []
        
        for instance in instances:
            scorer = RouteScorer(instance)
            for episode in range(episodes_per_epoch):
                agent.receive_instance(instance)
                
                solution = agent.generate_solution(
                    greedy=False,
                    return_log_probs=True,
                )
                
                # Use score_route from scoring module
                reward = scorer.score_route(solution)
                epoch_rewards.append(reward)
                all_rewards.append(reward)
                
                if reward > best_reward:
                    best_reward = reward
                    best_solution = solution.copy()
                
                loss = agent.compute_loss_with_baseline(reward, baseline)
                epoch_losses.append(loss.item())
                
                optimizer.zero_grad()
                loss.backward()
                optimizer.step()
                
                baseline = baseline_decay * baseline + (1 - baseline_decay) * reward
        
        if verbose:
            avg_reward = np.mean(epoch_rewards)
            avg_loss = np.mean(epoch_losses)
            print(f"Epoch {epoch + 1}/{num_epochs} | "
                  f"Avg Reward: {avg_reward:.4f} | "
                  f"Avg Loss: {avg_loss:.4f} | "
                  f"Best: {best_reward:.4f}")
    
    if verbose:
        print(f"\nTraining complete!")
        print(f"Best reward: {best_reward:.4f}")
        print(f"Best solution: {best_solution}")
    
    return agent


def evaluate(
    agent: PolicyGradientAgent,
    instances: List[Instance],
    greedy: bool = True,
) -> List[dict]:
    """
    Evaluate the trained agent on a set of instances.
    
    Args:
        agent: Trained PolicyGradientAgent.
        instances: List of instances to evaluate on.
        greedy: Whether to use greedy action selection.
        
    Returns:
        List of dicts with 'instance_idx', 'solution', and 'score'.
    """
    results = []
    
    agent.eval()
    with torch.no_grad():
        for idx, instance in enumerate(instances):
            agent.receive_instance(instance)
            scorer = RouteScorer(instance)
            solution = agent.generate_solution(greedy=greedy)
            score = scorer.score_route(solution)
            
            results.append({
                'instance_idx': idx,
                'solution': solution,
                'score': score,
            })
    
    return results


if __name__ == "__main__":
    # Get a problem instance using the scoring interface
    instance = get_problem_instance()
    
    if instance is not None:
        trained_agent = train(
            instances=[instance],
            num_epochs=10,
            episodes_per_epoch=5,
            verbose=True,
        )
        
        results = evaluate(trained_agent, [instance])
        print(f"\nEvaluation results: {results}")
    else:
        print("No problem instance available. Implement get_problem_instance().")
