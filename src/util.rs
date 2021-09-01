use std::path::Path;
use dochy::error::DpResult;

pub(crate) struct FileLen{
    file_name : String,
    len : u64,
}
impl FileLen{
    pub(crate) fn file_name(&self) -> &str{ &self.file_name }
    pub(crate) fn len(&self) -> u64{ self.len }
}

pub(crate) fn get_file_lens<P : AsRef<Path>>(dir : P, ext : &str) -> DpResult<Vec<FileLen>>{
    let dir = dir.as_ref();
    let mut vec : Vec<FileLen> = Vec::new();
    let ite = std::fs::read_dir(dir)?;
    for f in ite{
        let e = f?;
        if e.path().extension().map(|s| s.to_str()) == Some(Some(ext)) {
            let meta = e.metadata()?;
            vec.push(FileLen{
                file_name : e.file_name().to_str().unwrap().to_string(),
                len : meta.len(),
            })
        }
    }
    Ok(vec)
}