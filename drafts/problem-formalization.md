# Definición formal del problema

**El Comerciante Holandés**


## 1. Instancia

Una instancia es una tupla:

$$
\mathcal{I} = (G, v_0, M, w, p^+, p^-, c^+, c^-, B, f_0, S, T)
$$

donde:

* $G = (V, t)$ es un grafo completo ponderado de puertos, con pesos
  $t : V \times V \to \mathbb{R}^+$
  que representan el tiempo de viaje.
* $M$ es el conjunto de mercancías.
* $w : M \to \mathbb{R}^+$ es el peso de una unidad de mercancía.
* $p^+ : V \times M \to \mathbb{R}^+$ es el precio de compra de una mercancía por puerto.
* $p^- : V \times M \to \mathbb{R}^+$ es el precio de venta de una mercancía por puerto.
* $c^+ : V \times M \to \mathbb{Z}_{\ge 0}$ es la oferta máxima de una mercancía por puerto.
* $c^- : V \times M \to \mathbb{Z}_{\ge 0}$ es la demanda máxima por puerto.
* $S : V \to \mathbb{R}^+$ es el costo fijo de visitar cada puerto.
* $v_0 \in V$ es el puerto de salida y regreso.
* $f_0 \in \mathbb{R}^+$ es el capital inicial.
* $B \in \mathbb{R}^+$ es la capacidad del barco.
* $T \in \mathbb{R}^+$ es el tiempo límite del viaje.


## 2. Solución

Una solución es un par:

$$
\text{sol} = (R, Q)
$$

 donde $R$ es una ruta
$$
R = (v_0, v_{i_1}, \dots, v_{i_r}, v_0)
$$


donde
$$
v_{i_j} \neq v_{i_k} \quad \forall j,k \in \{1...r\}
$$ 
(los puertos aparecen a lo sumo una vez en la ruta).

$$
  \sum_{j=0}^{r} t(v_{i_j}, v_{i_{j+1}}) \le T
$$
(el tiempo del viaje no excede el tiempo límite)


y $Q$ es un vector $Q = (q_1, \dots, q_r)$ de transacciones

$$
q_j : M \to \mathbb{Z}
$$

donde:

* $q_j(m) > 0$: compra.
* $q_j(m) < 0$: venta.

en el puerto $v_{i_j}$


### Estado del barco

El estado del barco al llegar al $j$-ésimo puerto es:

$$
\Sigma_j = (f_j, I_j)
$$

donde:

* $f_j \in \mathbb{R}_{\ge 0}$: capital disponible.
* $I_j : M \to \mathbb{Z}_{\ge 0}$: inventario a bordo.

Estado inicial:
$$
\Sigma_0 = (f_0, I_0), \quad I_0(m) = 0 \ \forall m
$$


## 3. Restricciones por puerto (transición de estado)

La transición $\Sigma_j \to \Sigma_{j+1}$ es válida si, para todo $m \in M$ se tiene:

### Inventario

$$
I_{j+1}(m) = I_j(m) + q_j(m)
$$

$$
0 \le I_{j+1}(m)
$$

(el inventario cambia en consecuencia de acciones, no se tiene una cantidad negativa de una mercancía en el barco)

### Capacidad del barco

$$
\sum_{m \in M} I_{j+1}(m)  w(m) \le B
$$

(la cantidad de una mercancía por su peso no excede la capacidad del barco)

### Restricciones de stock

$$
-c^-(v_{i_j}, m) \le q_j(m) \le c^+(v_{i_j}, m)
$$

(se compra y se vende dentro de los límites que admite el puerto) 

### Capital

$$
f_{j+1} = f_j - \sum_{m: q_j(m) > 0} p^+(v_{i_j},m)q_j(m) - \sum_{m: q_j(m) < 0} p^-(v_{i_j},m)q_j(m) - S(v_{i_j})
$$

$$
f_{j+1} \ge 0
$$

(el capital actual, menos el precio de comprar, más los ingresos por vender menos el precio de los impuestos)

(* nótese que en la segunda sumatoria el signo es negativo puesto que $q_j$ es negativo)


## 4. Función objetivo

Maximizar el capital final al regresar a $v_0$:

$$
\max f_{r+1}
$$