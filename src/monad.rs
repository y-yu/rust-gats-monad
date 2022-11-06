#[allow(unused_imports)]
use crate::applicative::Applicative;

pub trait Monad {
    type A;
    type This<A>: Monad;

    fn pure_m<B>(b: B) -> Self::This<B>;

    fn flat_map<B, F>(self, f: F) -> Self::This<B>
    where
        F: FnMut(Self::A) -> Self::This<B>;
}

impl<M: Monad> Applicative for M {
    type A = <M as Monad>::A;
    type This<B> = <M as Monad>::This<B>;

    fn pure_a<B>(t: B) -> Self::This<B> {
        <M as Monad>::pure_m(t)
    }

    fn map2<B, C, F>(self, mb: Self::This<B>, f: F) -> Self::This<C>
    where
        F: FnMut(Self::A, B) -> C
    {
        self.flat_map(|a: Self::A| {
            mb.flat_map(|b: B| {
                M::pure_m(f(a, b))
            })
        })
    }
}
