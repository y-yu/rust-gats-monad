use crate::functor::{Functor};

pub trait Applicative {
    type A;
    type This<B>: Applicative;

    fn pure_a<B>(t: B) -> Self::This<B>;

    fn map2<F, B, C>(self, mb: Self::This<B>, f: F) -> Self::This<C>
    where
        F: FnMut(Self::A, B) -> C;
}

impl<M: Applicative> Functor for M {
    type A = <M as Applicative>::A;
    type This<B> = <M as Applicative>::This<B>;

    fn pure_f<B>(t: B) -> Self::This<B> {
        Self::pure_a(t)
    }

    fn map<F, B>(self, mut f: F) -> M::This<B>
        where
            F: FnMut(Self::A) -> B
    {
        self.map2(
            M::pure_a(()),
            |x, _| {
                f(x)
            }
        )
    }
}
