/*!
# mkpasswd - Library and command-line tool to generate passwords

## Command line tool

Install the command line tool with `cargo install mkpasswd`.
You can then use the `mkpasswd` command, run it with `--help` to get more information.

## Library

The library exposes only a few functions dedicated to random password generation.

- If you want it simple and can risk panics, use the [`mkpasswd`] function.

If you want more control and error handling, use [`generate`] and [`generate_with_rng`].
These functions take an alphabet, which is a set of characters that the password may contain.

- The most control is offered by [`generate_with_rng`], because it uses an external random number generator.
  You should this if you want to generate multiple passwords.

[`mkpasswd`]: fn.mkpasswd.html
[`generate`]: fn.generate.html
[`generate_with_rng`]: fn.generate_with_rng.html
*/

pub extern crate rand_core;
pub extern crate rand_os;

pub mod alphabets;
mod default;

pub use {alphabets::Password, default::mkpasswd};

use rand_core::{CryptoRng, Error, RngCore};
use rand_os::OsRng;

/// Generates a `length` long password made of bytes from `alphabet`.
///
/// This function uses the cryptographically secure [`OsRng`] internally.
///
/// Also see [`generate_with_rng`] for more information.
///
/// # Errors
///
/// See the [error section of `generate_with_rng`].
///
/// # Example
///
/// ```
/// use mkpasswd::{generate, Password};
///
/// let password = generate(&Password, 10).unwrap();
/// // `Password` contains only valid UTF-8 characters
/// let password = String::from_utf8(password).unwrap();
///
/// assert_eq!(password.len(), 10);
///
/// println!("Your password: {}", password);
/// ```
///
/// [`OsRng`]: https://docs.rs/rand_os/0.2.1/rand_os/struct.OsRng.html
/// [`generate_with_rng`]: fn.generate_with_rng.html
/// [error section of `generate_with_rng`]: fn.generate_with_rng.html#errors
#[inline]
pub fn generate(alphabet: &[u8], length: usize) -> Result<Vec<u8>, Error> {
    generate_with_rng(alphabet, length, &mut OsRng)
}

/// Generates a `length` long password made of bytes from `alphabet`
/// using random data provided by `rng`.
///
/// If the input alphabet only contains valid UTF-8 bytes, this function
/// is guaranteed to only return valid UTF-8 as well.
///
/// # Example
///
/// This example is using a custom alphabet as well.
///
/// ```
/// use mkpasswd::generate_with_rng;
/// use rand_os::OsRng;
///
/// let alphabet = b"qwerty+asdfgh-zxcvbn";
/// let mut rng = OsRng;
/// let password = generate_with_rng(alphabet, 16, &mut rng).unwrap();
///
/// assert_eq!(password.len(), 16);
/// ```
///
/// It is recommended to sanitize the alphabet before, removing duplicate values:
///
/// ```
/// let mut my_alphabet = b"Hello world!".to_vec();
///
/// my_alphabet.sort();
/// my_alphabet.dedup();
///
/// // 'l' and 'o' is only once in the buffer now
/// assert_eq!(my_alphabet, b" !Hdelorw");
/// ```
///
/// # Errors
///
/// This functions returns an error if `rng` fails to generate random data.
///
/// If `alphabet.len()` is `0`, this functions panics.
pub fn generate_with_rng<R>(alphabet: &[u8], length: usize, rng: &mut R) -> Result<Vec<u8>, Error>
where
    R: RngCore + CryptoRng,
{
    assert!(!alphabet.is_empty(), "The alphabet may not be empty");
    if length == 0 {
        return Ok(Vec::new());
    };

    let mut buf = Vec::with_capacity(length);

    while buf.len() < length {
        let bytes = rng.next_u64().to_ne_bytes();
        for i in bytes.into_iter() {
            if alphabet.contains(i) {
                buf.push(*i);
            };
        }
    }

    unsafe {
        // Drop excess bytes
        buf.set_len(length);
    };

    Ok(buf)
}
