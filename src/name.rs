#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamePart<'a>(pub &'a str, pub Option<usize>);

pub trait CommandName<'a>: Clone + Sized {
    type Iterator: std::iter::Iterator<Item = NamePart<'a>>;
    fn parts(&self) -> Self::Iterator;
}

impl<'a, 'b> CommandName<'a> for &'a [NamePart<'b>] {
    type Iterator = std::iter::Cloned<std::slice::Iter<'a, NamePart<'a>>>;
    fn parts(&self) -> Self::Iterator {
        self.iter().cloned()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Empty;

impl<'a> CommandName<'a> for Empty {
    type Iterator = std::iter::Empty<NamePart<'a>>;
    fn parts(&self) -> Self::Iterator {
        std::iter::empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameCons<'a, N> {
    first: N,
    last: NamePart<'a>,
}

#[derive(Clone, Debug)]
pub struct NameConsIter<'a, I> {
    firstiter: I,
    last: Option<NamePart<'a>>,
}

impl<'a, N> NameCons<'a, N> {
    pub const fn new(first: N, last: NamePart<'a>) -> Self {
        Self { first, last }
    }
}

impl<'a, N> CommandName<'a> for NameCons<'a, N>
where
    N: CommandName<'a>,
{
    type Iterator = NameConsIter<'a, N::Iterator>;
    fn parts(&self) -> Self::Iterator {
        NameConsIter {
            firstiter: self.first.parts(),
            last: Some(self.last.clone()),
        }
    }
}

impl<'a, I> std::iter::Iterator for NameConsIter<'a, I>
where
    I: std::iter::Iterator<Item = NamePart<'a>>,
{
    type Item = NamePart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_some() {
            self.firstiter.next().or_else(|| self.last.take())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandName, Empty, NameCons, NamePart};

    #[test]
    fn empty() {
        assert_eq!(Empty.parts().collect::<Vec<_>>(), Vec::<NamePart>::new());
    }

    #[test]
    fn slice() {
        let s: &[NamePart] =
            &[NamePart("hello", None), NamePart("there", Some(2))];
        assert_eq!(
            s.parts().collect::<Vec<_>>(),
            s.iter().cloned().collect::<Vec<_>>()
        );
    }

    #[test]
    fn cons() {
        let n = NameCons::new(
            NameCons::new(Empty, NamePart("hello", None)),
            NamePart("there", Some(2)),
        );
        assert_eq!(
            n.parts().collect::<Vec<_>>(),
            vec![NamePart("hello", None), NamePart("there", Some(2))]
        );
    }
}
