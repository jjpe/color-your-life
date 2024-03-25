//!

use crate::{Color, ColorDisplay, Format, Style};
use std::fmt::Write;

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
                    let style = format.calculate_style();
                    write!(sink, "{}", style.paint(format!("{self}")))
                }
            }

            #[derive(Clone, Copy)]
            pub struct [<$type:camel Format>] {
                pub indent: u16,
                pub style: Option<Style>,
            }

            impl Format for [<$type:camel Format>] {
                fn standard(indent: u16) -> Self {
                    Self {
                        indent,
                        style: Some(Style {
                            color: Color::Blue,
                            bold: true,
                            italic: false,
                            underline: false,
                            dimmed: false,
                        }),
                    }
                }
            }

            impl [<$type:camel Format>] {
                fn calculate_style(&self) -> ansi_term::Style {
                    if let Some(desc) = self.style {
                        let style = desc.color.normal();
                        let style = if desc.bold {
                            style.bold()
                        } else {
                            style
                        };
                        let style = if desc.italic {
                            style.italic()
                        } else {
                            style
                        };
                        let style = if desc.underline {
                            style.underline()
                        } else {
                            style
                        };
                        let style = if desc.dimmed {
                            style.dimmed()
                        } else {
                            style
                        };
                        style
                    } else {
                        ansi_term::Style::default()
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
                    fn [<color_fmt_ $type:lower>]() -> std::fmt::Result {
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
}
