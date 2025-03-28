use std::str::FromStr;

use windows::{
  core::{Error, HRESULT},
  Win32::Foundation::HWND,
};

/// 将 HWND 转换为字符串, 方便传输
pub fn hwnd_to_string(hwnd: HWND) -> String {
  (hwnd.0 as usize).to_string()
  // format!("{:p}", hwnd)
}

/// 将字符串转换为 HWND
pub fn string_to_hwnd(hwnd_str: &str) -> Result<HWND, Error> {
  // HWND(hwnd_str.parse().unwrap())
  let value = isize::from_str(hwnd_str);
  match value {
    Ok(value) => Ok(HWND(value as *mut std::ffi::c_void)),
    Err(err) => Err(Error::new(HRESULT(-1), err.to_string())),
  }
}
