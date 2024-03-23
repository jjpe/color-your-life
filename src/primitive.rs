//!

use crate::{Color, ColorDisplay, Style};
use std::fmt::{Alignment, Write};

macro_rules! impl_ColorDisplay_and_add_wrappers_for_numeric_types {
    ($($type:ty),* $(,)?) => { paste::paste! {
        $(
            impl ColorDisplay<[<$type:camel Format>]> for $type {
                fn color_fmt(
                    &self,
                    sink: &mut impl Write,
                    format: &[<$type:camel Format>],
                ) -> std::fmt::Result {
                    self.write_indentation(sink, format.indent, format)?;
                    if let Some(desc) = format.style {
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

            pub struct [<$type:camel Format>] {
                pub indent: u16,
                pub style: Option<Style>,
            }

            impl [<$type:camel Format>] {
                pub fn standard(indent: u16) -> Self {
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

            pub struct [<$type:camel DisplayWrapper>](pub $type);

            impl std::fmt::Display for [<$type:camel DisplayWrapper>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if let (Some(Alignment::Right), Some(width)) =
                        (f.align(), f.width())
                    {
                        let indent: u16 = width.try_into()
                            .map_err(|_| std::fmt::Error)?;
                        let format = [<$type:camel Format>]::standard(indent);
                        self.0.color_fmt(f, &format)
                    } else {
                        Err(std::fmt::Error)
                    }
                }
            }
        )*
    }}
}

impl_ColorDisplay_and_add_wrappers_for_numeric_types! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
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
                        let num = 42 as $type;
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
        f32, f64,
    }

    #[test]
    fn display_u8() -> std::fmt::Result {
        let num = 42;
        let displayed = format!("{:>2}", U8DisplayWrapper(num));
        let expected = format!("        {}", Color::Blue.paint("42"));
        assert_eq!(displayed, expected);
        Ok(())
    }
}
