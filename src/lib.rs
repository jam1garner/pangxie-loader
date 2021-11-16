use std::{
    alloc::{alloc, Layout},
    ffi::CStr,
    fs, mem,
    os::raw::c_char,
    path::Path,
    slice,
};

use skyline::install_hook;

#[repr(C)]
struct FileInfo {
    file_data_ptr: *const u8,
    unknown: u32, // zero initialized
    file_len: u32,
    allocation_size: u32,
    padding: u32,
}

/// Given the game's internal path ("spel2:/...") and a path to a mod mod_folder,
/// check if the file exists in the mod folder and if so load it into memory.
///
/// If the file does not exist, this returns `None`
/// If the file does exist, this returns a Vec containing the contents of the file
fn read_file(game_path: &str, mod_folder: &Path) -> Option<Vec<u8>> {
    let mod_path = Path::new(mod_folder).join(game_path.strip_prefix("spel2:/")?);
    fs::read(&mod_path).ok()
}

#[skyline::hook(offset = 0x31db90)]
fn read_encrypted_file(c_path: *const c_char) -> *mut FileInfo {
    let path = unsafe { CStr::from_ptr(c_path) }.to_string_lossy();
    println!("loading path: {:?}", path);

    if let Some(file_data) = read_file(&path, &Path::new("sd:/spelunky2/mods/my_mod")) {
        println!("Loading mod...");
        let file_len = file_data.len();
        let allocation_size = file_len + mem::size_of::<FileInfo>();

        unsafe {
            let file_info_ptr = alloc(
                Layout::from_size_align(allocation_size, mem::align_of::<FileInfo>()).unwrap(),
            );

            let file_data_ptr = file_info_ptr.add(mem::size_of::<FileInfo>());

            let file_data_slice = slice::from_raw_parts_mut(file_data_ptr, file_len);
            file_data_slice.copy_from_slice(&file_data);

            let file_data_ptr = file_data_ptr as *const u8;

            let file_info_ptr = file_info_ptr as *mut FileInfo;
            *file_info_ptr = FileInfo {
                file_data_ptr,
                unknown: 0,
                file_len: file_len as u32,
                allocation_size: allocation_size as u32,
                padding: 0,
            };

            file_info_ptr
        }
    } else {
        println!("Loading original file...");
        call_original!(c_path)
    }
}

#[skyline::main(name = "file-replacement")]
pub fn main() {
    assert_eq!(std::mem::size_of::<FileInfo>(), 0x18);
    install_hook!(read_encrypted_file);
}
