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
