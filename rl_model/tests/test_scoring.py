from ..schemas import Instance
from ..scoring import EvaluatorName, RouteScorer


def test_score_route():
    """
    Tests the basic scoring of a simple route.
    """
    instance = Instance(
        n_goods=1,
        n_ports=2,
        travel_time=[[0.0, 1.0], [1.0, 0.0]],
        travel_cost=1,
        weight=[1.0],
        buy_price=[[1.0], [10.0]],
        sell_price=[[0.0], [6.0]],
        buy_cap=[[2.0], [0.0]],
        sell_cap=[[0.0], [10.0]],
        visit_cost=[0.0, 0.0],
        start_port=0,
        capacity=2.0,
        time_limit=10.0,
        initial_capital=10.0,
    )
    scorer = RouteScorer(instance)

    # Replicating the logic from main.py's assertion
    expected_score = (6 - 1) * 2 - 2 + 10
    assert (
        scorer.score_route([0, 1, 0], EvaluatorName.IntervalEvaluator) == expected_score
    )
    assert (
        scorer.score_route([0, 1, 0], EvaluatorName.InfiniteCapacityDebtEvaluator)
        == expected_score
    )
    assert scorer.score_route(([0, 1, 0])) == expected_score
