use crate::operations;
//use ambassador::Delegate;

use derive_sabi::delegate_requesters;
//use operations::put_object::{ambassador_impl_Requester_body_single_struct};

//#[derive(Delegate)]
//#[delegate(Requester)]

#[delegate_requesters(operations::put_object::Requester)]
#[derive(Debug)]
pub struct S3Client {
    // todo: add bucket field
}

impl S3Client {
    define__operations__put_object__requester__methods!();
}
