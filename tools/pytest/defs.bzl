load("@aspect_rules_py//py:defs.bzl", "py_binary")

def run_py_test(name, **kwargs):
    py_binary(
        name = name,
        srcs = ["//tools/pytest:run_pytest.py"],
        main = "//tools/pytest:run_pytest.py",
        **kwargs
    )
