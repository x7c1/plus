use crate::operations::put_object;

#[derive(Debug)]
pub struct FileBody {
    pub file_path: String,
}

impl put_object::Request for FileBody {}
