use crate::{alphabets::Password, generate};

/// Generates an UTF-8 valid password with a length of 32 characters.
///
/// Only the first call to this function initializes a random number generator.
///
/// Internally, the [`Password`] set is used.
///
/// # Panics
///
/// This functions panics if the secure random number generator can not
/// be created or the generator fails.
///
/// # Example
///
/// ```
/// use mkpasswd::mkpasswd;
///
/// println!("Password 1: {}", mkpasswd());
/// println!("Password 2: {}", mkpasswd());
/// println!("Password 3: {}", mkpasswd());
///
/// assert_eq!(mkpasswd().len(), 32);
/// ```
///
/// [`Password`]: sets/constant.Password.html
pub fn mkpasswd() -> String {
    unsafe {
        String::from_utf8_unchecked(generate(&Password, 32).unwrap())
    }
}
