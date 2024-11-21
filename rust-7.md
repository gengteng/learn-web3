# Rust基础 - 引用

两种引用类型：

- `&T`：不可变引用，可以同时引用同一个值。
- `&mut T`：可变引用，只能有一个可变引用。

## 引用规则

- 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
- 引用必须有效。

例子：

```rust
fn main() {
    let mut x = 5;
    let y = &x;
    let z = &x;
    let w = &mut x; // error: cannot borrow `x` as mutable because it is also borrowed as immutable
}
```

## 切片

* 字符串切片：`&str`。
* 数组切片：`&[T]`。

例子：

```rust
fn main() {
    let s = String::from("hello, world");
    let s1 = &s[0..5];
    let s2 = &s[6..11];
    println!("{}", s1);
    println!("{}", s2);

    let v = vec![1, 2, 3, 4, 5];
    let v1 = &v[0..2];
    let v2 = &v[3..5];
    println!("{:?}", v1);
    println!("{:?}", v2);
}
```

## 课后习题

```rust
#[test]

fn test_lifetime() {
    let large = longest("a", "ab");
    println!("large one is {large}");

    // 加上生命周期参数就行
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
```