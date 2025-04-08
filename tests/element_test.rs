use rs_rpa::{
  element::{click_button, disable_element, enable_element, get_disabled_state, get_text_value, send_text_to},
  window::destroy_window,
};

mod test_utils;
use test_utils::create_test_window;

#[test]
fn test_send_text_to() {
  // 创建测试窗口
  let window = create_test_window();

  println!("创建测试窗口成功 {:?}", window);

  // 测试向文本框发送文本
  let test_text = "Hello, rs-rpa!";
  send_text_to(window.edit_hwnd, test_text);

  // 验证文本框内容
  let text_value = get_text_value(window.edit_hwnd);
  println!("文本框内容: {}", text_value);
  assert_eq!(text_value, test_text);

  // 关闭窗口
  destroy_window(window.hwnd);
}

#[test]
fn test_click_button() {
  // 创建测试窗口
  let window = create_test_window();

  // 测试点击按钮
  let result = click_button(window.button_hwnd);
  // 由于测试窗口不会对按钮点击做特殊处理，只能验证函数返回值

  println!("点击按钮结果: {}", result);

  // 关闭窗口
  destroy_window(window.hwnd);
}

#[test]
fn test_disable_and_enable_element() {
  // 创建测试窗口
  let window = create_test_window();

  // 测试禁用按钮
  disable_element(window.button_hwnd);
  let is_disabled = get_disabled_state(window.button_hwnd);
  assert_eq!(is_disabled, true);

  // 测试启用按钮
  enable_element(window.button_hwnd);
  let is_disabled = get_disabled_state(window.button_hwnd);
  assert_eq!(is_disabled, false);

  // 关闭窗口
  destroy_window(window.hwnd);
}

#[test]
fn test_get_disabled_state() {
  // 创建测试窗口
  let window = create_test_window();

  // 初始状态按钮应该是启用的
  let initial_state = get_disabled_state(window.button_hwnd);
  assert!(!initial_state);

  // 禁用按钮
  disable_element(window.button_hwnd);

  // 检查禁用状态
  let disabled_state = get_disabled_state(window.button_hwnd);
  assert!(disabled_state);

  // 关闭窗口
  destroy_window(window.hwnd);
}
