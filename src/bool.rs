//!

use crate::{Color, ColorDisplay, Format, Style};
use std::fmt::Write;

impl ColorDisplay<BoolFormat> for bool {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &BoolFormat,
    ) -> std::fmt::Result {
        self.write_indentation(sink, format.indent, format)?;
        let style = Style::style_from_desc(format.style);
        write!(sink, "{}", style.paint(if *self { "true" } else { "false" }))
    }
}

#[derive(Clone, Copy)]
pub struct BoolFormat {
    pub indent: u16,
    pub prefix: &'static str,
    pub style: Option<Style>,
}

impl Format for BoolFormat {
    fn standard(indent: u16) -> Self {
        Self {
            indent,
            prefix: "",
            style: Some(Style {
                color: Color::Purple,
                bold: true,
                italic: false,
                underline: false,
                dimmed: false,
            }),
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
        b.color_fmt(&mut sink, &BoolFormat::standard(0))?;
        let expected = format!(
            "{}", Color::Purple.bold().paint(format!("{b}"))
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
