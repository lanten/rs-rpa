use crate::{
  element::{click_button, enable_element, send_text_to},
  find::{find_all_window_hwnd, find_element},
  keyboard::send_keys_to,
  utils::{hwnd::string_to_hwnd, result},
  window::focus_window,
};
use clap::{crate_description, crate_version, Arg, ArgAction, Command};

pub fn commands() -> Command {
  Command::new("rs-rpa")
    .about(crate_description!())
    .version(crate_version!())
    .subcommand_required(true)
    .arg_required_else_help(true)
    // Find
    //
    // 查找目标窗口
    .subcommand(
      Command::new("find")
        .short_flag('f')
        .long_flag("find")
        .about("查找目标窗口")
        .arg(
          Arg::new("class")
            .long("class")
            .short('c')
            .help("目标窗口类名")
            // .conflicts_with("name")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("name")
            .long("name")
            .short('n')
            // .conflicts_with("class")
            .help("目标窗口名称/标题")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("express")
            .long("express")
            .short('e')
            .help("精确匹配窗口类名或标题")
            .action(ArgAction::SetTrue)
            .default_value("false"),
        )
        .arg(
          Arg::new("visible-only")
            .long("visible-only")
            .short('v')
            .help("只查找可见窗口")
            .action(ArgAction::SetTrue)
            .default_value("true"),
        ),
    )
    // find-element
    //
    // 查找子元素句柄
    .subcommand(
      Command::new("fel")
        .long_flag("find-element")
        .about("查找子元素句柄")
        .arg(
          Arg::new("parent")
            .long("parent")
            .short('p')
            .help("父窗口句柄")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("class")
            .long("class")
            .short('c')
            .help("目标元素类名")
            .action(ArgAction::Set)
            .num_args(1..),
        )
        .arg(
          Arg::new("name")
            .long("name")
            .short('n')
            .help("目标元素name")
            .action(ArgAction::Set)
            .num_args(1..),
        ),
    )
    // send-text
    //
    // 向目标句柄投递文本
    .subcommand(
      Command::new("send")
        .long_flag("send-text")
        .about("向目标句柄投递文本")
        .arg(
          Arg::new("handle")
            .long("handle")
            .action(ArgAction::Set)
            .required(true)
            .num_args(1..)
            .help("目标元素句柄"),
        )
        .arg(
          Arg::new("text")
            .long("text")
            .short('t')
            .action(ArgAction::Set)
            .num_args(1..)
            .help("投递的文本内容"),
        )
        .arg(
          Arg::new("key-emulation")
            .long("key-emulation")
            .short('k')
            .help("是否模拟键盘输入")
            .action(ArgAction::SetTrue)
            .default_value("false"),
        )
        .arg(
          Arg::new("key-duration")
            .long("key-duration")
            .help("键盘输入间隔")
            .action(ArgAction::Set)
            .default_value("50"),
        ),
    )
    // focus-window
    //
    // 聚焦窗口
    .subcommand(
      Command::new("focus").long_flag("focus-window").about("激活目标窗口").arg(
        Arg::new("handle")
          .long("handle")
          .action(ArgAction::Set)
          .required(true)
          .num_args(1..)
          .help("目标窗口句柄"),
      ),
    )
    // click-button
    //
    // 触发按钮点击事件
    .subcommand(
      Command::new("click").long_flag("click-button").about("触发按钮点击事件").arg(
        Arg::new("handle")
          .long("handle")
          .action(ArgAction::Set)
          .required(true)
          .num_args(1..)
          .help("目标元素句柄"),
      ),
    )
    // enable-button
    //
    // 解除元素禁用
    .subcommand(
      Command::new("enable").long_flag("enable-element").about("解除元素禁用").arg(
        Arg::new("handle")
          .long("handle")
          .action(ArgAction::Set)
          .required(true)
          .num_args(1..)
          .help("目标元素句柄"),
      ),
    )
}

pub fn match_commands() {
  let matches = commands().get_matches();

  match matches.subcommand() {
    Some(("find", sub_matches)) => {
      let class = sub_matches.get_one::<String>("class");
      let name = sub_matches.get_one::<String>("name");
      let express = sub_matches.get_flag("express");
      let visible_only = sub_matches.get_flag("visible-only");
      let result = find_all_window_hwnd(parse_args_string(class), parse_args_string(name), visible_only, express);
      println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    Some(("fel", sub_matches)) => {
      let parent = sub_matches.get_one::<String>("parent");
      let class = sub_matches.get_one::<String>("class");
      let name = sub_matches.get_one::<String>("name");
      let result = find_element(parse_args_string(parent), class.cloned(), name.cloned());
      println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    Some(("send", sub_matches)) => {
      let result = result::RpaResult::new();

      let handle = sub_matches.get_one::<String>("handle");
      let text = sub_matches.get_one::<String>("text");
      let key_emulation = sub_matches.get_flag("key-emulation");

      let hwnd = string_to_hwnd(handle.unwrap()).unwrap();

      match key_emulation {
        true => {
          focus_window(hwnd);
          let duration = sub_matches
            .get_one::<String>("key-duration")
            .and_then(|s| Some(s.parse::<u64>().map_err(|_| "`key-duration` 参数必须是一个有效的整数")))
            .unwrap()
            .unwrap();
          send_keys_to(text.unwrap(), Some(duration));
        }
        false => {
          send_text_to(hwnd, text.unwrap());
        }
      };

      let result = result.build();

      println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    Some(("focus", sub_matches)) => {
      let result = result::RpaResult::new();

      let handle = sub_matches.get_one::<String>("handle");
      let hwnd = string_to_hwnd(handle.unwrap()).unwrap();

      focus_window(hwnd);

      let result = result.build();
      println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    Some(("click", sub_matches)) => {
      let result = result::RpaResult::new();

      let handle = sub_matches.get_one::<String>("handle");
      let hwnd = string_to_hwnd(handle.unwrap()).unwrap();

      click_button(hwnd);

      let result = result.build();
      println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    Some(("enable", sub_matches)) => {
      let result = result::RpaResult::new();

      let handle = sub_matches.get_one::<String>("handle");
      let hwnd = string_to_hwnd(handle.unwrap()).unwrap();

      enable_element(hwnd);

      let result = result.build();
      println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }

    _ => unreachable!(),
  }
}

/// 处理命令行参数, 将 Option<&String> 转换为 String
fn parse_args_string(s: Option<&String>) -> String {
  match s {
    Some(s) => s.clone(),
    None => String::new(),
  }
}
