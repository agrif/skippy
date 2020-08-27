use crate::{Command, Empty, NameCons, NamePart};

#[derive(Clone, Debug)]
pub struct Builder<N> {
    name: N,
}

impl Builder<Empty> {
    pub const fn new() -> Self {
        Self {
            name: Empty,
        }
    }
}

impl<'a, N> Builder<N> {
    pub const fn new_named(name: N) -> Self {
        Self {
            name,
        }
    }

    pub fn append(&self, part: NamePart<'a>) -> Builder<NameCons<'a, N>>
    where
        N: Clone,
    {
        Builder {
            name: NameCons::new(self.name.clone(), part),
        }
    }

    pub fn call<A>(
        self,
        query: bool,
        arguments: A,
    ) -> Command<N, A> {
        Command::new(self.name, query, arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use crate::{Argument::*, NamePart};

    #[test]
    fn builder() {
        let s = Builder::new()
            .append(NamePart("BASEname", None))
            .append(NamePart("THENname", Some(2)))
            .call(false, [Name("SYMbol"), Int(2)]);
        assert_eq!(format!("{:#}", s), ":BASEname:THENname2 SYMbol 2");
    }
}
