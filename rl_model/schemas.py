"""
Pydantic schemas for the Dutch Merchant problem instance.

This mirrors the Rust Instance struct from dm-solution/src/model/instance.rs
"""

from typing import List
from pydantic import BaseModel, Field


# Type aliases for clarity
PortId = int
GoodId = int


class Instance(BaseModel):
    """
    Dutch Merchant Problem instance.
    
    This Pydantic model mirrors the Rust Instance struct and contains
    all information needed to define a trading problem.
    """
    
    # Number of ports
    n_ports: int = Field(..., description="Number of ports")
    
    # Number of goods
    n_goods: int = Field(..., description="Number of goods types")
    
    # t(u,v): travel time from port u to port v
    # Shape: [n_ports][n_ports]
    travel_time: List[List[float]] = Field(
        ..., 
        description="Travel time matrix t(u,v) from port u to port v"
    )
    
    # w(m): weight of good m
    # Shape: [n_goods]
    weight: List[float] = Field(
        ..., 
        description="Weight of each good type"
    )
    
    # p+(v,m): buy price of good m at port v
    # Shape: [n_ports][n_goods]
    buy_price: List[List[float]] = Field(
        ..., 
        description="Buy price p+(v,m) of good m at port v"
    )
    
    # p-(v,m): sell price of good m at port v
    # Shape: [n_ports][n_goods]
    sell_price: List[List[float]] = Field(
        ..., 
        description="Sell price p-(v,m) of good m at port v"
    )
    
    # c+(v,m): buy stock/capacity of good m at port v
    # Shape: [n_ports][n_goods]
    buy_cap: List[List[float]] = Field(
        ..., 
        description="Buy capacity c+(v,m) of good m at port v"
    )
    
    # c-(v,m): sell stock/capacity of good m at port v
    # Shape: [n_ports][n_goods]
    sell_cap: List[List[float]] = Field(
        ..., 
        description="Sell capacity c-(v,m) of good m at port v"
    )
    
    # S(v): cost of visiting port v
    # Shape: [n_ports]
    visit_cost: List[float] = Field(
        ..., 
        description="Cost S(v) of visiting port v"
    )
    
    # v_0: Initial port
    start_port: PortId = Field(
        ..., 
        description="Initial port ID (v_0)"
    )
    
    # B: Boat capacity
    capacity: float = Field(
        ..., 
        description="Boat capacity (B)"
    )
    
    # T: Time limit
    time_limit: float = Field(
        ..., 
        description="Time limit (T)"
    )
    
    # f_0: Initial capital
    initial_capital: float = Field(
        ..., 
        description="Initial capital (f_0)"
    )
    
    class Config:
        """Pydantic configuration."""
        json_schema_extra = {
            "example": {
                "n_ports": 3,
                "n_goods": 2,
                "travel_time": [[0, 1, 2], [1, 0, 1], [2, 1, 0]],
                "weight": [1.0, 2.0],
                "buy_price": [[10, 20], [15, 25], [12, 22]],
                "sell_price": [[12, 22], [10, 20], [15, 25]],
                "buy_cap": [[100, 50], [80, 60], [90, 70]],
                "sell_cap": [[100, 50], [80, 60], [90, 70]],
                "visit_cost": [1.0, 2.0, 1.5],
                "start_port": 0,
                "capacity": 100.0,
                "time_limit": 50.0,
                "initial_capital": 1000.0
            }
        }
