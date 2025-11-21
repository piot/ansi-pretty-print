# ansi-pretty-print

A Rust library for pretty-printing structured data with ANSI color support and ligatures. Mostly used for debug outputs.

## Features

- **ANSI Color Support**: Syntax highlighting with customizable colors
- **Indentation Management**: Automatic indentation for nested structures
- **Ligature Support**: Optional use of Unicode ligatures (←, →, ↔, ϕ)
- **Semantic Coloring**: Different colors for keywords, types, literals, operators, etc.
- **Flexible**: Toggle colors and ligatures on/off

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ansi-pretty-print = "0.0.1"
```

## Usage

```rust
use ansi_pretty_print::Printer;
use std::fmt::Write;

let mut output = String::new();
let mut printer = Printer::new(&mut output)
    .with_colors(true)
    .with_ligature(true);

// Print with semantic coloring
printer.keyword("fn")?;
printer.write(" ")?;
printer.function_name("main")?;
printer.punctuation("()")?;

// Use blocks with automatic indentation
printer.block("", |p| {
    p.line("println!(\"Hello, world!\");")?;
    Ok(())
})?;

println!("{}", output);
```

## License

MIT
