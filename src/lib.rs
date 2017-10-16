//! The Vinegar crate provides simple constructs to make Rust tests more powerful.
//!
//! A simple function called `check` is provided to verify the result of the assertion macros.
//!
//! To use Vinegar in your Rust tests, you just write normal Rust tests, but instead of using the
//! `assert` and `assert_eq` combo, you use `check` to verify one or more assertion macro calls.
//!
//! For example:
//!
//! ```rust
//! # #[macro_use] extern crate vinegar;
//! # use vinegar::vinegar::*;
//! # fn main() {
//! check(vec![
//!     expect!(2 + 2 == 4),
//!     expect_eq!(2 + 2, 4),
//!     expect!({ 2 + 2 } > 3),
//!     expect!({ 2 + 2 } > { 1 + 1 + 1 })
//! ]);
//! # }
//! ```
//!
//! Notice that `expect` recognizes not only expressions, but also blocks, on both sides.
//!
//! > Prefer to use code blocks on both sides for the best possible error messages.
//!
//! The advantages of using `vinegar` over Rust's `assert` are:
//!
//! * all expectations are always run even if some of them fail.
//! * much better error messages.
//!
//! If an expectation fails, specially one using code blocks (so the result of the expression
//! can be computed and shown in the error message), then the error messages are much better
//! than you get with Rust's `assert`:
//!
//! For example, this test:
//!
//! ```rust,should_panic
//! # #[macro_use] extern crate vinegar;
//! # use vinegar::vinegar::*;
//! # fn main() {
//! check(vec![expect!({ 2 + 2 } < { 1 + 1 + 1 })]);
//! # }
//! ```
//!
//! Fails with this error message:
//!
//! ```text
//! iteration[0]:
//! * Condition failed: { 2 + 2 } < { 1 + 1 + 1 }
//!                     ---------   -------------
//!                         |             |
//!                         |             3
//!                         |
//!                         4
//! ```
//!
//! Much better than using `assert`:
//!
//! ```rust,should_panic
//! assert!(2 + 2 < 1 + 1 + 1);
//! ```
//!
//! Which gives a simpler but much less helpful error message:
//!
//! ```text
//! assertion failed: 2 + 2 < 1 + 1 + 1
//! ```
//!
//! When the expectation uses the equality `==` operator, if the test fails,
//! a colorized diff is shown.
//!
//! ```rust,should_panic
//! # #[macro_use] extern crate vinegar;
//! # use vinegar::vinegar::*;
//! # fn main() {
//! check(vec![expect!({ "Hello\nworld" } == { "Ola\nmundo" })]);
//! # }
//! ```
//!
//! Results in the diff (see the
//! [README](https://github.com/renatoathaydes/vinegar) page for screenshots):
//!
//! <pre>
//! * Condition failed: { "Hello\nworld" } == { "Ola\nmundo" }
//!                     ------------------    ----------------
//!                            |                     |
//!                            |                     Ola
//!                            |                     mundo
//!                            |
//!                            Hello
//!                            world
//! ----- Difference -----
//! <span style="color:red;">-Hello</span>
//! <span style="color:red;">-world</span>
//! <span style="color:green;">+Ola</span>
//! <span style="color:green;">+mundo</span>
//! ----------------------
//! </pre>
//!

#![crate_name = "vinegar"]
#![doc(html_root_url = "http://docs.rs/vinegar")]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate difference;
extern crate ansi_term;

/// The core module of the `vinegar` crate.
#[macro_use]
pub mod vinegar;

/// unit test module
#[cfg(test)]
mod tests {
    use vinegar::check;
    use ansi_term::Color::{Red, Green, White};

    /// Compare strings after removing trailing whitespaces from all lines
    fn assert_eq_multiline(left: &str, right: &str) {
        let l = left.split('\n').map(|line| line.trim_right())
            .collect::<Vec<_>>()
            .join("\n");
        let r = right.split('\n').map(|line| line.trim_right())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(l, r);
    }

    #[test]
    fn use_check() {
        // simple test: just check some expectations:
        check(vec![
            expect_eq!(2 + 2, 4),
            expect!({ 2 + 2 } == 4),
            expect_eq!("hi", "hi"),
        ]);

        // simple example-based test
        let examples = [1, 2, 3, ];

        check(examples.iter().map(|&ex| expect!(ex > 0)));

        // example-based test with both input and assertion as examples
        let examples = vec![
            // (input, expected result)
            (1, 2),
            (2, 4),
            (3, 6),
        ];

        check(examples.iter().map(|&(input, expected)| {
            let result = input * 2;
            expect_eq!(result, expected)
        }));
    }

    #[test]
    fn expect_block() {
        let one_to_100 = 1..100;
        check(vec![
            expect!({ &one_to_100.len() } > &90),
            expect!(&(45 * 2) < { &one_to_100.len() }),
            expect!(7 == { 2 + 5 }),
            expect!({ 2 + 5 } == 7),
            expect!({ 2 + 5 } == { 5 + 2 }),
            expect!({ 2 + 5 } >= { 5 + 2 }),
            expect!({ 2 + 5 } < { 5 + 3 }),
        ]);
    }

    #[test]
    fn expect_block_error_on_left() {
        let one_to_100 = 1..100;
        //check(vec![expect!({ one_to_100.len() } > 1000 )]);
        if let Err(msg) = expect!({ one_to_100.len() } > 1000 ) {
            assert_eq_multiline("\
* Condition failed: { one_to_100.len() } > 1000
                    --------------------
                              |
                              99\n", &msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!({ one_to_100.len() } < 1 ) {
            assert_eq_multiline("\
* Condition failed: { one_to_100.len() } < 1
                    --------------------
                              |
                              99\n", &msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!({ "hello".len() } > 25 ) {
            assert_eq_multiline("\
* Condition failed: { \"hello\".len() } > 25
                    -----------------
                            |
                            5\n", &msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!({ "hello".len() } > 5 * 5 ) {
            assert_eq_multiline("\
* Condition failed: { \"hello\".len() } > 5 * 5
                    -----------------   -----
                            |             |
                            |             25
                            |
                            5\n", &msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_block_error_on_right() {
        let one_to_100 = 1..100;
        //check(vec![expect!(100000000 < { one_to_100.len() })]);
        if let Err(msg) = expect!(1000 < { one_to_100.len() }) {
            assert_eq_multiline("\
* Condition failed: 1000 < { one_to_100.len() }
                           --------------------
                                      |
                                      99\n", &msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!(1 > { one_to_100.len() } ) {
            assert_eq_multiline("\
* Condition failed: 1 > { one_to_100.len() }
                        --------------------
                                  |
                                  99\n", &msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!(25 < { "hello".len() }) {
            assert_eq_multiline("\
* Condition failed: 25 < { \"hello\".len() }
                         -----------------
                                  |
                                  5\n", &msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_block_error_with_block_on_right() {
        let one_to_100 = 1..100;

        if let Err(msg) = expect!({ one_to_100.len() } > { 2000 + 22 }) {
            assert_eq_multiline("\
* Condition failed: { one_to_100.len() } > { 2000 + 22 }
                    --------------------   -------------
                              |                   |
                              |                   2022
                              |
                              99\n", &msg);
        } else {
            panic!("Should have failed");
        };

        if let Err(msg) = expect!({ one_to_100.len() } < { 3 * 5 + 2 }) {
            assert_eq_multiline("\
* Condition failed: { one_to_100.len() } < { 3 * 5 + 2 }
                    --------------------   -------------
                              |                   |
                              |                   17
                              |
                              99\n", &msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_string_eq_error() {
        // check(vec![expect!({ "hello" } == { "hevvo" })]);
        if let Err(msg) = expect!({ "hello" } == { "hevvo" }) {
            assert_eq_multiline(&format!("\
* Condition failed: {{ \"hello\" }} == {{ \"hevvo\" }}
                    -----------    -----------
                         |              |
                         |              hevvo
                         |
                         hello
----- Difference -----
{}{}
{}{}
----------------------\n", Red.paint("-"), White.on(Red).paint("hello"),
                                         Green.paint("+"), White.on(Green).paint("hevvo")), &msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_string_neq_error() {
        if let Err(msg) = expect!({ "hello" } != { "hello" }) {
            assert_eq_multiline("\
* Condition failed: { \"hello\" } != { \"hello\" }
                    -----------    -----------
                         |              |
                         |              hello
                         |
                         hello\n", &msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_simple_multi_line_eq_error() {
        //check(vec![expect!({ "Hello\nworld" } == { "Ola\nmundo" })]);

        if let Err(msg) = expect!({ "Hello\nworld" } == { "Ola\nmundo" }) {
            assert_eq_multiline(&format!("\
* Condition failed: {{ \"Hello\\nworld\" }} == {{ \"Ola\\nmundo\" }}
                    ------------------    ----------------
                             |                     |
                             |                     Ola
                             |                     mundo
                             |
                             Hello
                             world
----- Difference -----
{}
{}
{}
{}
----------------------\n",
                                         ["", &Red.paint("-Hello").to_string()].join(""),
                                         ["", &Red.paint("-world").to_string()].join(""),
                                         ["", &Green.paint("+Ola").to_string()].join(""),
                                         ["", &Green.paint("+mundo").to_string()].join(""),
            ), &msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_long_string_eq_error() {
        let text1 = "Roses are red, violets are blue,\n\
               I wrote this library here,\n\
               just for you.\n\
               (It's true).";

        let text2 = "Roses are red, violets are blue,\n\
               I wrote this documentation here,\n\
               just for you.\n\
               (It's quite true).";

        // check(vec![expect!({ text1 } == { text2 })]);

        if let Err(msg) = expect!({ text1 } == { text2 }) {
            assert_eq_multiline(&format!("\
* Condition failed: {{ text1 }} == {{ text2 }}
                    ---------    ---------
                        |            |
                        |            Roses are red, violets are blue,
                        |            I wrote this documentation here,
                        |            just for you.
                        |            (It's quite true).
                        |
                        Roses are red, violets are blue,
                        I wrote this library here,
                        just for you.
                        (It's true).
----- Difference -----
 Roses are red, violets are blue,
{}
{}
 just for you.
{}
{}
----------------------\n",
                                         [&Red.paint("-").to_string(), &Red.paint("I wrote this").to_string(), " ",
                                             &White.on(Red).paint("library").to_string(), " ",
                                             &Red.paint("here,").to_string()].join(""),
                                         [&Green.paint("+").to_string(), &Green.paint("I wrote this").to_string(), " ",
                                             &White.on(Green).paint("documentation").to_string(), " ",
                                             &Green.paint("here,").to_string()].join(""),
                                         [&Red.paint("-").to_string(), &Red.paint("(It's").to_string(),
                                             " ", &Red.paint("true).").to_string()].join(""),
                                         [&Green.paint("+").to_string(), &Green.paint("(It's").to_string(),
                                             " ", &White.on(Green).paint("quite").to_string(),
                                             " ", &Green.paint("true).").to_string()].join(""))
                                , &msg);
        } else {
            panic!("Should have failed");
        }
    }
}