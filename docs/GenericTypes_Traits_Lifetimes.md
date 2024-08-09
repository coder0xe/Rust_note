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

* Trait告诉Rust编译器：某种类型具有哪些可以与其他类型共享的功能

* Trait：抽象的定义共享行为，**类似于Java中的接口**

* ``Trait bounds(约束)``：泛型类型参数指定为实现了特定行为的类型(要求泛型类型参数实现了某些trait)

* **定义一个Trait**

  * Trait的定义：把方法签名放在一起，来定义实现某种目的所必须的一组行为
  * **关键字：trait**
  * 只有方法签名，没有方法具体实现
  * trait可以有多个方法，每个方法签名占一行，以``;``结尾
  * 实现该trait的类型必须提供具体的方法实现

* **在类型上实现trait**

  ```rust
  impl trait_name for type_name { ... }
  ```

* **实现trait的约束**

  * 可以在某个类型上实现某个trait的前提条件是：
    * 这个类型或这个trait是在本地crate里定义的
  * **无法为外部类型来实现外部的trait**
    * 孤儿原则：父类型不存在，确保其他人写的代码不能破坏您的代码

* **默认实现**

  * 可以在trait定义中给出方法的默认实现
  * 如果类型的实现中没有方法实现即采用默认实现
  * 默认实现的方法可以调用trait中的其他方法，即使这些方法没有默认实现
  * **无法从方法的重写实现中调用默认实现**

* **trait作为参数：多态**

  * 参数类型指明为实现接口``item : impl trait_name``：适用于简单情况

  * ``Trait bound``语法，可用于复杂情况

  * **使用+指定多个Trait bound**

    ```rust
    pub fn notify<T: Summary + Display>(item : T) {
        println!("Breaking news! {}", item.summarize());
    }
    ```

  * **在方法签名后使用where子句指定trait bound**

    ```rust
    pub fn notify1<T>(item : T) 
    where T: Summary + Display
    {
        println!("Breaking news! {}", item.summarize());
    }
    ```

* **trait作为返回类型**

  * 返回类型实现接口``impl Trait``
    * 这种方法只能返回确定的同一种类型，返回不同类型的代码会报错
    * 不能实现多态

* **使用trait bound有条件的实现方法**

  * 在使用泛型类型参数的``impl``块上使用``trait bound``，我们可以有条件的为实现了特定``trait``的类型来实现方法

    ```rust
    impl <T: Display + PartialOrd> Pair<T> {...}
    ```

  * 也可以为实现了其他trait的任意类型有条件的实现某个trait

  * 为满足trait bound的所有类型上实现trait叫做覆盖实现(例如满足Display的trait bound的任意类型都实现toString这个trait)





