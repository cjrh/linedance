# linedance
Read lines from either stdin or filenames provided on the CLI.

This functionality is similar to the
[`fileinput`](https://docs.python.org/3/library/fileinput.html) module in Python.

## Usage

There is an example file in the `example` directory that shows
how you would use this in your own project. This is the example
code:

```rust
// examples/simple_usage.rs
use linedance::input;

fn main() -> std::io::Result<()> {
    for line in input()? {
        println!("{}", line?);
    }

    Ok(())
}
```

Let's build this example first so that we can run it directly.
(It gets cumbersome to test stdin and CLI parameters with `cargo run`.)

```bash
cargo build --example simple_usage
```

The binary `simple_usage` is located in the `target/debug/examples` directory. There is also a text file in the `example` directory that you can use to test the program, called `data.txt`.

First we'll show how to use stdin as the input source:

```bash
echo examples/data.txt | ./target/debug/examples/simple_usage
This is the first message
This is a test message
This is the last message
```

Now we'll show how to use a file as the input source:

```bash
./target/debug/examples/simple_usage --files example/data.txt
```

The output will be the same as the previous example.

When used with the `--files` flag, the program will read the lines from all the files provided on the CLI. This allows it to be used with shell wildcards.

```bash
$ ./target/debug/examples/simple_usage --files examples/*.txt
This is the first message
This is a test message
This is the last message
The first of more messages
The middle of the messages
The last of the more messages
```

## How to use with [Clap](https://clap.rs/)

`linedance` is designed to be used without any special libraries for argument parsing because it is a simple utility. However, it can be used with `clap` if you want to use it with a more complex CLI application.

To use it with `clap`, or any similar library, the basic idea is to just ignore the `--files` CLI parameter in your code. Leave that to `linedance` to handle. Here is an example of how you can do that:

1. First, add both `linedance` and `clap` to your `Cargo.toml`:

   ```toml
   [dependencies]
   linedance = "0.1.0"
   clap = { version = "4.5", features = ["derive"] }
   ```

2. In your `main.rs`, use clap to define your CLI structure, but leave the file handling to linedance:

    ```rust
    use clap::Parser;
    use linedance::input;

    #[derive(Parser)]
    #[command(name = "myapp")]
    #[command(about = "An example application using linedance and clap")]
    struct Cli {
        #[arg(long, help = "Enable verbose mode")]
        verbose: bool,

        #[arg(last = true, help = "Input files")]
        files: Vec<String>,
    }

    fn main() -> std::io::Result<()> {
        let cli = Cli::parse();

        if cli.verbose {
            println!("Verbose mode enabled");
        }

        for line in input()? {
            println!("{}", line?);
        }

        Ok(())
    }
    ```
3. When running your application, use the `--files` flag provided by linedance to specify input files:

    ```bash
    $ ./myapp --verbose --files file1.txt file2.txt
    ```

    Or use stdin:

    ```bash
    $ echo "some input" | ./myapp --verbose
    ``````




