use crate::{Argument, NamePart};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Command<'a> {
    name: &'a [NamePart<'a>],
    query: bool,
    arguments: &'a [Argument<'a>],
}

impl<'a> Command<'a> {
    pub const fn new(
        name: &'a [NamePart<'a>],
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

impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for part in self.name {
            part.fmt(f)?;
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

#[cfg(test)]
mod tests {
    use super::Argument::*;
    use super::Command;
    use crate::NamePart;

    #[test]
    fn format_command() {
        let s = Command::new(
            &[NamePart("BASEname", None), NamePart("THENname", Some(2))],
            false,
            &[Name("SYMbol"), Int(2)],
        );
        let q = Command::new(
            &[NamePart("BASEname", None), NamePart("THENname", Some(2))],
            true,
            &[],
        );
        assert_eq!(format!("{}", s), ":BASE:THEN2 SYM 2");
        assert_eq!(format!("{:#}", s), ":BASEname:THENname2 SYMbol 2");
        assert_eq!(format!("{}", q), ":BASE:THEN2?");
        assert_eq!(format!("{:#}", q), ":BASEname:THENname2?");
    }
}
