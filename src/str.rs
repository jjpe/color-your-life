//!

use ansi_term::Color;
use crate::{ColorDisplay, Format, StyleDesc};
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
        let style = StyleDesc::style_from_desc(format.style_desc);
        write!(sink, "{}", style.paint(Cow::Borrowed(*self)))
    }
}

#[derive(Clone, Copy)]
pub struct StrFormat {
    pub indent: u16,
    pub prefix: &'static str,
    pub style_desc: Option<StyleDesc>,
}

impl Format for StrFormat {
    fn standard(indent: u16) -> Self {
        Self {
            indent,
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
