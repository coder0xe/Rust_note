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

