# Invalid Combination

An Invalid Combination occurs when the active meanings contradict each other or violate domain laws.
For example, a byte that asserts both `SUCCESS` and `FAILURE` simultaneously in a mutually exclusive operation context represents an invalid combination.

When an invalid combination is detected, the system must enter a safe failure mode or reject the state entirely.
