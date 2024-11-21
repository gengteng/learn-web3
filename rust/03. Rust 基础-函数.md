# Rust 基础-函数

## 函数的组成

Rust 中的函数由以下几个部分组成：

- fn 关键字：用于声明一个函数。
- 函数名：函数的名称，用于调用函数；格式要求：由字母、数字、下划线组成，不能以数字开头。
- 参数列表：函数的参数，用于接收调用函数时传入的值。
- 返回值类型：函数的返回值类型，用于指定函数的返回值类型。

## 函数的声明

Rust 中的函数声明格式如下：

```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // 函数体
}
```

其中：

- `fn`：关键字，用于声明一个函数。
- `function_name`：函数名，用于调用函数。
- `parameter1: Type1, parameter2: Type2`：参数列表，用于接收调用函数时传入的值。
- `ReturnType`：返回值类型，用于指定函数的返回值类型。

## return

Rust 中的函数默认返回最后一个表达式的值，不需要使用 `return` 关键字。如果需要提前返回，可以使用 `return` 关键字。

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add(a: i32, b: i32) -> () {
    a + b;
    // ()
}

fn add() -> ! {
    loop {}
}
```

## 示例

下面是一个简单的函数示例：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

在这个示例中，`add` 函数接收两个 `i32` 类型的参数 `a` 和 `b`，返回一个 `i32` 类型的值，表示 `a` 和 `b` 的和。