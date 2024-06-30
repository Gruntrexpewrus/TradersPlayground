# Useful commands for RUST
learning from [link](https://rust-book.cs.brown.edu/experiment-intro.html)!

run a program (basics not advices):
- $ rustc programname.rs
- $ ./programname

## How to run a program (good way):
- $ cargo new repo (it initalizes a git if not already in one repo, but good for setup)
- $ cd repo
- $ cargo build (compiling)
- $ ./target/debug/file (execute file, not advices)
- $ cargo run (adviced to run file)
- $ cargo check (check for bugs not execute, run often to check the program compiles)
- **ready for release?** $ cargo build --release


## Work on existing propjects?
. In fact, to work on any existing projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build:
- $ git clone example.org/someproject
- $ cd someproject
- $ cargo build



