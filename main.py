
from rl_model import Solver
from rl_model.schemas import Instance


def create_test_instance():
    """Create a test instance for the Dutch Merchant problem."""
    return Instance(
        n_ports=4,
        n_goods=2,
        travel_time=[
            [0.0, 1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0, 1.0],
            [1.0, 1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0, 0.0],
        ],
        travel_cost=0.0,
        weight=[1.0, 2.0],
        buy_price=[
            [10.0, 200.0],
            [0.0, 0.0],
            [0.0, 200.0],
            [0.0, 0.0],
        ],
        sell_price=[
            [0.0, 0.0],
            [20.0, 0.0],
            [0.0, 0.0],
            [0.0, 1000.0],
        ],
        buy_cap=[
            [10.0, 5.0],
            [0.0, 0.0],
            [0.0, 5.0],
            [0.0, 0.0],
        ],
        sell_cap=[
            [0.0, 0.0],
            [10.0, 0.0],
            [0.0, 0.0],
            [0.0, 5.0],
        ],
        visit_cost=[0.0, 0.0, 0.0, 0.0],
        start_port=0,
        capacity=10.0,
        time_limit=100.0,
        initial_capital=10105.0,
    )

if __name__ == "__main__":
    # Initialize the solver
    solver = Solver()
    print("Solver initialized.")

    # Get a problem instance
    instance = create_test_instance()
    print(f"Loaded instance with {instance.n_ports} ports and {instance.n_goods} goods.")

    # Solve the instance
    solution = solver.solve(instance)
    
    # Print the result
    print(f"Solution: {solution}")
