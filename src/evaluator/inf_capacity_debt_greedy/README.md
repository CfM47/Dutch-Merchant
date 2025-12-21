# Subproblema: Maximizando ganancia dado un camino

## Definición formal del subproblema

Sea $R = (v_0, v_1, \dots, v_r, v_0)$ un camino (ruta) factible de puertos.
Queremos determinar las transacciones $Q = (q_1, \dots, q_r)$ que maximizan:

$$
f_{r+1} = f_0 - \sum_{j=1}^r \sum_{m\in M: q_j(m)>0} p^+(v_j,m) q_j(m) - \sum_{j=1}^r \sum_{m\in M: q_j(m)<0} p^-(v_j,m) q_j(m)
$$

bajo las restricciones:

1. Inventario: $I_{j+1}(m) = I_j(m) + q_j(m) \ge 0$
2. Stock de compra/venta: $-c^-(v_j,m) \le q_j(m) \le c^+(v_j,m)$

Asumiendo $B=+\infty$ y $f_j$ libre, las restricciones de capacidad y capital no negativo se eliminan.

---

## Observación clave

* Cada mercancía se puede tratar **independientemente**.

  * La ganancia total es $\sum_{m \in M} \text{ganancia}(m)$
  * Cada mercancía solo depende de las cantidades compradas y vendidas a lo largo de la ruta.

* Cada puerto impone **stock máximo** en compra/venta y **dirección temporal**: solo se puede vender algo que ya se haya comprado en un puerto anterior.

* Esto define un problema de **emparejamiento temporal entre compras y ventas** para cada mercancía.

---

## Idea del algoritmo (greedy temporal por mercancía)

Para cada mercancía $m$:

1. Recorremos la ruta $R$ en orden temporal (desde $v_0$ hasta $v_r$).

2. Mantenemos una lista de **compras previas disponibles**, ordenadas por precio creciente.

3. Cada vez que llegamos a un puerto con demanda $c^-(v_j,m)$, emparejamos la demanda con las compras previas más baratas posibles:

   $$
   \text{take} = \min(\text{cantidad disponible de la compra}, \text{demanda})
   $$

4. Actualizamos inventario, demanda y ganancia.

---

## Pseudocódigo

```text
GreedyMaxProfit(R, m):
    # R: lista de puertos en orden temporal
    # m: mercancía
    heap = empty min-heap keyed by buy_price
    profit = 0
    matches = []

    for j in 1..r:
        # 1) añadir compras al heap
        if c^+(v_j, m) > 0:
            heap.push( (p^+(v_j, m), c^+(v_j,m), j) )

        # 2) procesar ventas
        demand = c^-(v_j, m)
        while demand > 0 and heap not empty:
            (bprice, avail, bidx) = heap.pop_min()
            take = min(avail, demand)
            matches.append( (bidx, j, take) )
            profit += take * (p^-(v_j,m) - bprice)
            avail -= take
            demand -= take
            if avail > 0:
                heap.push( (bprice, avail, bidx) )
    return matches, profit
```

* `matches` indica qué cantidad de la compra en puerto `bidx` se empareja con venta en puerto `j`.
* Repetimos esto para cada mercancía $m \in M$.
* Suma de `profit` sobre todas las mercancías da la **ganancia máxima** posible para el camino $R$.

---

## Correctitud (demostración de optimalidad)

**Propiedad a demostrar:** El algoritmo anterior produce la asignación de compras y ventas que **maximiza la ganancia total** para cada mercancía (m).

**Demostración por intercambio:**

1. Sea $S^*$ una solución óptima con transacciones $Q^*$.

2. Supongamos que existe un puerto $v_j$ con venta de $d$ unidades que **no utiliza una compra previa más barata posible** sino una compra más cara.

   * Sea $c_1$ la compra utilizada por $v_j$ y $c_2$ la compra más barata disponible (no usada para esta venta).
   * Por definición del greedy, $p^+(c_2) \le p^+(c_1)$ y la venta es la misma $p^-(v_j)$.

3. **Intercambio:** reasignamos la venta de $v_j$ de $c_1$ a $c_2$.

   * Ganancia marginal de $c_2$ ≥ ganancia marginal de $c_1$
     $$
     p^-(v_j) - p^+(c_2) \ge p^-(v_j) - p^+(c_1)
     $$
   * Todas las demás restricciones siguen cumplidas: inventario ≥ 0 y stocks máximos respetados.

4. Se tiene que la solucion greedy es tan buena como la solucion optima $\mathcal{G} \geq S^*$.

**Conclusión:** Existe una solución óptima donde cada venta se empareja con las compras previas más baratas posibles. El algoritmo produce exactamente esa solución.

* **Complejidad:** $O(|M|*r log r)$ por mercancía.

---

## Observaciones finales

* Bajo las relajaciones de capacidad infinita y capital libre, **cada mercancía se puede optimizar independientemente**.
* La ganancia máxima total es simplemente la **suma de ganancias óptimas por mercancía**.
* Este algoritmo **reduce el subproblema de optimización de Q a un problema greedy temporal de asignación**, con demostración de optimalidad formal por intercambio.

