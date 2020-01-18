use crate::operations::put_object;

#[derive(Debug)]
pub struct PutFile {
    pub file_path: String,
}

impl put_object::Request for PutFile {}
