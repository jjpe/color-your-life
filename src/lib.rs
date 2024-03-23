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
}

#[derive(Clone, Copy, PartialEq)]
pub struct Style {
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dimmed: bool,
}

