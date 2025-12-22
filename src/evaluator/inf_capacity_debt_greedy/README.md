# Subproblema: Maximizando ganancia dado un camino

## Definición formal del subproblema0

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

* Esto define un problema de **emparejamiento entre compras y ventas** para cada mercancía.

## Idea del algoritmo

La mayor ganancia global que podemos obtener para una instancia de la mercancía $m$ es comprarla donde más barato se venda, y venderla en el puerto siguiente donde más caro se compre. No tiene sentido comprar una unidad de mercancía si luego no se puede vender.

- Añadimos todas los puertos $v_i$ a un heap $H_1$ de máximos, tomando como llave su precio de venta $p^-_i$.
- Mientras que el $H_1$ no este vacío (existe una posible venta)
  - Sea $v_i = \text{pop}(H_1)$ el puerto con precio de venta mas alto.
  - Para cada puerto $v_j$ con $j < i$ (vienen antes en la ruta) los añadimos a un heap de mínimos $H_2$, con su precio de compra $p^+_j$ como llave.
  - Mientras que $c^-_i \geq 0$ y $H_2$ no este vacío (exista una posible compra):
    - Sea $v_j = \text{pop}(H_2)$ el puerto con el menor precio de compra.
    - Si $p^-_i \leq p^+_j$ continua (no hay ganancia).
    - En caso contrario:
    - $q^-_i = q^+_j= x =\text{min}(c^-_i, c^+_j)$ (compra en $v_j$ todas las unidades que puedas vender en $v_i$) 
    - $c^-_i := c^-_i - x$ decrementa la cantidad posible a vender de $v_i$ en $x$
    - $c^+_j := c^+_j - x$ decrementa la cantidad posible a comprar de $v_j$ en $x$.

## Demostración de correctitud.

Consideramos el subproblema de maximizar la ganancia total asociada a una ruta fija
$$
R = (v_0, v_1, \dots, v_r, v_0)
$$
bajo las siguientes asunciones:

* La capacidad del barco es infinita $(B = +\infty)$.
* El capital puede ser negativo $(f_j \in \mathbb{R})$.
* No existen interacciones entre mercancías distintas más allá de la suma de ganancias.

Denotamos este subproblema como **MAX-PROFIT(R)**.

### Lema 1 (Independencia entre mercancías)

La función objetivo del problema puede escribirse como
$$
f_{r+1}
= f_0 + \sum_{m \in M}
\sum_{j=1}^{r}
\bigl(
p^-(v_j,m)q^-_j(m) -
p^+(v_j,m)q^+_j(m)
\bigr).
$$

Bajo las asunciones (B = +\infty) y (f_j \in \mathbb{R}), las restricciones de factibilidad se descomponen por mercancía, es decir, no existe ninguna restricción que involucre simultáneamente decisiones asociadas a mercancías distintas.

Por lo tanto, maximizar la ganancia total es equivalente a maximizar, de manera independiente, la ganancia asociada a cada mercancía.

$\square$

### Reformulación del subproblema para una mercancía fija

Fijemos una mercancía $m$. Para cada puerto $v_i$ de la ruta se definen:

* $c^+_i \ge 0$: capacidad máxima de compra a precio $p^+_i$,
* $c^-_i \ge 0$: capacidad máxima de venta a precio $p^-_i$.

Al no poder comprarse mas unidades de las que se vende, ni ser optimo comprar una unidad para no venderla, una solución factible queda completamente caracterizada por un conjunto de emparejamientos unitarios
$$
(i,j), \quad i < j,
$$
donde cada emparejamiento representa una unidad comprada en $v_i$ y vendida en $v_j$, respetando las capacidades $c^+_i$ y $c^-_j$.

La ganancia asociada a una pareja $(i,j)$ es $p^-_j - p^+_i$.


### Lema 2 (Estructura de las soluciones óptimas)

Sea $\mathcal{O}$ una solución óptima para una mercancía fija.

1. **No existen emparejamientos no rentables.**
   Toda unidad emparejada $(i,j)$ en $\mathcal{O}$ satisface $p^-_j - p^+_i \ge 0.$
2. **Saturación de emparejamientos rentables.**
   Para todo puerto $v_j$, si en $\mathcal{O}$ se tiene $q^-_j < c^-_j$, entonces no existe ningún puerto $v_i$ con $i<j$ tal que $q^+_i < c^+_i$ y $p^-_j - p^+_i > 0$

#### Demostración

1. Supóngase que una unidad comprada en $v_i$ se vende en $v_j$ con $p^-_j - p^+_i < 0$. Eliminando dicho emparejamiento se obtiene una solución factible con ganancia estrictamente mayor, contradiciendo la optimalidad. El caso $p^-_j - p^+_i = 0$ puede eliminarse sin afectar la ganancia total.

2. Si existiera un puerto $v_i$ con $i<j$, capacidad disponible de compra, y $p^-_j - p^+_i > 0$, entonces agregar una unidad comprada en $v_i$ y vendida en $v_j$ incrementaría estrictamente la ganancia, contradiciendo nuevamente la optimalidad de $\mathcal{O}$.

$\square$

### **Lema 3 (Maximalidad del número de emparejamientos)**

Las soluciones del algoritmo greedy contienen tantos emparejamientos retables como cualquier solucion óptima.

#### Demostración

Supongamos que el algoritmo greedy termina con una solución $\mathcal{G}$ y que existe otra solución factible $\mathcal{O}$ con más emparejamientos rentables.

Entonces, en el momento en que el algoritmo greedy se detiene, no existe ningún par $(i,j)$ con capacidad disponible y $p^-_j - p^+_i > 0$.

Pero entonces $\mathcal{O}$ tampoco puede contener tal emparejamiento, pues violaría el Lema 2.

Por lo tanto, ninguna solución factible puede contener más emparejamientos rentables que el greedy.

$\square$

### Teorema (Correctitud del algoritmo greedy)

El algoritmo greedy descrito produce una solución óptima para **MAX-PROFIT(R)**.

#### Demostración

Consideremos una mercancía fija. El algoritmo procede iterativamente emparejando unidades:

* selecciona el puerto $v_j$ con mayor precio de venta disponible,
* lo empareja con el puerto $v_i$ con $i < j$ con menor precio de compra disponible tal que $p^-_j - p^+_i > 0$.

Demostraremos por inducción sobre el número de emparejamientos que el algoritmo greedy **always stays ahead** de cualquier solución óptima.

**Caso base.**
En el primer emparejamiento, el algoritmo selecciona el mayor $p^-_j$ posible y el menor $p^+_i$ posible con $i<j$, maximizando la ganancia del primer emparejamiento. Ninguna solución óptima puede obtener una ganancia mayor en su primer emparejamiento.

**Paso inductivo.**
Supongamos que tras $k$ emparejamientos, la ganancia acumulada del algoritmo greedy es al menos la de cualquier solución óptima parcial con $k$ emparejamientos. En el paso $k+1$, el greedy selecciona el emparejamiento con mayor ganancia disponible restante. Luego, tras el emparejamiento $k+1$, el greedy continúa manteniéndose por delante.

Por el Lema 3, el greedy y realiza tantos emparejamientos como cualquier solucion óptima. Por tanto, al finalizar, la ganancia total del greedy es al menos la de cualquier solución óptima.

$\square$

### Corolario

Dado que el problema global se descompone por mercancía (Lema 1), aplicar el algoritmo greedy de manera independiente a cada mercancía produce una solución óptima global para **MAX-PROFIT(R)**.
