# koji-retriever

## Contents

- [Introduction](#introduction)
- [Versions](#versions)
- [Compilation](#compilation)
- [Usage](#usage)
- [Tests](#tests)
- [Coverage Tests](#coverage-tests)

## Introduction

This simple software allows to download a bunch of packages
generated by koji by just specifying the URL where they stand.

## Versions

- 0.1.0:  First version
- 0.2.0:  Test mode (-t). Allows testing package access without downloading
- 0.3.0:  Code refactoring

## Compilation

Compilation in koji-retriever is executed through *cargo* tool, as usual in Rust:

```bash
$ cargo build
```
For compilation in release mode, use --release flag:

```bash
$ cargo build --release
```

## Usage

```bash

$ ./target/debug/koji-retriever -h

koji-retriever 0.3.0

USAGE:
    koji-retriever [OPTIONS] --url <URL>

OPTIONS:
    -d, --directory <DIRECTORY>
    -h, --help                     Print help information
    -t, --test
    -u, --url <URL>
    -v, --verbose
    -V, --version                  Print version information
```

## Tests

koji-retriever includes minimal tests to check stability.
At this moment, only CLI function tests exist.
Tests in koji-retriever are executed through *cargo* tool, as usual in Rust:

```bash
$ cargo test
...
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/koji_retriever-d8e7d3952c51b846)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/koji-retriever-test.rs (target/debug/deps/koji_retriever_test-631e068f7fce3978)

running 7 tests
test url_does_not_exist_test ... ok
test url_existing_file_does_not_exist_in_test_mode_test ... ok
test url_existing_test_mode_verbose_test ... ok
test url_existing_slash_end_directory_test ... ok
test url_existing_test ... ok
test url_existing_verbose_test ... ok
test url_existing_files_exist_test ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.71s

     Running tests/links-tests.rs (target/debug/deps/links_tests-06a835142a68b0f6)

running 2 tests
test links_not_dowloadable_link_test ... ok
test links_downloadable_link_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.91s

     Running tests/verbose-test.rs (target/debug/deps/verbose_test-ce808bf9693d2745)

running 3 tests
test verbose_disabled_test ... ok
test verbose_test ... ok
test verbose_default_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Option to execute tests in release mode is also possible through *--release* option:
```bash
$ cargo test --release
```

## Coverage Tests
This section describes how to execute coverage tests.

- Install *grcov* and *llvm-tools*: First of all, installation of appropriate tools needs to be done:

```bash
$ cargo install grcov
$ rustup component add llvm-tools-preview
```

- Then, appropriate compilation flags need to be exported:

```bash
$ export RUSTFLAGS="-Cinstrument-coverage"
$ export LLVM_PROFILE_FILE="koji-retriever-%p-%m.profraw"
```

- Finally, execute tests via *cargo test*, generate report with *grcov* (in HTML mode for this example) and open it with your preferred browser:

```bash
$ cargo test
$ grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
$ firefox ./target/debug/coverage/index.html
```
