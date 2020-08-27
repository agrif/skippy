struct Root<N = skippy::Empty>(skippy::Builder<N>);

static DS1054Z: Root = Root(skippy::Builder::new());

impl<N: Clone> Root<N> {
    pub fn acquire(&self) -> Acquire<skippy::NameCons<N>> {
        Acquire(self.0.append(skippy::NamePart("ACQuire", None)))
    }
}

struct Acquire<N>(skippy::Builder<N>);

impl<N: Clone> Acquire<N> {
    pub fn averages(
        &self,
        count: usize,
    ) -> skippy::Command<skippy::NameCons<N>, [skippy::Argument; 1]> {
        self.0
            .append(skippy::NamePart("AVERages", None))
            .call(false, [skippy::Argument::Int(count as isize)])
    }

    pub fn averages_p(
        &self,
        count: usize,
    ) -> skippy::Command<skippy::NameCons<N>, [skippy::Argument; 0]> {
        self.0
            .append(skippy::NamePart("AVERages", None))
            .call(true, [])
    }
}

fn main() {
    println!("{:#}", DS1054Z.acquire().averages(2));
    println!("{:#}", DS1054Z.acquire().averages_p());
}
