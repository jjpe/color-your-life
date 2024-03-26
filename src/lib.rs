//!

pub use ansi_term::Color;
use std::fmt::Write;

pub trait ColorDisplay<F> {
    const INDENTATION: &'static str = "    ";
    const NEWLINE: &'static str = "\n";

    fn color_fmt(
        &self,
        sink: &mut impl Write,
        format: &F,
    ) -> std::fmt::Result;

    /// Utility method to simpify writing the proper amount of indentation.
    /// In order to print the right indentation token, it takes into account
    /// the implementing type as well as the format type `F`.
    fn write_indentation(
        &self,
        sink: &mut impl Write,
        count: u16,
        _: &F,
    ) -> std::fmt::Result {
        for _ in 0..count {
            write!(sink, "{}", <Self as ColorDisplay<F>>::INDENTATION)?;
        }
        Ok(())
    }

    /// Utility method to simpify writing newlines.
    /// In order to print the right newline character(s), it takes into
    /// account the implementing type as well as the format type `F`.
    fn write_newlines(
        &self,
        sink: &mut impl Write,
        count: u16,
        _: &F,
    ) -> std::fmt::Result {
        for _ in 0..count {
            write!(sink, "{}", <Self as ColorDisplay<F>>::NEWLINE)?;
        }
        Ok(())
    }
}

pub trait Format {
    fn standard(indent: u16) -> Self;
}

#[derive(Clone, Copy, PartialEq)]
pub struct StyleDesc {
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dimmed: bool,
}

impl StyleDesc {
    pub fn style_from_desc(desc: impl Into<Option<Self>>) -> ansi_term::Style {
        if let Some(desc) = desc.into() {
            let style = desc.color.normal();
            let style = if desc.bold      { style.bold()      } else { style };
            let style = if desc.italic    { style.italic()    } else { style };
            let style = if desc.underline { style.underline() } else { style };
            let style = if desc.dimmed    { style.dimmed()    } else { style };
            style
        } else {
            ansi_term::Style::default()
        }
    }
}

pub mod bool;
pub mod btree_map;
pub mod btree_set;
pub mod hash_map;
pub mod hash_set;
pub mod primitive;
pub mod result;
pub mod slice;
pub mod str;
pub mod vec;
pub mod vec_deque;
