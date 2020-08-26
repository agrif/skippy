use std::fmt;

#[derive(Clone, Debug)]
pub struct Command<'a> {
    name: &'a CommandName<'a>,
    query: bool,
    arguments: &'a [Argument<'a>],
}

#[derive(Clone, Debug)]
pub struct CommandName<'a> {
    parent: Option<&'a CommandName<'a>>,
    name: &'a str,
    number: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum Argument<'a> {
    Str(&'a str),
    Name(&'a str),
    Int(isize),
}

impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)?;
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

impl<'a> CommandName<'a> {
    pub const fn new(name: &'a str, number: Option<usize>) -> Self {
        Self {
            parent: None,
            name,
            number,
        }
    }

    pub const fn append(&'a self, name: &'a str, number: Option<usize>) -> Self {
        Self {
            parent: Some(self),
            name,
            number,
        }
    }

    fn name(&'a self, verbose: bool) -> &'a str {
        if verbose {
            self.name
        } else {
            let end = self
                .name
                .find(|c: char| !c.is_ascii_uppercase())
                .unwrap_or(self.name.len());
            &self.name[..end]
        }
    }

    pub fn call(&'a self, query: bool, arguments: &'a [Argument<'a>]) -> Command<'a> {
        Command {
            name: self,
            query,
            arguments,
        }
    }
}

impl<'a> fmt::Display for CommandName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let verbose = f.alternate();
        if let Some(parent) = self.parent {
            parent.fmt(f)?;
        }
        write!(f, ":{}", self.name(verbose))?;
        if let Some(number) = self.number {
            write!(f, "{}", number)?;
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
                let part = if verbose {
                    s
                } else {
                    let end = s.find(|c: char| !c.is_ascii_uppercase()).unwrap_or(s.len());
                    &s[..end]
                };
                write!(f, "{}", part)?;
            }

            Int(n) => write!(f, "{}", n)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Argument::*;
    use super::CommandName;

    #[test]
    fn format_name() {
        let a = CommandName::new("BASEname", None);
        let b = a.append("THENname", Some(2));
        assert_eq!(format!("{}", b), ":BASE:THEN2");
        assert_eq!(format!("{:#}", b), ":BASEname:THENname2");
    }

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
        let a = CommandName::new("BASEname", None);
        let b = a.append("THENname", Some(2));
        let s = b.call(false, &[Name("SYMbol"), Int(2)]);
        let q = b.call(true, &[]);
        assert_eq!(format!("{}", s), ":BASE:THEN2 SYM 2");
        assert_eq!(format!("{:#}", s), ":BASEname:THENname2 SYMbol 2");
        assert_eq!(format!("{}", q), ":BASE:THEN2?");
        assert_eq!(format!("{:#}", q), ":BASEname:THENname2?");
    }
}
