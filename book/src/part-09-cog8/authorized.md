# AUTHORIZED

The **AUTHORIZED** bit asserts that the relationship between the actor (source) and the target is valid for the requested verb.

This maps directly back to `Relation64`. The cognitive field looks at the active relations and checks them against the `Operation64` contract. If the contract requires an `Authority` relation, and the relation matrix shows it exists, the `AUTHORIZED` bit is set active.

Authorization is not a string match on a role; it is a structural relation check.
