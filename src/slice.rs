//!

use super::*;

impl<TF, T> ColorDisplay<SliceFormat<TF>> for [T]
where
    T: ColorDisplay<TF>
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &SliceFormat<TF>,
    ) -> std::fmt::Result {
        self.write_newlines(sink, format.prefix_newlines, &format)?;
        for (idx, item) in self.iter().enumerate() {
            if idx > 0 {
                self.write_newlines(sink, format.intersperse_newlines, &format)?;
            }
            item.color_fmt(sink, &format.item_format)?;
        }
        self.write_newlines(sink, format.suffix_newlines, &format)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct SliceFormat<TF> {
    pub prefix_newlines: u16,
    pub intersperse_newlines: u16,
    pub suffix_newlines: u16,
    pub item_format: TF,
}

impl<TF: Format> Format for SliceFormat<TF> {
    fn colored(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            item_format: TF::colored(indent),
        }
    }

    fn monochrome(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            item_format: TF::monochrome(indent),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{
        Color, ColorDisplay,
        primitive::U8Format,
    };
    use super::*;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let slice: &[u8] = &[10, 20, 30];
        let mut sink = String::with_capacity(1024);
        slice.color_fmt(&mut sink, &SliceFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            item_format: U8Format::colored(1),
        })?;
        let expected = format!(
            "\n    {}\n    {}\n    {}\n",
            Color::Blue.bold().paint("10"),
            Color::Blue.bold().paint("20"),
            Color::Blue.bold().paint("30"),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
