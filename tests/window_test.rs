use rs_rpa::{
  find::find_all_window_hwnd,
  window::{
    enum_all_window_handles, enum_visible_window_handles, find_element_hwnd, focus_window, get_class_by_hwnd,
    get_process_id_by_hwnd, get_title_by_hwnd,
  },
};

mod test_utils;

/// 枚举所有窗口句柄
#[test]
fn test_enum_all_window_handles() {
  let handles = enum_all_window_handles();
  assert!(!handles.is_empty(), "应该能获取到至少一个窗口句柄");
  println!("找到 {} 个窗口句柄", handles.len());
}

/// 测试枚举可见窗口句柄
#[test]
fn test_enum_visible_window_handles() {
  let visible_handles = enum_visible_window_handles();
  assert!(!visible_handles.is_empty(), "应该能获取到至少一个可见窗口句柄");
  println!("找到 {} 个可见窗口句柄", visible_handles.len());
}

/// 测试获取窗口标题、类名和进程ID
#[test]
fn test_window_properties() {
  let test_win = test_utils::create_test_window();

  // std::thread::spawn(move || test_utils::message_loop());
  // test_utils::message_loop();

  println!("创建测试窗口成功 {:?}", test_win);

  let find_res = find_all_window_hwnd(test_win.class_name.clone(), test_win.title.clone(), false, true);

  println!("查找窗口结果: {:?}", find_res);

  // std::thread::sleep(std::time::Duration::from_secs(10));
  // if let Some(hwnd) = visible_handles.first() {
  //   // 测试获取窗口标题
  //   let title = get_title_by_hwnd(*hwnd);
  //   println!("窗口标题: {}", title);

  //   // 测试获取窗口类名
  //   let class_name = get_class_by_hwnd(*hwnd);
  //   println!("窗口类名: {}", class_name);
  //   assert!(!class_name.is_empty(), "窗口类名不应为空");

  //   // 测试获取进程ID
  //   let process_id = get_process_id_by_hwnd(*hwnd);
  //   println!("进程ID: {}", process_id);
  //   assert!(process_id > 0, "进程ID应该大于0");
  // } else {
  //   panic!("未找到任何可见窗口进行测试");
  // }
}

#[test]
fn test_find_element_hwnd() {
  // 此测试可能需要根据系统环境调整
  // 例如，测试查找桌面上的某个特定元素
  let visible_handles = enum_visible_window_handles();

  if let Some(parent_hwnd) = visible_handles.first() {
    // 尝试在第一个可见窗口中查找元素
    // 注意：这个测试可能需要根据实际环境调整参数
    let result = find_element_hwnd(*parent_hwnd, None, None);
    println!("查找元素结果: {:?}", result);
  } else {
    println!("未找到可用窗口进行元素查找测试");
  }
}

#[test]
#[ignore] // 这个测试会改变窗口焦点，可能干扰用户操作，默认忽略
fn test_activate_window() {
  let visible_handles = enum_visible_window_handles();
  if let Some(hwnd) = visible_handles.first() {
    let title = get_title_by_hwnd(*hwnd);
    println!("尝试激活窗口: {}", title);

    let result = focus_window(*hwnd);
    assert!(result, "激活窗口应该成功");

    // 给用户一点时间观察窗口是否被激活
    std::thread::sleep(std::time::Duration::from_secs(1));
  } else {
    panic!("未找到任何可见窗口进行测试");
  }
}
