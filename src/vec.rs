//!

use super::*;

impl<F, T> ColorDisplay<VecFormat<F>> for Vec<T>
where
    T: ColorDisplay<F>
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &VecFormat<F>,
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

pub struct VecFormat<F> {
    prefix_newlines: u16,
    intersperse_newlines: u16,
    suffix_newlines: u16,
    item: F,
}

#[cfg(test)]
mod test {
    use crate::{primitive::U8Format, Color, ColorDisplay, Style};
    use super::VecFormat;

    #[test]
    fn basic() -> std::fmt::Result {
        let slice: Vec<u8> = vec![10, 20, 30];
        let mut sink = String::with_capacity(1024);
        slice.color_fmt(&mut sink, &VecFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            item: U8Format {
                indent: 0,
                style: Some(Style {
                    color: Color::Purple,
                    bold: false,
                    italic: false,
                    underline: false,
                    dimmed: false,
                })
            }
        })?;
        let expected = format!(
            "\n{}\n{}\n{}\n",
            Color::Purple.paint("10"),
            Color::Purple.paint("20"),
            Color::Purple.paint("30"),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
