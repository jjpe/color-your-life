//!

use ansi_term::Color;
use crate::{ColorDisplay, Format, Style};
use std::borrow::Cow;
use std::fmt::Write;

impl ColorDisplay<StrFormat> for &'_ str {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &StrFormat,
    ) -> std::fmt::Result {
        self.write_indentation(sink, format.indent, format)?;
        let style = format.calculate_style();
        write!(sink, "{}", style.paint(Cow::Borrowed(*self)))
    }
}

#[derive(Clone, Copy)]
pub struct StrFormat {
    pub indent: u16,
    pub style: Option<Style>,
}

impl StrFormat {
    fn calculate_style(&self) -> ansi_term::Style {
        if let Some(desc) = self.style {
            let style = desc.color.normal();
            let style = if desc.bold      { style.bold()      } else { style };
            let style = if desc.italic    { style.italic()    } else { style };
            let style = if desc.underline { style.underline() } else { style };
            let style = if desc.dimmed    { style.dimmed()    } else { style };
            style
        } else {
            ansi_term::Style::default()
        }
    }
}

impl Format for StrFormat {
    fn standard(indent: u16) -> Self {
        Self {
            indent,
            style: Some(Style {
                color: Color::Blue,
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
    use crate::{Color, ColorDisplay, Style};
    use super::*;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let s = "Alice and Bob have a drink, but Alice poisons Bob.";
        let mut sink = String::with_capacity(1024);
        s.color_fmt(&mut sink, &StrFormat {
            indent: 0,
            style: Some(Style {
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
