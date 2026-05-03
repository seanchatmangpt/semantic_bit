# Named Graph

A **Named Graph** is a graph that has its own identity.

In a complex system, we do not use one giant graph. We use many named graphs, each with a specific purpose.

$$
\boxed{NamedGraph = (Identity, Graph)}
$$

## 17.5 Specializing Memory

Example Named Graphs:
- `urn:graph:identity`: Triples about users and roles.
- `urn:graph:config`: Triples about hardware setup.
- `urn:graph:runtime`: Triples about the current session state.

## 17.6 Graph Selection

The machine can "Select" which named graphs to use for a specific operation. 
"Check access using the `identity` graph and the `policy` graph."
By naming graphs, we can isolate concerns and prevent unauthorized facts from influencing critical decisions.
