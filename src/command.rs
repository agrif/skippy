use crate::{CommandName, NamePart};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Command<'a, N = &'a [NamePart<'a>]> {
    name: N,
    query: bool,
    arguments: &'a [Argument<'a>],
}

#[derive(Clone, Debug)]
pub enum Argument<'a> {
    Str(&'a str),
    Name(&'a str),
    Int(isize),
}

fn pretty_name(name: &str, verbose: bool) -> &str {
    if verbose {
        name
    } else {
        let end = name
            .find(|c: char| !c.is_ascii_uppercase())
            .unwrap_or(name.len());
        &name[..end]
    }
}

impl<'a, N> Command<'a, N> {
    pub const fn new(
        name: N,
        query: bool,
        arguments: &'a [Argument<'a>],
    ) -> Self {
        Self {
            name,
            query,
            arguments,
        }
    }
}

impl<'a, N> fmt::Display for Command<'a, N>
where
    N: CommandName<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let verbose = f.alternate();
        for part in self.name.parts() {
            write!(f, ":{}", pretty_name(part.0, verbose))?;
            if let Some(n) = part.1 {
                write!(f, "{}", n)?;
            }
        }
        if self.query {
            write!(f, "?")?;
        }
        for arg in self.arguments {
            write!(f, " ")?;
            arg.fmt(f)?;
        }
        Ok(())
    }
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
    use super::Command;
    use super::NamePart;

    #[test]
    fn format_arguments() {
        assert_eq!(format!("{}", Str("HELlo")), "HELlo");
        assert_eq!(format!("{:#}", Str("HELlo")), "HELlo");

        assert_eq!(format!("{}", Name("HELlo")), "HEL");
        assert_eq!(format!("{:#}", Name("HELlo")), "HELlo");

        assert_eq!(format!("{}", Int(42)), "42");
        assert_eq!(format!("{}", Int(-42)), "-42");
    }

    #[test]
    fn format_command() {
        let s = Command::new(
            [NamePart("BASEname", None), NamePart("THENname", Some(2))]
                .as_ref(),
            false,
            &[Name("SYMbol"), Int(2)],
        );
        let q = Command::new(
            [NamePart("BASEname", None), NamePart("THENname", Some(2))]
                .as_ref(),
            true,
            &[],
        );
        assert_eq!(format!("{}", s), ":BASE:THEN2 SYM 2");
        assert_eq!(format!("{:#}", s), ":BASEname:THENname2 SYMbol 2");
        assert_eq!(format!("{}", q), ":BASE:THEN2?");
        assert_eq!(format!("{:#}", q), ":BASEname:THENname2?");
    }
}
