use crate::{alphabets::Password, generate_with_rng};
use rand_core::RngCore;
use rand_os::OsRng;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<OsRng> = {
        let mut rng = OsRng::new().expect("Failed to create secure random number generator");
        let mut buf = [0];
        rng.try_fill_bytes(&mut buf)
            .expect("Failed to read random numbers");

        Mutex::new(rng)
    };
}

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
        String::from_utf8_unchecked(
            generate_with_rng(&Password, 32, &mut *RNG.lock().unwrap()).unwrap(),
        )
    }
}
