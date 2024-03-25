//!

use super::*;

impl<TF, T> ColorDisplay<VecFormat<TF>> for Vec<T>
where
    T: ColorDisplay<TF>,
    TF: Clone,
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &VecFormat<TF>,
    ) -> std::fmt::Result {
        self.as_slice().color_fmt(sink, &crate::slice::SliceFormat {
            prefix_newlines: format.prefix_newlines,
            intersperse_newlines: format.intersperse_newlines,
            suffix_newlines: format.suffix_newlines,
            item_format: format.item_format.clone(),
        })
    }
}

#[derive(Clone, Copy)]
pub struct VecFormat<TF> {
    pub(crate) prefix_newlines: u16,
    pub(crate) intersperse_newlines: u16,
    pub(crate) suffix_newlines: u16,
    pub(crate) item_format: TF,
}

#[cfg(test)]
mod test {
    use crate::{primitive::U8Format, Color, ColorDisplay, Style};
    use super::VecFormat;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let slice: Vec<u8> = vec![10, 20, 30];
        let mut sink = String::with_capacity(1024);
        slice.color_fmt(&mut sink, &VecFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            item_format: U8Format {
                indent: 0,
                prefix: "",
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
