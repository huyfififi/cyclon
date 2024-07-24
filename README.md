This command line tool counts Cyclomatic complexity of input files

## Sample output

```
$ cargo run fixtures
cyclon/fixtures/comment.py
Found 1 functions.
0	def a
cyclon/fixtures/normal.py
Found 3 functions.
1	def a
3	def c
2	def b
cyclon/fixtures/err_exceed_seven.py
Found 2 functions.
9	def is_even
8	def is_odd
cyclon/fixtures/nested.py
Found 1 functions.
4	def func
```

## Run commands for development

```
$ cargo run -- /path/to/file
```

## How to install the command for development

1. Compilte the tool

```
$ cargo build --release
```

2. Install the tool

```
$ cargo install --path .
```

3. Add Cargo Bin Directory to your path (if not added yet)

Add the following line to your shell\'s profile file such as `~/.zshrc`

```
export PATH="$HOME/.cargo/bin:$PATH"
```
