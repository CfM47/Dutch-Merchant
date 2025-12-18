"""
Scoring interface for Dutch Merchant solutions.

Two functions for external use:
1. get_problem_instance() - returns a problem instance
2. score_route(nodes) - returns a score for a list of nodes
"""

from typing import List

def score_route(nodes: List[int]) -> float:
    """
    Calculate the score for a given sequence of nodes (ports).
    
    Args:
        nodes: A list of port IDs representing the route/solution.
        
    Returns:
        A float representing the score of the solution.
    """
    pass
