use crate::{
  utils::{
    hwnd::{hwnd_to_string, string_to_hwnd},
    result::RpaResult,
  },
  window::{
    enum_all_window_handles, enum_visible_window_handles, find_element_hwnd, get_class_by_hwnd, get_process_id_by_hwnd,
    get_title_by_hwnd,
  },
};

#[derive(Debug, serde::Serialize)]
pub struct WindowInstance {
  handle: String,
  title: String,
  class: String,
  process_id: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct ElementInstance {
  handle: String,
  name: String,
  class: String,
}

/// 查找符合条件的窗口
pub fn find_all_window_hwnd(find_class: String, find_name: String, visible_only: bool, exact: bool) -> RpaResult {
  let result = RpaResult::new();

  let window_handles = match visible_only {
    true => enum_visible_window_handles(),
    false => enum_all_window_handles(),
  };

  let mut window_list: Vec<WindowInstance> = Vec::new();

  window_handles.into_iter().for_each(|hwnd| {
    let class = get_class_by_hwnd(hwnd);
    let title = get_title_by_hwnd(hwnd);

    if !find_class.is_empty() {
      match exact {
        true => {
          if class != find_class {
            return;
          }
        }
        false => {
          if !class.contains(&find_class) {
            return;
          }
        }
      }
    }

    if !find_name.is_empty() {
      match exact {
        true => {
          if title != find_name {
            return;
          }
        }
        false => {
          if !title.contains(&find_name) {
            return;
          }
        }
      }
    }

    window_list.push(WindowInstance {
      handle: hwnd_to_string(hwnd),
      title,
      class,
      process_id: get_process_id_by_hwnd(hwnd),
    });
  });

  result.set_data(&window_list).build()
}

/// 查找目标窗口内的元素句柄
pub fn find_element(hwnd_str: String, find_class: Option<String>, find_name: Option<String>) -> RpaResult {
  let result = RpaResult::new();

  let hwnd = string_to_hwnd(&hwnd_str).unwrap();
  let mut element_list: Vec<ElementInstance> = Vec::new();
  let element_hwnd = find_element_hwnd(hwnd, find_class.as_deref(), find_name.as_deref());

  match element_hwnd {
    Ok(hwnd) => {
      element_list.push(ElementInstance {
        handle: hwnd_to_string(hwnd),
        name: get_title_by_hwnd(hwnd),
        class: get_class_by_hwnd(hwnd),
      });
    }
    Err(_) => {}
  };

  result.set_data(&element_list).build()
}
