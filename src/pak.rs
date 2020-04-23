pub mod file;
pub mod interface;

pub use self::{
    file::{
        File,
        FileHandle,
    },
    interface::PakInterface,
};

#[cfg(test)]
mod tests {
    use super::*;
    const PAK_1_PATH: &str = ".\\test_data\\Simple Building.pak";

    #[test]
    fn pak_interface_sanity() {
        let mut interface = PakInterface::new();
        let loaded_pak = interface.add_pak_file(PAK_1_PATH.as_bytes());
        assert!(loaded_pak);

        let iter = interface.find_file(b"*").unwrap();

        for data in iter {
            println!(
                "{:?} [{:?}], ({}) [is_dir: {}]",
                data.filename(),
                data.alternate_filename(),
                data.file_size(),
                data.is_dir()
            );
        }

        let paths = interface.list_all_file_paths();
        for path in &paths {
            println!("File path: {:?}", path);
        }

        let extract_path = "test-extract";
        let _ = std::fs::remove_dir_all(extract_path);
        std::fs::create_dir(extract_path).unwrap();

        for path in paths.iter() {
            let path_bytes = path.to_bytes();
            let name = path_bytes
                .rsplit(|c| *c == b'/' || *c == b'\\')
                .next()
                .unwrap();
            let path_dir = &path_bytes[0..path_bytes.len() - name.len()];
            std::fs::create_dir_all(format!(
                "{}/{}",
                extract_path,
                String::from_utf8_lossy(path_dir)
            ))
            .unwrap();

            let mut src = interface.open_file(path_bytes, b"rb").unwrap();
            let mut dest =
                std::fs::File::create(format!("{}/{}", extract_path, path.to_string_lossy()))
                    .unwrap();
            std::io::copy(&mut src, &mut dest).unwrap();
        }
    }
}
