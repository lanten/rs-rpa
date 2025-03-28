# rs-rpa

基于 windows-rs 实现的 win32 gui rpa 工具

Rust | Windows

通过 cli 命令与其它语言交互

```bash
Usage: rs-rpa.exe <COMMAND>

Commands:
  find, -F, --find                查找目标窗口
  find-element, --find-element    查找子元素句柄
  send-text, --send-text          向目标句柄投递文本
  active-window, --active-window  激活目标窗口
  click-button, --click-button    触发按钮点击事件
  help                            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## find

查找窗口句柄

```bash
Usage: rs-rpa.exe {find|--find|-F} [OPTIONS]

Options:
  -c, --class <class>...  目标窗口类名
  -n, --name <name>...    目标窗口名称/标题
  -e, --express           精确匹配窗口类名或标题
  -v, --visible-only      只查找可见窗口
  -h, --help              Print help
```

## find-element

查找子元素句柄

```
Usage: rs-rpa.exe {find-element|--find-element} [OPTIONS] --parent <parent>...

Options:
  -p, --parent <parent>...  父窗口句柄
  -c, --class <class>...    目标元素类名
  -n, --name <name>...      目标元素name
  -h, --help                Print help
```

## send-text

向目标句柄投递文本

```bash
Usage: rs-rpa.exe {send-text|--send-text} [OPTIONS] --handle <handle>...

Options:
      --handle <handle>...           目标元素句柄
  -t, --text <text>...               投递的文本内容
  -k, --key-emulation                是否模拟键盘输入
      --key-duration <key-duration>  键盘输入间隔 [default: 50]
  -h, --help                         Print help
```

## active-window

激活目标窗口

```bash
Usage: rs-rpa.exe {active-window|--active-window} --handle <handle>...

Options:
      --handle <handle>...  目标窗口句柄
  -h, --help                Print help
```

## click-button

触发按钮点击事件

```bash
Usage: rs-rpa.exe {click-button|--click-button} --handle <handle>...

Options:
      --handle <handle>...  目标元素句柄
  -h, --help                Print help
```
