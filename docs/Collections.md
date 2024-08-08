# <div align = "center">Collections</div>

> Rust标准库中包含的数据结构称为collections

## 1. Vector

* ``Vec<T>``，叫做vector

* 由标准库提供

* 可存储多个值，数据类型相同

* 在内存中连续存放

* **创建Vector**：``new``关键字创建空vector

  ```rust
  let v:Vec<i32> = Vec::new();
  ```

* 使用初始值创建``Vec<T>``，使用``vec!``宏，无需指明初始值类型

  ```rust
  let v1 = vec![1,2,3];
  ```

* **添加元素：push方法**

* **读取元素：**

  * 索引：访问越界``panic``
  * ``get``方法：访问越界返回``None``

* 所有权和借用规则

  * 不能在同一作用域内同时拥有可变和不可变引用
  * ``push``等修改操作是可变引用

* 遍历Vector中的值

  * ``for``循环

* Vector中只能存储一种数据类型，可以使用``Enum``实现存储不同数据

  * ``Enum``的变体定义在一个``enum``类型下，附加不同类型数据

## 2. String

* Rust的核心语言层面，只有一个字符串类型：字符串切片``str``(``&str``)

* 字符串切片：对存储在其他地方、``UTF-8``编码的字符串的引用

  * 字符串字面值：存储在二进制文件中，也是字符串切片

* String类型

  * 来自于标准库而不是核心语言
  * 可增长，内容可修改

* 字符串：``String / &str``

* Rust标准库还包含了很多其他的字符串类型，例如：``OsString, OsStr, CString, CStr``

* String是一个字节集合，很多``Vec<T>``的操作都可用于String

  * ``String::new()``

  * 使用初始值来创建String：``to_string/String::from()``方法

    ```rust
    let s1 = "initial contents".to_string();
    let s1 = String::from("initial contents");
    ```

  * 更新String

    * 追加字符串：``push_str()``方法

    * 追加单个字符：``push()``方法

    * 拼接字符串：``+``，使用了类似这个签名的方法，涉及到所有权

      ```rust
      fn add(self, s:&str) -> String{...}
      ```

    * ```format!```宏连接多个字符串：不涉及所有权变化的问题

      ```rust
      let s = format!("{}-{}",s,s1);
      ```

  * Rust的字符串不支持按照索引的方式进行访问

* String是对``Vec<u8>``的包装

  * 占用字节数：``len()``方法，**不同语言中一个字符占用的字节数不同**
    * 字符占用字节数：Unicode标量值

* 字节、标量值、字型簇

* 切割String：索引范围切片

  ```rust
  let s = &str[0..4];
  ```

  * **必须按照字符边界进行切割，例如某种语言每个字符占用两个字节，如果跨越了字符边界，会产生panic**
  * **英文字符一个占一个字节**

* 遍历String

  * 对于标量值：``chars()``方法
  * 对于字节：``bytes()``方法
  * 对于字型簇(最接近于字符)：标准库未提供，去找[crates](https://crates.io/)





