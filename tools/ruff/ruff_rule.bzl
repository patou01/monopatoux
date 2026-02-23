def _get_ruff_impl(rctx):
    rctx.file("BUILD.bazel",
'''filegroup(
    name = "ruff",
    srcs = ["ruff-x86_64-unknown-linux-gnu/ruff"],
    visibility = ["//visibility:public"],
)
''',
    )
    url = "https://github.com/astral-sh/ruff/releases/download/{version}/ruff-x86_64-unknown-linux-gnu.tar.gz".format(version = rctx.attr.version)
    result = rctx.download_and_extract(
        url = url,
        sha256 = rctx.attr.sha256,
    )
    if result.success != True:
        fail("Failed to download and extract ruff")

get_ruff = repository_rule(
    attrs = {
        "sha256": attr.string(mandatory = True),
        "version": attr.string(mandatory = True),
    },
    implementation = _get_ruff_impl,
)
