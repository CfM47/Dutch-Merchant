# Dutch Merchant RL Model

Este directorio contiene una implementación de un agente de **Aprendizaje por Refuerzo (Reinforcement Learning)** diseñado para resolver el problema del "Dutch Merchant". El objetivo es encontrar una ruta óptima de puertos que maximice el beneficio total, respetando las restricciones de tiempo y todas las demas restricciones del problema.

## ¿Cóm funciona el Solver?

La clase `Solver` (en `solver.py`) actúa como la interfaz principal para resolver instancias del problema utilizando RL. Este enfoque entrena al agente específicamente para la instancia que se desea resolver.

### El Proceso de Resolución:

1.  **Codificación de la Instancia**: El solver recibe una `Instance` (tiempos de viaje, precios, capacidades, etc.) y la convierte en un vector de características (embeddings) que la red neuronal puede procesar.
2.  **Entrenamiento "On-the-fly"**: Se ejecutan múltiples *epoche* de entrenamiento. En cada epoche:
    *   Se generan varias soluciones (episodios) de forma estocástica.
    *   Se utiliza un mecanismo de **disminución de temperatura**: al inicio (alta temperatura), el agente explora rutas diversas de forma casi aleatoria; conforme avanza el entrenamiento (baja temperatura), el agente se vuelve más "codicioso" (greedy), explotando las mejores estrategias descubiertas.
3.  **Evaluación y Selección**: El solver registra la mejor solución encontrada durante todo el entrenamiento. Al finalizar, realiza una última pasada puramente determinista (greedy) y devuelve la mejor de las dos (la mejor histórica o la final).

---

## El Modelo: Arquitectura de RL

El motor de este solver es un modelo basado en el algoritmo **REINFORCE** (un tipo de *Policy Gradient*).

### Componentes Clave:

*   **Policy Network (`agent.py`)**: Es una red neuronal profunda que, dado el estado actual (puerto actual y características de la instancia), genera una distribución de probabilidad sobre el siguiente puerto a visitar.
*   **Instance Encoder**: Una sub-red que comprime toda la información global del problema (matriz de tiempos, precios de bienes, etc.) en un contexto vectorial denso que guía las decisiones del agente.
*   **Masking**: Fundamental para la eficiencia. Antes de que el agente elija el próximo puerto, se aplica una "máscara" que establece como probabilidad cero (`-inf` en los logits) a:
    *   Puertos ya visitados.
    *   Puertos que son inalcanzables si se quiere volver al puerto de inicio dentro del límite de tiempo.

### Algoritmo de Aprendizaje:

El modelo aprende maximizando la recompensa esperada. La recompensa es el beneficio total calculado por el `RouteScorer`, el cual utiliza una extensión de Rust para realizar cálculos rápidos de trading óptimo dentro de la ruta propuesta.

Para estabilizar el aprendizaje, se utiliza un **Baseline**: se resta el promedio de recompensas pasadas a la recompensa actual. Esto hace que el agente se "anime" a seguir acciones que superan el promedio y "desanime" de las que están por debajo, acelerando la convergencia hacia rutas de alto beneficio.

---

## Estructura del Módulo

*   **`agent.py`**: Define la arquitectura de la red neuronal y la lógica del agente REINFORCE.
*   **`solver.py`**: Implementa la lógica de entrenamiento por instancia y la interfaz de resolución.
*   **`schemas.py`**: Define las estructuras de datos (Pydantic) que representan el problema.
*   **`scoring.py`**: Interfaz con el evaluador de Rust para obtener el beneficio (recompensa) de una ruta.
*   **`train.py`**: Script de utilidad para entrenar el modelo en múltiples instancias de forma general.
