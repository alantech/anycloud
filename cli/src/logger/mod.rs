#[macro_export]
macro_rules! error {
  ($errCode:tt, $($message:tt)+) => {async{
      eprintln!($($message)+);
      client_error($errCode, &format!($($message)+)).await;
  }};
  (metadata: $metadata:tt, $errCode:tt, $($message:tt)+) => {async{
    let value = json!($metadata);
    eprintln!($($message)+);
    client_error($errCode, &format!($($message)+)).await;
  }}
}
