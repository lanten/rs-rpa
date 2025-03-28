use core::panic;
use rs_rpa::window::get_process_id_by_hwnd;
use std::sync::Once;
use windows::{
  core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::System::LibraryLoader::GetModuleHandleW,
  Win32::UI::WindowsAndMessaging::*,
};

static INIT: Once = Once::new();
static mut WINDOW_CLASS_ATOM: u16 = 0;

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  match msg {
    WM_DESTROY => {
      PostQuitMessage(0);
      LRESULT(0)
    }
    _ => DefWindowProcW(hwnd, msg, wparam, lparam),
  }
}

#[derive(Debug)]
pub struct TestWindow {
  pub hwnd: HWND,
  pub class_name: String,
  pub title: String,
  pub process_id: u32,
}

pub fn create_test_window() -> TestWindow {
  let window_class_name = "RsRpaTestWindow";
  let window_class_name_u16 = windows::core::w!("RsRpaTestWindow").as_ptr();
  let window_title = "rs-rpa 测试窗口";
  let window_title_u16 = windows::core::w!("rs-rpa 测试窗口").as_ptr();

  unsafe {
    INIT.call_once(|| {
      // 注册窗口类
      let h_instance = GetModuleHandleW(None).unwrap();
      let wc = WNDCLASSW {
        hInstance: h_instance.into(),
        lpszClassName: PCWSTR(windows::core::w!("RsRpaTestWindow").as_ptr()),
        lpfnWndProc: Some(window_proc),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        ..Default::default()
      };

      WINDOW_CLASS_ATOM = RegisterClassW(&wc);
      if WINDOW_CLASS_ATOM == 0 {
        panic!("Failed to register window class, error: {}", GetLastError().0);
      }
    });

    let h_instance = GetModuleHandleW(None).unwrap();

    // 创建窗口
    let hwnd = CreateWindowExW(
      WINDOW_EX_STYLE::default(),
      PCWSTR(window_class_name_u16),
      PCWSTR(window_title_u16),
      WS_OVERLAPPEDWINDOW,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      800,
      600,
      None,
      None,
      Some(h_instance.into()),
      None,
    )
    .unwrap();

    if hwnd.0 == std::ptr::null_mut() {
      panic!("Failed to create window, error: {}", GetLastError().0);
    }

    let _ = ShowWindow(hwnd, SW_SHOW);
    let _ = UpdateWindow(hwnd);

    return TestWindow {
      hwnd,
      class_name: window_class_name.to_string(),
      title: window_title.to_string(),
      process_id: get_process_id_by_hwnd(hwnd),
    };
  }
}

// 处理消息循环的函数
pub fn message_loop() {
  unsafe {
    let mut msg = MSG::default();
    while GetMessageW(&mut msg, Some(HWND::default()), 0, 0).into() {
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }
  }
}
