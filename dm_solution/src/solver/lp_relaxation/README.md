# LP Relaxation - Route Profit Evaluator

## Subproblem: Optimal Trading Strategy for a Given Route

This module solves a **subproblem** of the Dutch Merchant problem: **given a fixed route of ports**, it finds the optimal buying and selling strategy to maximize profit using **Linear Programming (LP)**.

> **Note**: This does NOT solve the full Dutch Merchant problem. It assumes the route is already given and only optimizes the trading decisions along that route.

## Assumptions

1. **Fixed Route**: The sequence of ports to visit is provided as input. This module does NOT decide which ports to visit or in what order.

2. **Divisible Goods**: Quantities $q_{j,m}^+$ (bought) and $q_{j,m}^-$ (sold) can be any non-negative real number, not just integers. This assumes goods are infinitely divisible.

3. **Initial Conditions**: 
   - Initial inventory is zero for all goods
   - Initial capital is given by the instance

4. **No Time Validation**: This module does not validate whether the given route is feasible within the time limit. It assumes the route has already been validated.

## Model Definition

### Variables
- $q_{j,m}^+$ : Amount of good $m$ bought at port $j$
- $q_{j,m}^-$ : Amount of good $m$ sold at port $j$
- $I_{j,m}$ : Inventory of good $m$ after visiting port $j$
- $f_j$ : Capital after visiting port $j$

### Objective Function
$$\text{Maximize } f_{r}$$

where $r$ is the last port in the route.

### Constraints

1. **Inventory Balance**:
$$I_{j,m} = I_{j-1,m} + q_{j,m}^+ - q_{j,m}^-$$

2. **Boat Capacity**:
$$\sum_m I_{j,m} \cdot w_m \leq B$$

3. **Capital Update** (non-negative capital):
$$f_j = f_{j-1} - \sum_m (p_{j,m}^+ \cdot q_{j,m}^+ - p_{j,m}^- \cdot q_{j,m}^-) - S(v_j) \geq 0$$

4. **Port Limits**:
$$0 \leq q_{j,m}^+ \leq c_{j,m}^+ \quad \text{(buy limits)}$$
$$0 \leq q_{j,m}^- \leq c_{j,m}^- \quad \text{(sell limits)}$$

5. **Sell Constraint**:
$$q_{j,m}^- \leq I_{j-1,m} \quad \text{(can only sell what we have)}$$

