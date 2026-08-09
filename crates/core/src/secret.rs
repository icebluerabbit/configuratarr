//! Credential wrapper with redacted Debug and zero-on-drop semantics.
//!
//! The field type for any API key, password, or sensitive string. It doesn't
//! eliminate plaintext-in-memory (the value must exist as plaintext during JSON
//! construction and HTTP send) but bounds its lifetime: `zeroize` on drop
//! overwrites the backing bytes, and `Debug` never prints the contents.

use std::fmt;

use zeroize::Zeroize;

/// Owns a plaintext credential. Constructed only by the resolver phase when
/// it reads from env vars or files.
///
/// Not `Clone` by design — secrets shouldn't proliferate casually. If you need
/// to pass one to a codec, borrow it.
pub struct SecretValue {
    inner: Box<str>,
}

impl SecretValue {
    /// Wrap a plaintext credential. Caller is responsible for ensuring the
    /// source `String` (or its parent buffer) is dropped immediately after.
    pub fn new(plaintext: String) -> Self {
        Self {
            inner: plaintext.into_boxed_str(),
        }
    }

    /// Borrow the plaintext as `&str`. Only invoke at the encode/HTTP-send
    /// boundary; do not store the returned slice past the current scope.
    pub fn expose(&self) -> &str {
        &self.inner
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretValue([redacted])")
    }
}

impl Drop for SecretValue {
    fn drop(&mut self) {
        // Overwrite the box's backing bytes before deallocation. `Box<str>`
        // points at a UTF-8 region we own exclusively.
        // SAFETY: `inner` is owned, contiguous, and the bytes are valid for
        // the lifetime of the box.
        let bytes: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(self.inner.as_mut_ptr(), self.inner.len()) };
        bytes.zeroize();
    }
}
