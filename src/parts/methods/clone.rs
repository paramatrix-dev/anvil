use crate::Part;

impl Clone for Part {
    fn clone(&self) -> Self {
        match &self.inner {
            Some(inner) => Self::from_occt(inner),
            None => Part { inner: None },
        }
    }
}
