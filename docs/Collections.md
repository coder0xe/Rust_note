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

## 3. HashMap

* ```HashMap<K,V>```

* 内部实现：``Hash``函数，决定在内存中如何存放K和V

* 创建空HashMap：``new``函数

* 插入键值对：``insert()``方法

* ``HashMap``不在预导入(``Prelude``)模块中，需要手动导入

  ```rust
  use std::collections::HashMap;
  ```

* 数据存储在heap上

* **同构的**：所有的K为一个类型，所有的V为一个类型

* **另一种创建``HashMap``的方式：collect方法(实际上可以把数据整合成很多种集合类型，包括HashMap)**

  * 在元素类型为Tuple的Vector上使用collect方法，可以组建一个``HashMap``(要求Tuple有两个值K/V)

    ```rust
    let h: HashMap<_, _> = scores.collect();
    ```

* HashMap和所有权

  * 对于实现了Copy trait的类型，值会被复制到``HashMap``中
  * 对于拥有所有权的值(例如String)，值会被移动，所有权转移到``HashMap``(如果将值的引用插入HashMap，值本身不会移动)
  * 在HashMap有效期间，被引用的值必须保持有效

* **访问HashMap中的值**

  * ``get()``方法：参数K，返回Option<&V>

* **遍历HashMap**

  * for循环

    ```rust
    for (k,v) in &scores {}
    ```

    **注：这里需要使用HashMap的引用，否则会发生所有权的移动(``into_iter()``方法)**

* **更新HashMap**

  * 大小可变
  * 每个K只能对应一个V
    * 插入相同的K，会覆盖掉原来的V
  * ``entry()``方法：检查指定的K是否对应一个V
    * 返回``enum Entry``：代表值是否存在
    * ``Entry``的``or_insert()``方法：
      * 如果K存在，返回到对应的V的一个可变引用
      * 如果K不存在，将方法参数K作为K的新值插进去，返回到这个值的可变引用

* **Hash函数**

  * 默认情况下HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务的(``Dos``)攻击
    * 不是最快的Hash算法
    * 但是具有更好的安全性
  * 可以指定不同的hasher来切换到另一个函数



