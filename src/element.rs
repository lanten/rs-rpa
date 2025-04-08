use windows::{
  Win32::Foundation::{HWND, LPARAM, WPARAM},
  Win32::UI::Input::KeyboardAndMouse::{EnableWindow, IsWindowEnabled},
  Win32::UI::WindowsAndMessaging::{SendMessageW, BM_CLICK, WM_GETTEXT, WM_GETTEXTLENGTH, WM_SETTEXT},
};

/// 发送文本
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

/// 获取文本框的值
pub fn get_text_value(hwnd: HWND) -> String {
  // 先获取文本长度
  let text_length = unsafe { SendMessageW(hwnd, WM_GETTEXTLENGTH, Some(WPARAM(0)), Some(LPARAM(0))).0 as usize };

  // 如果长度为0，直接返回空字符串
  if text_length == 0 {
    return String::new();
  }

  // 创建足够长的缓冲区 (加1是为了存放字符串结束符)
  let mut buffer: Vec<u16> = vec![0; text_length + 1];

  // 获取文本
  unsafe {
    SendMessageW(
      hwnd,
      WM_GETTEXT,
      Some(WPARAM(text_length + 1)),
      Some(LPARAM(buffer.as_mut_ptr() as isize)),
    );
  }

  // 将UTF-16字符串转换为Rust String
  String::from_utf16_lossy(&buffer[0..text_length])
}

/// 解除按钮禁用
pub fn enable_element(hwnd: HWND) -> bool {
  unsafe { EnableWindow(hwnd, true).as_bool() }
}

/// 禁用目标元素
pub fn disable_element(hwnd: HWND) -> bool {
  unsafe { EnableWindow(hwnd, false).as_bool() }
}

/// 检查目标是否禁用
pub fn get_disabled_state(hwnd: HWND) -> bool {
  unsafe { !IsWindowEnabled(hwnd).as_bool() }
}

/// 触发按钮点击
pub fn click_button(button_hwnd: HWND) -> bool {
  unsafe {
    // 发送 BM_CLICK 消息
    SendMessageW(button_hwnd, BM_CLICK, Some(WPARAM(0)), Some(LPARAM(0))).0 != 0
  }
}
