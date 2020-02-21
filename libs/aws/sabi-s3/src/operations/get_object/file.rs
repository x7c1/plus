use crate::core::S3Result;

use crate::internal::{RequestResource, ResourceLoader};
use crate::verbs::HasObjectKey;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::now;
use sabi_core::io::CanLoan;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

#[derive(Debug)]
pub struct FileRequest {
    object_key: String,
    //    outfile: BufWriter<File>,
    outfile: PathBuf,
    //    tmp: NamedTempFile,
}

impl FileRequest {
    pub fn new(object_key: String, file_path: PathBuf) -> S3Result<Self> {
        //        Path::dir
        //        dirname

        //        let f = tempfile::NamedTempFile::new_in()?;

        //        let p: &Path = file_path.parent().unwrap();
        //        let tmp: NamedTempFile = NamedTempFile::new_in(p)?;

        Ok(FileRequest {
            object_key,
            //            outfile: BufWriter::new(File::create(file_path)?),
            outfile: file_path,
            //            tmp,
        })
    }
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

impl ResourceLoader for FileRequest {
    fn load(&self) -> S3Result<RequestResource> {
        let resource = RequestResource {
            body: None,
            hash: HashedPayload::empty(),
            region: None,
            content_type: None,
            requested_at: now(),
        };
        Ok(resource)
    }
}

//impl Write for FileRequest {
//    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//        self.tmp.write(buf)
//        // self.outfile.write(buf)
//    }
//
//    fn flush(&mut self) -> std::io::Result<()> {
//        self.tmp.flush()
//
//        /*
//        let f = &self.outfile;
//        self.tmp.persist(f);
//        // todo
//        Ok({})
//        */
//        //        self.outfile.flush()
//    }
//}

impl CanLoan for FileRequest {
    fn copy_from<A: Read>(&mut self, mut reader: A) -> io::Result<u64> {
        // todo: remove unwrap
        let dir: &Path = self.outfile.parent().unwrap();

        let mut tmp: NamedTempFile = NamedTempFile::new_in(dir)?;

        let size = io::copy(&mut reader, &mut tmp)?;

        tmp.persist(&self.outfile)?;
        Ok(size)
    }
}

/*
impl CanLoan for FileRequest {
    fn loan<F, Y>(&mut self, mut f: F) -> Y
    where
        F: FnMut(&mut Self) -> Y,
    {
        //        let p: &Path = file_path.parent().unwrap();
        //        let tmp: NamedTempFile = NamedTempFile::new_in(p)?;

        let result = f(self);
        result
    }
}
*/

impl super::Request for FileRequest {}
