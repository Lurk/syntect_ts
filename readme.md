# Syntect SyntaxSet with Typescript support

## Usage

```shell
cargo run
```

Will generate `syntaxes.bin` file. Which can be used from any project like that: 

```rust
let syntax_set: SyntaxSet = from_uncompressed_data(include_bytes!("./syntaxes.bin"))?;
```

## Motivation

For some reason [Syntect](https://github.com/trishume/syntect/) does not have Typescript support 
[by default](https://github.com/trishume/syntect/issues/447). 


