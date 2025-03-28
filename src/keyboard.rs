use std::thread::sleep;

use windows::Win32::UI::Input::KeyboardAndMouse::{
  MapVirtualKeyW, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, VIRTUAL_KEY,
};

/// 发送键盘输入
pub fn send_keys_to(text: &str, duration: Option<u64>) {
  let mut inputs = Vec::new();

  for c in text.chars() {
    // 获取虚拟键码和扫描码
    let vk = c as u16;
    let scan = unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) } as u16;

    match duration {
      Some(duration) => sleep(std::time::Duration::from_millis(duration)),
      None => {}
    };

    // 按下键
    inputs.push(INPUT {
      r#type: INPUT_KEYBOARD,
      Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
        ki: KEYBDINPUT {
          wVk: VIRTUAL_KEY(vk),
          wScan: scan,
          dwFlags: KEYBD_EVENT_FLAGS(0),
          time: 0,
          dwExtraInfo: 0,
        },
      },
    });

    // 释放键
    inputs.push(INPUT {
      r#type: INPUT_KEYBOARD,
      Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
        ki: KEYBDINPUT {
          wVk: VIRTUAL_KEY(vk),
          wScan: scan,
          dwFlags: KEYEVENTF_KEYUP,
          time: 0,
          dwExtraInfo: 0,
        },
      },
    });
  }

  // 发送输入
  unsafe {
    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
  }
}
