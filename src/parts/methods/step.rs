use std::path::Path;

use opencascade_sys::ffi;

use crate::{Error, Part};

impl Part {
    /// Write the `Part` to a file in the STEP format.
    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match &self.scale(1000.).inner {
            Some(inner) => {
                let mut writer = ffi::STEPControl_Writer_ctor();
                let status = ffi::transfer_shape(writer.pin_mut(), inner);
                if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
                    return Err(Error::StepWrite(path.as_ref().to_path_buf()));
                }
                let status = ffi::write_step(
                    writer.pin_mut(),
                    path.as_ref().to_string_lossy().to_string(),
                );
                if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
                    return Err(Error::StepWrite(path.as_ref().to_path_buf()));
                }
            }
            None => return Err(Error::EmptyPart),
        }
        Ok(())
    }
}
