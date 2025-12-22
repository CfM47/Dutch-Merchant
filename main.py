
from rl_model import Solver
from rl_model.schemas import Instance


def create_test_instance():
    """Create a test instance for the Dutch Merchant problem."""
    return Instance(
        n_ports=5,
        n_goods=2,
        travel_time=[
            [0.0, 10.0, 15.0, 20.0, 25.0],
            [10.0, 0.0, 12.0, 18.0, 22.0],
            [15.0, 12.0, 0.0, 10.0, 15.0],
            [20.0, 18.0, 10.0, 0.0, 10.0],
            [25.0, 22.0, 15.0, 10.0, 0.0],
        ],
        travel_cost=0.1,
        weight=[1.0, 2.0],
        buy_price=[
            [100.0, 200.0],
            [50.0, 180.0],
            [150.0, 250.0],
            [80.0, 210.0],
            [120.0, 230.0],
        ],
        sell_price=[
            [90.0, 190.0],
            [45.0, 170.0],
            [140.0, 240.0],
            [75.0, 200.0],
            [110.0, 220.0],
        ],
        buy_cap=[
            [10.0, 5.0],
            [10.0, 5.0],
            [10.0, 5.0],
            [10.0, 5.0],
            [10.0, 5.0],
        ],
        sell_cap=[
            [10.0, 5.0],
            [10.0, 5.0],
            [10.0, 5.0],
            [10.0, 5.0],
            [10.0, 5.0],
        ],
        visit_cost=[0.0, 0.0, 0.0, 0.0, 0.0],
        start_port=0,
        capacity=100.0,
        time_limit=100.0,
        initial_capital=1000.0,
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
