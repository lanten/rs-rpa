use windows::{
  core::Error,
  core::PCWSTR,
  Win32::Foundation::{HWND, LPARAM, WPARAM},
  Win32::UI::Input::KeyboardAndMouse::EnableWindow,
  Win32::UI::WindowsAndMessaging::{FindWindowExW, SendMessageW, BM_CLICK, WM_SETTEXT},
};

pub fn send_text_to(textbox_hwnd: HWND, text: &str) {
  // 将字符串转换为 UTF-16 编码的宽字符数组
  let wide_text: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();

  // 使用 WM_SETTEXT 设置文本框内容
  unsafe {
    SendMessageW(
      textbox_hwnd,
      WM_SETTEXT,
      Some(WPARAM(0)),
      Some(LPARAM(wide_text.as_ptr() as isize)),
    );
  }
}

/// 解除按钮禁用
pub fn enable_button(button_hwnd: HWND) -> bool {
  unsafe {
    // 启用按钮
    EnableWindow(button_hwnd, true).as_bool()
  }
}

/// 触发按钮点击
pub fn click_button(button_hwnd: HWND) -> bool {
  unsafe {
    // 发送 BM_CLICK 消息
    SendMessageW(button_hwnd, BM_CLICK, Some(WPARAM(0)), Some(LPARAM(0))).0 != 0
  }
}
