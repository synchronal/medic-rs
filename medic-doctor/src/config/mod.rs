use crate::AppResult;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Manifest {}

impl Manifest {
    pub fn new(_path: PathBuf) -> AppResult<Manifest> {
        Ok(Manifest {})
    }
}
