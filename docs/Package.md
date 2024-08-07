# <div align = "center">Package, Crate, Module</div>

## 1. Rust的代码组织

* Rust的代码组织主要包括：哪些细节可以暴露，哪些细节是私有的
* 模块系统
  * ``Package``(包)：Cargo的特性，让你构建、测试、共享crate
  * ``Crate``(单元包)：一个模块树，他可以产生一个``library``或可执行文件
  * ``Module``(模块), use：控制代码的组织、作用域、私有路径
  * ``Path``：为``struct/function/module``等项命名的方式

## 2. Package和Crate

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

    ```rust
    Creating binary (application) `my-project` package
    ```

  * ``src/main.rs``

    * ``binary crate``的``crate root``
    * ``crate``名与``package``名相同

  * ``src/lib.rs``

    * ``library crate``的``crate root``
    * ``crate``名与``package``名相同

  * 一个``package``可以同时包含``src/main.rs``和``src/lib.rs``

  * **一个package可以有多个binary crate**

    * 文件放在``src/bin``：每个文件是单独的``binary crate``

  * ``Crate``作用
    * 将相关功能组合到一个作用域内，便于在项目间共享
    * 例如``rand crate``，访问他的功能需要通过他的名字``rand``
  * **定义``Module``来控制作用域和私有性**
    * 在一个``crate``内，将代码进行分组
    * 增加可读性，易于复用
    * 控制项目(item)的私有性：private/public
  * 建立``module``
    * ``mod``关键字
    * 可嵌套
    * 可包含其他项(``struct/enum/常量/trait/函数``)的定义
  * ```src/main.rs```和``src/lib.rs``叫做``crate roots``，这两个文件中的任意一个都形成了名为``crate``的模块，位于整个模块树的根部

## 3. Path

* 为了在Rust的模块中找到某个条目，需要使用路径
* 路径的两种形式
  * 绝对路径：从``crate root``开始，使用``crate``名或字面值``crate``
  * 相对路径：从当前模块开始，使用``self(.)/super(..)``或当前模块的标识符
* 路径至少由一个标识符组成，标识符之间使用``::``
* 私有边界(``privacy boundary``)
  * 模块不仅可以组织代码，还可以定义私有边界
  * 如果想把函数或``struct``等设为私有，可以将他放入到某个模块中
  * Rust中所有的条目(函数，方法，``struct``，``enum``，模块，常量)默认是私有的
    * 前加``pub``关键字标记为公共的
  * **父级模块无法访问子模块中的私有条目**
  * **子模块中可以使用所有祖先模块中的条目**
* ``super``关键字
  * 表示父级模块(上级目录)
* ``pub struct``
  * **对于``struct``中每个字段仍需要使用``pub``关键字来声明是公共的**
* ``pub enum``
  * 将``enum``声明为公共的后，其中变体也是公共的

## 4. use

* 可以使用use关键字将路径导入到作用域内
* **对于函数通常引入到该函数的父模块路径即可**，不必引用到具体的函数名，防止与本地函数混淆
* ``struct/enum``指定完整路径(指定到本身)
* **同名条目：指定到父级，通过命名空间区分**
* **``as``**关键字
  * as关键字可以为引入的路径指定本地的别名
* 使用``pub use``重新导出名称
  * 使用``use``将路径导入到作用域内后，该名称在此作用域内是私有的

## 5. 使用外部包

> 可以换国内源，但是目前没出问题，先不换了

1. ``Cargo.toml``添加依赖的包(``package``)[crates](https://crates.io/)
2. use将特定条目引入作用域

* 标准库(``std``)也被当作外部包

  * 不需要修改``Cargo.toml``来包含``std``
  * 需要使用``use``将``std``中的特定条目引入当前作用域

* **使用嵌套路径来清理大量的use语句**

  * 如果使用同一个包或模块下的多个条目

  * 可以使用嵌套路径在同一行内将上述条目进行引入

    ```rust
    use 相同的部分::{ , , }
    ```

    * 一个引用是两一个引用的子路径：使用``self``

* 通配符``*``：将所有公共条目都引入到作用域

  * 谨慎使用，多数在测试时使用

## 6. 将模块拆分为不同文件

* 可以将模块内容移动到其他文件
* 模块定义时，如果模块名后边是``";"``，而不是代码块
* Rust会从与模块同名的文件中加载内容
* 拆分多级嵌套``mod``需要建立对应的文件夹



































































































