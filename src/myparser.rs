pub struct Parser<A> {
    p: fn(String) -> (Result<A, String>, String),
}
impl<A> Parser<A> {
    fn map<B>(&self, f: &dyn Fn(A) -> B) -> Parser<B> {
        todo!()
    }
}

pub enum PResult<T> {
    Success(T),
    Fail(String),
    Left(String),
}
