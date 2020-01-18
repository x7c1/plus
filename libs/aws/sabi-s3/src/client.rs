use crate::operations;
//use ambassador::Delegate;

use derive_sabi::define_requesters;
//use operations::put_object::{ambassador_impl_Requester_body_single_struct};

//#[derive(Delegate)]
//#[delegate(Requester)]

#[define_requesters(operations::put_object::Requester)]
#[derive(Debug)]
pub struct S3Client {
    //    put: ImplSample,
}

//use crate::define_methods;
//use operations::put_object::define_methods;

//use operations::put_object;

impl S3Client {
    define__put_object__requester__methods!();
}

//impl S3Client {
////    pub fn put_object<A: operations::put_object::Request>(&self, request: A) -> String {
////        operations::put_object::Requester::put_object(self, request)
////    }
//}

//use derive_sabi::{show_streams, AnswerFn};
//
//#[show_streams(define<'a, 'b>() -> Definition<'a, 'b, CommandResult>)]
//#[derive(AnswerFn, new, Debug)]
//struct Struct;
//
//impl Struct {
//    //    #[show_streams(define<'a, 'b>() -> Definition<'a, 'b, CommandResult>)]
//    //    #[derive(AnswerFn, new, Debug)]
//}
//
//#[show_streams(define<'a, 'b>() -> Definition<'a, 'b, CommandResult>)]
//fn piyo1() {}
//
////println!("answer...{}", answer());
////println!("new...{:?}", Struct::new());
