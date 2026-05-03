# PROJECT_REQUIRED

The **PROJECT_REQUIRED** bit asserts that the operation serves a purpose within a larger, admitted workflow or project.

Operations should not happen in a vacuum. A system might be asked to delete a file. The operation might be authorized and consistent. But if that file is part of an active project that explicitly forbids deletion during its lifecycle, the `PROJECT_REQUIRED` bit (or rather, its inverse, checking against project bounds) fails to activate.

This bit ensures actions align with the macro-intent of the system.
