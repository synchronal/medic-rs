use std::path::PathBuf;

#[derive(Debug)]
pub struct Manifest {}

impl Manifest {
    pub fn new(_path: PathBuf) -> Result<Manifest, Box<dyn std::error::Error>> {
        Ok(Manifest {})
    }
}
