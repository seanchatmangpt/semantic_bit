# Graph Closure

**Graph Closure** is the process of expanding a graph by applying inference rules.

$$
\boxed{\textbf{Closure is the discovery of implicit truth.}}
$$

## 17.9 Inference as Motion

If we have:
1. $(User, hasRole, Admin)$
2. $(Admin, hasPermission, Shutdown)$

The **Closure** of this graph includes the implicit fact:
3. $(User, hasPermission, Shutdown)$

## 17.10 The Bounds of Closure

Closure can be dangerous if not bounded. We only perform closure over admitted, deterministic rules. We do not "Guess" relations; we compute them from the existing triples. This ensures that the closure is as stable and verifiable as the original facts.
