# Authority Relation

The **Authority Relation** answers the question: *By what right does this connection exist?*

While a Dependency Relation blocks execution until a target is evaluated, an Authority Relation specifically links an action to a permission. It usually connects a `SUBJECT` or an `EVENT` to a `RULE` or a `PROOF`.

For example, when a user (`SUBJECT`) attempts to access a channel (`LOCUS`), there must be an Authority Relation linking them. This relation isn't just a generic line on a diagram; it is a specific cell in the 64-cell grid, and it carries the strict requirement that a valid `ConditionCode8` receipt must back the claim of authority.

If a process attempts to execute an Operation64 cell without an established Authority Relation between the actor and the target, the Operation64 module immediately categorizes it as a Forbidden Operation. The Relation64 spine provides the map that Operation64 uses to verify permission.
