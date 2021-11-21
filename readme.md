# Advent of Code 2021

### Howto

Create src-files in src directory named like `dayN.rs` and implement a fuction called `run` that returns an [`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html) like `pub fn run() -> io::Result<()>`.

Inputs go to `inputs` folder.

Then You include the day in and add a corresponding match in `main.rs`.

Run specific day with `cargo run N`

Please also use [tests](https://doc.rust-lang.org/cargo/guide/tests.html), run them with `cargo test dayN`
