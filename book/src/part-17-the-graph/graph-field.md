# Graph Field

The **Graph Field** is the intersection of the 8-bit field and the triple graph.

$$
\boxed{\textbf{The field activates the graph. The graph interprets the field.}}
$$

## 17.7 State-Graph Coupling

A status bit in a field (e.g., `Status8::OK`) is a "State". 
A triple in a graph (e.g., `Sensor hasType TemperatureSensor`) is a "Relation".

The **Graph Field** is the mechanism where the current state of a resource (from its 8-bit field) is combined with its relational context (from the graph) to select a high-level condition.

## 17.8 Example: Maintenance Mode

If a machine's field is `OK`, but the graph contains the triple $(Machine, mode, Maintenance)$, the selection rule will override the `OK` state and treat the machine as `BLOCKED`. The graph provides the "Policy" that governs the "Bit".
