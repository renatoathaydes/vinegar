#[macro_use]
pub mod vinegar;

#[cfg(test)]
mod tests {
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

    #[test]
    fn expect_len() {
        let empty_vec = Vec::<u8>::new();
        let one_to_100 = 1..100;

        check(vec![
            expect_len!(empty_vec, 0),
            expect_len!([1,2,3], 3),
            expect_len!("hello", 5),
            expect_len!(one_to_100, 99),
        ]);
    }

    #[test]
    fn expect_len_error() {
        let empty_vec = Vec::<u8>::new();
        let one_to_100 = 1..100;

        if let Err(msg) = expect_len!(empty_vec, 3) {
            assert_eq!("Length of empty_vec is 0, not 3 -- []", msg);
        } else {
            panic!("Should have failed");
        }
        if let Err(msg) = expect_len!(one_to_100, 66) {
            assert_eq!("Length of one_to_100 is 99, not 66 -- 1..100", msg);
        } else {
            panic!("Should have failed");
        }
        if let Err(msg) = expect_len!("hello", 2) {
            assert_eq!("Length of \"hello\" is 5, not 2 -- \"hello\"", msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_block() {
        let one_to_100 = 1..100;
        check(vec![
            expect!(|| { one_to_100.len() } > 90),
            expect!(|| { &one_to_100 }.len() > 90),
            expect!(|| { 2 + 5 } == 7),
        ]);
    }

    #[test]
    fn expect_block_error() {
        let one_to_100 = 1..100;
        if let Err(msg) = expect!(|| { one_to_100.len() } > 1000 ) {
            assert_eq!("\
* Condition failed: { one_to_100.len() } > 1000
                    --------------------
                              |
                              99\n", msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!(|| { one_to_100.len() } < 1 ) {
            assert_eq!("\
* Condition failed: { one_to_100.len() } < 1
                    --------------------
                              |
                              99\n", msg);
        } else {
            panic!("Should have failed");
        }

        if let Err(msg) = expect!(|| { "hello".len() } > 25 ) {
            assert_eq!("\
* Condition failed: { \"hello\".len() } > 25
                    -------------------
                             |
                             5\n", msg);
        } else {
            panic!("Should have failed");
        }
    }

    #[test]
    fn expect_block_error_with_expr_on_right() {
        let one_to_100 = 1..100;

        //check(vec![expect!(|| { one_to_100.len() } > || { 2000 + 22 })]);

        if let Err(msg) = expect!(|| { one_to_100.len() } > || { 2000 + 22 }) {
            assert_eq!("\
* Condition failed: { one_to_100.len() } > { 2000 + 22 }
                    --------------------   -------------
                              |                   |
                              |                   2022
                              |
                              99\n", msg);
        } else {
            panic!("Should have failed");
        }
    }
}