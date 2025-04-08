# rs-rpa

基于 windows-rs 实现的 win32 gui rpa 命令行工具

```
Usage: rs-rpa.exe <COMMAND>

Commands:
  find, -f, --find              查找目标窗口
  destroy, --destroy-window     销毁目标窗口
  fel, --find-element           查找子元素句柄
  send, --send-text             向目标句柄投递文本
  get-text, --get-text          获取文本值
  focus, --focus-window         激活目标窗口
  click, --click-button         触发按钮点击事件
  enable, --enable-element      解除元素禁用
  disable, --disable-element    禁用目标元素
  get-disabled, --get-disabled  获取目标元素禁用状态
  help                          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## find

查找窗口句柄

```
Usage: rs-rpa.exe {find|--find|-f} [OPTIONS]

Options:
  -c, --class <class>...  目标窗口类名
  -n, --name <name>...    目标窗口名称/标题
  -e, --exact           精确匹配窗口类名或标题
  -v, --visible-only      只查找可见窗口
  -h, --help              Print help
```

## destroy (--destroy-window)

销毁目标窗口

```

Usage: rs-rpa.exe {destroy|--destroy-window} --handle <handle>...

Options:
      --handle <handle>...  目标元素句柄
  -h, --help                Print help
```

## fel (--find-element)

查找子元素句柄

```
Usage: rs-rpa.exe {fel|--find-element} [OPTIONS] --parent <parent>...

Options:
  -p, --parent <parent>...  父窗口句柄
  -c, --class <class>...    目标元素类名
  -n, --name <name>...      目标元素name
  -h, --help                Print help
```

## send (--send-text)

向目标句柄投递文本

```
Usage: rs-rpa.exe {send|--send-text} [OPTIONS] --handle <handle>...

Options:
      --handle <handle>...           目标元素句柄
  -t, --text <text>...               投递的文本内容
  -k, --key-emulation                是否模拟键盘输入
      --key-duration <key-duration>  键盘输入间隔 [default: 50]
  -h, --help                         Print help
```

## get-text (--get-text)

获取文本值

```
Usage: rs-rpa.exe {get-text|--get-text} --handle <handle>...

Options:
      --handle <handle>...  目标元素句柄
  -h, --help                Print help
```

## focus (--focus-window)

聚焦目标窗口

```
Usage: rs-rpa.exe {focus|--focus-window} --handle <handle>...

Options:
      --handle <handle>...  目标窗口句柄
  -h, --help                Print help
```

## click (--click-button)

触发按钮点击事件

```
Usage: rs-rpa.exe {click|--click-button} --handle <handle>...

Options:
      --handle <handle>...  目标元素句柄
  -h, --help                Print help
```

## enable (--enable-element)

解除元素禁用

```
Usage: rs-rpa.exe {enable|--enable-element} --handle <handle>...

Options:
      --handle <handle>...  目标元素句柄
  -h, --help                Print help
```

## disable (--disable-element)

禁用目标元素

```
Usage: rs-rpa.exe {disable|--disable-element} --handle <handle>...

Options:
      --handle <handle>... 目标元素句柄
  -h, --help Print help
```

## get-disabled (--get-disabled)

获取目标元素禁用状态

```
Usage: rs-rpa.exe {get-disabled|--get-disabled} --handle <handle>...

Options:
      --handle <handle>... 目标元素句柄
  -h, --help Print help
```
