# <div align = "center">Cargo和crates.io</div>

## 1. 通过release profile来自定义构建

* ``release profile``：发布配置
  * 是预定义的
  * 可自定义
* 每个profile的配置都独立于其他的profile
* **Cargo主要的两个profile**
  * ``dev profile``：适用于开发``cargo build``
  * ``release profile``：适用于发布``cargo build --release``
* **自定义``profile``**
  * 针对每个``profile``，Cargo都提供了默认的配置
  * 自定义 ```xxx profile```的配置
    * 在``Cargo.toml``中添加``[profile.xxx]``区域，在里面覆盖默认配置的子集
  * 例如```opt-level = 0```：编译优化等级
  * [配置官方文档](https://doc.rust-lang.org/cargo/)

## 2. 发布crate到[crates.io](https://crates.io)

* 通过发布包来共享你的代码
* 分发已注册的包的源代码
* 主要托管开源的代码

> 文档注释：```///```
>
> * 用于生成HTML文档
> * 显示公共API的文档注释：如何使用API
> * 允许markdown语法
> * **生成文档：``cargo doc``(target/doc目录下)**

* 常用章节
  * ``# Examples``：使用样例
  * ``# Panics``：可能发生panic的场景
  * ``# Errors``：描述错误种类和错误条件
  * ``# Safety``：如果函数处于不安全调用，就应该解释函数``unsafe``的原因，以及调用者确保的使用前提
* **文档注释作为测试**
  * **示例代码块在运行cargo test时会作为测试运行**
* 为包含注释的项添加文档注释
  * 符号``//!``
  * 用于描述crate和模块
    * ``crate root``
* **运行cargo login [API token]**
  * 通知cargo把你的API token存储在本地
* **为新的crate添加元数据：在[package]区域添加一些数据**
  * ``name``：要求唯一
  * ``description``
  * ``lisence`` [许可证网站](https://spdx.org/licenses/)
  * ``version``
  * ``author``
* **发布``crate``：``cargo public``**
  * 发布版本无法覆盖，代码不能删除
* **发布新版本：version -> public**
* **撤回版本：``cargo yank --vers xxx (--undo)``**
  * 不能删除，但可以防止新项目依赖于该版本(已经使用的项目可以继续使用)
  * 已经产生``Cargo.lock``的项目不会中断
  * 任何将来生成的``Cargo.lock``文件都不会用被yank的版本

## 3. 使用pub use导出方便使用的公共API

> 问题：crate的程序结构在开发时对于开发者很合理，但对于他的使用者不够方便
>
> * 例如难以找到开发时深层结构中的某个类型``x::y::z::...``

* 不需要重新组织内部代码结构
* **使用``pub use``：重新导出到顶层结构中，创建一个与内部私有结构不同的对外公共结构**

## 4. 从crates.io安装二进制crate

* 命令``cargo install``：只能安装``binary crate(src/main.rs或其他被指定为二进制文件)``
* 二进制目标/库目标

## 5. 自定义命令扩展cargo

* cargo被设计成可以使用子命令来扩展

* 如果$PATH中的某个二进制是``cargo-something``

  ```sh
  cargo something
  ```

* ``cargo --list``，列出所有命令



