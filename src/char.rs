//!

use ansi_term::Color;
use crate::{compute_leaf_style, ColorDisplay, Format, StyleDesc};
use std::borrow::Cow;
use std::fmt::Write;

impl ColorDisplay<CharFormat> for char {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &CharFormat,
    ) -> std::fmt::Result {
        self.write_indentation(sink, format.indent, format)?;
        write!(sink, "{}", format.prefix)?;
        let style = compute_leaf_style(format.style_desc);
        let d = format.delimiter;
        write!(sink, "{d}{}{d}", style.paint(Cow::Owned(self.to_string())))
    }
}

#[derive(Clone, Copy)]
pub struct CharFormat {
    pub indent: u16,
    pub delimiter: &'static str,
    pub prefix: &'static str,
    pub style_desc: Option<StyleDesc>,
}

impl Format for CharFormat {
    fn colored(indent: u16) -> Self {
        Self {
            indent,
            delimiter: "",
            prefix: "",
            style_desc: Some(StyleDesc {
                color: Color::Green,
                bold: true,
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
    use crate::{Color, ColorDisplay};
    use super::*;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let c = 'C';
        let mut sink = String::with_capacity(1024);
        c.color_fmt(&mut sink, &CharFormat::colored(0))?;
        let expected = format!("{}", Color::Green.bold().paint(&c.to_string()));
        assert_eq!(sink, expected);
        Ok(())
    }
}
