//!

use std::fmt::Write;
use ansi_term::Color;

pub trait ColorDisplay<A> {
    const INDENTATION: &'static str = "    ";

    fn color_fmt(
        &self,
        sink: &mut impl Write,
        args: &A,
    ) -> std::fmt::Result;

    /// Utility method to simpify writing the proper amount of indentation.
    /// In order to print the right indentation token, it takes into account
    /// the implementing type as well as the print argument type `A`.
    fn write_indentation(
        &self,
        sink: &mut impl Write,
        count: u16,
        _args: &A,
    ) -> std::fmt::Result {
        for _ in 0..count {
            write!(sink, "{}", <Self as ColorDisplay<A>>::INDENTATION)?;
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

