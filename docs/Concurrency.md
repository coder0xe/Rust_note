# <div align="center">Fearless Concurrency</div>

## 1. 使用线程同时运行代码

> 编程语言实现线程的方式
>
> * 通过调用OS的API来创建线程，``1:1``模型(一个操作系统的线程对应一个语言线程)
>   * 需要较小的运行时
> * 语言自己实现的线程(绿色线程)，``M:N``模型(M个绿色线程对应N个系统线程)
>   * 需要较大的运行时
> * Rust标准库仅提供``1:1``模型的线程

* **通过spawn创建新线程**
  * ``thread::spawn``函数
* ``thread::sleep``导致当前线程暂停执行
* **主线程运行结束会导致其他线程停止**
* **通过join Handle来等待所有线程完成**
  * ``thread::spawn``函数返回值类型是``JoinHandle``
  * ``JoinHandle``持有值的所有权
  * 调用其``join``方法可以等待对应的其他线程的完成
* **join方法通过调用handle的join方法会阻止当前运行线程的执行，直到handle所表示的这些线程终结**

* 使用``move``闭包
  * ``move``闭包通常和``thread::spawn``函数一起使用，**他允许你使用其他线程的数据**

## 2. 使用消息传递来跨线程传递数据

* 消息传递：线程通过彼此发送消息来进行通信
* **Channel(管道)**
  * 包含：发送端/接收端
* **创建channel**
  * 使用``mpsc::channel``函数来创建channel
  * ``mpsc : multiple producer, single consumer``
  * 返回一个元组，里面元素分别为发送端，接收端 
* 发送端的send方法
  * 参数：要发送的数据
  * 返回：``Result<T, E>``
    * 如果有问题，如接收端已经被丢弃，返回一个错误
* 接收端的方法
  * ``recv``方法：阻止当前线程执行，直到channel中有值被送来
    * 一旦有值收到，就会返回Result<T, E>
  * ``try_recv``方法：不会阻塞
    * 立即返回``Result<T, E>``
    * 有数据到达：返回``Ok(data)``
* **channel和所有权转移**
  * 所有权在消息传递中非常重要，保证安全并发
* **通过克隆创建多个发送者**

## 3. 共享状态的并发

* channel类似于单所有权，一旦将值的所有权转移至channel,就无法使用它了
* 共享内存并发类似多所有权：多个线程可以同时访问同一块内存
* **使用``Mutex``来每次只允许一个线程来访问数据**
  * ``Mutex``是互斥锁
  * 保证单线程访问数据，避免数据竞争
  * 想要访问数据必须首先获取互斥锁(lock)
  * 使用完数据释放锁
* ``Mutex<T>``的``API``
  * 通过``Mutex::new()``来创建``Mutex<T>``
  * 访问数据前，``lock``方法获取锁
    * 阻塞当前线程
    * lock可能会失败
    * 返回``MutexGuard<T>``智能指针
* **多线程共享Mutex**
  * **多线程的多重所有权**
  * **使用``Arc<T>``来进行原子计数：Arc和Rc类似(atomic Rc)，但是可以用于并发场景**
* ``Mutex<T>``提供了内部可变性
  * 使用``RefCell<T>``来改变``Rc<T>``中的内容：``Rc<RefCell<T>>``
  * 使用``Mutex<T>``来改变``Arc<T>``中的内容：``Arc<Mutex<T>>``

## 4. 通过Send和Sync Trait来扩展并发

### 4.1 Send Trait

* 实现Send trait的类型可在线程间转移所有权

* Rust中几乎所有的类型都实现了Send
  * ``Rc<T>``没有实现Send,只用于单线程
* 任何完全由Send类型组成的类型也被标记为Send

### 4.2 Sync Trait

* 实现Sync的类型可以可以安全地被多个线程引用
* 基础类型都是Sync
  * ``Rc<T>/RefCell<T>/Cell<T>家族``不是Sync的
* 完全由Sync类型组成的类型也是Sync

* **手动实现Send / Sync Trait 不安全**

