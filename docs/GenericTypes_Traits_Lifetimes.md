# <div align = "center">Generic Types, Traits, Lifetimes</div>

## 1. 泛型

* 泛型：提高代码复用能力，解决重复代码问题

* **是具体类型或其他属性的抽象代替**

* 泛型代码不是最终的代码，而是一种模板，里边有一些“占位符”，编译时将“占位符”替换为具体的类型

* 类型参数：惯例为``T``，CamelCase

* **函数定义中的泛型：在函数名后声明泛型**

  * 参数类型

  * 返回类型

    ```rust
    fn largest<T>(list : &[T]) -> T {
    	/*code*/
    }
    ```

* **Struct定义中的泛型：在结构体名后声明泛型**

  * 字段中使用泛型

  ```rust
  struct Point<T, U> {
      x: T,
      y: U,
  }
  ```

  * 可以使用多个泛型

* **Enum定义中的泛型：让枚举的变体持有泛型数据类型**

  * 例如``Option<T>``和``Result<T, E>``

    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

* **方法定义中的泛型**

  * 为``struct``或``enum``实现方法时使用泛型

  * 把T放在``impl``关键字后，表示在类型T上实现方法

    ```rust
    impl<T> Point<T> 
    ```

  * 只针对具体类型实现方法

    ```rust
    impl Point<f32>
    ```

  * **struct里的泛型类型参数可以和方法的泛型类型参数不同**

* **泛型代码的性能：使用泛型代码和使用具体类型的代码运行速度是一样的**

  * 单态化：在编译时将泛型类型替换为具体类型的过程

## 2. Trait







