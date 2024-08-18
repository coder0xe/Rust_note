# <div align="center">模式匹配</div>

## 1. 模式

* 模式是Rust中的一种特殊语法，用于匹配简单和复杂类型的结构
* 将模式与匹配表达式和其他构造结合使用，可以更好地控制程序的控制流
* 模式由以下组成
  * 字面值
  * 解构的数组、enum、struct和tuple
  * 变量
  * 通配符
  * 占位符

## 2. 用到模式的地方

### 2.1 match的arm

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

* match表达式的要求：包含所有可能性
* 一个特殊的模式：``_``匹配任何，通常用于match的最后一个arm,或用于忽略某些值

### 2.2 条件if let表达式

* if let表达式主要是作为一种简短的方式来代替只有一个匹配项的match
* if let 可选的可以有``else if / else if let``

### 2.3 while let 条件循环

```rust
while let PATTERN {
    //
}
```

### 2.4 for循环

* for循环中模式就是紧跟``for``循环后的值

  ```rust
  for PATTERN in xxx

### 2.5 let语句

* 例如声明元组中的模式匹配

```rust
let PATTERN = EXPRESSION;
let (x,y) = (1,2);
```

### 2.6 函数参数

## 3. 可辩驳性（refutable）：模式是否会无法匹配

* 模式：可辩驳的/无可辩驳的

* 能匹配任何可能传递的值的模式：无可辩驳的，例如

  ```rust
  let x = 5;
  ```

* 对某些可能的值，无法进行匹配的模式：可辩驳的

  ```rust
  if let Some(x) = a_value // None
  ```

* 函数参数/let语句/for循环只接受无可辩驳的模式

* if let 和 while let接受可辩驳和无可辩驳的模式

## 4. 模式语法

### 4.1 匹配字面值

* 模式可以直接匹配字面值，例如

  ```rust
  match x {
      1 => println!("one"),
      _ => println!("else"),
  }
  ```

### 4.2 匹配命名变量

* 命名的变量是可匹配任何值的无可辩驳的模式，例如在``()``内命名一个变量，之后使用这个变量

  ```rust
  match x {
      Some(5) => println!("is five"),
      Some(y) => println!("is {}", y),
  }
  ```

  * 或者``Ok(x), Err(x)``中的例子

### 4.3 多重模式

* 在match表达式中，使用``|``来匹配多重模式

  ```rust
  match x {
      1 | 2 => //
  }
  ```

### 4.4 使用 ``..=``匹配某个范围的值

```rust
match y {
    'a'..='j' => println!("early ASCII"),
    'k'..='z' => println!("lately ASCII"),
    _ => println!("else"),
}
```

* ``..``表示范围，该范围为左闭右开，``=``表示右闭

### 4.5 解构以分解值

* 可以使用模式来解构``struct, enum, tuple``，从而获得不同部分

  ```rust
  match p {
      Point{x:0, y} => println!("x is 0"),
      Point{x, y: 0} => println!("y is 0"),
      Point { x, y } => println!("Normal"),
  }
  ```

### 4.6 在模式中忽略值

* ``_``

* ``_``配合其他模式

* 使用以``_``开头的名称

* ``..``忽略值的剩余部分

  ```rust
  let a = (1,2,3,4,5);
  match a {
      (first, .. , last) => {
          println!("first is {}", first);
      }
  }
  ```

### 4.7 使用match守卫来提供额外的条件

* match守卫就是在arm分支后的if子句

  ```rust
  match num {
      Some(x) if x < 5 => {
          println!("less than 5");
      },
      _ => {
          println!("eg 5");
      }
  }
  ```

### 4.8 @绑定

* @符号让我们可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值

