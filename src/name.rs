use crate::pretty_name;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamePart<'a>(pub &'a str, pub Option<usize>);

impl<'a> fmt::Display for NamePart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let verbose = f.alternate();
        write!(f, ":{}", pretty_name(self.0, verbose))?;
        if let Some(n) = self.1 {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::NamePart;

    #[test]
    fn name() {
        let n = NamePart("SOMEthing", None);
        assert_eq!(format!("{}", n), ":SOME");
        assert_eq!(format!("{:#}", n), ":SOMEthing");
    }

    #[test]
    fn name_number() {
        let n = NamePart("SOMEthing", Some(2));
        assert_eq!(format!("{}", n), ":SOME2");
        assert_eq!(format!("{:#}", n), ":SOMEthing2");
    }
}
