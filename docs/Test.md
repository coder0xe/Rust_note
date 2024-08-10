# <div align = "center">Test</div>

## 1. 编写和运行测试

* 测试：函数
* 准备数据/状态
* 运行被测试代码
* 断言结果
* **测试函数需要使用``test``属性(Attribute)进行标注**
  * Attribute就是一段代码的元数据
  * **在函数上加``#[test]``，可把函数变成测试函数**
* **运行测试：**
  * **``cargo test``运行所有测试函数**
  * Rust会构建一个``Test Runner``可执行文件，他会运行标注了test的函数，并报告其运行是否成功
* 当使用cargo创建library项目的时候，会生成一个``test module``，里面有一个``test``函数
  * 可以添加任意数量的``test module``或函数
* **测试失败**
  * 测试函数``panic``就表示失败
  * 每个测试运行在一个新线程
  * 当主线程拿到某个测试线程挂掉了，那个测试标记为失败了

## 2. Assert

* 使用``assert!``宏检查测试结果
  * 来自标准库，用来确定某个状态是否为``true``
  * ``true``：测试通过
  * ``false``：调用``panic!``，测试失败
* ``assert_eq!()/assert_ne!()``

## 3. 自定义错误信息

* 可以向``assert!/assert_eq!/assert_ne!``添加可选的自定义信息
* ``assert!()``：第一个参数必填，自定义消息作为第二个参数
* ``assert_eq!()/assert_ne!()``：前两个参数必填，自定义消息作为第三个参数
* 自定义信息参数会被传入``format!()``宏，可以使用``{}``占位符

## 4. 用should_panic检查恐慌

* 验证代码是否如期处理了发生错误的情况

* ``should_panic``属性(attribute)：``#[should_panic]``

  * 函数发生``panic``：测试通过
  * 不发生``panic``：测试失败

* **让``should_panic``更精确：为should_panic属性添加一个可选的expected参数**

  * 检查失败消息中是否包含指定的文字(``substring``)

    ```rust
    #[should_panic(expected = "less")] 
    ```

## 5. 在测试中使用Result<T,E>

* 无需panic,将Result<T,E>作为返回类型编写测试：
  * 返回Ok：测试通过
  * 返回Err：测试失败
* 不要在Result<T, E>编写的测试上使用``#[should_panic]``

## 6. 控制测试运行

* 改变``cargo test``的行为：添加命令行参数

* 默认行为：

  * 并行运行所有测试
  * 不显示所有输出，使读取与测试相关的结果更加容易

* **命令行参数**

  * 针对``cargo test``的参数

    ```shell
    cargo test --help
    ```

  * 针对测试可执行文件，放在``--``之后

    ```shell
    cargo test -- --help
    ```

* **并行运行测试**

  * 运行多个测试：默认使用多个线程并行运行

  * 确保测试之间不会互相依赖，不依赖于某个共享状态(环境、工作目录、环境变量等)

  * ``--test-threads``参数

    ```shell
    cargo test -- --test-threads=1
    ```

* **显式函数输出**

  * 默认如果测试通过，Rust的test库会捕获所有打印到标准输出的内容
  * 想看到成功的测试中打印的内容：``--show-output``

## 7. 按名称运行测试

* 将测试的名称作为``cargo test``的参数
* 运行单个测试：指定测试名
* 运行多个测试：指定测试名的一部分(指定模块名)

## 8. 忽略测试

* ``ignore``属性(attribute)

* ``#[ignore]``

* 运行被忽略的测试

  ```sh
  cargo test -- --ignored
  ```

## 9. 测试的组织

* 测试的分类
  * 单元测试：只对一个模块进行隔离测试，可测试private接口
  * 集成测试：在每个测试中使用多个模块，只能使用public接口

### 9.1 单元测试

* test**模块**上的**``#[cfg(test)]``**标注(configuration)：只有运行``cargo test``时才会编译和运行代码
* ``cfg:configuration(配置)``
  * 告诉Rust下面的条目只有在指定的配置选项下才被包含
* 配置选项``test``，由Rust提供，用来编译和运行测试
* 只有``cargo test``才会编译代码，包括模块中的helper函数和``#[test]``标记的函数
* **测试私有函数**

### 9.2 集成测试

* Rust中集成测试完全位于被测试库的外部
* 被测试库的多个部分是否能正确的一起工作
* **覆盖率**
* **创建集成测试：tests目录**
* tests目录下的每个测试文件都是一个单独的crate，**不共享行为**
* **无需标注``#[cfg(test)]``，tests目录会被特殊对待，只有cargo test才会编译tests目录下的文件**
* **运行指定的集成测试**
  * 运行一个特定的集成测试：``cargo test 函数名``
  * 运行某个测试文件：``cargo test --test 文件名``
* **如果需要创建测试中需要使用的工具函数，可以在tests下新建文件夹(例如common)，在文件夹下编写工具函数，在测试中导入**
* **针对binary crate进行集成测试**
  * 如果项目是``binary crate``，只含有``src/main.rs``不含有``src/lib.rs``
  * 不能在tests目录下创建集成测试
  * 无法把``main.rs``的函数导入作用域
  * 只有``library crate``才能暴露函数给其他``crate``用









