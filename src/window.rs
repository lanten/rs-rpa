use windows::{
  core::{Error, PCWSTR},
  Win32::Foundation::{HWND, LPARAM},
  Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowExW, GetClassNameW, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
    SetForegroundWindow,
  },
};
use windows_core::BOOL;

extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
  unsafe {
    // 将句柄添加到 Vec 中
    let handles = &mut *(lparam.0 as *mut Vec<HWND>);
    handles.push(hwnd);
  }
  BOOL::from(true) // 继续枚举
}

/// 获取所有窗口句柄
pub fn enum_all_window_handles() -> Vec<HWND> {
  let mut window_handles: Vec<HWND> = Vec::new();

  unsafe {
    EnumWindows(Some(enum_window_callback), LPARAM(&mut window_handles as *mut _ as isize)).unwrap();
  }

  return window_handles;
}

/// 获取所有可见窗口句柄
pub fn enum_visible_window_handles() -> Vec<HWND> {
  let window_handles = enum_all_window_handles();

  window_handles
    .into_iter()
    .filter(|hwnd| unsafe { IsWindowVisible(*hwnd).as_bool() })
    .collect()
}

/// 获取窗口标题
pub fn get_title_by_hwnd(hwnd: HWND) -> String {
  unsafe {
    let length = GetWindowTextLengthW(hwnd);
    if length > 0 {
      let mut buffer = vec![0u16; (length + 1) as usize];
      GetWindowTextW(hwnd, &mut buffer);
      let title = String::from_utf16_lossy(&buffer);
      // 去除末尾的空字符
      let title = title.trim_end_matches('\0').to_string();
      title
    } else {
      String::new()
    }
  }
}

/// 激活窗口
pub fn focus_window(hwnd: HWND) -> bool {
  unsafe { SetForegroundWindow(hwnd).as_bool() }
}

/// 通过句柄取类名
pub fn get_class_by_hwnd(hwnd: HWND) -> String {
  // 创建一个缓冲区来存储类名
  let mut buffer = [0u16; 256];

  unsafe {
    // 调用 GetClassNameW 获取类名
    let result = GetClassNameW(hwnd, &mut buffer);

    if result > 0 {
      // 将 UTF-16 编码的类名转换为 Rust 字符串
      let class_name = String::from_utf16_lossy(&buffer[..result as usize]);
      class_name
    } else {
      String::new()
    }
  }
}

/// 通过句柄取进程ID
pub fn get_process_id_by_hwnd(hwnd: HWND) -> u32 {
  unsafe {
    let mut process_id = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));
    process_id
  }
}

/// 查找目标窗口内的元素句柄
pub fn find_element_hwnd(parent_hwnd: HWND, class_name: Option<&str>, name: Option<&str>) -> Result<HWND, Error> {
  // 将 class_name 转换为 UTF-16 编码（如果存在）
  let class_name_u16: Option<Vec<u16>> = class_name.map(|s| s.encode_utf16().chain(std::iter::once(0)).collect());

  // 将 name 转换为 UTF-16 编码（如果存在）
  let name_u16: Option<Vec<u16>> = name.map(|s| s.encode_utf16().chain(std::iter::once(0)).collect());

  unsafe {
    // 调用 FindWindowExW
    let element_hwnd = FindWindowExW(
      Some(parent_hwnd),
      None,
      class_name_u16
        .as_ref()
        .map(|c| PCWSTR(c.as_ptr()))
        .unwrap_or(PCWSTR(std::ptr::null())),
      name_u16
        .as_ref()
        .map(|n| PCWSTR(n.as_ptr()))
        .unwrap_or(PCWSTR(std::ptr::null())),
    );

    return element_hwnd;
  }
}
