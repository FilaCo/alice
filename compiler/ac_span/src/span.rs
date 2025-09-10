use std::marker::PhantomData;

pub struct Spanned<T> {
    m: PhantomData<T>,
}
