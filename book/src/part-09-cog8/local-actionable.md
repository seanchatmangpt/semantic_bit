# LOCAL_ACTIONABLE

The **LOCAL_ACTIONABLE** bit asserts that the current node, thread, or process actually possesses the physical or logical resources required to execute the operation *right now*.

An operation might be authorized and evidenced, but if the local disk is full, or the required channel is locked, it is not `LOCAL_ACTIONABLE`. This bit distinguishes between global validity and local capability.
