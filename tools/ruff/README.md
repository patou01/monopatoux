# Ruff bazel

## Idea

Run `ruff` through `bazel run`. Not necessarily valuable but simplifies
repeatability, assuming
the links won't die. 

## Running

Just run `bazel run //tools/ruff:run_ruff -- <the_args>` instead of
`ruff <the_args>`. Note the `--` is important.
