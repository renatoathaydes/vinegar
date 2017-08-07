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

#[macro_export]
macro_rules! expect {

  (|| $b:block $($a:tt)+) => {{
      if $b $($a)* {
          Result::Ok(())
      } else {
          let bs = format!("{:?}", $b);
          let be = stringify!($b);
          let spaces =        "                    ";
          let underlines = "-".repeat(be.len());
          let arrow_line_spaces = " ".repeat(be.len() / 2);
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
          Result::Err(format!("Length of {} is {} != {:?} -- {}",
                stringify!($a), $a.len(), $b, short_a_str))
      }
  }}
}

