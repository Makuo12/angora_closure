# Environment variables for compiling

- `USE_FAST=1`: use fast mode to compile the program. It includes branch counting, getting the feedback of the fuzzing constraint (the output of its function).
- `USE_TRACK=1`: use taint tracking and collect all constraints.
- `USE_DFSAN=1`: use taint tracking.

# Environment variables for running

- `RUST_LOG=trace`: enable tracing output
- `RUST_LOG=debug`: enable debugging output
