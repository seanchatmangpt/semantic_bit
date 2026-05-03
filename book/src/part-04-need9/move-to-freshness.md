# Move to Freshness

Sometimes, the ninth meaning is temporal: "Is this state still valid?" A **Move to Freshness** takes the concept of time and expiration out of the 8-bit multiplex and ties it to a verifiable timestamp or generation sequence. 

Freshness is evaluated against a clock or monotonic sequence at the time of Selection, removing the burden of maintaining continuous temporal state from the 8-bit field itself.