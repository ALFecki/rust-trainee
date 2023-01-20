
pub trait Add<Rhs = Self> {
    type Result;

    fn add(self, second: Rhs) -> Self::Result;
}
