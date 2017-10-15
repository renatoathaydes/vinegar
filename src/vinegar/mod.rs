use difference::Changeset;
use difference::Difference;
use ansi_term::Colour;
use ansi_term::Colour::{Green, Red, White};
use std::string::ToString;
use std::ops::Deref;

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
pub fn internal_build_error(bs: &str, be: &str, op: &str, astr: &str, ae: &str) -> String {
    let spaces = "                    ";
    // quotes are rendered with an escape character, so we need to add to the length
    let belen = be.len();
    let be_underlines = "-".repeat(belen);
    let be_arrow_spaces = " ".repeat(belen / 2);

    let aelen = ae.len();
    let ae_underlines = "-".repeat(aelen);
    let ae_arrow_spaces = " ".repeat(aelen / 2);

    let op_spaces = " ".repeat(op.len() + 2);

    let last_lines_prefix = format!("{}{}", spaces, be_arrow_spaces);
    let first_lines_prefix = format!("{}|{}{}{}",
                                     last_lines_prefix, be_arrow_spaces, op_spaces, ae_arrow_spaces);

    let line1 = format!("{}{}{}{}", spaces, be_underlines, op_spaces, ae_underlines);
    let line2 = format!("{}|", first_lines_prefix);
    let line3 = astr.split('\n')
        .map(|line| format!("{}{}", first_lines_prefix, line))
        .collect::<Vec<_>>()
        .join("\n");
    let line4 = format!("{}|", last_lines_prefix);
    let val_line = bs.split('\n')
        .map(|line| format!("{}{}", last_lines_prefix, line))
        .collect::<Vec<_>>()
        .join("\n");

    let error_diff = if op == "==" { get_diff(bs, astr) } else { String::new() };

    format!("* Condition failed: {} {} {}\n{}\n{}\n{}\n{}\n{}\n{}",
            be, op, ae, line1, line2, line3, line4, val_line, error_diff)
}

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

    ($b:block $($a:tt)+) => {{
        if $b $($a)* {
            Result::Ok(())
        } else {
            let bs = format!("{:?}", $b);
            let be = stringify!($b);
            // quotes are rendered with an escape character, so we need to add to the length
            let belen = be.len() + be.matches("\"").count();
            let spaces = "                    ";
            let underlines = "-".repeat(belen);
            let arrow_line_spaces = " ".repeat(belen / 2);
            let arrow_line = format!("{}{}|", spaces, arrow_line_spaces);
            let val_line = format!("{}{}{}", spaces, arrow_line_spaces, bs);
            Result::Err(format!("* Condition failed: {} {}\n{}{}\n{}\n{}\n",
                be, stringify!($($a)*), spaces, underlines, arrow_line, val_line))
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

#[macro_export]
macro_rules! expect_eq {
    ($a:expr, $b: expr) => {{
        if $a == $b {
            Result::Ok(())
        } else {
            Result::Err(format!("Equality failed: {:?} != {:?}", $a, $b))
        }
    }}
}
