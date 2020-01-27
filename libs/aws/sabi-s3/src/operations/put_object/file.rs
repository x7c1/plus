use crate::verbs::HasObjectKey;

#[derive(Debug)]
pub struct FileRequest {
    pub file_path: String,
    pub object_key: String,
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

impl super::Request for FileRequest {}
