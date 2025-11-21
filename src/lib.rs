use std::fmt;

#[derive(Clone, Copy)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl AnsiColor {
    const fn to_code(self) -> u8 {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
        }
    }
}

pub struct Print<'a> {
    out: &'a mut dyn fmt::Write,
    indent_level: usize,
    spaces_per_indent: usize,
}

impl<'a> Print<'a> {
    pub fn new(out: &'a mut dyn fmt::Write) -> Self {
        Self {
            out,
            indent_level: 0,
            spaces_per_indent: 2,
        }
    }
}

impl Print<'_> {
    #[must_use]
    pub const fn with_indent(mut self, spaces: usize) -> Self {
        self.spaces_per_indent = spaces;
        self
    }

    #[inline]
    fn pad(&mut self) -> fmt::Result {
        for _ in 0..(self.indent_level * self.spaces_per_indent) {
            self.out.write_char(' ')?;
            //            self.out.write_char(',')?;
        }
        Ok(())
    }

    #[inline]
    pub fn line<S: AsRef<str>>(&mut self, s: S) -> fmt::Result {
        self.write("\n")?;
        self.pad()?;
        self.out.write_str(s.as_ref())
    }

    #[inline]
    pub fn newline(&mut self) -> fmt::Result {
        self.write("\n")
    }

    #[inline]
    pub fn write<S: AsRef<str>>(&mut self, s: S) -> fmt::Result {
        self.out.write_str(s.as_ref())
    }

    /// Convenience `{ body }` block helper that uses the RAII guard internally.
    pub fn block<F>(&mut self, header: &str, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Print<'_>) -> fmt::Result,
    {
        if !header.is_empty() {
            todo!()
        }
        self.line("{")?;
        {
            f(self)?;
        }
        self.line("}")
    }
}

pub struct Printer<'a> {
    use_color: bool,
    use_ligature: bool,
    print: Print<'a>,
}

impl<'a> Printer<'a> {
    pub fn new(out: &'a mut dyn fmt::Write) -> Self {
        Self {
            print: Print::new(out),
            use_color: true,
            use_ligature: true,
        }
    }
    #[must_use]
    pub const fn with_colors(mut self, on: bool) -> Self {
        self.use_color = on;
        self
    }

    #[must_use]
    pub const fn with_ligature(mut self, on: bool) -> Self {
        self.use_ligature = on;
        self
    }

    #[inline]
    pub fn pad(&mut self) -> fmt::Result {
        self.print.pad()
    }
    #[inline]
    pub fn line<S: AsRef<str>>(&mut self, s: S) -> fmt::Result {
        self.print.line(s)
    }
    #[inline]
    pub fn newline(&mut self) -> fmt::Result {
        self.print.newline()
    }
    #[inline]
    pub fn write<S: AsRef<str>>(&mut self, s: S) -> fmt::Result {
        self.print.write(s)
    }

    pub fn with_indent<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Printer<'_>) -> fmt::Result,
    {
        self.print.indent_level += 1;
        let r = f(self);
        self.print.indent_level -= 1;
        r
    }

    pub fn block<F>(&mut self, header: &str, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Printer<'_>) -> fmt::Result,
    {
        if !header.is_empty() {
            self.punctuation(&header.to_string())?;
        }

        self.line("{")?;

        {
            self.print.indent_level += 1;
            let _ = f(self);
            self.print.indent_level -= 1;
        }

        self.line("}")
    }

    #[inline]
    fn with_color(&mut self, s: &str, color: AnsiColor, bold: bool) -> fmt::Result {
        if self.use_color {
            if bold {
                self.write(format!("\x1b[1;{}m", color.to_code()))?;
            } else {
                self.write(format!("\x1b[{}m", color.to_code()))?;
            }
        }
        self.write(s)?;
        if self.use_color {
            self.write("\x1b[0m")?;
        }
        Ok(())
    }

    pub fn keyword(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Cyan, true) // bold cyan
    }

    pub fn label(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Blue, true)
    }

    pub fn symbol(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Yellow, false)
    }

    pub fn literal(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Green, false)
    }

    pub fn number(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Cyan, true)
    }

    pub fn constant(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::BrightGreen, true)
    }

    pub fn boolean(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::BrightMagenta, true)
    }

    pub fn string(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::BrightYellow, true)
    }

    pub fn octets(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::BrightRed, true)
    }

    pub fn comment(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::BrightBlack, false)
    }

    pub fn type_name(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Magenta, false)
    }

    pub fn signature(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Magenta, false)
    }

    pub fn low_level_type_name(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::BrightMagenta, false)
    }

    #[inline]
    fn annotation_with_prefix(&mut self, prefix: &str, s: &str, color: AnsiColor) -> fmt::Result {
        self.with_color(&format!("{prefix}{s}"), color, false)
    }

    pub fn annotation_type_name(&mut self, s: &str) -> fmt::Result {
        self.annotation_with_prefix(": ", s, AnsiColor::BrightBlack)
    }

    pub fn annotation_type_name_reference(&mut self, s: &str) -> fmt::Result {
        self.annotation_with_prefix(": &", s, AnsiColor::BrightBlack)
    }

    pub fn annotation_low_level_type_name(&mut self, s: &str) -> fmt::Result {
        self.annotation_with_prefix(": ", s, AnsiColor::BrightBlack)
    }

    pub fn function_name(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Blue, true)
    }

    pub fn function_name_reference(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Blue, false)
    }

    pub fn opcode(&mut self, s: &str) -> fmt::Result {
        self.with_color(s, AnsiColor::Cyan, false)
    }

    pub fn operator(&mut self, s: &str) -> fmt::Result {
        self.write(s)
    }

    #[inline]
    fn colored(&mut self, s: &str, color: AnsiColor, bold: bool) -> fmt::Result {
        self.with_color(s, color, bold)
    }

    pub fn register(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::White, false)
    }

    pub fn register_read(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::Green, false)
    }

    pub fn register_write(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::Red, false)
    }

    pub fn frame_address(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::Cyan, true)
    }

    pub fn frame_address_write(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::Red, true)
    }

    pub fn frame_address_read(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::Green, true)
    }

    pub fn heap_address(&mut self, s: &str) -> fmt::Result {
        self.colored(s, AnsiColor::Cyan, true)
    }

    pub fn punctuation(&mut self, s: &str) -> fmt::Result {
        self.write(s)
    }

    #[must_use]
    pub const fn left_arrow(&self) -> &'static str {
        if self.use_ligature { " ← " } else { " <- " }
    }

    #[must_use]
    pub const fn right_arrow(&self) -> &'static str {
        if self.use_ligature { " → " } else { " -> " }
    }

    #[must_use]
    pub const fn both_arrows(&self) -> &'static str {
        if self.use_ligature { " ↔ " } else { " <-> " }
    }

    #[must_use]
    pub const fn phi(&self) -> &'static str {
        if self.use_ligature { "ϕ" } else { "phi" }
    }

    pub fn spaces(&mut self, count: usize) -> fmt::Result {
        self.punctuation(&" ".repeat(count))
    }
}

pub trait PrettyPrint {
    fn pretty(&self, p: &mut Printer) -> fmt::Result;
}
