use std::fmt::Debug;

pub trait HasObjectKey: Debug {
    fn get_object_key(&self) -> &str;
}
