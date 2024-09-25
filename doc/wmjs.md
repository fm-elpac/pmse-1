# wmjs: "外卖" JS (风格代码)

- 执行 JS 引擎: `QuickJS` <https://bellard.org/quickjs/>

  对应 (游戏内) 芯片 (单片机) 型号: `wm32v003` (热销) RISC-V (QuickJS) 内核, 2KB
  SRAM, 32KB flash (SLC 10 万次擦写), LQFP48 封装, 1 熵/片.

- 代码运行 (资源) 限制: 内存限制, js 代码长度限制 (32KB).

  CPU 限制: 单个时间片执行最大长度 100ms (防止死循环),
  每秒允许的最多执行次数 1000.

## wmjs API (全局函数)

- `time()` 返回启动以来的秒数 (浮点数).

- (async) `wait_a()` 等待秒数 (异步, 返回 Promise).

- `gpio_r()` 读取 GPIO 引脚信号.

- `gpio_w()` 写入 GPIO 引脚信号.

- `set()` 配置.

- `wifi(pass)` 连接 "wifi" (游戏中模拟) 信号.

- `send()` 发送消息.

- (async) `recv()` 接收消息.

- `on()` 事件回调 (全局).

- (async) `get()` 下载数据.

- (async) `post()` 上传数据.

- `console.log()` 输出运行日志 (调试日志).

---

内部实现:

- `__wmjs_unstable_api` 全局变量. 默认加载 js 兼容层 (js wrapper).

- wmjs 代码开头的版本标记: `#! wmjs v0.1`

TODO
