//!

use super::*;
use std::collections::HashMap;

impl<KF, VF, K, V> ColorDisplay<HashMapFormat<KF, VF>> for HashMap<K, V>
where
    K: ColorDisplay<KF>,
    V: ColorDisplay<VF>,
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &HashMapFormat<KF, VF>,
    ) -> std::fmt::Result {
        self.write_newlines(sink, format.prefix_newlines, &format)?;
        for (idx, (key, value)) in self.iter().enumerate() {
            if idx > 0 {
                self.write_newlines(sink, format.intersperse_newlines, &format)?;
            }
            key.color_fmt(sink, &format.key_format)?;
            write!(sink, "{}", format.key_value_separator)?;
            value.color_fmt(sink, &format.value_format)?;

        }
        self.write_newlines(sink, format.suffix_newlines, &format)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct HashMapFormat<KF, VF> {
    pub prefix_newlines: u16,
    pub intersperse_newlines: u16,
    pub suffix_newlines: u16,
    pub key_value_separator: &'static str,
    pub key_format: KF,
    pub value_format: VF,
}

impl<KF: Format, VF: Format> Format for HashMapFormat<KF, VF> {
    fn standard(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            key_value_separator: ": ",
            key_format: KF::standard(indent),
            value_format: VF::standard(indent),
        }
    }

    fn monochrome(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            key_value_separator: ": ",
            key_format: KF::monochrome(indent),
            value_format: VF::monochrome(indent),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{
        Color, ColorDisplay,
        primitive::U8Format,
        str::StrFormat,
    };
    use super::*;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let mut map: HashMap<u8, &str> = HashMap::new();
        map.insert(10, "ten");
        map.insert(20, "twenty");
        map.insert(30, "thirty");
        let mut sink = String::with_capacity(1024);
        map.color_fmt(&mut sink, &HashMapFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            key_value_separator: " = ",
            key_format: U8Format {
                prefix: "-> ",
                ..U8Format::standard(1)
            },
            value_format: StrFormat::standard(0),
        })?;
        // Collect into a Vec<_> to obtain order stability, as the iteration
        // order is not stable between different HashMap<_> instances. However,
        // it seems the case that for a given HashMap<_> instance, calling
        // `set.iter()` repeatedly will always yield the same iteration order.
        let vec: Vec<(&u8, &&str)> = map.iter().collect();
        let expected = format!(
            "\n    -> {} = {}\n    -> {} = {}\n    -> {} = {}\n",
            Color::Blue.bold().paint(format!("{}", &vec[0].0)),
            Color::Green.paint(format!("{}", &vec[0].1)),
            Color::Blue.bold().paint(format!("{}", &vec[1].0)),
            Color::Green.paint(format!("{}", &vec[1].1)),
            Color::Blue.bold().paint(format!("{}", &vec[2].0)),
            Color::Green.paint(format!("{}", &vec[2].1)),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
