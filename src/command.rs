use crate::{Argument, NamePart};

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command<N, A> {
    name: N,
    query: bool,
    arguments: A,
}

impl<N, A> Command<N, A> {
    pub const fn new(name: N, query: bool, arguments: A) -> Self {
        Self {
            name,
            query,
            arguments,
        }
    }
}

impl<'a, N, A> fmt::Display for Command<N, A>
where
    N: std::borrow::Borrow<[NamePart<'a>]>,
    A: std::borrow::Borrow<[Argument<'a>]>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for part in self.name.borrow() {
            part.fmt(f)?;
        }
        if self.query {
            write!(f, "?")?;
        }
        let mut first = true;
        for arg in self.arguments.borrow() {
            if first {
                write!(f, " ")?;
                first = false;
            } else {
                write!(f, ",")?;
            }
            arg.fmt(f)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! command {
    ($(: $a:ident $([ $n:expr ])? )* $(, $($e:expr),* $(,)? )? ) => {
        $crate::Command::new(
            $crate::name!($( : $a $([ $n ])? )*),
            false,
            $crate::arguments![ $($($e),*)? ],
        )
    };
    ($(: $a:ident $([ $n:expr ])? )* ? $(, $($e:expr),* $(,)? )? ) => {
        $crate::Command::new(
            $crate::name!($( : $a $([ $n ])? )*),
            true,
            $crate::arguments![ $($($e),*)? ],
        )
    };
    ($(: * $a:ident $([ $n:expr ])? )* $(, $($e:expr),* $(,)? )? ) => {
        $crate::Command::new(
            $crate::name!($( : * $a $([ $n ])? )*),
            false,
            $crate::arguments![ $($($e),*)? ],
        )
    };
    ($(: * $a:ident $([ $n:expr ])? )* ? $(, $($e:expr),* $(,)? )? ) => {
        $crate::Command::new(
            $crate::name!($( : * $a $([ $n ])? )*),
            true,
            $crate::arguments![ $($($e),*)? ],
        )
    };
}

#[cfg(test)]
mod tests {
    use super::Argument::*;
    use super::Command;

    #[test]
    fn format_command() {
        let s = Command::new(
            crate::name!(:BASEname:THENname[2]),
            false,
            crate::arguments![Discrete("SYMbol"), 2],
        );
        assert_eq!(format!("{}", s), ":BASE:THEN2 SYM,2");
        assert_eq!(format!("{:#}", s), ":BASEname:THENname2 SYMbol,2");
        assert_eq!(
            s,
            crate::command!(:BASEname:THENname[2], Discrete("SYMbol"), 2)
        );
    }

    #[test]
    fn format_query() {
        let q = Command::new(
            crate::name!(:BASEname:THENname[2]),
            true,
            crate::arguments![],
        );
        assert_eq!(format!("{}", q), ":BASE:THEN2?");
        assert_eq!(format!("{:#}", q), ":BASEname:THENname2?");
        assert_eq!(q, crate::command!(:BASEname:THENname[2]?));
    }

    #[test]
    fn format_asterisk() {
        let q = Command::new(crate::name!(:*IQN), true, crate::arguments![]);
        assert_eq!(format!("{}", q), ":*IQN?");
        assert_eq!(format!("{:#}", q), ":*IQN?");
        assert_eq!(q, crate::command!(:*IQN?));
    }
}
