# CLOSED

The **CLOSED** bit asserts that the operation's boundaries are fully known and sealed. 

An operation is not `CLOSED` if it relies on unbounded iteration, unbounded memory allocation, or unbounded external network calls where the response size is unknown.

To activate the `CLOSED` bit, the system must prove that the operation has a finite, predictable cost and scope. If an operation cannot be closed, it cannot be safely reasoned about by a deterministic system.
