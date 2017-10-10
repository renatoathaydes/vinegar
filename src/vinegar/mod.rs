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

pub fn build_error(bs: &str, be: &str, op: &str, astr: &str, ae: &str) -> String {
    let spaces = "                    ";
    // quotes are rendered with an escape character, so we need to add to the length
    let belen = be.len() + be.matches("\"").count();
    let be_underlines = "-".repeat(belen);
    let be_arrow_spaces = " ".repeat(belen / 2);

    let aelen = ae.len() + ae.matches("\"").count();
    let ae_underlines = "-".repeat(aelen);
    let ae_arrow_spaces = " ".repeat(aelen / 2);

    let op_spaces = " ".repeat(op.len() + 2);

    let line1 = format!("{}{}{}{}", spaces, be_underlines, op_spaces, ae_underlines);
    let line2 = format!("{}{}|{}{}{}|",
                        spaces, be_arrow_spaces, be_arrow_spaces, op_spaces, ae_arrow_spaces);
    let line3 = format!("{}{}|{}{}{}{}", spaces, be_arrow_spaces, be_arrow_spaces, op_spaces, ae_arrow_spaces, astr);
    let line4 = format!("{}{}|", spaces, be_arrow_spaces);
    let val_line = format!("{}{}{}", spaces, be_arrow_spaces, bs);
    format!("* Condition failed: {} {} {}\n{}\n{}\n{}\n{}\n{}\n",
            be, op, ae, line1, line2, line3, line4, val_line)
}

#[macro_export]
macro_rules! expect {

    (|| $b:block $op:tt || $a:block) => {{
        if $b $op $a {
            Result::Ok(())
        } else {
            Result::Err(::vinegar::build_error(
                &format!("{:?}", $b), stringify!($b), stringify!($op),
                &format!("{:?}", $a), stringify!($a)))
        }
    }};

    (|| $b:block $($a:tt)+) => {{
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

#[macro_export]
macro_rules! expect_len {
    ($a:expr, $b: expr) => {{
        use std::cmp::min;
        if $a.len() == $b {
            Result::Ok(())
        } else {
            let a_str = format!("{:?}", $a);
            let max_i = min(a_str.chars().count(), 50);
            let mut short_a_str = a_str[..max_i].to_string();
            if short_a_str.chars().count() < a_str.chars().count() {
                short_a_str = format!("{}...", short_a_str);
            }
            Result::Err(format!("Length of {} is {}, not {:?} -- {}",
                stringify!($a), $a.len(), $b, short_a_str))
      }
  }}
}

