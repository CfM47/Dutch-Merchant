# Análisis de complejidad del problema **El Comerciante Holandés**



## 1. Problema de decisión asociado

Consideramos la versión de decisión del problema de optimización definido previamente.

**Instancia:**

* Una instancia
  $$
  \mathcal{I} = (V, t, \kappa, S, M, w, p^+, p^-, c^+, c^-, B, v_0, f_0, T)
  $$
  del problema *El Comerciante Holandés*.
* Un umbral de capital $K \in \mathbb{R}^+$.

**Pregunta:**
¿Existe una solución factible $(R,Q)$ tal que el capital final al regresar a $v_0$ satisface
$$
f_{r+1} \ge K?
$$

Denotaremos este problema como **CH-DEC**.



## 2. Problema base: Traveling Salesman Problem

Usamos como problema base el **TSP en versión de decisión**.

**Instancia:**

* Un grafo completo $G = (V,E)$,
* Una función de pesos $d : E \to \mathbb{R}^+$,
* Un umbral $D \in \mathbb{R}^+$.

**Pregunta:**
¿Existe un ciclo Hamiltoniano que visite todos los vértices exactamente una vez y cuyo costo total sea a lo sumo $D$?

Este problema es NP-completo.



## 3. Idea general de la reducción

La reducción fuerza al comerciante a:

1. Visitar **todos los puertos distintos del puerto inicial**.
2. Recolectar exactamente **una unidad de mercancía en cada uno**.
3. Regresar al puerto inicial para vender todas las unidades recolectadas.
4. Respetar un límite de tiempo idéntico al umbral del TSP.

La única forma de alcanzar el capital objetivo es recorriendo todos los puertos exactamente una vez, lo que impone una estructura idéntica a un ciclo Hamiltoniano.



## 4. Construcción de la reducción

Sea una instancia de TSP de decisión $(V_{TSP},d,D)$, con
$V_{TSP} = {v_0, v_1, \dots, v_{n-1}}.$

Construimos en tiempo polinomial una instancia $\mathcal{I}'$ de **CH-DEC** como sigue.



### 4.1 Puertos, tiempos y costos de viaje

$$
V=V_{TSP} \quad
t(u,v) = d(u,v) \quad
\kappa(u,v) = 0 \quad
\forall u,v \in V
$$

El puerto inicial y final es $v_0$.



### 4.2 Mercancías

$$
M = {m}, \quad w(m) = 1
$$



### 4.3 Precios y stock

Para cada puerto $v_i \neq v_0$:

$$
c^+(v_i,m) = 1, \quad
c^-(v_i,m) = 0
$$
$$
p^+(v_i,m) = p^-(v_i,m)= 0
$$

El puerto $v_0$ se define como el único comprador:

$$
c^+(v_0,m) = 0, \quad
c^-(v_0,m) = n-1
$$
$$
p^+(v_0,m) = 0, \quad
p^-(v_0,m) = 1
$$



### 4.4 Capacidad, capital e impuestos

$$
B = +\infty, \quad f_0 = 0, \quad S(v) = 0 \quad \forall v \in V
$$



### 4.5 Tiempo máximo y umbral de capital

$$
T = D, \quad K = n-1
$$



## 5. Correctitud de la reducción

Demostramos la equivalencia entre soluciones factibles de ambas instancias.



### 5.1 Desde TSP a Comerciante Holandés

Sea
$$
C = (v_0, v_{i_1}, \dots, v_{i_{n-1}}, v_0)
$$
un ciclo Hamiltoniano de costo total $\leq D$.

Definimos la solución del Comerciante Holandés:

* Ruta: $R = C$.
* Transacciones:
  $$
  q^+_j(m) = 1 \quad q^-_j(m) = 0 \quad \text{en cada } v_{i_j}
  $$
  $$
  q^+_j(m) = 0 \quad q^-_{r+1}(m) = n-1 \quad \text{en } v_0
  $$

**Verificación de factibilidad:**

* Inventario: se incrementa en una unidad por puerto visitado y nunca es negativo. En el puerto $v_0$, el inventario acumulado es exactamente $n-1$, por lo que la restricción $0 \leq q^-_{r+1}(m) \leq I_{r+1}(m)$ se satisface.
* Stock: cada puerto $v_i \neq v_0$ permite a lo sumo una compra.
* Capacidad: $B$ es suficiente.
* Capital:
  $$
  f_{r+1} = (n-1) \cdot 1 = n-1 = K
  $$
* Tiempo: coincide con el costo del ciclo TSP y es $\le D = T$.

Luego, la solución es factible y satisface $f_{r+1} \ge K$.



### 5.2 Desde Comerciante Holandés a TSP

Supóngase que existe una solución factible $(R,Q)$ de **CH-DEC** en $\mathcal{I}'$ tal que:
$$
f_{r+1} \ge K = n-1.
$$

**Observaciones clave:**

1. El capital solo puede aumentar mediante ventas en $v_0$.
2. Cada unidad vendida incrementa el capital en exactamente 1.
3. Cada puerto $v_i \neq v_0$ puede aportar a lo sumo una unidad.
4. El puerto $v_0$ no aporta mercancía.

Luego, para alcanzar $f_{r+1} \ge n-1$, es necesario recolectar exactamente una unidad en cada puerto $v_i \neq v_0$.

Esto implica que:

* Todos los puertos $v_i \neq v_0$ son visitados.
* Cada uno se visita a lo sumo una vez (por definición de ruta).
* La ruta tiene la forma:
  $$
  R = (v_0, v_{i_1}, \dots, v_{i_{n-1}}, v_0)
  $$

Finalmente, como:
$$
\sum_{j=0}^{n-1} t(v_{i_j}, v_{i_{j+1}})
 \le T = D,
$$
la ruta corresponde a un ciclo Hamiltoniano de costo a lo sumo $D$.



## 6. Conclusión

La reducción es polinomial y establece una equivalencia exacta entre:

* Soluciones del TSP de costo $\le D$, y
* Soluciones del Comerciante Holandés con capital final $\ge K$.

Por lo tanto, **CH-DEC es NP-duro**, y el problema de optimización **El Comerciante Holandés** es **NP-duro**