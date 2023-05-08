pub trait Lambda {
    type Output;
    type Inner;
    type Index;

    fn lambda<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner;

    fn lambda_index<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner;

    fn lambda_enumerate<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner;
}

pub trait LambdaMut<'x> {
    type Output;
    type Inner;
    type Index;

    fn lambda_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner;

    fn lambda_index_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner;

    fn lambda_enumerate_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner;
}

pub trait Map<Rhs> {
    type Output;
    type Inner;
    type Index;

    fn map<F>(&self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner;

    fn map_enumerate<F>(&self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner;
}

pub trait MapMut<'x, Rhs> {
    type Output;
    type Inner;
    type Index;

    fn map_mut<F>(&'x mut self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner;

    fn map_enumerate_mut<F>(&'x mut self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner;
}