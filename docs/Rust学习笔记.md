# <div align = "center">Rust学习笔记</div>

> * Ubuntu22.04 
> * Rust 1.8.0

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

