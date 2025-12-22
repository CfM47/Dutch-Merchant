
from unittest.mock import MagicMock
from rl_model import Solver
# from rl_model.schemas import get_problem_instance

def get_mock_instance():
    instance = MagicMock()
    instance.n_ports = 5
    instance.n_goods = 3
    # Create valid data structures that won't crash the encoder
    instance.travel_time = [[1.0] * 5 for _ in range(5)]
    instance.weight = [1.0] * 3
    instance.buy_price = [[10.0] * 3 for _ in range(5)]
    instance.sell_price = [[20.0] * 3 for _ in range(5)]
    instance.buy_cap = [[5] * 3 for _ in range(5)]
    instance.sell_cap = [[5] * 3 for _ in range(5)]
    instance.visit_cost = [1.0] * 5
    instance.start_port = 0
    instance.capacity = 50.0
    instance.time_limit = 100.0
    instance.initial_capital = 1000.0
    return instance

if __name__ == "__main__":
    # Initialize the solver
    solver = Solver()
    print("Solver initialized.")

    # Get a problem instance
    # instance = get_problem_instance()
    instance = get_mock_instance()
    print(f"Loaded instance with {instance.n_ports} ports and {instance.n_goods} goods.")

    # Solve the instance
    solution = solver.solve(instance)
    
    # Print the result
    print(f"Solution: {solution}")
