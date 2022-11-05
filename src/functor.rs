pub trait Functor {
    type A;
    type This<B>: Functor;

    fn pure_f<B>(t: B) -> Self::This<B>;

    fn map<F, B>(self, f: F) -> Self::This<B>
    where
        F: FnMut(Self::A) -> B;
}
