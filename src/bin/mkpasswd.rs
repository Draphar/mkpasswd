extern crate mkpasswd;
extern crate rand_core;
extern crate rand_os;

use mkpasswd::{alphabets, generate, generate_with_rng};
use rand_core::RngCore;
use rand_os::OsRng;

use std::borrow::Cow;
use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let (count, alphabet, length) = parse_args().unwrap_or_else(|message| {
        eprintln!("error: {}", message);
        process::exit(1)
    });

    if count == 1 {
        generate(&alphabet, length)
            .map_err(|e| e.into())
            .and_then(|password| io::stdout().write_all(&password))
            .unwrap_or_else(|e| {
                eprintln!("error: failed to generate password: {}", e);
                process::exit(1)
            });
    } else {
        let mut rng = OsRng::new().unwrap_or_else(|e| {
            eprintln!("error: failed to create random number generator: {}", e);
            process::exit(1)
        });
        let mut test = [0];
        rng.try_fill_bytes(&mut test).unwrap_or_else(|e| {
            eprintln!("error: failed to create random number generator: {}", e);
            process::exit(1)
        });

        let _stdout = io::stdout();
        let mut stdout = _stdout.lock();
        for _ in 0..count {
            match generate_with_rng(&alphabet, length, &mut rng) {
                Ok(password) => {
                    stdout.write_all(&password).unwrap();
                    stdout.write(b"\n").unwrap();
                }
                Err(e) => eprintln!("error: failed to get random data: {}", e),
            }
        }
    }
}

fn parse_args() -> Result<(usize, Box<[u8]>, usize), Cow<'static, str>> {
    let mut args = env::args_os().skip(1);

    let mut count: usize = 0;
    let mut length: usize = 0;
    let mut alphabet: Option<Box<[u8]>> = None;

    while let Some(arg) = args.next() {
        if (arg == "-n" || arg == "--count") && count == 0 {
            let arg = if let Some(arg) = args.next() {
                arg
            } else {
                return Err(Cow::from(format!("expected value for {:?}", arg)));
            };
            match arg
                .into_string()
                .map_err(|_| Cow::from("invalid count"))
                .and_then(|n| {
                    n.parse()
                        .map_err(|e| Cow::from(format!("invalid number: {}", e)))
                }) {
                Ok(n) => {
                    if n == 0 {
                        return Err(Cow::from("the count can not be 0"));
                    } else {
                        count = n;
                    }
                }
                Err(e) => return Err(Cow::from(e)),
            };
        } else if arg == "-h" || arg == "--help" {
            process::exit(match io::stdout().write_all(include_bytes!("help.txt")) {
                Ok(_) => 0,
                Err(_) => 1,
            });
        } else {
            let mut set_alphabet = |new_alphabet| {
                if alphabet.is_some() {
                    eprintln!("warning: only one alphabet can be used");
                } else {
                    alphabet = Some(new_alphabet);
                };
            };

            if arg == "-a" || arg == "--alphabet" {
                if let Some(arg) = args.next() {
                    if alphabet.is_some() {
                        // Save the calculation if a alphabet is already in use
                        eprintln!("warning: only one alphabet can be used");
                        args.next();
                        continue;
                    };

                    if arg.is_empty() {
                        return Err(Cow::from("a custom may alphabet not be empty"));
                    };

                    let mut custom_alphabet = match arg.into_string() {
                        Ok(string) => string.into_bytes(),
                        Err(_) => return Err(Cow::from("a custom alphabet must be valid UTF-8")),
                    };

                    custom_alphabet.sort();
                    custom_alphabet.dedup();

                    alphabet = Some(custom_alphabet.into_boxed_slice());
                } else {
                    return Err(Cow::from(format!("expected value for {:?}", arg)));
                };
            } else if arg == "--password" {
                set_alphabet(Box::new(alphabets::Password));
            } else if arg == "--latin-numbers" {
                set_alphabet(Box::new(alphabets::LatinNumbers));
            } else if arg == "--base64" {
                set_alphabet(Box::new(alphabets::Base64));
            } else if arg == "--base64-url" {
                set_alphabet(Box::new(alphabets::Base64Url));
            } else if arg == "--numbers" {
                set_alphabet(Box::new(alphabets::Numbers));
            } else if arg == "--latin" {
                set_alphabet(Box::new(alphabets::Latin));
            } else if arg == "--latin-lower" {
                set_alphabet(Box::new(alphabets::LatinLower));
            } else if arg == "--latin-lower-numbers" {
                set_alphabet(Box::new(alphabets::LatinLowerNumbers));
            } else if arg == "--latin-upper" {
                set_alphabet(Box::new(alphabets::LatinUpper));
            } else if arg == "--latin-upper-numbers" {
                set_alphabet(Box::new(alphabets::LatinUpperNumbers));
            } else if length == 0 {
                if let Ok(n) = arg
                    .into_string()
                    .map_err(|_| ())
                    .and_then(|n| n.parse().map_err(|_| ()))
                {
                    if n == 0 {
                        return Err(Cow::from("password length can not be 0"));
                    };
                    length = n;
                } else {
                    return Err(Cow::from("invalid password length"));
                };
            } else {
                eprintln!("warning: unexpected {:?}", arg);
            };
        };
    }

    if count == 0 {
        count = 1;
    };
    if length == 0 {
        length = 32;
    };

    Ok((
        count,
        alphabet.unwrap_or_else(|| Box::new(alphabets::Password)),
        length,
    ))
}
