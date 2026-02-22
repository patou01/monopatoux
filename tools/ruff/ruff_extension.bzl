load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _ruff_impl(ctx):
    RUFF_VERSION = "0.15.2"
    http_archive(
        name = "ruff_bin",
        sha256 = "278b307eccb4eef6a153d811466dd8170d4fd74970cc4a44c793b40bd897e403",
        urls = [
           "https://github.com/astral-sh/ruff/releases/download/{0}/ruff-x86_64-unknown-linux-gnu.tar.gz".format(RUFF_VERSION),
        ],
    )

ruff = module_extension(
    implementation = _ruff_impl,
)

