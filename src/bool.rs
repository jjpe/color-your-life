//!

use crate::{compute_leaf_style, Color, ColorDisplay, Format, StyleDesc};
use std::fmt::Write;

impl ColorDisplay<BoolFormat> for bool {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &BoolFormat,
    ) -> std::fmt::Result {
        self.write_indentation(sink, format.indent, format)?;
        let style = compute_leaf_style(format.style_desc);
        write!(sink, "{}", style.paint(if *self { "true" } else { "false" }))
    }
}

#[derive(Clone, Copy)]
pub struct BoolFormat {
    pub indent: u16,
    pub prefix: &'static str,
    pub style_desc: Option<StyleDesc>,
}

impl Format for BoolFormat {
    fn colored(indent: u16) -> Self {
        Self {
            indent,
            prefix: "",
            style_desc: Some(StyleDesc {
                color: Color::Purple,
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
        let b = true;
        let mut sink = String::with_capacity(1024);
        b.color_fmt(&mut sink, &BoolFormat::colored(0))?;
        let expected = format!(
            "{}", Color::Purple.bold().paint(format!("{b}"))
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
