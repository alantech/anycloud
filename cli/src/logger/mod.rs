#[repr(u8)]
pub enum ErrorType {
  InvalidPwd = 100,
  NoEnvFile = 101,
  GitChanges = 102,
  NoGit = 103,
  DeleteTmpAppTar = 104,
  InvalidCredentialsFile = 108,
  InvalidAnycloudFile = 110,
  AuthFailed = 113,
  NoDnsVms = 114,
  PostStats = 115,
  NoClusterSecret = 116,
  NoDns = 117,
  NoPrivateIp = 118,
  NoDnsPrivateIp = 119,
  ScaleFailed = 120,
  PostFailed = 121,
}

#[macro_export]
macro_rules! error {
  ($errCode:expr, $($message:tt)+) => {async{
      eprintln!($($message)+);
      crate::deploy::client_error($errCode, &format!($($message)+)).await;
  }};
  (metadata: $metadata:tt, $errCode:tt, $($message:tt)+) => {async{
    let value = json!($metadata);
    eprintln!($($message)+);
    client_error($errCode, &format!($($message)+)).await;
  }}
}
