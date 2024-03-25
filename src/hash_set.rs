//!

use super::*;
use std::collections::HashSet;

impl<TF, T> ColorDisplay<HashSetFormat<TF>> for HashSet<T>
where
    T: ColorDisplay<TF>
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &HashSetFormat<TF>,
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
pub struct HashSetFormat<TF> {
    pub prefix_newlines: u16,
    pub intersperse_newlines: u16,
    pub suffix_newlines: u16,
    pub item_format: TF,
}

impl<TF: Format> Format for HashSetFormat<TF> {
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
        let mut set: HashSet<u8> = HashSet::new();
        set.insert(10);
        set.insert(20);
        set.insert(30);
        let mut sink = String::with_capacity(1024);
        set.color_fmt(&mut sink, &HashSetFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            item_format: U8Format {
                prefix: "->",
                style: Some(Style {
                    color: Color::Red,
                    bold: true,
                    italic: false,
                    underline: false,
                    dimmed: false,
                }),
                ..U8Format::standard(1)
            },
        })?;
        // Collect into a Vec<_> to obtain order stability, as the iteration
        // order is not stable between different HashSet<_> instances. However,
        // it seems the case that for a given HashSet<_> instance, calling
        // `set.iter()` repeatedly will always yield the same iteration order.
        let vec: Vec<_> = set.iter().collect();
        let expected = format!(
            "\n    ->{}\n    ->{}\n    ->{}\n",
            Color::Red.bold().paint(format!("{}", vec[0])),
            Color::Red.bold().paint(format!("{}", vec[1])),
            Color::Red.bold().paint(format!("{}", vec[2])),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
