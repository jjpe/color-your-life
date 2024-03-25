//!

use super::*;
use std::collections::VecDeque;

impl<TF, T> ColorDisplay<VecDequeFormat<TF>> for VecDeque<T>
where
    T: ColorDisplay<TF>
{
    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &VecDequeFormat<TF>,
    ) -> std::fmt::Result {
        self.write_newlines(sink, format.prefix_newlines, &format)?;
        let (front, back) = self.as_slices();
        for (idx, item) in front.iter().enumerate() {
            if idx > 0 {
                self.write_newlines(sink, format.intersperse_newlines, &format)?;
            }
            item.color_fmt(sink, &format.item_format)?;
        }
        write!(sink, "\n")?;
        for _ in 0..format.front_back_separator_count {
            write!(sink, "{}", format.front_back_separator_token)?;
        }
        write!(sink, "\n")?;
        for (idx, item) in back.iter().enumerate() {
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
pub struct VecDequeFormat<TF> {
    pub prefix_newlines: u16,
    pub intersperse_newlines: u16,
    pub suffix_newlines: u16,
    pub front_back_separator_token: &'static str,
    pub front_back_separator_count: u16,
    pub item_format: TF,
}

impl<TF: Format> Format for VecDequeFormat<TF> {
    fn standard(indent: u16) -> Self {
        Self {
            prefix_newlines: 0,
            intersperse_newlines: 1,
            suffix_newlines: 0,
            front_back_separator_token: "-",
            front_back_separator_count: 40,
            item_format: TF::standard(indent),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{Color, ColorDisplay};
    use super::*;

    #[test]
    fn color_fmt() -> std::fmt::Result {
        let mut vec_deque = VecDeque::new();
        vec_deque.push_front(20);
        vec_deque.push_front(10);
        vec_deque.push_back(30);
        vec_deque.push_back(40);
        let mut sink = String::with_capacity(1024);
        vec_deque.color_fmt(&mut sink, &VecDequeFormat::standard(1))?;
        let expected = format!(
            "    {}\n    {}\n{}\n    {}\n    {}",
            Color::Blue.bold().paint("10"),
            Color::Blue.bold().paint("20"),
            "----------------------------------------",
            Color::Blue.bold().paint("30"),
            Color::Blue.bold().paint("40"),
        );
        assert_eq!(sink, expected);
        Ok(())
    }
}
