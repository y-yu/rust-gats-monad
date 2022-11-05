use crate::monad::Monad;

impl<A> Monad for Option<A> {
    type A = A;
    type This<B> = Option<B>;

    fn pure_m<B>(t: B) -> Option<B> {
        Some(t)
    }

    fn flat_map<B, F>(self, mut f: F) -> Option<B>
        where
            F: FnMut(A) -> Option<B>
    {
        self.and_then(f)
    }
}
