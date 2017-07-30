# Vinegar

A collection of functions and macros to help testing Rust code.

## Macros

* `expect` checks a boolean condition, similar to `assert`.
* `expect_eq` checks two values for equality, similar to `assert_eq`.

## Functions

* `fn check<I>(expects: I) where I: IntoIterator<Item=Result<(), String>>`

Takes a collection of `Result<(), String>`, which happens to be the type of values
 returned by the expectation macros.

So, the `check` function is, basically, meant to check the assertions made with 
the `expect` macros.

## Usage

Example usage:

```rust
use vinegar::check;

#[test]
fn use_check() {
    // simple test: just check some expectations:
    check(vec![
        expect_eq!(2 + 2, 4),
        expect!(2 + 2 == 4),
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
```
