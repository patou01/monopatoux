# pytest bazel

## Idea

Run `pytest` through `bazel`. It could be done using the `py_test` rule
from `aspect`, but this enables it to be a `run` and not a `testonly` target.

## Running

Just run `bazel run //tools/pytest: -- <the_args>` instead of
`pytest <the_args>`. Note the `--` is important.
