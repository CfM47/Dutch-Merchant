# Asunciones implícitas del modelo

(no explícitas en la definición formal del problema)

### Estructura métrica del espacio de puertos
* El tiempo/distancia entre dos puertos es independiente de la dirección del viaje:
    $$
    t(u,v) = t(v,u).
    $$
* Los tiempos de viaje satisfacen la desigualdad triangular:
    $$
    t(u,w) \le t(u,v) + t(v,w).
    $$
Esto descarta trayectorias “no físicas” donde desviarse reduce el tiempo total y permite interpretar el problema como ruteo en un espacio métrico real.


### Relación entre tiempo y costo
El costo de desplazamiento no es una magnitud independiente, sino que está determinado por el tiempo de viaje:
$$
\kappa(u,v) = \alpha \cdot t(u,v), \quad \alpha > 0.
$$
Esto implica que minimizar costo y minimizar tiempo son objetivos compatibles y evita que existan rutas baratas pero arbitrariamente largas (o viceversa).

El factor $\alpha$ es constante en toda la red: no depende del puerto, del tramo ni del momento del viaje.
Se asume implícitamente un único medio de transporte con comportamiento estable.

### Temporalidad y sincronización

Las compras, ventas y viajes se coordinan sin demoras adicionales, colas ni conflictos operativos. El tiempo fluye únicamente como consecuencia del desplazamiento.

Precios, tiempos de viaje y costos no cambian a lo largo del recorrido.
El modelo no contempla dinámicas temporales, por lo que se asume un entorno estático durante toda la planificación.

### Ausencia de incertidumbre

No existen eventos aleatorios: accidentes, retrasos, pérdidas de mercancía, fluctuaciones de mercado o fallas logísticas.
Esto no está prohibido por la formulación, pero se asume implícitamente para que el problema sea puramente combinatorio.