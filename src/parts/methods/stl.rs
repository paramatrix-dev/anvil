use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use opencascade_sys::ffi;
use tempfile::NamedTempFile;

use crate::{Error, Part};

impl Part {
    /// Write the `Part` to a file in the STL format.
    pub fn write_stl(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        self.write_stl_with_tolerance(path, 0.0001)
    }

    /// Write the `Part` to a file in the STL format with a specified tolerance.
    ///
    /// Smaller tolerances lead to higher precision in rounded shapes, but also larger file size.
    pub fn write_stl_with_tolerance(
        &self,
        path: impl AsRef<Path>,
        tolerance: f64,
    ) -> Result<(), Error> {
        match &self.inner {
            Some(inner) => {
                let mut writer = ffi::StlAPI_Writer_ctor();
                let mesh = ffi::BRepMesh_IncrementalMesh_ctor(inner, tolerance);
                let success = ffi::write_stl(
                    writer.pin_mut(),
                    mesh.Shape(),
                    path.as_ref().to_string_lossy().to_string(),
                );
                if success {
                    Ok(())
                } else {
                    Err(Error::StlWrite(path.as_ref().to_path_buf()))
                }
            }
            None => Err(Error::EmptyPart),
        }
    }
    /// Return the STL lines that describe this `Part`.
    pub fn stl(&self) -> Result<Vec<String>, Error> {
        match &self.inner {
            Some(_) => {
                let temp_file = NamedTempFile::new().expect("could not create tempfile");
                let path = temp_file.path();

                self.write_stl(path)?;

                let file = fs::File::open(path).map_err(|_| Error::StlWrite(path.into()))?;
                let lines = io::BufReader::new(file)
                    .lines()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| Error::StlWrite(path.into()))?;

                Ok(lines)
            }
            None => Err(Error::EmptyPart),
        }
    }
}
