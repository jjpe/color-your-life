//!
//!

use super::*;
use std::result::Result;

impl<OF, EF, O, E> ColorDisplay<ResultFormat<OF, EF>> for Result<O, E>
where
    O: ColorDisplay<OF>,
    E: ColorDisplay<EF>,
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &ResultFormat<OF, EF>,
    ) -> std::fmt::Result {
        match self {
            // TODO: paint `ok` in green and `err` in red
            Ok(ok) => {
                write!(sink, "{}", format.ok_prefix)?;
                ok.color_fmt(sink, &format.ok_format)?;
            },
            Err(err) => {
                write!(sink, "{}", format.err_prefix)?;
                err.color_fmt(sink, &format.err_format)?;
            },
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct ResultFormat<OF, EF> {
    pub ok_prefix: &'static str,
    pub ok_format: OF,
    pub err_prefix: &'static str,
    pub err_format: EF,
}

impl<OF: Format, EF: Format> Format for ResultFormat<OF, EF> {
    fn colored(indent: u16) -> Self {
        Self {
            ok_prefix: "✅ ",
            ok_format: OF::colored(indent),
            err_prefix: "❌ ",
            err_format: EF::colored(indent),
        }
    }

    fn monochrome(indent: u16) -> Self {
        Self {
            ok_prefix: "✅ ",
            ok_format: OF::monochrome(indent),
            err_prefix: "❌ ",
            err_format: EF::monochrome(indent),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{
        Color, ColorDisplay,
        str::StrFormat,
    };
    use super::*;

    #[test]
    fn ok_color_fmt() -> std::fmt::Result {
        let ok_result: Result<&str, &str> = Ok("hello");
        let mut sink = String::with_capacity(1024);
        ok_result.color_fmt(&mut sink, &ResultFormat{
            ok_format: StrFormat {
                style_desc: Some(StyleDesc {
                    color: Color::Green,
                    bold: false,
                    italic: false,
                    underline: false,
                    dimmed: false,
                }),
                ..StrFormat::colored(0)
            },
            err_format: StrFormat {
                style_desc: Some(StyleDesc {
                    color: Color::Red,
                    bold: true,
                    italic: false,
                    underline: false,
                    dimmed: false,
                }),
                ..StrFormat::colored(0)
            },
            ..ResultFormat::colored(1)
        })?;
        let expected = format!("✅ {}", Color::Green.paint("hello"));
        assert_eq!(sink, expected);
        Ok(())
    }

    #[test]
    fn err_color_fmt() -> std::fmt::Result {
        let ok_result: Result<&str, &str> = Err("oh no");
        let mut sink = String::with_capacity(1024);
        ok_result.color_fmt(&mut sink, &ResultFormat{
            ok_format: StrFormat {
                style_desc: Some(StyleDesc {
                    color: Color::Green,
                    bold: false,
                    italic: false,
                    underline: false,
                    dimmed: false,
                }),
                ..StrFormat::colored(0)
            },
            err_format: StrFormat {
                style_desc: Some(StyleDesc {
                    color: Color::Red,
                    bold: true,
                    italic: false,
                    underline: false,
                    dimmed: false,
                }),
                ..StrFormat::colored(0)
            },
            ..ResultFormat::colored(1)
        })?;
        let expected = format!("❌ {}", Color::Red.bold().paint("oh no"));
        assert_eq!(sink, expected);
        Ok(())
    }
}
