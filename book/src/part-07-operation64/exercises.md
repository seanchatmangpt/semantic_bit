# Operation64 Exercises

## Exercise 1: Mapping the Matrix

Draw an 8x8 grid. On the X-axis, list the Eight Verbs: ACTIVATE, SELECT, DEPOSIT, REVOKE, REPLAY, BIND, EMIT, HALT. On the Y-axis, list the Eight Nouns: FIELD, RECORD, RECEIPT, CHANNEL, JOURNAL, AUTHORITY, RELATION, CONSTRUCT.

1. Pick three cells that represent common, valid operations in a system you are familiar with. Describe what they do.
2. Pick three cells that represent "Invalid Combinations" (actions that make no semantic sense). Explain why they are invalid.
3. What is the consequence if a system allows an operation that falls outside of this grid?

## Exercise 2: Tracing the Authority

Assume a user wants to execute `REVOKE AUTHORITY` on a specific identity. 

1. Write the sequence of prior operations that would be necessary to generate the `ConditionCode8` receipt required to authorize this action.
2. What specific checks must the Operation Authority module perform before moving this from a Declared Operation to a Selected Operation?
3. If the authority check fails, what information must be included in the resulting Forbidden Operation Receipt?

## Exercise 3: Defining the Contract

For the operation cell `BIND RELATION` (which links two nouns together):

1. Define a strict, bit-level Operation Input Contract for this cell. What data types are required?
2. How does the system prove that the two nouns being bound actually exist before the operation is selected?
