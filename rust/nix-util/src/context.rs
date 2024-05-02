use anyhow::{bail, Result};
use nix_c_raw as raw;
use std::ptr::null_mut;
use std::ptr::NonNull;

pub struct Context {
    inner: NonNull<raw::nix_c_context>,
}

impl Context {
    pub fn new() -> Self {
        let ctx = unsafe { raw::nix_c_context_create() };
        if ctx.is_null() {
            panic!("nix_c_context_create returned a null pointer");
        }
        let ctx = Context {
            inner: NonNull::new(ctx).unwrap(),
        };
        ctx
    }
    pub fn ptr(&self) -> *mut raw::nix_c_context {
        self.inner.as_ptr()
    }
    pub fn check_err(&self) -> Result<()> {
        let err = unsafe { raw::nix_err_code(self.inner.as_ptr()) };
        if err != raw::NIX_OK.try_into().unwrap() {
            // msgp is a borrowed pointer, so we don't need to free it
            let msgp = unsafe { raw::nix_err_msg(null_mut(), self.inner.as_ptr(), null_mut()) };
            // Turn the i8 pointer into a Rust string by copying
            let msg: &str = unsafe { core::ffi::CStr::from_ptr(msgp).to_str()? };
            bail!("{}", msg);
        }
        Ok(())
    }
    /// NIX_ERR_KEY is returned when e.g. an attribute is missing. Return true if the error is of this type.
    pub fn is_key_error(&self) -> bool {
        unsafe { raw::nix_err_code(self.inner.as_ptr()) == raw::NIX_ERR_KEY }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            raw::nix_c_context_free(self.inner.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_new_and_drop() {
        // don't crash
        let _c = Context::new();
    }
}
