# <div align = "center">An I/O Project</div>

## 1. 接受命令行参数

* ```use std::env```
* 使用String类型的Vector接收命令行参数
* 产生一个字符串数组
* 数组中第一个元素为编译出的程序名

## 2. 读取文件内容

* ```use std::fs```

* 读取内容

  ```rust
      let contents = fs::read_to_string(filename)
      .expect("Something went wronng when reading the file");
  ```

## 3. 重构

* 二进制程序关注点分离的指导性原则
  * 将程序拆分为``main.rs``和``lib.rs``，**将业务逻辑放进``lib.rs``**
    * 将业务逻辑放进``lib.rs``还有一个好处是便于测试，``main.rs``无法进行测试
  * 当命令行解析逻辑较少时把它放在``main.rs``也行
  * 命令行解析逻辑比较复杂时，需要将他从``main.rs``提取到``lib.rs``
* 经过上述拆分后，留在``main``的功能有
  * 使用参数值调用命令行解析逻辑
  * 进行其他配置
  * 调用``lib.rs``中的``run``函数
  * 处理``run``函数中可能出现的错误

## 4. 使用TDD(测试驱动开发)开发库功能

* 编写一个会失败的测试，运行该测试，确保该测试是按照预期的原因失败
* 编写或修改刚好足够的代码，让测试通过
* 重构刚刚添加或修改的代码，确保测试会始终通过

## 5. 使用环境变量

* 使用```env::var(NAME)```获取环境变量

* 运行时设置环境变量

  ```rust
  NAME=1 cargo run ...
  ```

## 6. 将错误信息写到标准错误而不是标准输出

* 标准输出：``stdout``
* 标准错误：``stderr``
  * ``eprintln!``输出到标准错误







