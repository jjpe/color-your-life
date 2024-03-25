//!

use super::*;
use std::collections::BTreeSet;

impl<TF, T> ColorDisplay<BTreeSetFormat<TF>> for BTreeSet<T>
where
    T: ColorDisplay<TF>
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &BTreeSetFormat<TF>,
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
pub struct BTreeSetFormat<TF> {
    pub prefix_newlines: u16,
    pub intersperse_newlines: u16,
    pub suffix_newlines: u16,
    pub item_format: TF,
}

impl<TF: Format> Format for BTreeSetFormat<TF> {
    fn standard(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            item_format: TF::standard(indent),
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
        let mut set: BTreeSet<u8> = BTreeSet::new();
        set.insert(10);
        set.insert(20);
        set.insert(30);
        let mut sink = String::with_capacity(1024);
        set.color_fmt(&mut sink, &BTreeSetFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            item_format: U8Format {
                prefix: "->",
                ..U8Format::standard(1)
            },
        })?;
        let expected = format!(
            "\n    ->{}\n    ->{}\n    ->{}\n",
            Color::Blue.bold().paint("10"),
            Color::Blue.bold().paint("20"),
            Color::Blue.bold().paint("30"),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
