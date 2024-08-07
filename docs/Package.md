# <div align = "center">Package, Crate, Module</div>

## 1. Package, Crate, 定义Module

### 1.1 Rust的代码组织

* Rust的代码组织主要包括：哪些细节可以暴露，哪些细节是私有的
* 模块系统
  * ``Package``(包)：Cargo的特性，让你构建、测试、共享crate
  * ``Crate``(单元包)：一个模块树，他可以产生一个``library``或可执行文件
  * ``Module``(模块), use：控制代码的组织、作用域、私有路径
  * ``Path``：为``struct/function/module``等项命名的方式

### 1.2 Package和Crate

* ``Crate``的类型

  * ``binary``
  * ``library``

* ``Crate Root``

  * 是源代码文件
  * Rust编译器从这里开始，组成你的``Crate``的根``Module``

* 一个``Package``

  * 包含一个``Cargo.toml``，他描述了如何构建这些``Crates``
  * 只能有0-1个``library crate``
  * 可以有任意数量的``binary crate``
  * 至少包含一个``crate``
  * **通过``cargo new package_name``创建package**

  