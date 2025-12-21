"""
Policy Gradient RL Agent for the Dutch Merchant problem.

Uses REINFORCE algorithm to learn to generate port visit sequences.
"""

from typing import List, Tuple, Optional
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.distributions import Categorical

from schemas import Instance, get_problem_instance



class PolicyNetwork(nn.Module):
    """Neural network that outputs a probability distribution over ports."""
    
    def __init__(self, n_ports: int, embedding_dim: int = 128, hidden_dim: int = 256):
        super().__init__()
        self.n_ports = n_ports
        self.embedding_dim = embedding_dim
        
        self.port_embedding = nn.Linear(n_ports, embedding_dim)
        self.context_encoder = nn.Sequential(
            nn.Linear(embedding_dim * 2, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.ReLU(),
        )
        self.decoder = nn.Sequential(
            nn.Linear(hidden_dim + embedding_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, n_ports),
        )
    
    def forward(
        self, 
        instance_features: torch.Tensor,
        current_port: int,
        mask: Optional[torch.Tensor] = None,
    ) -> torch.Tensor:
        current_one_hot = F.one_hot(
            torch.tensor([current_port]), 
            num_classes=self.n_ports
        ).float()
        current_embed = self.port_embedding(current_one_hot)
        context = self.context_encoder(instance_features)
        combined = torch.cat([context, current_embed], dim=-1)
        logits = self.decoder(combined)
        
        if mask is not None:
            logits = logits.masked_fill(mask, float('-inf'))
        
        return logits


class PolicyGradientAgent(nn.Module):
    """Policy Gradient Agent using REINFORCE."""
    
    def __init__(
        self,
        n_ports: int,
        n_goods: int,
        embedding_dim: int = 128,
        hidden_dim: int = 256,
        max_steps: int = 50,
    ):
        super().__init__()
        
        self.n_ports = n_ports
        self.n_goods = n_goods
        self.embedding_dim = embedding_dim
        self.max_steps = max_steps
        
        self.instance_encoder = nn.Sequential(
            nn.Linear(n_ports * n_ports + n_goods + n_ports * n_goods * 4 + n_ports + 4, 
                      hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, embedding_dim * 2),
        )
        
        self.policy = PolicyNetwork(n_ports, embedding_dim, hidden_dim)
        self.current_instance: Optional[Instance] = None
        self.instance_features: Optional[torch.Tensor] = None
        self.saved_log_probs: List[torch.Tensor] = []
    
    def _encode_instance(self, instance: Instance) -> torch.Tensor:
        features = []
        for row in instance.travel_time:
            features.extend(row)
        features.extend(instance.weight)
        for row in instance.buy_price:
            features.extend(row)
        for row in instance.sell_price:
            features.extend(row)
        for row in instance.buy_cap:
            features.extend(row)
        for row in instance.sell_cap:
            features.extend(row)
        features.extend(instance.visit_cost)
        features.extend([
            float(instance.start_port),
            instance.capacity,
            instance.time_limit,
            instance.initial_capital,
        ])
        return torch.tensor([features], dtype=torch.float32)
    
    def receive_instance(self, instance: Instance) -> None:
        """Load a problem instance."""
        self.current_instance = instance
        if self.n_ports != instance.n_ports or self.n_goods != instance.n_goods:
            self.n_ports = instance.n_ports
            self.n_goods = instance.n_goods
        raw_features = self._encode_instance(instance)
        self.instance_features = self.instance_encoder(raw_features)
    
    def select_action(
        self, 
        current_port: int,
        mask: Optional[torch.Tensor] = None,
        greedy: bool = False,
    ) -> Tuple[int, Optional[torch.Tensor]]:
        """Select next port to visit."""
        if self.instance_features is None:
            raise ValueError("No instance loaded. Call receive_instance first.")
        
        logits = self.policy(self.instance_features, current_port, mask)
        
        if greedy:
            action = logits.argmax(dim=-1)
            return action.item(), None
        else:
            probs = F.softmax(logits, dim=-1)
            dist = Categorical(probs)
            action = dist.sample()
            log_prob = dist.log_prob(action)
            return action.item(), log_prob
    
    def generate_solution(
        self, 
        greedy: bool = False,
        return_log_probs: bool = False,
    ) -> List[int]:
        """Generate a complete solution (sequence of ports)."""
        if self.current_instance is None:
            raise ValueError("No instance loaded. Call receive_instance first.")
        
        solution = [self.current_instance.start_port]
        current_port = self.current_instance.start_port
        visited = set([current_port])
        
        if return_log_probs:
            self.saved_log_probs = []
        
        for step in range(self.max_steps):
            mask = torch.zeros(1, self.n_ports, dtype=torch.bool)
            for port in visited:
                mask[0, port] = True
            
            if mask.all():
                break
            
            next_port, log_prob = self.select_action(current_port, mask, greedy)
            
            if return_log_probs and log_prob is not None:
                self.saved_log_probs.append(log_prob)
            
            solution.append(next_port)
            current_port = next_port
            visited.add(next_port)
        
        return solution
    
    def compute_loss(self, reward: float) -> torch.Tensor:
        """Compute REINFORCE loss."""
        if not self.saved_log_probs:
            raise ValueError("No log probs saved.")
        
        policy_loss = [-log_prob * reward for log_prob in self.saved_log_probs]
        loss = torch.stack(policy_loss).sum()
        self.saved_log_probs = []
        return loss
    
    def compute_loss_with_baseline(self, reward: float, baseline: float = 0.0) -> torch.Tensor:
        """Compute REINFORCE loss with baseline."""
        if not self.saved_log_probs:
            raise ValueError("No log probs saved.")
        
        advantage = reward - baseline
        policy_loss = [-log_prob * advantage for log_prob in self.saved_log_probs]
        loss = torch.stack(policy_loss).sum()
        self.saved_log_probs = []
        return loss
