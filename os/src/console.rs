use core::{fmt::{Write, self}, str::Chars};

use crate::sbi::console_putchar;


struct Stdout;

impl Write for Stdout{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars(){
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments){
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ColorStr<'a> {
    inner_str: &'a str,
    color: Color,
}

pub trait ColorChar{
    fn default_color_chars(&self) -> ColorStrIter<'_>{
        self.render_color(Color::INFO)
    }
    fn render_color(&self,color: Color ) -> ColorStrIter<'_>;
}

impl ColorChar for &str{
    fn render_color(&self,color: Color ) -> ColorStrIter<'_> {
        let a =format_args!("\u{1B}[{}m{}\u{1B}[0m",1,2);
        let color_suffix = "\x1b[0m".chars();
        let color_prefix = match color{
            Color::ERROR => "\x1b[31m",
            Color::WARN => "\x1b[93m",
            Color::INFO =>"\x1b[34m",
            Color::DEBUG => "\x1b[32m",
            Color::TRACE =>"\x1b[90m",
        }.chars();

        ColorStrIter{
            inner_str: self.chars(),
            color_prefix,
            color_suffix,
        }
    }
}

pub struct ColorStrIter<'a>{
    inner_str: Chars<'a>,
    color_prefix: Chars<'a>,
    color_suffix: Chars<'a>,
}

impl<'a> Iterator for ColorStrIter<'a>{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.color_prefix.next(){
            return Some(c);
        }

        if let Some(c) = self.inner_str.next(){
            return Some(c);
        }

        if let Some(c) = self.color_suffix.next(){
            return Some(c);
        }
        None
    }
}



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    // red
    ERROR = 31,
    // yellow
    WARN = 93,
    // blue
    INFO = 34,
    // green
    DEBUG = 32,
    // gray
    TRACE = 90,
}

impl Color {

}