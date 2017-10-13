use difference::diff;

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

pub fn get_diff(text1: &str, text2: &str) -> String {
    use difference::Difference;
    use ansi_term::Colour::{Green, Red};
    use std::string::ToString;

    let (_, differences) = diff(text1, text2, "\n");
    let mut result = String::with_capacity(text1.len() + text2.len());

    result.push_str("----- Difference -----\n");

    for diff in differences {
        match diff {
            Difference::Same(ref x) => {
                result.push_str(x);
            }
            Difference::Add(ref x) => {
                result.push_str(&Green.paint(format!("{}{}", "+", x)).to_string());
            }
            Difference::Rem(ref x) => {
                result.push_str(&Red.paint(format!("{}{}", "-", x)).to_string());
            }
        }

        result.push('\n');
    }

    result.push_str("\n----------------------\n");

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

    let line1 = format!("{}{}{}{}", spaces, be_underlines, op_spaces, ae_underlines);
    let line2 = format!("{}{}|{}{}{}|",
                        spaces, be_arrow_spaces, be_arrow_spaces, op_spaces, ae_arrow_spaces);
    let line3 = format!("{}{}|{}{}{}{}", spaces, be_arrow_spaces, be_arrow_spaces, op_spaces, ae_arrow_spaces, astr);
    let line4 = format!("{}{}|", spaces, be_arrow_spaces);
    let val_line = format!("{}{}{}", spaces, be_arrow_spaces, bs);

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
