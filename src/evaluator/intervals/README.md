# Weighted Interval Scheduling Relaxation

## Assumptions

For this solution, we assume:

- The merchant can go into debt, i.e. the monetary balance may be negative at any point in the journey.
- Ports have infinite stock and infinite demand for every product, i.e. one can sell and buy as much as one wants on each port, as long as cargo capacity allows.
- The amounts of products one can buy and sell are non-negative real numbers.
- We cannot buy a product and sell it at the same port immediately (given that there is no buying or selling cap, this is either an 'infinite profit' scenario, or a 'no profit' scenario, depending on the prices, which is nonsensical either way), we **can** sell a product we had in our hold at a given port, then buy it at the same port again however.

## Correctness

### Equivalence to Weighted Interval Scheduling Problem

#### High level idea

We will take an optimal solution, and split the cargo hold in **units of cargo**, such that each of them is always full of one product, or empty, our ability to fill each of these units is independent of our ability to fill all the others, given that the amount of products we can sell and buy at a given port are only limited by the capacity of the hold. For a single unit, i.e. a ship that has a capacity equal to the size of the unit, there is some optimal solution, some optimal set of choices we can make to fill this space. Given that no set of choices will give us a greater profit per capacity than this for any of the units, we can simply make the same choices for each unit of cargo, simplifying our problem dramatically: we can either have our cargo hold full of one product, or empty.

#### Proof

First, let us assume we can only have our hold completely full of a single product or completely empty at any point. For this scenario, there is some optimal solution $S$.

Now, let us take an optimal solution of the problem without this restriction. In an optimal solution, let us interpret the cargo hold as an initially uncolored segment $H$, when we buy $L$ weight of product $m_i$, a set of uncolored sections (sub-segments) of $H$ such that the sum of their lengths is $L$, is colored with color $c_i$, and when we sell $L$ weight of a product $m_i$, we take a set of sub-segments colored $c_i$ from $H$, such that the sum of their lengths is $L$ (which must obviously exist), and remove their color. Now, let us divide $H$ into segments that were always either completely some color $c_i$ or uncolored. Given that the amount one can buy or sell at a port is infinite, these segments are independent, and none of them could have been used to obtain a higher profit than $S$ per unit of weight in the hold, because then $S$ wouldn't be optimal. Therefore, we can simply substitute the decisions made over each of these segments of cargo, by the ones made for the entire hold in $S$.

Therefore, there is an optimal solution such that the hold is always full with a given product, or completely empty.

Now, the problem is reduced to, given pairs of buying and selling prices $b_{im}$, $s_{jm}$ for cities $i, j$ and product $m$ such that $i \lt j$, obtain the set of non overlapping intervals $[i, j]$ with value $v = max_m\lbrace s_{jm} - b_{im} \rbrace$ $I$ such that the sum of their values is maximum. This is a weighted interval scheduling problem. We can find the intervals by going through every pair of cities in the path, and creating intervals for each product, taking the buying price from the first city and the selling price from the second. We can skip the intervals where the selling price is lower than the buying price. Either way, the amount of intervals will be $O(n^2m)$ where $n$ is the amount of cities, and $m$ the amount of products.

### Solution of Weighted Interval Scheduling Problem

Now, for each interval, we have two choices, we take it or not, taking it removes our ability to take intervals that overlap with it. We will solve this problem using dynamic programming. Let us sort the intervals by finish time, if we store the maximum weight we can achieve in the subset of all intervals that end at or before $i$ for every $i$, the best solution for finish time $t \leq i + 1$ is either taking one of the intervals that end in $i + 1$ and the best solution for the set of intervals that end when or before it starts, or not taking it, and simply taking $S[i]$ again.

Let $S[i]$ be the maximum weight we can achieve with all intervals with finish time less than or equal to $i$, then $S[i] = max(max_c\lbrace w_{ic} + S[h(c)] \rbrace, S[i - 1])$ such that $w_{ic}$ is the weight of the $c$-th interval with finish time $i$ and $h(x)$ is the start time of the $x$-th interval with finish time $i$.

#### Pseudocode

```python
# I <- intervals (i, j, w)[]

D = {}
for (i, j, w) in I:
  if j not in D:
    D[j] = []
  D[j] += [(i, w)]

def S(x):
  if x < 0:
    return 0

  take = max([w + S(i) for (i, w) in D[x]])
  return max(take, S(x - 1))

print(S(len(I) - 1))
```

#### Complexity

The initial setup of grouping the intervals by finish time is $O(n \cdot log(n))$, while the $S$ function's $take$ calculation will touch every interval exactly once, while the other $max$ call is a recursive call in the dynamic programming, therefore, the complexity will be $O(n \cdot log(n))$.

### Storing the solutions

In order to find not only the maximum weight but also the solution, we simply store the best solution along with the maximum weight, and we compose solutions by taking or not taking the interval in question at each given end time.

## Final Complexity

While the weighted interval scheduling problem can be solved in $O(N \cdot log(N))$, here, N is actually $O(n^2m)$ where $n$ is the amount of cities and $m$, the amount of types of products, therefore the complexity is $O(n^2m(log(n) + log(m)))$.

