To run:

`cargo build --release --bin main`

Runs in about ~51 seconds (single threaded) and ~13 seconds (with parallelization via rayon) for a 400x200 image with 100*antialiasing.
