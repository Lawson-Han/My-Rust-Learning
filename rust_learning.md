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



### 2.3 复合类型

#### 2.3.1 Tuple 元组

类型不必相同

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup; // 解构 *如果没使用xyz 前面加_
    println!("{}, {}, {}", tup.0, tup.1, tup.2); // 取索引访问
}
```
#### 2.3.2 数组

如果想让数据存Stack上而不是Heap上，或者想保证固定长度，使用数组比较好。

`let a: [i32, 5] = [1,2,3,4,5];` 主要用类型 + 长度

`let a = [3;5];` 就相当于`let a = [3,3,3,3,3];` 另一种方法 使用分号声明



## 3. 函数 与 控制流

变量要声明类型 例如 `fn p(num: i32)`

### 3.1 函数 语句和表达式

语句是<u>执行动作的指令</u>，表达式会<u>计算产生值。</u>

Rust是基于表达式的语言。函数由一系列语句组成，可选的由一个表达式结束。

*函数的定义也是一个语句，语句不返回值，所以不可以let 一个语句赋给变量。



在Rust里，返回值就是函数体里面最后一个表达式的值。

```rust
fn expression() {
    let x = 5;
    let y = {
        let x = 1;
        x + 3 // y类型是个tuple ()
    };
    println!("The value of y is : {}", y); // y = 4
}
```

可以使用 -> 指明返回类型 但是不可以命名。

```rust
fn five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32 {
    x + 5 // 如果加了分号 他就是个语句 不是个表达式了
}
```



### 3.2 if 控制流

if表达式中，与条件相关联的代码块就叫 **arm**

用 *else if* 继续判断 但是如果使用多于了一个 最好还是用match来重构

可以使用`let number = if condition {5} else {6};` 来赋值



### 3.3 循环

#### 3.3.1 loop

无限循环 直到喊停

```rust
let result = loop {
  counter += 1;
  if counter == 10 {
    break counter * 2; // 最后结果再乘2
  }
}
```

#### 3.3.2 while

每次执行前 做一次判断。

```rust
while index < 5 {
  index = index + 1;
}
```

#### 3.3.3 for

loop和while容易错且比较低效，for循环简洁紧凑。

```rust
let a = [10, 20, 30, 40, 50];

// element代表的是 &i32 他并没有把里面的值复制出来，而是相当于一个指针
for element in a.iter() {
  println!(element);
}
// range方法 使用rev()可以反转
for number in (1..4).rev(){}
```



 ## 4. Ownership 所有权

这让rust 无需使用Garbage Collector 垃圾回收 就可以控制内存的管理。

### 4.1 Stack vs. Heap

Stack按值的接收顺序来存储，相反顺序移除 **后进先出 LIFO**

- 存储已知的固定大小的东西 放入Stack

- 编译时大小会发生变化的放在Heap上

在Stack的**数据更快**，不需要寻找存储新数据的空间：在饭店安排好一个固定大小的桌子，后来的朋友询问你的位置以后，直接可以坐下。

Heap则要通过指针才能找到数据。

*所有权*可以跟踪哪些部分在使用heap的数据，最小化重复数据量，清理未使用的数据。

### 4.2 所有权的规则

- 每个值都有一个变量，这个变量就是该值的所有者

- 每个值同时只能有一个所有者
- 当所有者超过Scope的时候，该值将被释放，即调用drop函数。

当我们进行赋值的时候，`s2 = s1` 会将s1的指针，长度，容量都进行赋值，并没有复制所指向的heap的数据进行复制。当变量离开scope，heap内存释放。相当于是*浅拷贝 Shallow Copy*，但是为了保证安全，并没有复制内存，并且<u>赋值以后 S1会失效</u>

```rust
let s1 = String::from("hello"); // 使用String::from创造一个可变的String类型
let s2 = s1; // value bollowed here 已经发生了移动move

println!(s1); // error: 借用了已经move的值
```

如果一定要深复制heap上的值，可以使用clone，但是很消耗资源。

如果是Stack上的变量，则是叫复制，不会发生这些。例如`let x = 5; let y = x;` 因为是整数，已经确认了长度，会被存储在Stack上。

### 4.3 所有权与函数

```rust
pub fn main() {
    let s = String::from("hello");
    take_ownership(s);
    let x = 5;

    makes_copy(x);
    println!("x: {}", x);
    // println!("s: {}", s); 报错：所有权move
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}
```

一个函数的返回值 一样会发生转移。

```rust
fn take_ownership(s: String) {
    // s 进入作用域，获得所有权
    println!("{}", s);
} // 当函数结束时，s 离开作用域并释放内存

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);  // s1 的所有权转移给了函数 take_ownership

    // println!("{}", s1); // 错误，s1 的所有权已经转移，无法再使用
}
```

允许通过返回值重新获得所有权，防止丢失所有权。

```rust
fn give_ownership() -> String {
    // give_ownership 将返回一个 String 的所有权
    let s = String::from("hello");
    s // 返回 s，将所有权转移给调用函数的地方
}

fn take_and_give_back(a_string: String) -> String {
    // 接受一个 String 并返回它
    a_string  // 返回所有权给调用者
}

fn main() {
    let s1 = give_ownership(); // give_ownership 将其返回值的所有权转移给 s1

    let s2 = String::from("world");
    let s3 = take_and_give_back(s2); // s2 的所有权转移给了函数，函数返回值的所有权再转移给 s3

    // println!("{}", s2); // 错误，s2 的所有权已经被转移给了 s3
    println!("{}", s3); // s3 拥有所有权，可以正常使用
}
```

### 4.4 引用和借用

参数的类型是`&String` 而不是 String，没有获得所有权，也不可以修改借用的东西。

```rust
fn calculate_length(s: &String) -> usize {
    s.push_str(", world"); // 会报错
    s.len()
}
// 可变引用 可以使用 相当于指针了
fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}
```

某一块数据 只能有一个可变引用。也不可以同时有一个可变引用和不可变引用。

多个不变引用则是可以的

```rust
let r1 = &s;
let r2 = &s;
let s1 = &mut s; //报错
```

**悬空引用** Dangling Reference：指的是一个指针指向一个地址，但这块内存已经被释放或占用了。

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s;
}
```

`dangle` 函数返回的是对 `s` 的引用（`&s`）。但是当函数执行结束时，`s` 变量已经被释放，指向 `s` 的引用就变成了无效的引用，因为它指向了已经被回收的内存。

### 4.5 切片

字符串切片，指向一部分内容的引用。

`[开始索引，结束索引 + 1]`  

```rust
let hello = &s[0..5]; // [..5]
let world = &s[6..11]; // [6..]
let whole = &s[..];
```

使用字符串切片代替String类型，可以接受两种形态。



## 5 Struct 结构体

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user1 = User {
        email: String::from("asdf"),
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 556,
      	// 不可以只创建三个值
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        email, // email: email,
        username, // username: username, 简写
        active: true,
        sign_in_count: 0,
    }
}
```

使用点标记访问值: `user1.email`  一旦struct的实例可变，他的所有field都是可变的。

可以作为函数的返回值。

### 5.1 Update 更新语法

使用其他实例，创建新实例，可以使用struct更新语法。

```rust
let user2 = User {
    email: String::from("something.."),
    username: String::from("username..."),
    ..user1 // active: user1.active, 
    				// sign_in_count: user1.sign_in_count, 简写 类似于深拷贝user1的值
}
```

**Tuple Struct**

`struct Color(i32, i32, i32); let black = Color(0, 0, 0);`

行为比较像tuple，想给这个tuple起名，也想区别于其他的。

**例子： 计算长方形面积**

```rust
fn main() {
    let w = 30;
    let l = 50;
    println!("{}", area(width: u32, length: u32))
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}
// 或者使用struct
struct Rectangle {
    width: u32,
    length: u32,
}

#[derive(Debug)] // 派生，配合{:?}输出。使用的是调试格式
fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    println!("{}", area(&rect));
    println!("{:?}", rect); // 配合derive的派生，{:#?} 是结构更加清晰的派生
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
```

### 5.2 struct的方法

```rust
impl Rectangle {
    fn area(&self) -> u32 { // &self可以被推断，也可以使用&mut 可变借用，也可以直接获得所有权
        self.width * self.length
    }
  	fn can_hold(&self, other: &Rectange) -> bool {
      self.width > other.width && self.length > other.length
  	}
  	fn square(size: u32) -> Rectangle {
      Rectangle {
        width: size, // **关联函数**
        length: size // 可以在impl块里定义，不把self作为第一个参数的函数，它们叫<u>关联函数</u>。 
      }  // 例如 String::from()
  	}
} 
```

每个struct可以有多个impl 方法，但没有太必要这么做。



## 6 枚举

```rust
// 普通实现方式
enum IpAddKind {
    v4,
    v6,
}

// 枚举
enum IpAddKind {
		v4(u8, u8, u8, u8),
		v6(String)
}

fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    route(four);
    route(six);
    route(IpAddKind::v4(127, 0, 0, 1));
}
fn route(ip_kind: IpAddKind) {}
```

**结构体**：适合表示一个具有多个属性的对象，所有属性同时存在。它适用于“<u>这个东西由这些部分组成</u>”的场景。

**枚举**：适合表示可能的多种状态或类别，每次只能是其中一种。它适用于“<u>这个东西可以是几种不同的形态之一</u>”的场景。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing RGB({}, {}, {})", r, g, b),
    }
}
```



### 6.1 Option\<T> 枚举

这是个类似于null的存在，表示因为某种原因变为无效或者缺失的值。它用于处理**可能存在也可能不存在**的值。

```rust
// 标准库
enum Option<T> {
		Some(T)，
  	None,
}

fn main() {
    let some_num = Some(5); // 显式 let x: Option<i32> = Some(10);
    let some_str = Some("A String");

    let absent_number: Option<i32> = None;
}
```

- `Some(T)` 表示值存在，且该值是类型 `T` 的数据。这里的 `T` 是泛型，意味着可以用于任何类型。
- `None` 表示没有值（类似于其他语言中的 `null` 或 `None`，但 Rust 通过枚举更安全地处理）。

Option\<T> 和 T 不是同一个类型，先把Option\<T>转化为T再说。

### 6.2 match

```rust
// 定义枚举 Coin，其中 Quarter 包含一个附加值（例如：一个字符串来表示州）
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // Quarter 现在绑定了一个 String 类型的值
}

// 定义函数来返回硬币的面值
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);  // 使用绑定的值
            25
        }
    }
}

fn main() {
    // 测试不同硬币的值
    let coin1 = Coin::Quarter(String::from("California"));
    let coin2 = Coin::Penny;

    println!("Value of coin1: {} cents", value_in_cents(coin1));
    println!("Value of coin2: {} cents", value_in_cents(coin2));
}

// 可以给枚举类型绑定值。
输出：
State quarter from California!
Value of coin1: 25 cents
Value of coin2: 1 cents
```

**match匹配必须穷举所有的可能**, 可以使用通配符_ 来代替 `_ => (),` 

**if let 简单的控制流**： `if let` 是一种简化的模式匹配语法，用于只关心某些特定模式的情况，忽略其他不匹配的情况。

```rust
fn main() {
    let v = Some(0u8); // 这里的 v 是 Option<u8> 类型，表示变量 v 中确实包含了一个 u8 值为 0。
  	// match 所有情况
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
		// 简化匹配
    if let Some(3) = v {
        println!("three");
    } else {
      	println!("others")
  }
}
```



