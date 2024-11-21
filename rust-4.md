# Rust 基础课程 - 流程控制

## if 表达式

在 Rust 中，`if` 是一个表达式，可以返回一个值。例如：

```rust
fn main() {
    let x = 5;
    let y = if x == 5 {
        10
    } else {
        20
    };
    println!("The value of y is: {}", y);
}
```

上面的代码中，`if` 表达式会根据 `x` 的值返回 `10` 或 `20`，然后将返回值赋给 `y`。

if-else-if 链式表达式：

```rust
fn main() {
    let x = 5;
    let y = if x == 5 {
        10
    } else if x == 6 {
        20
    } else {
        30
    };
    println!("The value of y is: {}", y);
}
```

## loop 循环

在 Rust 中，`loop` 是一个无限循环，可以使用 `break` 语句退出循环。例如：

```rust
fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            break;
        }
    }
    println!("The value of x is: {}", x);
}
```

## while 循环

在 Rust 中，`while` 是一个条件循环，只要条件为真，就会一直执行循环体。例如：

```rust

fn main() {
    let mut x = 0;
    while x < 5 {
        x += 1;
    }
    println!("The value of x is: {}", x);
}
```

## for 循环

在 Rust 中，`for` 是一个迭代循环，用于遍历集合中的元素。例如：

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("{}", i);
    }

    for i in arr.iter().rev() {
        println!("{}", i);
    }
}
```

### 迭代器中的所有权转移

```rust
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    for i in arr {
        println!("{}", i);
    }
    // println!("{:?}", arr); // error: use of moved value: `arr`
}
```

上面的代码中，`arr` 是一个 `Vec` 类型的变量，`for` 循环会将 `arr` 的所有权转移给 `i`，所以在循环结束后，`arr` 就不能再使用了。

```rust
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    for i in &arr {
        println!("{}", i);
    }
    println!("{:?}", arr);
}
```

### 迭代器中的可变性

```rust
fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    for i in &mut arr {
        *i += 1;
    }
    println!("{:?}", arr);
}
```

## match 表达式

在 Rust 中，`match` 是一个模式匹配表达式，用于匹配不同的模式。例如：

```rust
fn main() {
    let x = 5;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

上面的代码中，`match` 表达式会根据 `x` 的值匹配不同的模式，如果没有匹配的模式，会执行 `_` 分支。

## 示例

下面是一个简单的示例，使用 `if` 表达式、`loop` 循环、`while` 循环、`for` 循环和 `match` 表达式：

```rust
fn main() {
    let x = 5;
    let y = if x == 5 {
        10
    } else {
        20
    };
    println!("The value of y is: {}", y);

    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            break;
        }
    }
    println!("The value of x is: {}", x);

    let mut x = 0;
    while x < 5 {
        x += 1;
    }
    println!("The value of x is: {}", x);

    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("{}", i);
    }

    let x = 5;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

