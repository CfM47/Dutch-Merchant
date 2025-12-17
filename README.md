# The Dutch Merchant

**Final Project – Design and Analysis of Algorithms**
Bachelor in Computer Science (4th Year)

## Overview

This repository contains the final project for the course *Algorithms Analysis and Design *. The objective of the project is to apply theoretical and practical tools from algorithm design and computational complexity to a challenging real-world–inspired problem that is computationally intractable (NP-hard or NP-complete).

The project covers the full analytical cycle: formal problem modeling, complexity analysis, algorithm design, implementation, and empirical evaluation. Special emphasis is placed on rigorous reasoning, correctness, and performance analysis.

## Project Objectives

The main goals of this project are:

* To model and formalize a complex optimization problem in precise mathematical terms.
* To analyze its computational complexity and formally justify its NP-hardness or NP-completeness via polynomial-time reductions.
* To design and implement algorithmic solutions for intractable problems, including:

  * Exact algorithms for small instances (brute force).
  * Approximation algorithms and/or heuristics for larger instances.
* To experimentally evaluate solution quality and runtime performance.
* To communicate results clearly through code, documentation, and a formal technical report.

## Problem Description: *The Dutch Merchant*

The problem models a commercial expedition undertaken by a merchant captain of the Dutch East India Company.

Starting and ending in Amsterdam, the captain must plan a maritime route that visits a subset of available ports, subject to the constraint that no port may be visited more than once. The expedition is limited by a maximum total travel time.

At each port, a set of goods is available with specified buying and selling prices, which may vary across ports. The captain may buy and sell goods at each stop, subject to several constraints:

* The ship has a limited cargo capacity.
* Goods carried do not need to be sold immediately and may be retained for sale at future ports.
* After each transaction, the captain must retain enough capital to cover operational costs (crew wages, port taxes, repairs) until the next port.
* The total duration of the journey, including travel times between ports, must not exceed the given deadline.

The objective is to determine:

1. The subset and order of ports to visit.
2. The buying and selling decisions at each port.

Such that, upon returning to Amsterdam, the final capital is maximized.

This problem combines elements of routing, scheduling, knapsack-style constraints, and economic decision-making, and serves as a rich case study for the design and analysis of algorithms for NP-hard optimization problems.
