# <div align = "center">Error Handling</div>

## 0. Rust错误处理概述

* Rust的可靠性：错误处理，在编译时提示错误并处理
* 错误的分类
  *  可恢复：例如文件未找到，可再次尝试
  * 不可恢复(``bug``)：例如访问的索引超出范围
* Rust没有类似异常的机制
  * 可恢复错误：``Result<T, E>``
  * 不可恢复错误：``panic!``宏，程序立即停止执行

## 1. panic! 不可恢复的错误

* 当``panic!``宏执行

  * 你的程序会打印一个错误信息
  * 展开(``unwind``)、清理调用栈(``stack``)
  * 退出程序

* 默认情况下，当``panic``发生

  * 程序展开调用栈(工作量大)：

    * Rust沿着调用栈往回走
    * 清理每个遇到的函数中的数据

  * 立即终止调用栈

    * 不进行内存清理，直接停止程序
    * 内存需要OS进行清理

  * **想要二进制文件更小**，**把设置从展开改为终止**

    * **在Cargo.toml中适当的profile部分设置**

      ```toml
      [profile.release]
      panic = 'abort'
      ```

* ``panic!``可能出现在我们写的代码中或我们所依赖的代码中，可通过调用``panic!``函数的回溯信息来定位引起问题的代码

* 通过设置环境变量``RUST_BACKTRACE``获得回溯信息

  ```shell
  RUST_BACKTRACE=1 cargo run # 展示完整调用栈信息
  RUST_BACKTRACE=full cargo run # 更加细节
  ```

* 为了获取带有调试信息的回溯，必须启用调试符号(``cargo build/run``不带``--release``)

## 2. Result枚举与可恢复的错误

* **Result枚举**

  ```rust
  enum Result<T,E>{
      Ok(T),
      Err(E),
  }
  ```

  * T：操作成功情况下，Ok变体中返回的数据类型
  * E：操作失败情况下，Err变体中返回的数据类型

* **处理Result的一种方式：``match``表达式**

  * 和Option枚举一样，Result及其变体也是由prelude带入作用域

  * **匹配不同的错误：处理错误时再次match ``error.kind()``**

    ```rust
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file : {:?}", e),
            }
            other_error => panic!("Failed to open the file : {:?}", other_error),
        }
    };
    ```

  * 使用match多层嵌套比较原始，Result<T,E>可以使用闭包来解决，接收闭包作为参数，使用``match``实现

* **处理match表达式的一个快捷方法：``unwrap``**

  * 如果Result结果为``Ok``，返回``Ok``里面的值
  * 如果Result结果为Err，调用panic!宏
  * 一点缺点是无法自定义``panic message``

* **expect：和unwrap类似，但可指定错误信息**

## 3. 传播错误

> 传播错误：将错误返回给调用者
>
> match等处理：在函数内解决错误

* **``？``运算符：传播错误的一种快捷方式，只能用于返回类型为Result<T,E>的函数**

  * 使用``?``和使用``match``的效果相同

    ```rust
    // match
    fn read_username_from_file() -> Result<String, io::Error> { // Result <T,E>
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // ?
    fn read_username_from_file() -> Result<String, io::Error> { // Result <T,E>
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    ```

  * 如果表达式执行的Result为``Ok``，``Ok``中的值就是表达式的结果，然后继续执行程序

  * 如果是Err，**Err就是整个函数的返回值**，就像使用了``return``

* ``?``与``from``函数

  * ``from``函数：**用于错误之间的转换**

    ```rust
    Trait std::convert::From
    ```

  * 被``?``接收的错误会隐式地被from函数处理

  * **当``?``调用``from``函数时**

    * 它所接收的错误类型会被转化为**当前函数返回类型所定义的错误类型**
    * 用于：针对不同错误原因，返回同一种错误类型(只要每个错误类型实现了转换为所返回错误类型的from函数)

* ``?``与``main``函数

  * ``?``运算符只能用于返回值为``Result<T, E>``的函数

  * ``main``函数的返回值类型为``()``空元组，可以修改返回值类型为``Result<(), Box<dyn Error>>``

    ```rust
    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;
        Ok(())
    }
    ```

    * 即正常情况下返回空元组``()``，或者说没有返回值
    * 发生错误时返回一个``trait``对象``Box<dyn Error>``(任何可能的错误对象)

## 4. 什么时候该用panic!

* 总体原则：在定义一个可能失败的函数时，优先考虑返回``Result``，否则使用``panic!``（用于一些肯定不可恢复的场景）

* 编写示例、原型代码、测试：可以使用``panic!``

  * 演示某些概念``unwrap``
  * 原型代码``unwrap/expect``
  * 测试``unwrap/expect``

* 有时你比编译器掌握更多的信息

  * 你可以确定``Result``就是``Ok``：``unwrap``

* 错误处理的指导性建议

  * 当代码最终可能处于损坏状态时，最好使用``panic!``
  * 损坏状态(``Bad State``)：某些假设、保证、约定或不可变性被打破
    * 预期不会发生
    * 在此之后如果处于这种损坏状态就无法运行
    * 在使用的类型中没有一个好的方法来将损坏信息进行编码

* **场景建议**

  * 调用代码，传入无意义的参数值：``panic!``
  * 调用外部不可控代码，返回非法状态，你无法修复：``panic!``
  * 如果失败是可预期的：``Result``
  * 当代码对值进行操作，首先应该验证这些值：``panic!``

* **为验证值的有效性：创建自定义类型：创建一个新的类型，将验证逻辑放在构造实例的函数里**

  * **只有通过验证才会创建出实例，后续无需担心值的有效性**

  * 以guess_number为例

    ```rust
    pub struct Guess {
        value : i32,
    }
    
    impl Guess {
        pub fn new(value : i32) -> Guess { // constructer
            if !(1..=100).contains(&value) {
                panic!("invalid number : not in the field [1,100]");
            }
    
            Guess {value}
        } 
    
        pub fn value(&self) -> i32 { // operation
            self.value
        }
    }
    ```

    

























