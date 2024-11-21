# Rust入门 第一节、环境搭建

## 1.1 Rust是什么

Rust是一门系统编程语言，它的设计目标是提供一种安全、并发、实用的编程语言。Rust的设计者是Mozilla公司的Graydon
Hoare，Rust的开发始于2006年，2010年11月开源，2015年5月1.0版发布。

## 1.2 Rust的特点

Rust的特点主要有以下几点：

1. 内存安全：Rust的内存安全是通过编译器的静态检查来保证的，编译器会在编译时检查内存访问是否合法，如果不合法则会拒绝编译。
2. 并发安全：Rust的并发安全是通过所有权系统来保证的，所有权系统会在编译时检查线程之间的数据访问是否合法，如果不合法则会拒绝编译。
3. 零成本抽象：Rust提供了零成本抽象的能力，可以通过抽象来提高代码的可读性和可维护性，而不会引入额外的运行时开销。

## 1.3 Rust的安装

Rust的安装非常简单，只需要在[Rust官网](https://www.rust-lang.org)下载安装包，然后运行安装包即可。

## 1.4 Rust的开发环境

Rust的开发环境主要有以下几种：

1. [Visual Studio Code](https://code.visualstudio.com)：Visual Studio Code是一款轻量级的编辑器，支持Rust的语法高亮、代码补全、调试等功能。
2. [RustRover](https://www.jetbrains.com/rust/)：RustRover是JetBrains公司推出的一款 Rust IDE，支持Rust的语法高亮、代码补全、调试等功能。

## 1.5 Rust的Hello World

下面是一个简单的Rust程序，用于打印"Hello, World!"：

```rust
fn main() {
    println!("Hello, World!");
}
```

要运行这个程序，只需要在终端中执行以下命令：

```bash
$ rustc hello.rs
$ ./hello
Hello, World!
```

## 1.6 使用 cargo

Rust的包管理工具是 cargo，它可以用来创建、构建、测试和发布 Rust 项目。要创建一个新的 Rust 项目，只需要执行以下命令：

```bash
$ cargo new hello
$ cd hello
```

然后在 hello 目录下会生成一个 Cargo.toml 文件和一个 src 目录，src 目录下会生成一个 main.rs 文件，内容如下：

```rust
fn main() {
    println!("Hello, world!");
}
```

要构建这个项目，只需要执行以下命令：

```bash
$ cargo build
```

要运行这个项目，只需要执行以下命令：

```bash
$ cargo run
```

要添加依赖，可以使用 cargo add 命令，例如：

```bash
$ cargo add rand
```

或者直接在 Cargo.toml 文件中添加依赖：

```toml
[dependencies]
rand = "0.8.4"
```

### 国内镜像配置

```toml
[source.crates-io]
# To use sparse index, change 'rsproxy' to 'rsproxy-sparse'
#replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[build]
target-dir = "/Users/gengteng/rust-target"
```

## 1.7 小结

本节介绍了 Rust 的基本情况、特点、安装、开发环境、Hello World 程序以及 cargo 包管理工具的使用方法。