load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello",
    srcs = ["hello.rs"],
)

rust_binary(
    name = "local",
    srcs = ["local.rs"],
    deps = [
        "//mylib",
    ],
)
