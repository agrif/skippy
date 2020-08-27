use crate::pretty_name;

use std::fmt;

#[derive(Clone, Debug)]
pub enum Argument<'a> {
    Str(&'a str),
    Name(&'a str),
    Int(isize),
}

impl<'a> fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Argument::*;
        let verbose = f.alternate();
        match self {
            Str(s) => write!(f, "{}", s)?,

            Name(s) => {
                write!(f, "{}", pretty_name(s, verbose))?;
            }

            Int(n) => write!(f, "{}", n)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Argument::*;

    #[test]
    fn format_arguments() {
        assert_eq!(format!("{}", Str("HELlo")), "HELlo");
        assert_eq!(format!("{:#}", Str("HELlo")), "HELlo");

        assert_eq!(format!("{}", Name("HELlo")), "HEL");
        assert_eq!(format!("{:#}", Name("HELlo")), "HELlo");

        assert_eq!(format!("{}", Int(42)), "42");
        assert_eq!(format!("{}", Int(-42)), "-42");
    }
}
