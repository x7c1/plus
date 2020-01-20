use crate::operations;

#[derive(Debug)]
pub struct S3Client {
    // todo: add bucket field
}

impl S3Client {
    pub fn put_object<A>(&self, request: A) -> String
    where
        A: operations::put_object::Request,
    {
        operations::put_object::Requester::put_object(self, request)
    }
}
