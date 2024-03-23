//!

use super::*;

impl<F, T> ColorDisplay<SliceFormat<F>> for [T]
where
    T: ColorDisplay<F>
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &SliceFormat<F>,
    ) -> std::fmt::Result {
        self.write_newlines(sink, format.prefix_newlines, &format)?;
        for (idx, item) in self.iter().enumerate() {
            if idx > 0 {
                self.write_newlines(sink, format.intersperse_newlines, &format)?;
            }
            item.color_fmt(sink, &format.item)?;
        }
        self.write_newlines(sink, format.suffix_newlines, &format)?;
        Ok(())
    }
}

pub struct SliceFormat<F> {
    prefix_newlines: u16,
    intersperse_newlines: u16,
    suffix_newlines: u16,
    item: F,
}

#[cfg(test)]
mod test {
    use crate::{primitive::U8Format, Color, ColorDisplay, Style};
    use super::SliceFormat;

    #[test]
    fn basic() -> std::fmt::Result {
        let slice: &[u8] = &[10, 20, 30];
        let mut sink = String::with_capacity(1024);
        slice.color_fmt(&mut sink, &SliceFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            item: U8Format {
                indent: 0,
                style: Some(Style {
                    color: Color::Blue,
                    bold: false,
                    italic: false,
                    underline: false,
                    dimmed: false,
                })
            }
        })?;
        let expected = format!(
            "\n{}\n{}\n{}\n",
            Color::Blue.paint("10"),
            Color::Blue.paint("20"),
            Color::Blue.paint("30"),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
