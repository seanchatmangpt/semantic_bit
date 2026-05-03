# Condition to Operation

Once a singular `ConditionCode8` has been selected, the Spine proceeds to determine what action, if any, is required. This is the domain of **Operation64**.

The mapping from Condition to Operation is not arbitrary. A specific Condition (like `RETRY`) dictates the boundaries of the admitted Operation. 

If the Condition is `OK`, an `Operation64` cell (a combination of one of 8 Nouns and 8 Verbs) is selected. However, this selection is merely a declaration of intent. It is not an authorization to execute. It defines *what* is requested, bounded by the 64 available cells, setting the stage for evaluating the necessary relationships and authority.
