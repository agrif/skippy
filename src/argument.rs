use crate::pretty_name;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Argument<'a> {
    Int(isize),
    Float(f64),
    IntUnit(isize, &'a str),
    FloatUnit(f64, &'a str),
    Discrete(&'a str),
    Str(&'a str),
    Bool(bool),
}

impl<'a> fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Argument::*;
        let verbose = f.alternate();
        match self {
            Int(n) => write!(f, "{}", n)?,
            Float(n) => write!(f, "{:E}", n)?,
            IntUnit(n, s) => write!(f, "{}{}", n, s)?,
            FloatUnit(n, s) => write!(f, "{:E}{}", n, s)?,
            Discrete(s) => {
                write!(f, "{}", pretty_name(s, verbose))?;
            }
            Str(s) => write!(f, "\"{}\"", s)?, // FIXME escape " as ""
            Bool(b) => write!(f, "{}", if *b { "1" } else { "0" })?,
        }
        Ok(())
    }
}

pub trait IntoArgument<'a> {
    fn into_argument(self) -> Argument<'a>;
}

impl<'a> IntoArgument<'a> for Argument<'a> {
    fn into_argument(self) -> Argument<'a> {
        self
    }
}

macro_rules! into_argument_simple {
    ($wrapper:ident($inner:ty), $t:ty) => {
        impl<'a> IntoArgument<'a> for $t {
            fn into_argument(self) -> Argument<'a> {
                Argument::$wrapper(self as $inner)
            }
        }
    };
}

into_argument_simple!(Int(isize), u8);
into_argument_simple!(Int(isize), i8);
into_argument_simple!(Int(isize), u16);
into_argument_simple!(Int(isize), i16);
into_argument_simple!(Int(isize), u32);
into_argument_simple!(Int(isize), i32);
into_argument_simple!(Int(isize), u64);
into_argument_simple!(Int(isize), i64);
into_argument_simple!(Int(isize), usize);
into_argument_simple!(Int(isize), isize);

into_argument_simple!(Float(f64), f32);
into_argument_simple!(Float(f64), f64);

impl<'a> IntoArgument<'a> for &'a str {
    fn into_argument(self) -> Argument<'a> {
        Argument::Str(self)
    }
}

into_argument_simple!(Bool(bool), bool);

#[macro_export]
macro_rules! arguments {
    [] => {{let x: [$crate::Argument; 0] = []; x}};
    [$($e:expr),*,] => {$crate::arguments!($($e),*)};
    [$($e:expr),*] => {
        [ $($crate::IntoArgument::into_argument($e)),* ]
    };
}

#[cfg(test)]
mod tests {
    use super::Argument::*;

    #[test]
    fn str() {
        let a = Str("HELlo");
        assert_eq!(format!("{}", a), "\"HELlo\"");
        assert_eq!(format!("{:#}", a), "\"HELlo\"");
        assert_eq!(a, crate::arguments!["HELlo"][0]);
    }

    // #[test]
    // fn str_escape() {
    //     let a = Str("quote\"here");
    //     assert_eq!(format!("{}", a), "\"quote\"\"here\"");
    //     assert_eq!(a, crate::arguments!["quote\"here"][0]);
    // }

    #[test]
    fn discrete() {
        assert_eq!(format!("{}", Discrete("HELlo")), "HEL");
        assert_eq!(format!("{:#}", Discrete("HELlo")), "HELlo");
    }

    #[test]
    fn int() {
        assert_eq!(format!("{}", Int(42)), "42");
        assert_eq!(format!("{}", Int(-42)), "-42");
        assert_eq!(Int(42), crate::arguments![42][0]);
        assert_eq!(Int(-42), crate::arguments![-42][0]);
    }

    #[test]
    fn float() {
        assert_eq!(format!("{}", Float(4.2)), "4.2E0");
        assert_eq!(format!("{}", Float(-4.2)), "-4.2E0");
        assert_eq!(format!("{}", Float(4.2e99)), "4.2E99");
        assert_eq!(Float(4.2), crate::arguments![4.2][0]);
        assert_eq!(Float(-4.2), crate::arguments![-4.2][0]);
    }

    #[test]
    fn empty_macro() {
        crate::arguments![];
    }
}
