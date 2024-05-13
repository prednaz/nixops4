use anyhow::Result;
use nix_c_raw as raw;
use nix_util::string_return::{callback_get_vec_u8, callback_get_vec_u8_data};

pub struct StorePath {
    raw: *mut raw::StorePath, // NonNull // `raw` vs `inner`
}
impl StorePath {
    // replace this with a `Clone` implementation.
    // edit: there is the problem with this that `drop` will then be called
    // where it previously was not.
    pub fn new_raw_clone(raw: *const raw::StorePath) -> Self {
        Self::new_raw(unsafe { raw::nix_store_path_clone(raw as *mut raw::StorePath) })
    }
    pub fn new_raw(raw: *mut raw::StorePath) -> Self {
        StorePath { raw }
    }
    pub fn name(&self) -> Result<String> {
        let mut vec = Vec::new();
        unsafe {
            raw::nix_store_path_name(
                self.raw,
                Some(callback_get_vec_u8),
                callback_get_vec_u8_data(&mut vec),
            );
        }
        String::from_utf8(vec).map_err(|e| e.into())
    }
}
impl Drop for StorePath {
    fn drop(&mut self) {
        unsafe {
            raw::nix_store_path_free(self.raw);
        }
    }
}
