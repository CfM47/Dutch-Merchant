# Definición formal del problema

**El Comerciante Holandés**


## 1. Instancia

Una instancia es una tupla:

$$
\mathcal{I} = (V, t, \kappa, S, M, w, p^+, p^-, c^+, c^-, B, v_0, f_0, T)
$$

donde:
* $V$ es un conjunto de vértices de un grafo completo, que representan los puertos. 
* $t : V \times V \to \mathbb{R}^+$ representa el tiempo de viaje entre pares de ciudades.
* $\kappa : V \times V \to \mathbb{R}^+$ representa el costo de viaje entre pares de ciudades.
* $S : V \to \mathbb{R}^+$ es el costo fijo de visitar cada puerto.
* $M$ es el conjunto de mercancías.
* $w : M \to \mathbb{R}^+$ es el peso de una unidad de mercancía.
* $p^+ : V \times M \to \mathbb{R}^+$ es el precio de compra de una mercancía por puerto.
* $p^- : V \times M \to \mathbb{R}^+$ es el precio de venta de una mercancía por puerto.
* $c^+ : V \times M \to \mathbb{Z}_{\ge 0}$ es la oferta máxima de una mercancía por puerto.
* $c^- : V \times M \to \mathbb{Z}_{\ge 0}$ es la demanda máxima por puerto.
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


y $Q$ es un vector de tuplas $Q = ((q^+_1, q^-_1), \dots, (q^+_r, q^-_r))$ de transacciones

donde:
* $q^+_j : M \to \mathbb{Z}_{\geq 0}$
* $q^-_j : M \to \mathbb{Z}_{\geq 0}$

representan la compra y venta respectivamente del producto $m$ en el puerto $v_{i_j}$


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
I_{j+1}(m) = I_j(m) - q^-_j(m) + q^+_j(m)
$$

(el inventario cambia en consecuencia a las de acciones)

$$
0 \le I_{j+1}(m)
$$

(no se tiene una cantidad negativa de una mercancía en el barco)

$$
0 \leq q^-_j(m) \leq I_j(m)
$$

(se vende y luego se compra, por tanto solo se puede vender lo que se tiene en el inventario en el momento)

### Capacidad del barco

$$
\sum_{m \in M} I_{j+1}(m)  w(m) \le B
$$

(la cantidad de una mercancía por su peso no excede la capacidad del barco)

### Restricciones de stock

$$
0 \le q^+_j(m) \le c^+(v_{i_j}, m)
$$
$$
0 \le q^-_j(m) \le c^-(v_{i_j}, m) 
$$
(se compra y se vende dentro de los límites de stock que admite el puerto) 

### Capital

$$
f_{j+1} = f_j - \kappa(v_{i_j}, v_{i_{j+1}}) - \sum_m p^+(v_{i_j},m)q^+_j(m) + \sum_{m} p^-(v_{i_j},m)q^-_j(m) - S(v_{i_j})
$$

$$
f_{j+1} \ge 0
$$

(el capital actual, menos el costo de moverse de puerto, menos el precio de comprar, más los ingresos por vender menos el precio de los impuestos portuarios)


## 4. Función objetivo

Maximizar el capital final al regresar a $v_0$:

$$
\max f_{r+1}
$$