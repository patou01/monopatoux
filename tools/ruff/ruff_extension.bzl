load("//tools/ruff:ruff_rule.bzl", "get_ruff")

def _ruff_impl(module_ctx):
    for module in module_ctx.modules:
        for attr in module.tags.base:
            attrs = {
                key: getattr(attr, key)
                for key in dir(attr)
                if not key.startswith("_")
            }
            get_ruff(**attrs)

_ruff_attrs = {
    "name": attr.string(default = "ruff"),
    "sha256": attr.string(mandatory = True),
    "version": attr.string(mandatory = True),
}

ruff = module_extension(
    implementation = _ruff_impl,
    tag_classes = {
        "base": tag_class(attrs = _ruff_attrs),
    },
)
