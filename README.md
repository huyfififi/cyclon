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
