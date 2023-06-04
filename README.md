# Simple `grep` Rust implementation

## What is this repo

This repo was built by following the [chapter 12](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html) of Brown University version of the "The Rust Programming Language" book, which was written by Steve Klabnik and Carol Nichols, with contributions from the Rust Community.

## Usage

-   Run a case **sensitive** search:

```
cargo run -- <search_query> <file_path>
# For example, you may run the command below:
cargo run -- to ./poem.txt
```

-   Run a case **insensitive** search:

```
IGNORE_CASE=1 cargo run -- <search_query> <file_path>
# For example, you may run the command below:
IGNORE_CASE=1 cargo run -- to poem.txt
```

-   You may run the unit tests with the following command:

```
cargo test
```
