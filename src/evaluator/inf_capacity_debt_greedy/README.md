# Subproblema: Maximizando ganancia dado un camino

## Definición formal del subproblema

Sea $R = (v_0, v_1, \dots, v_r, v_0)$ un camino (ruta) factible de puertos.
Queremos determinar las transacciones $Q = ((q_1^+,q_1^-), \dots, (q_r^+, q_r^-))$ que maximizan:

$$
f_{r+1} = f_0 - \sum_{j=1}^r \sum_{m\in M} p^+(v_j,m) q_j^+(m) + \sum_{j=1}^r \sum_{m\in M} p^-(v_j,m) q_j^-(m)
$$

bajo las restricciones:

1. Inventario: $I_{j+1}(m) = I_j(m) + q_j(m) \ge 0$
2. Stock de compra/venta: $-c^-(v_j,m) \le q_j(m) \le c^+(v_j,m)$

Asumiendo $B=+\infty$ y $f_j$ libre, las restricciones de capacidad y capital no negativo se eliminan.

## Observación clave

* Cada mercancía se puede tratar **independientemente**.

  * La ganancia total es $\sum_{m \in M} \text{ganancia}(m)$
  * Cada mercancía solo depende de las cantidades compradas y vendidas a lo largo de la ruta.

* Cada puerto impone **stock máximo** en compra/venta y **dirección temporal**: solo se puede vender algo que ya se haya comprado en un puerto anterior.

* Esto define un problema de **emparejamiento entre compras y ventas** para cada mercancía.

## Idea del algoritmo

La mayor ganancia global que podemos obtener para una instancia de la mercancía $m$ es comprarla donde más barato se venda, y venderla en el puerto siguiente donde más caro se compre. No tiene sentido comprar una unidad de mercancía si luego no se puede vender.

- Sea $A := \lbrace v_j| \quad c^-_j > 0 \rbrace$ (puertos donde es posible vender)
- Mientras que $H \neq \empty$ sea $v_j = _{}\text{argmax}\lbrace p^-_k | v_k \in A \rbrace$
  - Sea $B := \lbrace v_i| \quad c^-_i > 0, p^+_i < p^-_j, i<j \rbrace$ (puertos anteriores donde se puede comprar con ganancia)
  - Si $B=\empty$ entonces $v_j$ no da ganancia vender nada en $v_j$, hacemos $A = A - \{v_j\}$
  - En caso contrario sea $v_i = _{}\text{argmin}\lbrace p^-_k | v_k \in A \rbrace$
  - Sea $x = \text{min}(c^+_i, c^-_j)$
  - Guardamos las decisiones $q^+_i = q^-j = x$ (comprar y vender respectivamente, en esa pareja de puertos)
  - Hacemos $c^+_i = c^+_i - x$ y $c^-_j = c^-_j - x$ (actualizar inventario de los puertos, notese que los conjuntos $A$ y $B$, cambian en este paso)

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

Bajo las asunciones $B = +\infty$ y $f_j \in \mathbb{R}$, las restricciones de factibilidad se descomponen por mercancía, es decir, no existe ninguna restricción que involucre simultáneamente decisiones asociadas a mercancías distintas.

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

## Complejidad temporal

Analizamos la complejidad del algoritmo greedy para el subproblema **MAX-PROFIT(R)**.

### Parámetros

Sea:

* $r = |R|$: número de puertos en la ruta (sin contar el retorno a $v_0$).
* $|M|$: número de mercancías.
* Para una mercancía fija $m$, sean:

  * $\{(p^+_i, c^+_i)\}_{i=1}^r$ los precios y capacidades de compra,
  * $\{(p^-_j, c^-_j)\}_{j=1}^r$ los precios y capacidades de venta.

---

### Complejidad para una mercancía fija

Para una mercancía $m$, el algoritmo realiza las siguientes operaciones conceptuales:

1. **Ordenamiento de puertos por precios**

   * Los puertos se ordenan una vez por precio de venta $p^-_j$(orden decreciente).
   * Los puertos se ordenan una vez por precio de compra $p^+_i$ (orden creciente).

   Esto puede realizarse en:
   $$
   O(r \log r).
   $$

2. **Emparejamiento greedy de compras y ventas**

   Para cada puerto de venta $v_j$, el algoritmo recorre los puertos de compra anteriores $v_i$ con $i<j$, filtrando aquellos que:

   * tienen capacidad disponible,
   * producen ganancia positiva $(p^-_j > p^+_i)$.

   Cada par $(i,j)$ se considera a lo sumo una vez. Por lo tanto, el número total de operaciones de emparejamiento está acotado por:

   $$
   O(r^2).
   $$

   No se itera unidad por unidad, sino que en cada paso se empareja la máxima cantidad posible.

### Complejidad total por mercancía

Sumando ambas fases, el costo total para una mercancía fija es:

$$
O(r \log r + r^2) = O(r^2).
$$

### Complejidad total del subproblema **MAX-PROFIT(R)**

Por el **Lema 1 (Independencia entre mercancías)**, el problema se descompone en $|M|$ subproblemas independientes, uno por mercancía.

Aplicando el algoritmo greedy a cada mercancía, se obtiene una complejidad total de:

$$
O(|M| \cdot r^2).
$$


