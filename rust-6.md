# Rust - ownership 所有权

> Rust 为什么是内存安全的系统编程语言？

## 内存回收

* 静态语言：C / C++ / Rust, 编译时对变量类型和内存分配进行检查。
* 动态语言：Python / JavaScript / Ruby, 运行时对变量类型和内存分配进行检查。

### 回收方式对比

* 手动回收：C / C++
* 垃圾回收：Python / JavaScript / Ruby
* 所有权系统：Rust

## 栈和堆

* 栈：后进先出，固定大小，快速访问，地址增长从高到低。
* 堆：动态分配，大小不固定，访问速度慢，地址增长从低到高。

## 为什么要有所有权

* 避免内存泄漏：堆内存分配后没有释放。
* 避免二次释放：堆内存释放后继续使用。
* 避免悬垂指针：栈内存释放后继续使用。
* 避免数据竞争：多个线程同时访问同一块内存。
* 避免空指针：指针为空时访问。
* 避免野指针：指针指向无效内存。

所有权规则：

* 每个值都有一个所有者。
* 同一时间只能有一个可变引用或多个不可变引用。
* 引用必须在所有者离开作用域前结束。

例子：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1); // error: value borrowed here after move
}
```

## 所有权转移

* 所有权转移：将值的所有权从一个变量转移到另一个变量。
* 所有权复制：整数、浮点数、布尔值、字符、元组等固定大小类型。
* 所有权借用：引用一个值而不获取其所有权。

例子：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}
```

## 课后习题

```rust
// TODO
fn take_ownership(s: String) -> String {
    s
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = take_ownership(s1);

    // 以下代码不能改
    println!("{}", s1);
    println!("{}", s2);
}
```

解法1：

```rust
fn take_ownership(s: String) -> String {
    s
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = take_ownership(s1.clone());

    // 以下代码不能改
    println!("{}", s1);
    println!("{}", s2);
}
```

解法2：

```rust
fn take_ownership(s: &str) -> &str {
    s
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = take_ownership(&s1);

    // 以下代码不能改
    println!("{}", s1);
    println!("{}", s2);
}
```



