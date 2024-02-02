#![doc = include_str!("../README.md")]

use std::io::Write;

/// A function that can be used to try to read a password (or a string
/// containing sensible information) from `/dev/tty`, instead of `stdin`,
/// thus hiding the input from the screen while it is being typed.
///
/// # Arguments
///
/// * `max_len` - The maximum length (in bytes) allowed for the user input,
/// point beyond which the inputted characters will be ignored. This value
/// must be greater than 0, else the process will be terminated.
/// * `prompt` - An optional argument to be used as prompt on `stdout`.
///
/// # Returned value
///
/// The function returns a [`Result`] containing a [`String`] if successful,
/// else at [`std::ffi::IntoStringError`], which, in this context,
/// could occur because `max_len` was reached and the `getpass` output was split
/// in the middle of a multi-byte UTF-8 character. For instance, let's say that we set
/// `max_len = 1` but the user inputs `é`, which latter character maps to UTF-8 bytes `c3 a9`.
/// In this case, the function will be calling `into_string` (on a [`std::ffi::CString`]
/// object) on the `[0xc3]` buffer, which will cause the error.
///
/// # Warning
///
/// The user should get acquainted with the information available in `man 3 getpass`
/// on the target system before using this function.
///
/// # Examples
/// ```rust,no_run
/// use c_getpass::getpass;
///
/// // Using a prompt
/// let max_len: usize = 5;
/// let prompt = Some("Enter a secret value: ");
/// let secret_value = getpass(max_len, prompt).expect("a multi-byte character was probably split");
/// assert!(secret_value.len() <= max_len);
/// println!("secret_value[{}] = {}", secret_value.len(), secret_value);
///
/// // Without a prompt
/// let max_len: usize = 50;
/// let secret_value = getpass(max_len, <Option<String>>::None)
///     .expect("a multi-byte character was probably split");
/// assert!(secret_value.len() <= max_len);
/// println!("secret_value[{}] = {}", secret_value.len(), secret_value);
/// ```
pub fn getpass(
    max_len: usize,
    prompt: Option<impl ToString>,
) -> Result<String, std::ffi::IntoStringError> {
    assert!(max_len > 0);
    if let Some(prefix) = prompt.as_ref() {
        print!("{}", prefix.to_string());
        std::io::stdout().flush().expect("failed to flush 'stdout'");
    }
    let effective_max_len: usize = max_len + 1; // We add one byte for the '\0' character required for a C string.
    let mut buffer = vec![0u8; effective_max_len];
    let n = unsafe { my_lib_getpass(buffer.as_mut_ptr(), effective_max_len) };
    buffer.truncate(n);
    std::ffi::CString::new(buffer)
        .expect("failed to create CString from buffer")
        .into_string()
}

#[link(name = "my_lib", kind = "static")]
extern "C" {
    // In C, this function's signature is `size_t my_lib_getpass(char *dst, size_t size)`,
    // where `size` corresponds to the allocated memory size to where `dst` points. The returned
    // `size_t` value corresponds to the actual number of bytes that were copied into `dst`,
    // not including the extra byte for the `\0` character. That means that `dst` can contain
    // at most `size - 1` bytes from the `getpass`'s output.
    fn my_lib_getpass(dst: *mut u8, size: libc::size_t) -> libc::size_t;
}
