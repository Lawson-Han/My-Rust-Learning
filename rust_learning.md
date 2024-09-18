# 学习Rust
## 0. 基础
> rust 可以编译好，直接给别人运行

使用cargo new hello_world 生成新文件
1. `cargo run` 如果有发生改变会触发compiling 没有就不会 直接进行run并输出
2. `cargo build` 用来编译 不生成执行文件
3. `cargo check` 只进行语法和类型检查，而不生成最终的可执行文件。适合频繁检查代码是否可运行
4. `cargo build --release` 可以生成 生产级别的代码 发布用

## 1. Guessing Game

### 1.1 readline 输出命令行的输入

```rust
use std::io;

fn main() {
    println!("开始猜测数字");

    let mut guess_num = String::new();
  	
  	// io::Result OK, Err
    io::stdin().read_line(&mut guess_num)
    .expect("无法读取行");

    println!("你猜测的数字是: {}", guess_num);
}
```

`&mut guess_num` 是 `read_line` 方法的参数，需要传递一个<u>可变引用</u>，这里 `guess_num` 必须是一个 `String` 类型的变量。<u>可变引用意味着 `read_line` 会修改 `guess_num` 的内容，将读取到的新行存储其中</u>。

**如果你需要修改引用的数据，你必须使用可变引用 `&mut`**

返回一个 `Result` 类型的对象，用于错误处理。`Result` 类型通常有两种可能的值：

1. `Ok` 表示操作成功，并包含成功时的值（这里是读取的字节数）
2. `Err` 表示操作失败，并包含错误信息。

`.expect()` 会在出错时中断当前函数，返回信息。

### 1.2 生成猜测的数字 无限循环

rust里的库叫作 crate，在Cargo.toml里的[dependencies] 添加，用 `cargo build` 下载包

```rust
[dependencies]
rand = "^0.3.14"
// ^ 表示与这个版本兼容的所有版本，允许版本在不改变 主要版本 的前提下升级
```

生成神秘数字

` let secret_num = rand::thread_rng().gen_range(1, 101)`

首先需要import一些库

```rust
use rand::Rng; //trait
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess_num = String::new();

        // io::Result OK, Err
        io::stdin().read_line(&mut guess_num).expect("无法读取行");

      	// 使用match 来判断 防止输入其他字符
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的数字！");
                continue;
            }
        };

        match guess_num.cmp(&secret_num) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}

```

## 2. 变量

let生成的变量是不可变的 不能 `let x = 5 `又进行了 ` x = 6`

const 常量也是不可变的 **不可以使用mut 关键字** 也无法绑定到函数的调用结果，只可以绑定到常量表达式。命名使用全大写字母 `const MAX_POINTS : u32 = 100_000;`

### 2.1 隐藏 Shadowing

```rust
let x = 5;
let x = x + 1;
let x = x * 2;
// 输出 x = 12

let spaces = "very long string";
let spaces = spaces.len();

```

后续代码中的变量就是新的变量。不用重复取名了。

### 2.2 数据类型：标量

`let guess : u32 = "42".parse().expect("not a number")`

需要有`u32` 来确定parse成什么形式

#### 2.2.1 整数类型

- 无符号类型是u开头 代表Unsigned
- 有符号类型 代表Signed
- i32 就是32-bit，
  - arch（代表isize, usize)，取决于64位系统还是32位系统。*一般对某种集合索引

除了byte以外，可以使用后缀。e.g. 57u8 说明57使用的u8，范围是0 - 255, 如果设置成256 则会报错panic，如果是release模式下，则会被进行环绕操作 变成1。

***Rust的默认整数类型就是i32。**



#### 2.2.2 浮点类型

f32 是32位 单精度，f64 则是64位 双精度

现在CPU情况下 速度差不多 所以***默认选择精度更高的f64**

#### 2.2.3 布尔Boolean类型

符号为bool 占用一个字节大小

#### 2.2.4 字符类型

Rust的Char类型被描述为基础的单个字符 使用单引号 占用四个字节

*可以表示比Unicode里多很多的字符 例如多种语言，中文日文，表情符号。



