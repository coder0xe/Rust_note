# <div align ="center">Smart Pointers</div>

> 智能指针：行为和指针相似，有额外的数据和功能
>
> * 引用计数智能指针类型
>   * 可以被多次引用
>   * 没有引用释放内存

## 1. 使用``Box<T>``来指向Heap上的数据

* ``Box<T>``是最简单的智能指针：**指向数据类型T的一个指针**

  * 允许在``heap``上存储数据，而不是``stack``
  * ``stack``上保存指向``heap``数据的指针
  * 没有性能开销
  * 没有其他额外功能
  * **实现了Deref trait和Drop trait**

* **使用Box在heap上存储数据**

  ```rust
  Box::new(xx) // 申请内存
  ```

* **使用Box赋能递归类型**

  * **在编译时，Rust需要知道一个类型所占空间的大小，而递归类型无法在编译时确定**

    <img src="img/recursive.svg" alt="recursive" height = "300">

  * 关于``Cons list``

    * 来自lisp语言的一种数据结构
    * ``Cons list``中的每个成员由两个元素组成
      * 当前项
      * 下一个元素
    * 最后一个元素只包含一个``Nil``，表示结束

  * **使用Box来获得确定大小的递归类型：``Box<T>``是一个指针，Rust知道他需要多少空间**

    * 即用指针来指向下一个元素

      <img src="img/boxpointer.svg" alt="boxpointer" style="zoom:50%;" />

  * ``Box<T>``提供了间接存储和heap内存分配的功能，没有其他额外功能

## 2. Deref Trait

* ``Deref : dereference``

* 实现``Deref Trait``使我们可以自定义解引用运算符``*``的行为

* 实现``Deref``，智能指针可像常规引用一样来处理

* **解引用运算符***

  * 常规引用``&``是指针

* **定义自己的智能指针**

  * ``Box<T>``被定义为拥有一个元素的``tuple struct``

* **实现``Deref Trait``**

  * 标准库中的``Deref Trait``要求我们实现一个``deref``方法

  * 该方法借用self(``&self``)

  * **返回一个指向内部数据的引用**，例如

    ```rust
    &MyBox<T> -> deref -> &T
    ```

    **通过调用deref获得内部数据的引用后才可以使用解引用***，或者说

    ```rust
    let x = MyBox::new(5); // x:MyBox<T>类型指针 
    // 自定义数据类型要求实现 Deref Trait
    *x = *(x.deref()) = *(&i32);
    ```

* **函数和方法的隐式解引用转化(自动拆箱)**

  * 隐式解引用转化(``Deref Coercion``)是为函数和方法提供的一种便捷特性
  * 假设T实现了``Deref Trait``
    * ``Deref Coercion``可以把T的引用转化为T经过``Deref``操作后生成的引用
  * 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配
    * ``Deref Coercion``自动发生
    * 编译器会**对``deref()``进行一系列调用**，来把它转为所需的参数类型
    * **编译时完成，无额外开销**
  
* 解引用与可变性

  * **使用``DerefMut trait``重载可变引用的*运算符**
  * 在类型和trait满足下面三种情况，Rust执行``deref conercion``
    * 当``T:Deref<Target=U>``, 允许``&T``转换为``&U``
    * 当``T:DerefMut<Target=U>``, 允许``&mut T``转换为``&mut U``
    * 当``T:Deref<Target=U>``, 允许``&mut T``转换为``&U``

## 3. Drop Trait

* 实现``drop trait``，可以让我们自定义**当值要离开作用域时发生的动作(例如文件/网络释放)**

* 任何类型都可以实现``Drop Trait``

* ``Drop Trait``只要求实现一个``drop``方法

  * 参数：对self的可变引用

* ``Drop Trait``在预导入模块中(prelude)

* **使用``std::mem::drop``来提前drop值**

  * 很难直接禁用自动的drop功能，也没必要
  * **Drop trait的目的就是进行自动的释放处理逻辑**

* Rust不允许手动调用Drop trait的drop方法，即

  ```rust
  c.drop();// Err
  ```

* 可以调用标准库的drop函数来提前drop值

  ```rust
  drop(c); // Ok
  ```

## 4. ``Rc<T>``：引用计数智能指针

* 有时一个值会有多个所有者

* 多重所有权：``Rc<T>:reference counting``

* 0个引用：可以清理

* 使用场景：需要在heap上分配数据，这些数据被程序的多个部分**只读**，但在编译时无法确定哪个部分最后使用完这些数据

  * 回顾所有权规则：**一个值可以有多个不可变引用**(只读)

* ``Rc<T>``只能用于**单线程**

* ``Rc<T>``不在预导入模块，需要手动导入

* **需要使用``Rc::new()``创建需要进行多次引用的类型**

* **``Rc::clone(&a)``：增加引用计数**

* **``Rc::strong_count(&a)``：获得强引用计数**

* ``Rc::weak_count(&a)``：获得弱引用计数

* 使用普通指针``Box<T>``的错误：不支持多次对同一值移动，第一次指向该值所有权就会发生移动

  ![rc](img/rc.svg)

* ``Rc<T>``实现了``Drop Trait``

* **``Rc<T>::clone()``与类型的clone()方法**

  * 增加引用次数/深克隆

## 5. ``RefCell<T>``和内部可变性

### 5.1 内部可变性

### 5.2 ``RefCell<T>``







