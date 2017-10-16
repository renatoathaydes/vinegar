use difference::Changeset;
use difference::Difference;
use ansi_term::Colour;
use ansi_term::Colour::{Green, Red, White};
use std::string::ToString;
use std::ops::Deref;

enum ValuesToPrint {
    Both,
    First,
    Second,
    None

}

impl ValuesToPrint {
    fn has_first(&self) -> bool {
        match *self {
            ValuesToPrint::Both | ValuesToPrint::First => true,
            _ => false
        }
    }

    fn has_second(&self) -> bool {
        match *self {
            ValuesToPrint::Both | ValuesToPrint::Second => true,
            _ => false,
        }
    }

    fn has_both(&self) -> bool {
        match *self {
            ValuesToPrint::Both => true,
            _ => false
        }
    }
}

/// Check whether the given expectations have been met successfully.
///
/// # Panics
///
/// If any expectation fails, this function panics with an error message showing why each
/// expectation failed.
pub fn check<I>(expects: I)
    where I: IntoIterator<Item=Result<(), String>> {
    let mut failures = Vec::new();

    for (index, expect) in expects.into_iter().enumerate() {
        if let Err(err) = expect {
            failures.push(format!("iteration[{}]:\n{}", index, err));
        }
    }

    if !failures.is_empty() {
        panic!("\n{}", failures.join("\n"));
    }
}

fn get_diff(text1: &str, text2: &str) -> String {
    enum SecondIteration {
        SkipWithNewLine,
        SkipNoNewLine,
        NoNewLine,
        WithNewLine
    }

    let differences = Changeset::new(text1, text2, "\n").diffs;
    let diff_pairs = differences.windows(2);
    let mut result = String::with_capacity(text1.len() + text2.len());
    let mut second_iteration: SecondIteration;

    result.push_str("----- Difference -----\n");

    for diff_pair in diff_pairs {
        let prev = &diff_pair[0];
        let current = &diff_pair[1];

        match *prev {
            Difference::Same(ref x) => if x.is_empty() {
                second_iteration = SecondIteration::SkipNoNewLine;
            } else {
                result.push_str(&line_diff(&x, Option::None, ' '));
                second_iteration = SecondIteration::SkipWithNewLine;
            },
            Difference::Rem(ref x) => {
                if x.contains('\n') {
                    // several lines included in Rem, show them without word-by-word diff
                    result.push_str(&line_diff(&x, Option::Some(Red), '-'));
                } else {
                    // show word-by-word diff
                    match *current {
                        Difference::Add(ref y) => {
                            result.push_str(&word_by_word_diff(x, y, true));
                        }
                        _ => {
                            result.push_str(&line_diff(&x, Option::Some(Red), '-'));
                        }
                    }
                }
                second_iteration = SecondIteration::WithNewLine;
            }
            Difference::Add(_) => {
                second_iteration = SecondIteration::NoNewLine;
            }
        }

        match second_iteration {
            SecondIteration::SkipWithNewLine => {
                result.push('\n');
                continue
            }
            SecondIteration::SkipNoNewLine => continue,
            SecondIteration::NoNewLine => (),
            SecondIteration::WithNewLine => result.push('\n')
        }

        match *current {
            Difference::Same(_) => (),
            Difference::Add(ref x) => {
                if x.contains('\n') {
                    // several lines included in Rem, show them without word-by-word diff
                    result.push_str(&line_diff(&x, Option::Some(Green), '+'));
                } else {
                    // show word-by-word diff
                    match *prev {
                        Difference::Rem(ref y) => {
                            result.push_str(&word_by_word_diff(y, x, false));
                        }
                        _ => {
                            result.push_str(&line_diff(&x, Option::Some(Green), '+'));
                        }
                    }
                }
                result.push('\n');
            }
            Difference::Rem(_) => ()
        }
    }

    result.push_str("----------------------\n");

    result
}

fn line_diff(lines: &str, color: Option<Colour>, prefix: char) -> String {
    let format_line = |line: &str| {
        match color {
            Option::Some(c) => c.paint(format!("{}{}", prefix, line)).to_string(),
            Option::None => format!("{}{}", prefix, line)
        }
    };

    lines.split('\n').map(format_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn word_by_word_diff(x: &str, y: &str, is_removal: bool) -> String {
    let mut result = String::with_capacity(x.len() + y.len() + 20);
    let line_diffs = Changeset::new(x, y, " ").diffs;
    let base_color = if is_removal { Red } else { Green };
    result.push_str(&base_color.paint(if is_removal { "-" } else { "+" }).to_string());
    let mut line_diff_parts = Vec::with_capacity(line_diffs.len());
    for diff in line_diffs {
        match diff {
            Difference::Same(ref z) => if !z.is_empty() {
                line_diff_parts.push(base_color.paint(z.deref()).to_string());
            },
            Difference::Rem(ref z) => if !z.is_empty() {
                if is_removal {
                    line_diff_parts.push(White.on(base_color).paint(z.deref()).to_string());
                }
            },
            Difference::Add(ref z) => {
                if !is_removal {
                    line_diff_parts.push(White.on(base_color).paint(z.deref()).to_string());
                }
            }
        }
    }
    result.push_str(&line_diff_parts.join(" "));
    result
}

#[doc(hidden)]
pub fn internal_build_error(val1: &str, expr1: &str, op: &str, val2: &str, expr2: &str) -> String {
    let intro = "* Condition failed: ";

    let values_to_print: ValuesToPrint = if expr1 == val1 {
        if expr2 == val2 { ValuesToPrint::None } else { ValuesToPrint::Second }
    } else {
        if expr2 == val2 { ValuesToPrint::First } else { ValuesToPrint::Both }
    };

    let spaces = " ".repeat(intro.len());
    // quotes are rendered with an escape character, so we need to add to the length
    let expr1_len = expr1.len();
    let val1_underlines = (if values_to_print.has_first() { "-" } else { " " }).repeat(expr1_len);
    let val1_arrow_spaces = " ".repeat(expr1_len / 2);
    let val1_arrow = if values_to_print.has_first() { "|" } else { " " };

    let expr2_len = expr2.len();
    let val2_underlines = (if values_to_print.has_second() { "-" } else { " " }).repeat(expr2_len);
    let val2_arrow_spaces = " ".repeat(expr2_len / 2);
    let val2_arrow = if values_to_print.has_second() { "|" } else { " " };

    let op_spaces = " ".repeat(op.len() + 2);

    let last_lines_prefix = format!("{}{}", spaces, val1_arrow_spaces);
    let first_lines_prefix = format!("{}{}{}{}{}", last_lines_prefix, val1_arrow,
                                     val1_arrow_spaces, op_spaces, val2_arrow_spaces);

    let underlines_line = format!("{}{}{}{}\n", spaces, val1_underlines, op_spaces, val2_underlines);
    let both_arrows_line = format!("{}{}\n", first_lines_prefix, val2_arrow);
    let val2_lines = if values_to_print.has_second() {
        format!("{}\n", val2.split('\n')
            .map(|line| format!("{}{}", first_lines_prefix, line))
            .collect::<Vec<_>>()
            .join("\n"))
    } else {
        String::new()
    };
    let line4 = if values_to_print.has_both() {
        format!("{}{}\n", last_lines_prefix, val1_arrow)
    } else {
        String::new()
    };
    let val1_lines = if values_to_print.has_first() {
        format!("{}\n", val1.split('\n')
            .map(|line| format!("{}{}", last_lines_prefix, line))
            .collect::<Vec<_>>()
            .join("\n"))
    } else {
        String::new()
    };

    let error_diff = if op == "==" { get_diff(val1, val2) } else { String::new() };

    format!("{}{} {} {}\n{}{}{}{}{}{}",
            intro, expr1, op, expr2, underlines_line,
            both_arrows_line, val2_lines, line4, val1_lines, error_diff)
}


///
/// Create a general expectation that can be checked with [`check`][check].
///
/// This macro accepts expressions or blocks on either side of a boolean operator.
///
/// Code blocks on both sides should be preferred because that allows `vinegar` to run
/// the blocks and resolve a value which can be shown in the error message in case of failure,
/// which makes it much easier to understand why a test may have failed.
/// Also, if the `==` operator is used, a diff between the values can be shown.
///
/// [check]: vinegar/fn.check.html
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate vinegar;
/// # fn main() {
/// use vinegar::vinegar::check;
/// check(vec![
///     expect!({ 2 + 2 } == 4),
///     expect!({ 2 * 5 } == { 5 * 2 }),
///     expect!({ 2 * 5 } < { 3 * 5 }),
///     expect!("Hello world" == { format!("{} {}", "Hello", "world") })
/// ]);
/// # }
///
#[macro_export]
macro_rules! expect {

    ($b:block $op:tt $a:block) => {{
        if $b $op $a {
            Result::Ok(())
        } else {
            Result::Err($crate::vinegar::internal_build_error(
                &format!("{}", $b), stringify!($b), stringify!($op),
                &format!("{}", $a), stringify!($a)))
        }
    }};

    ($b:tt $op:tt $a:block) => {{
        if $b $op $a {
            Result::Ok(())
        } else {
            Result::Err($crate::vinegar::internal_build_error(
                &format!("{}", $b), stringify!($b), stringify!($op),
                &format!("{}", $a), stringify!($a)))
        }
    }};

    ($b:block $op:tt $($a:tt)+) => {{
        if $b $op $($a)* {
            Result::Ok(())
        } else {
            Result::Err($crate::vinegar::internal_build_error(
                &format!("{}", $b), stringify!($b), stringify!($op),
                &format!("{}", $($a)*), stringify!($($a)*)))
        }
    }};

    ($($a:tt)*) => {{
        if $($a)* {
            Result::Ok(())
        } else {
            Result::Err(format!("Condition failed: {}", stringify!($($a)*)))
        }
    }};

}

///
/// Create an equality expectation that can be checked with [`check`][check].
///
/// A call of the form `expect_eq!(a, b)` is just an alias for `expect!({ a } == { b })`.
///
/// [check]: vinegar/fn.check.html
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate vinegar;
/// # fn main() {
/// use vinegar::vinegar::check;
/// check(vec![
///     expect_eq!(2 + 2, 4),
///     expect_eq!("Hello world", &format!("{} {}", "Hello", "world"))
/// ]);
/// # }
/// ```
#[macro_export]
macro_rules! expect_eq {
    ($a:expr, $b: expr) => {{
        expect!({ $a } == { $b })
    }}
}
