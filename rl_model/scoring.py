"""
Scoring interface for Dutch Merchant solutions.

Two functions for external use:
1. get_problem_instance() - returns a problem instance
2. score_route(nodes) - returns a score for a list of nodes
"""

from enum import StrEnum
from typing import List, Tuple
from .schemas import Instance
from dm_solution import PathEvaluator


class EvaluatorName(StrEnum):
    LpProfitCalculator = "LpProfitCalculator"
    IntervalEvaluator = "IntervalEvaluator"
    InfiniteCapacityDebtEvaluator = "InfiniteCapacityDebtEvaluator"


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
        self.evaluator = PathEvaluator(instance.model_dump_json())

    def score_route(
        self,
        nodes: List[int],
        evaluator_name: EvaluatorName = EvaluatorName.LpProfitCalculator,  # more general case
    ) -> float:
        """
        Calculate the score for a given sequence of nodes (ports).

        Args:
            nodes: A list of port IDs representing the route/solution.

        Returns:
            A float representing the score of the solution.
        """
        answ: Tuple[float, List[List[Tuple[float, float]]]] = (
            self.evaluator.score_route(nodes, evaluator_name.value)
        )
        return answ[0]  # Return only the score part
