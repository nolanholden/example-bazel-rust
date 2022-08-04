Usage:

```sh
$ bazel --version
bazel 5.2.0
$ # Update ./Cargo.Bazel.lock
$ CARGO_BAZEL_REPIN=true bazel sync --only=crate_index
$ bazel run //:hello
$ bazel run //:my_rust_binary
$ bazel test //mylib:tests
$ bazel test //tests:all_tests
```
