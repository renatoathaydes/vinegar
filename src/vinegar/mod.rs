pub fn check<I>(expects: I)
    where I: IntoIterator<Item=Result<(), String>> {
    let mut failures = Vec::new();

    for (index, expect) in expects.into_iter().enumerate() {
        if let Err(err) = expect {
            failures.push(format!("iteration[{}] {}", index, err));
        }
    }

    if !failures.is_empty() {
        panic!("\n{}", failures.join("\n"));
    }
}

#[macro_export]
macro_rules! expect {
  ($($a:tt)*) => {{
      if $($a)* {
          Result::Ok(())
      } else {
          Result::Err(format!("Condition failed: {}", stringify!($($a)*)))
      }
  }}
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

