# Dependency Relation

One of the most critical uses of Relation64 is expressing **Dependency**. 

A Dependency Relation asserts that the Source cannot be fully evaluated, activated, or selected until the Target has reached a specific state. 

For instance, consider a `SUBJECT` attempting an `EVENT` (an operation). That operation might have a Dependency Relation on a `PROOF` (a prior receipt). 

Dependencies must form a Directed Acyclic Graph (DAG). If A depends on B, and B depends on A, the system enters an infinite loop, breaking the requirement for determinism. The Relation64 grid allows us to enforce DAG properties by analyzing the categories. If we strictly define that `EVENT` can depend on `PROOF`, but `PROOF` can never depend on `EVENT` (only on `STATE`), we structurally prevent certain classes of circular dependencies before any logic is executed.

When the system encounters a Dependency Relation, it halts the evaluation of the Source, traverses to the Target, evaluates the Target, and only returns to the Source if the Target yields an `OK` condition. If the Target yields `BLOCKED` or `ABEND`, the dependency fails, and the Source inherits the failure.
