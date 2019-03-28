# mkpasswd - generate passwords

[![travis-badge]][travis]
[![appveyor-badge]][appveyor]
[![crates.io-badge]][crates.io]
[![license-badge]][license]

[travis-badge]: https://img.shields.io/travis/com/Draphar/mkpasswd.svg?branch=master&style=flat-square
[travis]: https://travis-ci.com/Draphar/mkpasswd
[appveyor-badge]: https://img.shields.io/appveyor/ci/Draphar/mkpasswd.svg?style=flat-square
[appveyor]: https://ci.appveyor.com/project/Draphar/mkpasswd
[crates.io-badge]: https://img.shields.io/crates/v/mkpasswd.svg?style=flat-square
[crates.io]: https://crates.io/crates/mkpasswd
[license-badge]: https://img.shields.io/crates/l/mkpasswd.svg?style=flat-square
[license]: https://github.com/Draphar/mkpasswd/blob/master/LICENSE

## Installation

Using [`cargo`]:

`cargo install mkpasswd`

You then have access to the `mkpasswd` command.

## Usage

If you simply want a securely generated random password, simply run
`mkpasswd` to get a password with special characters and a length of 32 letters.

However, the tool offer more options.

- The password length can be set by passing a command line argument like `mkpasswd 10`,
which will generate a 10-character password.

- If you want to generate multiple passwords, pass `--count` or `-n` like `mkpasswd -n 5`.

You can always pass `--help` or `-h` to `mkpasswd` to retrieve this help.

### Using alphabets

An alphabet defines the character the password can contain.

The standard alphabet is `--password`.

There a few predefined alphabets, which can passed directly:
```
--password              a-z, A-Z, 0-9, each of =+-*/,;:.!?&|^<>(){}_%@#
--latin-numbers         a-z, A-Z, 0-9
--base64                a-z, A-Z, 0-9, +, /
--base64-url            a-z, A-Z, 0-9, -, _
--numbers               0-9
--latin                 a-z, A-Z
--latin-lower           a-z
--latin-lower-numbers   a-z, 0-9
--latin-upper           A-Z
--latin-upper-numbers   A-Z, 0-9
```
Another option is passing a string containg all the characters you want after `--alphabet` or `-a`:

`mkpass -a "qwertyQWERTY"`

[`cargo`]: https://doc.rust-lang.org/cargo/
