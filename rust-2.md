# Rust 基础 - 变量和可变类型

## 可变与不可变变量

在 Rust 中，变量默认是不可变的，这意味着一旦变量被赋值后，就不能再次修改它的值。例如：

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // error: cannot assign twice to immutable variable `x`
    println!("The value of x is: {}", x);
}
```

上面的代码会报错，因为变量 `x` 是不可变的。要使变量可变，需要使用 `mut` 关键字：

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

## 常量

在 Rust 中，常量使用 `const` 关键字声明，常量必须显式指定类型，并且只能被赋值常量表达式，不能使用函数调用的结果等运行时计算的值。例如：

```rust
const MAX_POINTS: u32 = 100_000;
```

* 必须指定类型
* 使用大写字母和下划线分隔的命名规范
* 必须使用常量表达式，即编译时就能确定的值
* 常量不支持变量遮蔽
* 编译时会内联常量的值

## 静态变量

在 Rust 中，静态变量使用 `static` 关键字声明，静态变量的生命周期与整个程序的生命周期相同。例如：

```rust
static HELLO: &str = "Hello, World!";
```

* 必须指定类型
* 必须使用 `&'static str` 类型
* 静态变量的生命周期与整个程序的生命周期相同
* 静态变量不支持变量遮蔽

## 作用域和变量遮蔽

在 Rust 中，变量的作用域由它的位置决定，变量遮蔽是指在同一作用域中，可以定义同名的变量，后面的变量会遮蔽前面的变量。例如：

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

上面的代码会输出 `12`，因为后面的 `x` 遮蔽了前面的 `x`。