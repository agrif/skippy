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

#[macro_export]
macro_rules! name {
    () => {{let x: [NamePart; 0] = []; x}};
    ($(: $e:ident $([ $n:expr ])? )*) => {
        [ $($crate::name_part!(: $e $([ $n ])?)),* ]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! name_part {
    (: $i:ident) => {
        $crate::NamePart(stringify!($i), ::std::option::Option::None)
    };
    (: $i:ident [ $n:expr ]) => {
        $crate::NamePart(stringify!($i), ::std::option::Option::Some($n))
    };
}

#[cfg(test)]
mod tests {
    use super::NamePart;

    #[test]
    fn name() {
        let n = NamePart("SOMEthing", None);
        assert_eq!(n, crate::name_part!(:SOMEthing));
        assert_eq!(format!("{}", n), ":SOME");
        assert_eq!(format!("{:#}", n), ":SOMEthing");
    }

    #[test]
    fn name_number() {
        let n = NamePart("SOMEthing", Some(2));
        assert_eq!(n, crate::name_part!(:SOMEthing[2]));
        assert_eq!(format!("{}", n), ":SOME2");
        assert_eq!(format!("{:#}", n), ":SOMEthing2");
    }

    #[test]
    fn empty_macro() {
        crate::name!();
    }
}
