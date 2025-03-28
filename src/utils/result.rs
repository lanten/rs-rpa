use std::time::Instant;

#[derive(serde::Serialize, Debug)]
pub struct RpaResult {
  pub code: i32,
  pub message: String,
  pub data: Option<String>,
  pub duration: String,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub enum RpaResultCode {
  Success = 0,
  Failed = 1,
  Unknown = 2,
}

impl Default for RpaResultCode {
  fn default() -> Self {
    RpaResultCode::Success
  }
}

impl RpaResult {
  pub fn new() -> RpaResultBuilder {
    RpaResultBuilder::default()
  }
}

#[derive(Default)]
pub struct RpaResultBuilder {
  pub code: i32,
  pub message: String,
  pub data: Option<String>,
  instant: Option<Instant>,
}

impl RpaResultBuilder {
  fn default() -> Self {
    let start = Instant::now();
    Self {
      code: RpaResultCode::Success as i32,
      message: String::from("success"),
      data: None,
      instant: Some(start),
    }
  }

  pub fn code(mut self, code: RpaResultCode) -> Self {
    self.code = code as i32;
    self
  }

  pub fn message(mut self, message: String) -> Self {
    self.message = message;
    self
  }

  pub fn set_data<T>(mut self, data: &T) -> Self
  where
    T: ?std::marker::Sized + serde::Serialize,
  {
    self.data = Some(serde_json::to_string(data).unwrap());
    self
  }

  // 构建最终的 RpaResult 实例
  pub fn build(self) -> RpaResult {
    let end = match self.instant {
      Some(start) => start.elapsed(),
      None => Instant::now().elapsed(),
    };

    RpaResult {
      code: self.code,
      message: self.message,
      data: self.data,
      duration: format!("{}ms", end.as_millis()),
    }
  }
}
