trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

trait Add {
    type Output;
    fn add<Rhs>(self, rhs: Rhs) -> Self::Output;
}
