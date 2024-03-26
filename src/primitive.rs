//!

use crate::{Color, ColorDisplay, Format, StyleDesc};
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
                    write!(sink, "{}", format.prefix)?;
                    let style = StyleDesc::style_from_desc(format.style_desc);
                    write!(sink, "{}", style.paint(format!("{self}")))
                }
            }

            #[derive(Clone, Copy)]
            pub struct [<$type:camel Format>] {
                pub indent: u16,
                pub prefix: &'static str,
                pub style_desc: Option<StyleDesc>,
            }

            impl Format for [<$type:camel Format>] {
                fn standard(indent: u16) -> Self {
                    Self {
                        indent,
                        prefix: "",
                        style_desc: Some(StyleDesc {
                            color: Color::Blue,
                            bold: true,
                            italic: false,
                            underline: false,
                            dimmed: false,
                        }),
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
    use crate::{Color, ColorDisplay, StyleDesc};
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
                            prefix: "",
                            style_desc: Some(StyleDesc {
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
