use crate::pak::file::{
    File,
    FileHandle,
};
use popcap_framework_sys as ffi;
use std::{
    convert::TryInto,
    ffi::{
        CStr,
        CString,
    },
    mem::MaybeUninit,
};

pub struct PakInterface(pub(crate) *mut ffi::pak_interface_t);

impl PakInterface {
    /// Create a new PakInterface. Panics if OOM.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::pak_interface_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

impl PakInterface {
    /// Adds a pak file. Uses A-type windows functions. Verify your strings. Panics if invalid c-string is passed in, NOT invalid ansi string.
    pub fn add_pak_file(&mut self, path: &[u8]) -> bool {
        let path = CString::new(path).unwrap();
        unsafe { ffi::pak_interface_add_pak_file(self.0, path.as_ptr()) }
    }

    /// Opens a file. May or may not be inside a pak file. Panics if the name or access contains '\0'.
    pub fn fopen(&mut self, name: &[u8], access: &[u8]) -> Option<FileHandle> {
        let name = CString::new(name).unwrap();
        let access = CString::new(access).unwrap();
        let file = unsafe { ffi::pak_interface_fopen(self.0, name.as_ptr(), access.as_ptr()) };

        FileHandle::new(file)
    }

    /// Closes a file. Returns a 0 if successful.
    pub fn fclose(&mut self, file: &mut FileHandle) -> i32 {
        unsafe { ffi::pak_interface_fclose(self.0, file.0) }
    }

    /// Opens a file. May or may not be inside a pak file. Panics if the name or access contains '\0'. High Level interface
    pub fn open_file(&mut self, name: &[u8], access: &[u8]) -> Option<File> {
        File::open(self, name, access)
    }

    /// Finds a file like the windows FindFirstFile function. Panics if serach is not a valid c-string. Uses ANSI codepages.
    pub fn find_first_file(&mut self, search: &[u8]) -> Option<(FileSearch, FileData)> {
        let search = CString::new(search).unwrap();
        let mut data: MaybeUninit<ffi::_WIN32_FIND_DATAA> = MaybeUninit::zeroed();
        let handle = unsafe {
            ffi::pak_interface_find_first_file(self.0, search.as_ptr(), data.as_mut_ptr())
        };

        if handle == ffi::INVALID_HANDLE_VALUE {
            return None;
        }

        let data = unsafe { FileData::new(data.assume_init()) };

        Some((FileSearch::new(handle), data))
    }

    /// Advances to the next element of a search
    pub fn find_next_file(&mut self, search: &mut FileSearch) -> Option<FileData> {
        let mut data: MaybeUninit<ffi::_WIN32_FIND_DATAA> = MaybeUninit::zeroed();
        let ret = unsafe { ffi::pak_interface_find_next_file(self.0, search.0, data.as_mut_ptr()) };
        if !ret {
            return None;
        }

        let data = unsafe { FileData::new(data.assume_init()) };
        Some(data)
    }

    /// Closes a search
    pub fn find_close(&mut self, search: &mut FileSearch) -> bool {
        unsafe { ffi::pak_interface_find_close(self.0, search.0) }
    }

    /// A high level iterator interface for searching. Includes files found in cwd, ignore all files past the filename '.' to compensate.
    pub fn find_file(&mut self, query: &[u8]) -> Option<FileSearchIter> {
        FileSearchIter::new(self, query)
    }

    /// List all loaded paths. Excludes paths without filetimes for safety reasons.
    pub fn list_all_file_paths(&mut self) -> Vec<CString> {
        let mut size = 0;
        let mut data = Vec::new();
        loop {
            let ret = unsafe {
                ffi::pak_interface_list_all_file_paths(self.0, data.as_mut_ptr(), &mut size)
            };

            if ret {
                return data
                    .into_iter()
                    .map(|p| {
                        assert!(!p.is_null());
                        unsafe { CStr::from_ptr(p).to_owned() }
                    })
                    .collect();
            } else {
                data.resize(size.try_into().unwrap(), std::ptr::null());
            }
        }
    }
}

impl Drop for PakInterface {
    fn drop(&mut self) {
        unsafe {
            ffi::pak_interface_destroy(self.0);
        }
    }
}

impl Default for PakInterface {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FileData(ffi::_WIN32_FIND_DATAA);

impl FileData {
    pub fn new(data: ffi::_WIN32_FIND_DATAA) -> Self {
        Self(data)
    }

    pub fn filename(&self) -> &CStr {
        unsafe { CStr::from_ptr(&self.0.cFileName as *const i8) }
    }

    pub fn alternate_filename(&self) -> &CStr {
        unsafe { CStr::from_ptr(&self.0.cAlternateFileName as *const i8) }
    }

    pub fn file_size(&self) -> u64 {
        let high =
            u64::from(self.0.nFileSizeHigh) << (std::mem::size_of::<ffi::DWORD>() as u64 * 8);
        let low = u64::from(self.0.nFileSizeLow);
        high | low
    }

    pub fn is_dir(&self) -> bool {
        self.0.dwFileAttributes & ffi::FILE_ATTRIBUTE_DIRECTORY != 0
    }
}

impl std::fmt::Debug for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileData")
            .field("filename", &self.filename())
            .field("alternate_filename", &self.alternate_filename())
            .field("file_size", &self.file_size())
            .field("is_dir", &self.is_dir())
            .finish()
    }
}

#[derive(Debug)]
pub struct FileSearch(pub(crate) ffi::HANDLE);

impl FileSearch {
    pub fn new(h: ffi::HANDLE) -> Self {
        assert!(!h.is_null());
        Self(h)
    }
}

pub struct FileSearchIter<'a> {
    search: FileSearch,
    data: Option<FileData>,
    interface: &'a mut PakInterface,
}

impl<'a> FileSearchIter<'a> {
    pub(crate) fn new(interface: &'a mut PakInterface, query: &[u8]) -> Option<Self> {
        let (search, data) = interface.find_first_file(query)?;

        Some(Self {
            search,
            data: Some(data),
            interface,
        })
    }
}

impl<'a> Iterator for FileSearchIter<'a> {
    type Item = FileData;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self.interface.find_next_file(&mut self.search);
        std::mem::swap(&mut self.data, &mut ret);

        ret
    }
}

impl<'a> Drop for FileSearchIter<'a> {
    fn drop(&mut self) {
        self.interface.find_close(&mut self.search);
    }
}
