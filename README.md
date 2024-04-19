# Color Your Life

[![](https://img.shields.io/crates/v/color-your-life?label=color-your-life)](https://crates.io/crates/color-your-life)
![Rust](https://github.com/jjpe/color-your-life/workflows/Rust/badge.svg)
![](https://img.shields.io/badge/rustc-1.26+-darkcyan.svg)
![](https://img.shields.io/crates/l/color-your-life)

## Synopsis

This Rust crate provides a trait-based, `std::fmt::Display`-like architecture
that lets you color as well as format print your datastructure.

## Usage

Add a dependency on this crate to your project's `Cargo.toml`:
``` toml
[dependencies]
color-your-life = "0.7.0"
```

We now can print e.g. strings:
```rust
use color_your_life::{Color, ColorDisplay, str::StrFormat};

// A piece of text to format and color.
let text = "Alice and Bob have a drink.";

// The sink can be any type `T: std::fmt::Write`, including e.g.
// `std::fmt::Formatter<'_>` used in the `std::fmt::Display` and
// `std::fmt::Debug` traits.  Here, using a `String` is convenient.
let mut sink = String::with_capacity(1024);

// The trait `ColorDisplay<StrFormat>` is implemented for `&str`, so we can
// now call `text.color_fmt()`.
// The `text` is written to the `sink`, where the `&StrFormat` value provides
// a description of *how* to format the text, and in particular the `style_desc`
// field describes the color and style to be used.
text.color_fmt(&mut sink, &StrFormat {
    indent: 0,
    delimiter: "",
    prefix: "",
    style_desc: Some(StyleDesc {
        color: Color::Red,
        bold: true,
        italic: false,
        underline: true,
        dimmed: false,
    }),
})?;

let expected = format!("{}", Color::Red.bold().underline().paint(text));
assert_eq!(sink, expected);
```

## Writing a user-defined `impl ColorDisplay<F> for T`:

It is no coincidence that writing a `ColorDisplay<F>` impl for a type works
a lot like writing an impl for `std::fmt::Display`, or `std::fmt::Debug` for
that same type.
It is intended to work a lot like them, as well as to make it easy
to implement e.g. `std::fmt::Display` in terms of `ColorDisplay<F>`.

The new bits, unsurprisingly, deal mostly with the style and coloring
of data values:
``` rust
use color_your_life::{Color, ColorDisplay, Format, StrFormat, StyleDesc};

/// A simple 2-line string. Indentation is managed for both lines at once.
struct TwoLineString {
    line0: String,
    line1: String,
}

impl ColorDisplay<TwoLineStringFormat> for TwoLineString {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &TwoLineStringFormat,
    ) -> std::fmt::Result {

        self.line0.as_str().color_fmt(sink, &StrFormat {
            indent: format.indent, // override
            ..format.line0_format
        })?;
        for _ in 0..format.vertical_spacing {
            writeln!(sink)?;
        }
        self.line1.as_str().color_fmt(sink, &StrFormat {
            indent: format.indent, // override
            ..format.line1_format
        })?;
        Ok(())

    }
}

// This types allows a user to describe how to style,
// color and format values of type `TwoLineString`:
struct TwoLineStringFormat {
    /// The indentation of `self` as a whole.  Overrides
    /// `self::line0_format.indent` and `self::line1_format.indent`.
    pub indent: u16,
    /// The number of newlines to write between the 2 lines.
    /// A value of:
    ///   - 0 means the lines will be concatenated
    ///   - 1 truly separates the 2 lines
    ///   - 2 writes 1 empty line between the 2 lines,
    ///   - etc
    pub vertical_spacing: u8,
    /// Describes how to format the first line
    pub line0_format: StrFormat,
    /// Describes how to format the second line
    pub line1_format: StrFormat,
}

// The `Format` trait gives a user a standard way of creating
// colored as well as monochrome values of type `TwoLineString`.
impl Format for TwoLineStringFormat {
    fn colored(indent: u16) -> Self {
        Self {
            indent,
            vertical_spacing: 1,
            line0_format: StrFormat::colored(indent),
            line1_format: StrFormat::colored(indent),
        }
    }

    fn monochrome(indent: u16) -> Self {
        Self {
            indent,
            vertical_spacing: 1,
            line0_format: StrFormat::monochrome(indent),
            line1_format: StrFormat::monochrome(indent),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_line_string() -> std::fmt::Result {
        let tls = TwoLineString {
            line0: "hello".to_string(),
            line1: "world".to_string(),
        };

        // Perform a write to the sink of a colored representation
        // of `tls` and one newline to separate the 2 lines
        let mut sink = String::with_capacity(1024);
        tls.color_fmt(&mut sink, &&TwoLineStringFormat {
            indent: 0,
            vertical_spacing: 1,
            line0_format: StrFormat::colored(0/*doesn't matter*/),
            line1_format: StrFormat::colored(0/*doesn't matter*/),
        })?;
        let expected = format!(
            "{}\n{}",
            Color::Green.paint("hello"),
            Color::Green.paint("world"),
        );
        assert_eq!(sink, expected);

        // Perform a write to the sink of a partially monochrome representation
        // of `tls` and 2 newlines (i.e. 1 blank line) to separate the 2 lines,
        // as well as 1 horizontal indentation for both lines
        let mut sink = String::with_capacity(1024);
        tls.color_fmt(&mut sink, &&TwoLineStringFormat {
            indent: 1,
            vertical_spacing: 2,
            line0_format: StrFormat::monochrome(0/*doesn't matter*/),
            line1_format: StrFormat::colored(0/*doesn't matter*/),
        })?;
        let expected = format!(
            "    {}\n\n    {}",
            "hello",
            Color::Green.paint("world"),
        );
        assert_eq!(sink, expected);

        Ok(())
    }
}

```
