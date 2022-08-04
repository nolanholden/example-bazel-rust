load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "05e15e536cc1e5fd7b395d044fc2dabf73d2b27622fbc10504b7e48219bb09bc",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.8.1/rules_rust-v0.8.1.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.8.1/rules_rust-v0.8.1.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    include_rustc_srcs = True,
    version = "1.62.1",
)

# External crates.io crates:
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

crates_repository(
    name = "crate_index",
    # Update this using:
    # `CARGO_BAZEL_REPIN=true bazel sync --only=crate_index`
    cargo_lockfile = "//:Cargo.Bazel.lock",
    packages = {
        "tempfile": crate.spec(
            version = "3.3.0",
        ),
    },
    # Setting the default package name to `""` forces the use of the macros defined in this repository
    # to always use the root package when looking for dependencies or aliases. This should be considered
    # optional as the repository also exposes alises for easy access to all dependencies.
    render_config = render_config(
        default_package_name = "",
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# Help out rust-analyzer
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_deps")

rust_analyzer_deps()
# and add .vscode/tasks.json
#
# // Reference:
# // https://bazelbuild.github.io/rules_rust/rust_analyzer.html
# {
#     "version": "2.0.0",
#     "tasks": [
#         {
#             "label": "Generate rust-project.json",
#             "command": "bazel",
#             "args": ["run", "@rules_rust//tools/rust_analyzer:gen_rust_project"],
#             "options": {
#                 "cwd": "${workspaceFolder}"
#             },
#             "group": "build",
#             "problemMatcher": [],
#             "presentation": {
#                 "reveal": "never",
#                 "panel": "dedicated",
#             },
#             "runOptions": {
#                 "runOn": "folderOpen"
#             }
#         },
#     ]
# }
