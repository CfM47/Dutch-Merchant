# Diseño y Análisis de la Solución de Aprendizaje por Refuerzo

Este documento detalla el diseño, la arquitectura y la formalización de la solución basada en **Aprendizaje por Refuerzo (Reinforcement Learning - RL)** para el Problema del Comerciante Holandés. Este enfoque se enmarca dentro de la categoría de *Neural Combinatorial Optimization*, donde se entrena una red neuronal para construir soluciones heurísticas de alta calidad mediante aprendizaje.

## 1. Formalización como Proceso de Decisión de Markov (MDP)

Para aplicar RL, modelamos la construcción de la ruta del comerciante como un problema de toma de decisiones secuencial.

*   **Estado ($S_t$)**: El estado en el paso $t$ se define por la tupla $(v_t, \mathcal{I}, \mathcal{V}_t)$, donde:
    *   $v_t$: Es el puerto actual en el que se encuentra el agente.
    *   $\mathcal{I}$: Representa la información estática de la instancia del problema (matrices de tiempos de viaje, precios de compra/venta, capacidades, límites de tiempo, etc.).
    *   $\mathcal{V}_t$: Es el conjunto de puertos ya visitados o inalcanzables debido a las restricciones de tiempo.
    *   **Nota sobre el Inventario**: Se excluye deliberadamente el inventario actual del vector de estado. Este diseño desacopla el problema de **navegación** (decisiones de la red neuronal) del de **comercio** (decisiones del evaluador logístico). La red aprende a proponer rutas topológicamente prometedoras (ej. conectar puertos de oferta barata con demanda cara) asumiendo que el evaluador \textit{a posteriori} optimizará las cantidades exactas de carga y descarga.

*   **Acción ($A_t$)**: La acción corresponde a la elección del siguiente puerto $v_{t+1}$ a visitar desde el puerto actual $v_t$.
    *   El espacio de acciones es el conjunto de todos los puertos disponibles $\mathcal{P} = \{0, \dots, N-1\}$.
    *   Se aplica un *enmascaramiento* (masking) para asegurar la validez de las acciones: solo se permite viajar a puertos no visitados tal que el tiempo de viaje más el tiempo de retorno al inicio no exceda el límite $T$.

*   **Transición**: Dado el estado actual y la elección del puerto siguiente, el sistema transita determinísticamente al nuevo puerto, actualizando el tiempo consumido acumulado.

*   **Recompensa ($R$)**: La recompensa se modela como el **beneficio total** obtenido por la ruta completa generada.
    *   A diferencia de problemas de RL clásicos con recompensas inmediatas, aquí tratamos con una recompensa episódica al final de la secuencia.
    *   La recompensa $R(\tau)$ para una trayectoria completa $\tau = (v_0, v_1, \dots, v_k, v_0)$ se calcula utilizando el evaluador exacto del problema (implementado en Rust), que resuelve el subproblema de flujo de carga y descarga óptimo para esa secuencia de puertos fija.

## 2. Arquitectura del Modelo Neuronal

Utilizamos una arquitectura de red neuronal profunda diseñada para procesar la información de la instancia y tomar decisiones de navegación. El modelo parametriza una política estocástica $\pi_\theta(a|s)$.

### 2.1. Encoder de Instancia (Context Encoder)

El primer componente es un codificador que transforma las características crudas de la instancia en un vector de contexto denso (embedding).

*   **Entrada**: Un vector concatenado que contiene:
    *   Matriz de tiempos de viaje (aplanada).
    *   Vectores de pesos de los bienes.
    *   Matrices de precios de compra y venta.
    *   Matrices de capacidades de compra y venta.
    *   Costos de visita y parámetros globales ($T$, capacidad del barco, capital inicial).
*   **Procesamiento**: Un Perceptrón Multicapa (MLP) con funciones de activación ReLU.
*   **Salida**: Un vector de características latentes $h_{context}$ que resume la "topología comercial" del problema.

### 2.2. Policy Network (Actor)

La red de política decide el siguiente movimiento basándose en el contexto global y la ubicación actual.

*   **Embedding de Puerto Actual**: El puerto actual $v_t$ se codifica mediante *One-Hot Encoding* y se pasa por una capa lineal para obtener un vector denso $h_{port}$.
*   **Decodificador**:
    *   Concatena el contexto de la instancia $h_{context}$ con el embedding del puerto actual $h_{port}$.
    *   Pasa esta información combinada a través de capas densas (MLP).
    *   **Salida**: Un vector de *logits* de tamaño $N$ (número de puertos), que representa la preferencia no normalizada por cada posible puerto destino.

### 2.3. Mecanismo de Enmascaramiento y Selección

Para garantizar la factibilidad de las soluciones:
1.  Se calcula una **máscara de validez** $M_t \in \{0, 1\}^N$, donde $M_t(i) = 1$ si el puerto $i$ es visitable (no visitado previamente y dentro del alcance temporal), y $0$ en caso contrario.
2.  Los logits correspondientes a puertos inválidos se establecen en $-\infty$.
3.  Se aplica una función **Softmax** para obtener una distribución de probabilidad válida sobre los puertos siguientes:
    $$ \pi_\theta(v_{t+1} | v_t, \mathcal{I}) = \text{Softmax}(\text{Logits}) $$

### 2.4. Especificaciones Técnicas y Capas de la Red

La implementación inicial en `PyTorch` se estructura en dos módulos principales con las siguientes dimensiones y características:

*   **Hiperparámetros Base**:
    *   Dimensión de Embedding ($d_{emb}$): 128
    *   Dimensión Oculta ($d_{hidden}$): 1024
    *   Función de Activación: ReLU
    *   Optimizador: Adam con learning rate $\alpha = 1\times 10^{-4}$

*   **1. Instance Encoder Global (Pre-procesamiento)**:
    *   Responsable de comprimir el vector plano de características de la instancia en un espacio latente inicial.
    *   `Linear(Input_Size, Hidden_Dim)` $\to$ `ReLU`
    *   `Linear(Hidden_Dim, Embedding_Dim * 2)`
    *   *Salida*: Tensor de características de instancia de tamaño $2 \times d_{emb}$.

*   **2. Policy Network (Decisor Paso a Paso)**:
    *   **Port Embedding**: `Linear(N, Embedding_Dim)`. Proyecta el *one-hot* del puerto actual.
    *   **Context Processor**: Procesa nuevamente las características de la instancia para el contexto actual.
        *   `Linear(Embedding_Dim * 2, Hidden_Dim)` $\to$ `ReLU`
        *   `Linear(Hidden_Dim, Hidden_Dim)` $\to$ `ReLU`
    *   **Decoder (Generador de Logits)**: Combina el contexto y el estado del puerto.
        *   Entrada: Concatenación de [Contexto ($d_{hidden}$) + Port Embedding ($d_{emb}$)].
        *   `Linear(Hidden_Dim + Embedding_Dim, Hidden_Dim)` $\to$ `ReLU`
        *   `Linear(Hidden_Dim, N_Ports)`
        *   *Salida*: Logits brutos antes del Softmax.

## 3. Algoritmo de Entrenamiento Base: REINFORCE

El entrenamiento base se realiza utilizando el algoritmo de gradiente de política **REINFORCE**, optimizando directamente la esperanza del beneficio.

### 3.1. Función Objetivo
El objetivo es maximizar el beneficio esperado $J(\theta)$:
$$ J(\theta) = \mathbb{E}_{\tau \sim \pi_\theta} [R(\tau)] $$

El gradiente se estima mediante:
$$ \nabla_\theta J(\theta) \approx \frac{1}{M} \sum_{i=1}^M \sum_{t=0}^{T-1} \nabla_\theta \log \pi_\theta(a_t^{(i)} | s_t^{(i)}) \cdot (R(\tau^{(i)}) - b) $$
Donde:
*   $R(\tau^{(i)})$ es el beneficio total de la ruta generada $i$.
*   $b$ es una **línea base (baseline)**, calculada como el promedio móvil exponencial de las recompensas pasadas. Esto reduce la varianza de la estimación del gradiente sin introducir sesgo.

### 3.2. Estrategia de Exploración
Para evitar óptimos locales prematuros, implementamos una estrategia de **Temperature Scaling** en la función Softmax:
$$ \pi(a_i) = \frac{\exp(z_i / \tau)}{\sum_j \exp(z_j / \tau)} $$
*   La temperatura $\tau$ comienza en un valor alto ($40.0$), promoviendo una exploración casi uniforme.
*   Decae exponencialmente hacia un valor bajo ($0.1$) durante el entrenamiento, haciendo que la política se vuelva más determinista y explote las mejores rutas encontradas.

### 3.3. Entrenamiento "On-the-Fly" (Active Search)
El modelo se entrena específicamente para cada instancia del problema ("Active Search"). En lugar de aprender una política general para cualquier grafo (lo cual es muy complejo dado la variabilidad de precios y tiempos), el solver:
1.  Inicializa los pesos de la red.
2.  Realiza múltiples iteraciones de entrenamiento sobre la **misma instancia**.
3.  Mantiene un registro de la mejor solución (best-so-far) encontrada durante el proceso de exploración estocástica.

## 4. Mejora del Modelo: Arquitectura V2 con Memory Replay

Para mejorar la estabilidad y capacidad de convergencia del modelo base, se desarrolló una versión actualizada (V2) que incorpora una arquitectura más profunda y un mecanismo de **Experience Replay**.

### 4.1. Nueva Arquitectura Profunda (V2)
Se ha incrementado significativamente la capacidad de la red para capturar relaciones complejas en el espacio de estados:

*   **Dimensiones Aumentadas**:
    *   Embedding Dimension: $128 \to 512$
    *   Hidden Dimension: $1024 \to 2048$
*   **Profundidad Adicional**: Se añadieron capas lineales adicionales tanto en el codificador de contexto como en el decodificador para permitir un razonamiento más abstracto.
    *   *Context Processor V2*: `Linear` -> `ReLU` -> `Linear` -> `ReLU` -> `Linear` -> `ReLU`
    *   *Decoder V2*: `Linear` -> `ReLU` -> `Linear` -> `ReLU` -> `Linear`

### 4.2. Memory Replay: Refuerzo de Soluciones Élite

Una mejora fundamental es la incorporación de un **buffer de memoria** que almacena las mejores soluciones encontradas. Este mecanismo permite re-entrenar periódicamente sobre trayectorias de alta calidad.

#### 4.2.1. Estructura y Mecanismo
Se mantiene una cola de prioridad $\mathcal{M}$ de tamaño fijo $K=4$ que almacena las mejores rutas $(\tau, R(\tau))$.
Con frecuencia configurable (cada 4 epochs), se ejecuta una fase de replay:

```
Para cada ruta élite en memoria:
    1. Forzar la generación de la ruta (Forced Solution)
    2. Calcular log-ratios con temperatura estable (1.0)
    3. Actualizar la política usando la recompensa almacenada
```

#### 4.2.2. Beneficios Observados
*   **Estabilización**: Las actualizaciones frecuentes sobre soluciones de alta calidad anclan la política hacia regiones prometedoras.
*   **Aceleración**: Reduce la varianza del gradiente al reforzar patrones exitosos múltiples veces.
