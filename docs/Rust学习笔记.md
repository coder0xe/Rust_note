# <div align = "center">Rust学习笔记</div>

> * Ubuntu22.04 
> * Rust 1.8.0
> * 可以使用rustup doc查看官方文档(在安装rust时都已经下载到本地)
> * 使用firefox打开时会遇到权限错误，可以使用转移文件夹并与原文件夹进行软链接的方式解决，参考[askubuntu](https://askubuntu.com/questions/1419528/how-do-i-access-rust-documentation)

## 1. Hello World

* ``rust``文件以``.rs``为后缀
* 编译``rust``代码需要使用``rustc filename.rs``进行编译
* 但``rustc``只适合简单的``rust``程序

```rust
fn main() {
    println!("hello world");
}
```

## 2. Cargo

### 2.1 Cargo创建项目

* ``Cargo``是``Rust``的构建系统和包管理工具：构建代码、下载依赖的库、构建这些库

  * 这里我的版本为``1.80.0``

* 使用``Cargo``创建项目

  ```shell
  cargo new project_name
  ```

* 使用``Cargo``创建的项目下有一个``src``文件夹(下有``main.rs``)和``Cargo.toml``

### 2.2 Cargo.toml

* TOML(Tom's Obivious, Minimal Language)，是Cargo的配置格式

  ```toml
  [package]
  name = "rust"
  version = "0.1.0"
  edition = "2021"
  
  [dependencies]
  ```

* package下方的内容使用来配置包(package)的

* dependencies中列出项目的依赖项

* **在Rust中，代码的包称为crate**

### 2.3 Cargo构建项目

> 在项目根目录下执行命令

* ``cargo build``：编译，可执行文件生成在``target/debug``目录下
* ```cargo run```：编译+执行
* 第一次运行会在顶层生成```cargo.lock```文件

* ``cargo check``：检查代码能否通过编译而不产生可执行文件，比``cargo build``速度更快

* 默认情况下``cargo build``用于开发时构建项目，当需要发布程序时，使用``cargo build --release``(加一个参数)
  * 代码运行时间更短但编译时间变长
  * 可执行文件生成在``target/release``目录下

## 3.猜数游戏

### 3.1 增加依赖项

​	我们需要增加一个随机数包``rand``，此时需要在项目的``Cargo.toml``中的``[dependencies]``增加对应的包以及版本(这里Dependi插件会进行版本检查，可以使用``^``表示兼容此版本的其他版本都可以)

```toml
# Cargo.toml
[dependencies]
rand = "^0.8.5"
```

​	cargo会下载对应的包以及该包依赖的其他包，并在编译时进行整合。cargo提供了包环境的管理，可以发现在``Cargo.lock``中有这些包的版本信息等，当其他人运行我们的代码时，**通过``Cargo.lock``一定会得到相同的复现结果。**

```toml
# Cargo.lock 
# 以rand为例
[[package]]
name = "rand"
version = "0.8.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
dependencies = [
 "libc",
 "rand_chacha",
 "rand_core",
]
```

​	如果需要更新package,可以使用``cargo update``进行index表的更新，之后使用index表进行包的安装而忽略``Cargo.lock``，最后将新的版本信息更新到``Cargo.lock``中，但不会更新到``Cargo.toml``中。

### 3.2 code

```rust
use std::io; // prelude 标准io库
use rand::Rng; // trait 接口
use std::cmp::Ordering; // enum

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret number is {}", secret_number);

    loop {
        println!("guess a number!"); 
        // ！表示println是一个宏

        let mut guess = String::new(); // let 声明变量 
        // rust中变量默认为不可变的 immutable 如果需要变量，则可以增加mut关键字
        
        io::stdin().read_line(&mut guess).expect("can not read a number"); // 方法参数按照引用传递
        // io::Result Ok,Err
        
        println!("Your number is {}", guess);
        // {}表示占位符 其中的内容为guess

        // let guess:u32 = guess.trim().parse().expect("Form Err! Please type a new number!");
        // 进行类型转换 将字符串类型String转换为u32
        // 可以使用原变量名 对旧变量进行隐藏

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 使用match来解决异常 
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too SMALL!"),
            Ordering::Greater => println!("Too BIG!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
    
}
```

## 4. Rust基础

### 4.1 变量与可变性

* 声明变量使用``let``关键字

* 默认情况下，变量是不可变，**在声明变量时，在变量前加上``mut``就可以使变量可变**

  ```rust
  let mut x = 5;
  ```

* 声明常量使用``const``关键字，他的类型必须被标注

  * 常量可以在任何作用域中声明，包括全局作用域

  * 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或之恩那个在运行时才能计算出的值

  * **命名规范：Rust中常量使用全大写字母，每个单词之间使用下划线分开**

    ```rust
    const MAX_POINTS: u32 = 100_000;
    ```

> Rust的编译器十分智能，对于使用let声明的变量，其变量类型会由编译器进行推断而不需要明确指出

* **Shadowing(隐藏)**
  * 在Rust中可以使用相同的变量名声明新的变量，新的变量会shadow之前的同名变量
  * 后续代码中变量为代表的是新变量
  * **shadow和把变量声明为mut是不同的**
    * 不使用let关键字，重新给非mut变量赋值产生编译错误
    * 使用let声明出的新变量也是不可变的
    * **使用let声明的同名新变量类型可以与之前不同**

### 4.2 数据类型

* **标量和复合类型**

* Rust是静态编译语言，在编译时必须知道所有变量的类型

  * **基于使用的值，Rust编译器通常能够推断出他的具体类型**

  * 但如果可能的类型比较多，例如把String转为整数的parse方法，就必须添加类型标注，否则可能出错，例如下例中42被推断为``u32/i32``都是合理的，需要手动指定

    ```rust
    let guess:i32= "42".parse().expect("Not a number!");
    ```

#### 4.2.1 标量类型

* 一个标量类型代表一个单个的值

* Rust有四个主要的标量类型

  * 整数类型

  * 浮点类型
  * 布尔类型
  * 字符类型

##### 4.2.1.1 整数类型

* 无符号``u``/有符号``i``
* 位宽：8/16/32/64/128/arch
* 关于arch：``isize/usize``位宽由计算机架构决定
* 整数字面值：Dec/Hex/Octal/Binary/Byte,可以使用下划线分割增强可读性
* 除了byte类型外，**数值字面值都允许使用类型后缀，例如10u32**
* 整数的默认类型为``i32``
* 整数溢出：
  * 在调试模式下编译(``build``)：``Rust``会检查整数溢出，如果发生溢出，程序在运行时就会``panic``
  * 在发布模式下编译(``build --release``)：``Rust``不会检查可能导致``panic``的整数溢出，如果发生溢出，``Rust``会进行环绕操作，例如256->0,257->1

##### 4.2.1.2 浮点类型

* 单精度/双精度：``f32/f64``
* 默认类型为``f64``（在现代CPU上f32与f64速度差不多，而且f64速度更快）
* 支持加减乘除余等``+-*/%``

##### 4.2.1.3 布尔类型

* true/false

##### 4.2.1.4 字符类型

* char类型被用来表述语言中最基础的单个字符

* **字符类型的字面值使用单引号**

* **与其他语言不同，Rust中字符类型占用4字节大小**

* Rust中字符类型是一个``Unicode``标量值，可以表示比ASCII多得多的字符内容：拼音、中日韩文、emoji表情等

  ```rust
  let emoji = '😻';
  ```

#### 4.2.2 复合类型

* 符合类型可以将多个值放在一个类型中
* Rust提供了两种基础的符合类型：元组/数组

##### 4.2.2.1 Tuple

* 将多个类型的多个值放在一个元组中

* Tuple长度是固定的，一旦声明就无法修改

* **创建tuple：在小括号中，将值用逗号分开，tuple中每个位置对应一个类型，其中各元素类型不必相同，可以显示指明每个元素的类型，也可以让编译器自动推断**

  ```rust
  let tup = (500, 6.4, 1);
  let tup1:(i32, f64, u8) = (500, 6.4, 1);
  ```

* **获取tuple元素值**：可以使用模式匹配来**解构**一个tuple来获取元素的值

  ```rust
  let tup = (500, 6.4, 1);
  let (a, b, c) = tup;
  println!("The value of a is: {}", a);
  println!("The value of b is: {}", b);
  println!("The value of c is: {}", c);
  ```

* **访问tuple的元素**：在tuple变量使用点标记法，后接元素的索引号

  ```rust
  println!("The value of tup.0 is: {}", tup.0);
  println!("The value of tup.1 is: {}", tup.1);
  println!("The value of tup.2 is: {}", tup.2);
  ```

##### 4.2.2.2 Array

* 数组中元素类型必须相同

* **声明一个数组**：用中括号声明，逗号分隔

  ```rust
  let arr = [1, 2, 3, 4, 5];
  ```

* **用处**：如果想让数据存放在栈内存上，而不是堆内存上，或者想要保有固定个数的元素，使用数组更好

* 标准库提供了Vector，比数组更加灵活(长度可变)

* **数组的类型**：``[类型; 长度]``

  * 另一种声明数组的方法：**如果数组中每个元素值都相同，可以在中括号中指定值，后接数组长度**

    ```rust
    let a = [3;5];
    // let a = [3,3,3,3,3];
    ```

* **访问数组的元素**：数组是在stack上分配的单个的块内存，可以使用**索引**进行访问

  * **数组越界**：编译通过、运行时报错(``runtime panic``)(**不允许继续访问相应地址的内存，与C/C++不同**)
    * 注：**绝大部分数组越界在编译时就会发现并报错**

### 4.3 函数

* 声明函数使用```fn```关键字

* 针对函数和变量名，Rust采用``snake case``命名规范：所有的字母都是小写的，单词之间使用下划线分开

* 不同于C/C++等较为古老的语言，**Rust中不必关心函数的声明先后以及调用顺序**

  ```rust
  fn main() {
      println!("Hello, world!");
      another_function();
  }
  
  fn another_function() {
      println!("Another function.");
  }
  ```

* **函数的参数**：``parameters(形参), arguments(实参)``

  * **在函数签名中，必须声明参数的类型**

    ```rust
    fn another_function(a: i32, b: i32) {
        println!("The value of a is: {}", a);
        println!("The value of b is: {}", b);
    }
    ```

* **Rust函数体中的语句与表达式(重要)**

  * 函数体由一系列**语句**组成，可选的由一个**表达式**结束

  * Rust是一个基于表达式的语言

  * 语句是执行一些动作的指令

  * 表达式会计算产生一个值

    ```rust
    // 用{}创建表达式
    let y = {
        let x = 3;
        x + 3
    }
    ```

  * 函数的定义也是语句

  * 语句不返回值(**返回值是一个空的tuple:``()``**)，不可以使用let将一个语句赋值给一个变量

  * **表达式与语句简单区分：看末尾有没有分号**

    ```rust
    x + 3 // expression 返回对应的返回值
    x + 3; // statement 返回空元组类型 ()
    ```

* **函数的返回值**：在``->``符号右边声明函数**返回值的类型**，**但是不可以为返回值命名**。

* **在Rust中，返回值就是函数体中最后一个表达式的值，如果要提前返回，需要使用return关键字**

  ```rust
  fn add(a:i32,b:i32) -> i32 {
      a + b // 表达式做返回值(没有分号)
  }
  ```

### 4.4 注释

> 同C语言注释

### 4.5 控制流

#### 4.5.1 if

* **if表达式中必须是``bool``类型，C++中会将非bool转换为bool，这里需要注意**

* 一点区别是if后的条件没有括号

  ```c++
  if x % 4 == 0 {
      println!("x is divisible by 4");
  } else if x % 3 == 0 {
      println!("x is divisible by 3");
  } else if x % 2 == 0 {
      println!("x is divisible by 2");
  } else {
      println!("x is not divisible by 4, 3, or 2");
  }
  ```

* **如果使用多于一个else if,最好使用match来重构代码**

  ```rust
  match x {
      x if x % 4 == 0 => println!("x is divisible by 4"),
      x if x % 3 == 0 => println!("x is divisible by 3"),
      x if x % 2 == 0 => println!("x is divisible by 2"),
      _ => println!("x is not divisible by 4, 3, or 2"),
  }
  ```

* **由于if是一个表达式，可以用来做赋值右值**

  ```rust
  fn main() {
      let condition = true;
  
      let number = if condition { 5 } else { 6 };
  
      println!("The value of number is: {}", number);
  }
  ```

#### 4.5.2 loop

* 反复执行代码块内代码，在loop循环中使用``break``来告诉程序何时停止循环

```rust
let mut cnt = 0;
let result = loop {
	cnt += 1;

    if cnt == 10 {
        break cnt * 2;
    }
};
```

* **可以作为表达式使用**

#### 4.5.3 while

```rust
while number != 0 {
    println!("{}", number);
    number -= 1;
}
```

#### 4.5.4 for

* 使用while/loop遍历集合易错且低效
* 使用for循环更简洁，可以针对集合中每一个元素来执行一些代码

```rust
let arr = [10, 20, 30, 40, 50];
for element in arr.iter() {
    println!("The value is: {}", element);
}
```

* **Range**：标准库提供，指定一个开始数字和一个结束数字，Range可以生成他们之间的数字(不含结束),**``rev``**方法可以反转Range

  ```rust
  for i in (1..4).rev() {
      println!("{}", i);
  }
  ```

### 4.6  所有权

* Rust的核心特性就是所有权
* 所有程序在运行时都必须管理他们使用计算机内存的方式
  * Java语言中有垃圾收集机制，在程序运行时会不断地寻找不再使用的内存
  * C/C++中需要程序员显示分配和释放内存
  * **Rust是通过一个所有权系统来管理的，其中包含一组编译器在编译时检查的规则**，当程序运行时，**所有权特性不会减慢程序的运行速度**

#### 4.6.1 Stack vs Heap

* Stack vs Heap
  * 对于Rust系统级编程来说，了解数据存储在Stack/Heap很有必要
  * Stack：LIFO，**进栈/出栈**
    * **所有存储在Stack上的数据必须拥有已知的固定的大小**
    * **编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上**
  * Heap
    * 内存组织性差一些
    * 当把数据放入heap时，会请求一定数量的空间，操作系统在heap中找到一块足够大的空间，把他标记为在用，并返回一个指针，这个过程称为**分配**
  * **访问heap中的数据要比访问stack中的数据慢，因为需要通过指针才能找到heap中的数据**
  * **当调用函数时，值被传入到函数(包括heap指针)，函数本地的变量被压到stack上，当函数结束后，这些值从stack上弹出**
* **所有权解决的问题 ： 管理heap数据**
  * 跟踪代码的哪些部分正在使用heap的哪些数据
  * 最小化heap上的重复数据量
  * 清理heap上未使用的数据以避免空间不足

#### 4.6.2 所有权规则

* 每个值都有一个变量、这个变量是该值的所有者

* 每个值同时只能有一个所有者

* 当所有者超出作用域，该值将被删除

* **变量作用域(scope)**

* 以String类型为例(**前面提到的简单数据类型都存储在Stack上，当他们离开作用域后，数据会弹出栈，String数据类型存储在Heap上，便于研究所有权**)

  * 字符串字面值：程序中写死的字符串值，不可变(``&str类型``)

  * 可变字符串值：例如需要输入字符串``=>``未知大小在Heap中申请内存(``String类型``)

    ```rust
    fn main() {
        let mut s = String::from("hello"); // s comes into scope
    
        s.push_str(", world!"); // push_str() appends a literal to a String
    
        println!("{}", s); // This will print `hello, world!`
    }
    ```

    * 使用``from``函数从字符串字面值创建出String类型，这类字符串是可以修改的，why?

* **内存和分配**

  * 对于字符串字面值，我们在编译时就知道了他的内容，其文本内容直接被硬编码到最终的可执行文件中(同时字符串字面值不可被声明为``mut``)

    ```rust
    let str = "hello, world";
    let met str1 = "hello"; // Err! 
    ```

    * 速度快、高效、**因为其不可变性**

  * 对于String类型，为了**支持可变性**，需要在heap上分配内存来保存编译时未知的文本内容

    ```rust
    let mut s = String::from("hello");
    ```

    * 操作系统必须在运行时申请内存，``String::from``
    
    * 当用完String后，需要将内存返还给操作系统
      * 在拥有GC的语言(Java)中，GC会跟踪并清理不再使用的内存
      * 没有GC就需要我们去识别内存何时不再使用，并调用代码将他返回(C/C++)
      * **在Rust中，对于某个值，当拥有他的变量走出作用域时，内存就会立即自动的交还给操作系统(不用手动free)**
    
    * **Rust中的drop函数：当变量走出作用域，Rust会自动调用drop函数进行内存回收**
    
    * **变量和数据交互的方式:Move**
    
      * 多个变量可以与同一个数据使用一种独特的方式来交互
    
        ```rust
        let x = 5;
        let y = x;
        ```
    
      * 整数是已知且固定大小的值，这两个5被压到了stack中
    
      * String类型
    
        ```rust
        let s1 = String::from("hello");
        let s2 = s1;
        ```
    
      * String类型的结构
    
        <img src = "img/String结构.svg" height = 300>
    
        * ``ptr/len/capacity``的大小确定，存放在stack
        * 字符串的内容存放在heap
    
      * s1的String的数据被复制了一份给s2，如下图
    
        <img src="img/s2=s1.svg" height = "300">
        
        * 在stack上复制了一份指针、长度、容量
        * **并没复制指针所指向的heap上的数据，最小化heap上的重复数据量**
        
      * 当s1,s2离开作用域时，Rust自动调用drop函数，并将变量使用的heap内存释放
    
      * **当s1,s2离开时，他们都尝试释放相同的一块内存：``double free bug``**
    
    * 为了保证内存安全，Rust没有尝试复制被分配的内存，而是选择**让``s1``失效**，**则当s1离开作用域时，并不需要释放任何内存，由s2进行内存释放**
    
      ```rust
      fn main() {
          let mut s = String::from("hello"); // s comes into scope
      
          s.push_str(", world!"); // push_str() appends a literal to a String
      
          println!("{}", s); // This will print `hello, world!`
      
          let s2 = s; // s2 is a copy of the pointer, length, and capacity of s
          // println!("{}", s); // This will throw an error because s has been moved to s2
      }
      ```
    
    * **浅拷贝/深拷贝**：Rust不会自动创建数据的深拷贝(影响运行时性能，类似于浅拷贝的概念)
    
    * **我们将Rust中这种使s1失效的复制行为称为移动Move**
    
      <img src = "img/Move.svg" height = 300>
    

* **克隆(Clone)**

  * 如果真想对heap上的数据进行深拷贝，可以使用``clone``方法

    ```rust
    fn main() {
        let mut s = String::from("hello"); // s comes into scope
    
        s.push_str(", world!"); // push_str() appends a literal to a String
        
        let s2 = s.clone(); // s2 is a copy of the pointer, length, and capacity of s
        println!("s: {}, s2: {}", s, s2); // This will print `hello, world!` twice
    }
    ```

  * 将heap上的String数据克隆后如下图：太消耗资源了！

    <img src = "img/clone.svg" height = 400>

* **复制(Copy)**

  * 对于stack上的数据，不需要clone，只需要copy

    ```rust
    fn main() {
        let x = 5;
        let y = x;
        println!("x: {}, y: {}", x, y);
    }
    ```

  * Rust提供了Copy trait，可以用于整数这样完全存放于stack上面的类型

  * 如果一个类型实现了Copy这个trait，那么旧的变量在赋值后仍然可用

    * 任何简单标量的组合类型都可以Copy：整数/布尔/浮点/字符/Tuple(其中字段都为Copy)
    * 需要分配内存的都不是Copy的

  * 如果一个类型或者该类型的一部分实现了Drop trait(针对于Heap),那么Rust不允许让他再去实现Copy trait(针对于Stack)

#### 4.6.3 所有权与函数

* 在语义上，把值传递给函数和把值传递给变量是类似的

* **给一个函数进行传参时会发生Move/Copy,这由传参的数据类型决定,和赋值的过程是相同的**

* 一个例子

  ```rust
  fn main() {
      let s = String::from("hello"); // s comes into scope
      takes_ownership(s); // s's value moves into the function and so is no longer valid here
      // println!("{}", s); // This will throw an error because s has been moved to the function
  
      let x = 5; // x comes into scope
      makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
      println!("{}", x); // This will print `5`
  
  }
  
  fn takes_ownership(some_string: String) { // some_string comes into scope
      println!("{}", some_string);
  } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
  
  fn makes_copy(some_integer: i32) { // some_integer comes into scope
      println!("{}", some_integer);
  } // Here, some_integer goes out of scope. Nothing special happens.
  ```

* **返回值与作用域**

  * 函数在返回值的过程中同样也会发生``ownership``的转移：返回值的所有权转向接收者

  * 一个例子

    ```rust
    fn main() {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let s2 = String::from("hello"); // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
        println!("s1: {}, s3: {}", s1, s3);
    }
    
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }
    
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }
    ```

* **一个变量的所有权总是遵循同样的模式**

  * **把一个值赋给其他变量时就会发生Move/Copy**
  * **当一个包含heap数据的变量离开作用域时，他的值就会被drop函数清理，除非数据所有权移动到了另一个变量上**

* 如何让一个变量使用另一个变量的值但不拿走所有权？

  * **可以通过返回原变量的方式拿回所有权**

    ```rust
    fn main() {
        let s1 = String::from("hello");
    
        let (s2, len) = calculate_length(s1);
    
        println!("The length of '{}' is {}.", s2, len);
    }
    
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
    
        (s, length)
    }
    ```

  * 但是该方式有些丑陋，**Rust提供了一个使用值而不拿走所有权的方式：引用(``refferences``)，我们在下一节中详细讲述**

### 4.7 引用和借用

#### 4.7.1 References

* ``&``符号就表示引用，**允许使用某些值而不取得其所有权**

* Rust中的引用和C++中的引用类似，本质上是一个指向原变量的指针

  <img src="img/Reference.svg" height = 200>

* 示例代码：传入一个String类型的引用

  ```rust
  fn main() {
      let s1 = String::from("hello");
      let len = calculate_length(&s1); // reference
      println!("The length of '{}' is {}.", s1, len);
  }
  
  fn calculate_length(s: &String) -> usize { // borrow
      s.len()
  }
  ```

#### 4.7.2 Borrow

* 把引用作为函数参数这个行为叫做借用
* **即传参的时候给引用，函数参数为引用的行为称为借用**

#### 4.7.3 Muttable Reference

* **在借用函数中不可以对普通的引用进行修改，因为普通的引用默认为immutable**

* **可以通过加mut关键字实现可变引用:``&mut``**

  ```rust
  fn main() {
      let mut s1 = String::from("hello");
      let len = calculate_length(&mut s1);
      println!("The length of '{}' is {}.", s1, len);
  }
  
  fn calculate_length(s: &mut String) -> usize {
      s.push_str(", world");
      s.len()
  }
  ```

* **可变引用有一个重要的限制：在特定作用域内，对某一块数据，只能有一个可变的引用**

  ```rust
  fn main() {
      let mut s = String::from("hello");
      let s1 = &mut s;
      let s2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
      println!("{}, {}", s1, s2);
  }
  ```

  * **这样做的好处是可以在编译时就避免数据竞争**

* **数据竞争**

  * 两个或多个指针同时访问同一个数据
  * 至少有一个指针用于写入数据
  * 没有使用任何机制同步数据访问

* 可以通过**创建新的作用域**，来允许**非同时的创建多个可变引用**

  ```rust
  fn main() {
      let mut s = String::from("hello");
      let s1 = &mut s;
      {
          let s2 = &mut s; 
      }
  }
  ```

* **不可以同时拥有一个可变引用和一个不可变引用，多个不变的引用是可以的**

#### 4.7.4 Dangling References

* 悬空指针(``Dangling Pointer``)：一个指针引用了内存中的某个地址，而这块地址可能已经释放并分配给其他人使用了
* 在Rust中，编译器可以保证引用永远都不是悬空引用，如果你引用了某些数据，编译器将保证在引用离开作用域之前数据不会离开作用域

### 4.8 切片



## TIPS: useful plugins for RUST

* rust analyzer：Run/Debug(当然可以通过命令行)
* Code LLDB：代码调试
* Even Better TOML：编辑.toml文件
* Dependi：管理crates版本

* git-commit-plugin：规范提交信息