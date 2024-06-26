//!

use ansi_term::Color;
use crate::{compute_leaf_style, ColorDisplay, Format, StyleDesc};
use std::borrow::Cow;
use std::fmt::Write;

impl ColorDisplay<StrFormat> for &'_ str {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &StrFormat,
    ) -> std::fmt::Result {
        self.write_indentation(sink, format.indent, format)?;
        write!(sink, "{}", format.prefix)?;
        let style = compute_leaf_style(format.style_desc);
        let d = format.delimiter;
        write!(sink, "{d}{}{d}", style.paint(Cow::Borrowed(*self)))
    }
}

#[derive(Clone, Copy)]
pub struct StrFormat {
    pub indent: u16,
    pub delimiter: &'static str,
    pub prefix: &'static str,
    pub style_desc: Option<StyleDesc>,
}

impl Format for StrFormat {
    fn colored(indent: u16) -> Self {
        Self {
            indent,
            delimiter: "",
            prefix: "",
            style_desc: Some(StyleDesc {
                color: Color::Green,
                bold: false,
                italic: false,
                underline: false,
                dimmed: false,
            }),
        }
    }

    fn monochrome(indent: u16) -> Self {
        Self {
            indent,
            delimiter: "",
            prefix: "",
            style_desc: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Color, ColorDisplay, StyleDesc};
    use super::*;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let s = "Alice and Bob have a drink, but Alice poisons Bob.";
        let mut sink = String::with_capacity(1024);
        s.color_fmt(&mut sink, &StrFormat {
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
        let expected = format!("{}", Color::Red.bold().underline().paint(s));
        assert_eq!(sink, expected);
        Ok(())
    }
}
