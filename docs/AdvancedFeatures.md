# <div align="center">高级特性</div>

## 1. Unsafe Rust

> 隐藏着第二个语言，没有强制的内存安全保证：Unsafe Rust
>
> Unsafe Rust存在的原因：
>
> * 静态分析是保守的（错杀代码，曹操行为）
> * 使用Unsafe Rust：我知道自己在做什么，并承担相应风险

* 使用``unsafe``关键字来切换到``unsafe Rust``，开启一个块，里面存放unsafe代码
* Unsafe Rust的超能力
  * 解引用原始指针
  * 调用unsafe函数或方法
  * 访问或修改可变的静态变量
  * 实现``unsafe trait``
* **unsafe并没有关闭借用检查或停用其他安全检查**
* 尽可能隔离unsafe代码，对外提供抽象接口

### 1.1 解引用原始指针

* 原始指针
  * 可变的：``*mut T``
  * 不可变的：``*const T``：在解引用后不能直接对其进行赋值
  * *不是解引用，是类型名的一部分
* 与引用不同，原始指针(约等于C指针)
  * 允许通过(同时具有不可变和可变指针或多个指向同一位置的可变指针)来忽略借用规则
  * 无法保证能指向合理的内存
  * 允许为null
  * 不实现任何自动清理
* 原始指针可以在unsafe外创建，但只能在``unsafe``块内解引用
* **与C语言进行接口**

### 1.2 调用unsafe函数或方法

* unsafe函数或方法：定义前加上了unsafe关键字
* 函数包含unsafe代码并不意味着要将整个函数标记为unsafe
* 将unsafe代码包裹在安全函数中是常见的抽象：**不安全代码的安全抽象**

### 1.3 有关FFI(Foriegn Function Interface)

* FFI:用一种编程语言能调用另一种编程语言写的函数
  * 在当前语言(host)中，调用其他语言(guest)提供的库
  * 当前语言(host)写库，其他语言(guest)调用

​	**关于FFI的原理：**我们知道所有语言在编译后都会以**二进制**方式执行，这为不同语言间调用提供了可能性。但是二进制太底层了，没有大家一致认可的调用约定，也不可能是互通的，于是出现了ABI(Application Binary Interface)。

​	**ABI(应用二进制接口)：是两个二进制程序模块间的接口(an interface between two program modules).** ABI定义了在机器码中如何获取数据结构和函数，在一个更低层次、更贴近硬件的层次。

> 对比API(Application Programming Interface)：可以理解为用户编程接口，在ABI的高层次

<img src="img/Linux_API_and_Linux_ABI.svg.png" height=400>

而 Rust 目前支持如下 ABI [约定](https://doc.rust-lang.org/nomicon/ffi.html)：

- stdcall
- aapcs
- cdecl
- fastcall
- vectorcall
- Rust
- rust-intrinsic
- system
- **C**
- win64
- sysv64

### 1.4 使用extern函数调用外部代码

* extern关键字：简化创建和使用外部函数接口(FFI)的过程
* 外部函数接口(``FFI:Foriegn Function Interface``)：他允许一种编程语言定义函数，并让其他编程语言能调用这些函数
* 应用二进制接口(``ABI:Application Binary Interface``)，定义函数在汇编层的调用方式
* "C" ABI就是最常见的ABI，它是遵循C语言的ABI(**Rust目前常用的编程语言只支持C**)
* 例如Rust调用C标准库中的abs函数

  ```rust
  extern "C" {
      fn abs(input: i32) -> i32; // 方法签名
  }
  
  fn main() {
      unsafe {
          println!("Absolute value of -3 according to C: {}", abs(-3));
      }
  }
  ```

  * 由于其他语言的函数不确定是否满足Rust的安全规则，故只能在unsafe代码中调用
* **从其他语言调用Rust函数：在fn关键字前使用extern接口，通过他们调用Rust函数**

  > Rust mangling：在编译时编译器会改变我们写的函数名，更改为包含更多编译信息的名字，每一种语言mangle的方式都有不同，为了使我们的函数可以被其他语言通过函数名识别出来，我们需要禁用Rust的mangle
  >
  > ```rust
  > #[no_mangle]
  > pub extern "C" fn call_from_c() {
  >     println!("Just called a Rust function from C!");
  > }
  > ```





