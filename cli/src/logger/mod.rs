#[macro_export]
macro_rules! error {
  ($errName:tt, $($message:tt)+) => {async{
      eprintln!($($message)+);
      client_error($errName).await;
  }};
  (metadata: $metadata:tt, $errName:tt, $($message:tt)+) => {async{
    let value = json!($metadata);
    eprintln!($($message)+);
    println!("Metadata: {}", value);
    client_error($errName).await;
  }}
}
