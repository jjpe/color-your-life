//!

use super::*;
use std::collections::BTreeMap;

impl<KF, VF, K, V> ColorDisplay<BTreeMapFormat<KF, VF>> for BTreeMap<K, V>
where
    K: ColorDisplay<KF>,
    V: ColorDisplay<VF>,
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &BTreeMapFormat<KF, VF>,
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
pub struct BTreeMapFormat<KF, VF> {
    pub prefix_newlines: u16,
    pub intersperse_newlines: u16,
    pub suffix_newlines: u16,
    pub key_value_separator: &'static str,
    pub key_format: KF,
    pub value_format: VF,
}

impl<KF: Format, VF: Format> Format for BTreeMapFormat<KF, VF> {
    fn colored(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            key_value_separator: ": ",
            key_format: KF::colored(indent),
            value_format: VF::colored(indent),
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
        let mut map: BTreeMap<u8, &str> = BTreeMap::new();
        map.insert(10, "ten");
        map.insert(20, "twenty");
        map.insert(30, "thirty");
        let mut sink = String::with_capacity(1024);
        map.color_fmt(&mut sink, &BTreeMapFormat {
            prefix_newlines: 1,
            intersperse_newlines: 1,
            suffix_newlines: 1,
            key_value_separator: " = ",
            key_format: U8Format {
                prefix: "-> ",
                ..U8Format::colored(1)
            },
            value_format: StrFormat::colored(0),
        })?;
        let expected = format!(
            "\n    -> {} = {}\n    -> {} = {}\n    -> {} = {}\n",
            Color::Blue.bold().paint("10"),
            Color::Green.paint("ten"),
            Color::Blue.bold().paint("20"),
            Color::Green.paint("twenty"),
            Color::Blue.bold().paint("30"),
            Color::Green.paint("thirty"),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
