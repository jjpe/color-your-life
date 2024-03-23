//!

use crate::{ColorDisplay, Style};
use std::fmt::Write;
use std::borrow::Cow;

impl ColorDisplay<StrFormat> for &'_ str {
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        pargs: &StrFormat,
    ) -> std::fmt::Result {
        self.write_indentation(sink, pargs.indent, pargs)?;
        if let Some(desc) = pargs.style {
            let style = desc.color.normal();
            let style = if desc.bold      { style.bold()      } else { style };
            let style = if desc.italic    { style.italic()    } else { style };
            let style = if desc.underline { style.underline() } else { style };
            let style = if desc.dimmed    { style.dimmed()    } else { style };
            write!(sink, "{}", style.paint(Cow::Borrowed(*self)))
        } else {
            write!(sink, "{}", self)
        }
    }
}

pub struct StrFormat {
    indent: u16,
    style: Option<Style>,
}


#[cfg(test)]
mod test {
    use crate::{Color, ColorDisplay, Style};
    use super::StrFormat;

    #[test]
    fn basic() -> std::fmt::Result {
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
        let expected = format!(
            "{}",
            Color::Red
                .bold()
                .underline()
                .paint("Alice and Bob have a drink, but Alice poisons Bob.")
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
