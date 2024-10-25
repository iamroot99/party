# party - Run a Party of Commands

**party** is a minimal command runner that aims to automate running a repetitive sequence of commands (usually during development).

It is designed to run a set of default or user-defined commands either sequentially or in parallel.

## Installation

```bash
cargo install --locked party
```

## How To Run

### Party with default commands

```bash
party run
```

By default `party run` will run sequentially the commands:
  1. `cargo fmt`
  2. `cargo clippy -- -Dwarnings`
  3. `cargo test`

### Run a custom party

`party` looks in the project for a `party.toml` file (or a specific file if run with `-f`).
For simplicity, the command below will create a `party.toml` file with the default commands in the current directory:
```bash
party init
```

This generates a local `party.toml`:

```toml
[[tasks]]
command = "cargo fmt"

[[tasks]]
command = "cargo clippy -- -Dwarnings"

[[tasks]]
command = "cargo test"
```

> [!IMPORTANT]  
> If using party <= 0.1.4 the syntax for the command is
> ```toml
> command = ["command", "arg1", "arg2", "arg3"]
> ``` 

Once the file is ready, invoking `party run` will run your custom party of commands!
To run a single party command, use `party run -i [COMMAND_NUMBER]`.

### Validate the command party is good

To validate the configuration file is correct or to just check what commands would be run, a summary can be generated by running:
```bash
party info
```

### Run a parallel party

Sometimes commands are independent and can be run in parallel to save a bit of time. `party` allows this via the `parallel` flag in the configuration flag.

In the example below, the second and third command are independent and have the `parallel` flag set to true. If the flag is missing, it is considered to be false.

```toml
[[tasks]]
command = "cargo fmt"

[[tasks]]
command = "cargo clippy -- -Dwarnings"
parallel = true

[[tasks]]
command = "cargo test"
parallel = true

[[tasks]]
command = "cat results.txt"
```

The commands that are paralelised in the configuration have a `[P]` tag in `party info`:
```
[ ][1/4]: cargo fmt
[P][2/4]: cargo clippy -- -Dwarnings
[P][3/4]: cargo test
[ ][4/4]: cat results.txt
```

For simplicity, `party` batches commands in subparties in the following way:
* A command that has the `parallel` flag set to false is run in its own batch
* All consecutive commands with the flag set to true are batched together
* The next command with the flag set to false is run in its own batch

Use `party batch` to verify this:
```
4 tasks will be run in 3 batches. All tasks in a batch are run in parallel.
Batch [1/3] with 1 commands:
  - cargo fmt
Batch [2/3] with 2 commands:
  - cargo clippy -- -Dwarnings
  - cargo test
Batch [3/3] with 1 commands:
  - cat results.txt
```

For more information, run `party help`, or `party [COMMAND] --help`.
