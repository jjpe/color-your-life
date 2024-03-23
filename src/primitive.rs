//!

use crate::{ColorDisplay, Style};
use std::fmt::Write;

macro_rules! impl_ColorDisplay_for_numeric_types {
    (
        $([impl ColorDisplay<$argtype:ident> for $type:ty]),*
            $(,)?
    ) => {
        $(
            impl ColorDisplay<$argtype> for $type {
                fn color_fmt(
                    &self,
                    sink: &mut impl Write,
                    pargs: &$argtype,
                ) -> std::fmt::Result {
                    self.write_indentation(sink, pargs.indent, pargs)?;
                    if let Some(desc) = pargs.style {
                        let style = desc.color.normal();
                        let style = if desc.bold      { style.bold()      } else { style };
                        let style = if desc.italic    { style.italic()    } else { style };
                        let style = if desc.underline { style.underline() } else { style };
                        let style = if desc.dimmed    { style.dimmed()    } else { style };
                        write!(sink, "{}", style.paint(format!("{self}")))
                    } else {
                        write!(sink, "{}", self)
                    }
                }
            }

            pub struct $argtype {
                pub indent: u16,
                pub style: Option<Style>,
            }
        )*
    }
}

impl_ColorDisplay_for_numeric_types! {
    [impl ColorDisplay<I8Format> for i8],
    [impl ColorDisplay<I16Format> for i16],
    [impl ColorDisplay<I32Format> for i32],
    [impl ColorDisplay<I64Format> for i64],
    [impl ColorDisplay<I128Format> for i128],
    [impl ColorDisplay<IsizeFormat> for isize],
    [impl ColorDisplay<U8Format> for u8],
    [impl ColorDisplay<U16Format> for u16],
    [impl ColorDisplay<U32Format> for u32],
    [impl ColorDisplay<U64Format> for u64],
    [impl ColorDisplay<U128Format> for u128],
    [impl ColorDisplay<UsizeFormat> for usize],
}

#[cfg(test)]
mod test {
    use crate::{Color, ColorDisplay, Style};
    use super::*;

    macro_rules! generate_tests_for_numeric_types {
        ( $($type:ident),* $(,)?) => {
            paste::paste! {
                $(
                    #[test]
                    fn [<basic_ $type:lower>]() -> std::fmt::Result {
                        let num = 42;
                        let mut sink = String::with_capacity(1024);
                        num.color_fmt(&mut sink, &[<$type:camel Format>] {
                            indent: 0,
                            style: Some(Style {
                                color: Color::Yellow,
                                bold: false,
                                italic: true,
                                underline: false,
                                dimmed: true,
                            }),
                        })?;
                        let expected = format!(
                            "{}", Color::Yellow.italic().dimmed().paint("42")
                        );
                        assert_eq!(sink, expected);
                        Ok(())
                    }
                )*
            }
        }
    }

    generate_tests_for_numeric_types! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize,
    }
}
