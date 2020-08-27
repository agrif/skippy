use crate::{Argument, Command, CommandName, NameCons, NamePart};

#[derive(Clone, Debug)]
pub struct Builder<N, T> {
    name: N,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, N, T> Builder<N, T> {
    pub const fn new(name: N) -> Self {
        Self {
            name,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, N, T> Builder<N, T>
where
    N: CommandName<'a>,
{
    pub fn append<S>(
        &self,
        part: NamePart<'a>,
    ) -> Builder<NameCons<'a, N>, S> {
        Builder {
            name: self.name.append(part),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn call(
        self,
        query: bool,
        arguments: &'a [Argument<'a>],
    ) -> Command<'a, N> {
        Command::new(self.name, query, arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use crate::{Argument::*, Empty, NamePart};

    #[test]
    fn builder() {
        let s = Builder::<_, ()>::new(Empty)
            .append::<()>(NamePart("BASEname", None))
            .append::<()>(NamePart("THENname", Some(2)))
            .call(false, &[Name("SYMbol"), Int(2)]);
        assert_eq!(format!("{:#}", s), ":BASEname:THENname2 SYMbol 2");
    }
}
