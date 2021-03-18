# m_calc

## 小学生口算练习程序

![video](https://user-images.githubusercontent.com/15625347/111419235-14a71a00-8724-11eb-8bee-02d5008be856.gif)

### 功能和特性

- 随机生成算式
- 确保答案为正整数
- 自选数字范围(20 以内，50 以内，100 以内)
- 错题订正
- 最快成绩记录
- 历史做题记录

### 难度及模式说明

- 练习模式: 每次 10 题
- 测试模式: 每次 50 题

- 难度 1: 正常加减法 `例: 5 + 8 = ()`
- 难度 2: 特殊加减法 `例: 5 + () = 13`
- 难度 3: 加减混合 `例: 5 + 8 - 6 = ()`

### 构建和运行(MacOS)

1. [安装 Rust](https://www.rust-lang.org/zh-CN/tools/install)
2. 克隆代码到本地目录
3. 打开 Terminal，进入代码所在目录，运行 `cargo build --release`
4. 将可执行文件所在路径添加到 PATH:

- `sudo vi /etc/paths`
- `` /Users/`whoami`/<可执行文件所在目录> ``

4. 打开一个新的 Terminal 窗口
5. 检查命令是否已经在 PATH 中:

- `where m_calc`

6. 运行程序:

- `m_calc <用户名>`
