"""
Scoring interface for Dutch Merchant solutions.

Two functions for external use:
1. get_problem_instance() - returns a problem instance
2. score_route(nodes) - returns a score for a list of nodes
"""

from typing import List
from schemas import Instance

class RouteScorer:
    """
    Scoring class for Dutch Merchant solutions.
    """
    
    def __init__(self, instance: Instance):
        """
        Initialize the scorer with a problem instance.
        
        Args:
            instance: The problem instance to use for scoring.
        """
        self.instance = instance

    def score_route(self, nodes: List[int]) -> float:
        """
        Calculate the score for a given sequence of nodes (ports).
        
        Args:
            nodes: A list of port IDs representing the route/solution.
            
        Returns:
            A float representing the score of the solution.
        """
        pass
