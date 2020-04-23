use crate::pak::PakInterface;
use popcap_framework_sys as ffi;
use std::{
    convert::TryInto,
    io::Read,
};

#[derive(Debug)]
pub struct FileHandle(pub(crate) *mut ffi::pak_file_t);

impl FileHandle {
    pub fn new(ptr: *mut ffi::pak_file_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
}

pub struct File<'a> {
    handle: FileHandle,
    interface: &'a mut PakInterface,
}

impl<'a> File<'a> {
    pub fn open(interface: &'a mut PakInterface, path: &[u8], mode: &[u8]) -> Option<Self> {
        interface
            .fopen(path, mode)
            .map(move |handle| Self { handle, interface })
    }

    pub fn is_eof(&self) -> bool {
        unsafe { ffi::pak_interface_feof(self.interface.0, self.handle.0) != 0 }
    }

    pub fn position(&mut self) -> Option<u64> {
        unsafe { ffi::pak_interface_ftell(self.interface.0, self.handle.0) }
            .try_into()
            .ok()
    }
}

impl<'a> Read for File<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let ret = unsafe {
            ffi::pak_interface_fread(
                self.interface.0,
                buf.as_mut_ptr() as *mut std::ffi::c_void,
                1,
                buf.len().try_into().unwrap(),
                self.handle.0,
            )
        };

        if ret == 0 && !self.is_eof() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "fread returned 0",
            ));
        }

        Ok(ret as usize)
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.interface.fclose(&mut self.handle);
    }
}

impl<'a> std::fmt::Debug for File<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("is_eof", &self.is_eof())
            .finish()
    }
}
