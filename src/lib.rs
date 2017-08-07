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
}
