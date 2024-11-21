# Rust 基础 - 数组与切片

## 简介

Rust 的数组类型是 `[T; N]`，其中 `T` 是元素类型，`N` 是数组长度，数组的长度是固定的，不能动态增加或减少。

```rust
fn main() {
    let a: [i32; 3] = [1, 2, 3];
    println!("{:?}", a);
}
```

Rust 的切片类型是 `&[T]` / `&mut [T]`，切片是对数组的引用，可以访问数组的一部分元素，切片的长度是动态的。

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[1..3];
    println!("{:?}", s);
}
```

### 索引检查

Rust 在编译时会检查数组和切片的索引是否越界，如果索引越界会导致编译错误。

```rust
fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let i = 3;
    let x = a[i]; // error: index out of bounds
    println!("{}", x);
}
```

索引的类型必须是 `usize`，否则会导致编译错误。

## 数组

* 数组的定义其实就是分配一段连续的相同数据类型的内存块。

* 数组是静态的。这意味着一旦定义和初始化，则永远不可更改它的长度。

* 数组的元素有着相同的数据类型，每一个元素都独占着数据类型大小的内存块。 也就是说，数组的内存大小等于数组的长度乘以数组的数据类型。

* 数组中的每一个元素都按照顺序依次存储，这个顺序号既代表着元素的存储位置，也是数 组元素的唯一标识。我们把这个标识称之为数组下标。
  注意，数组下标从 0 开始。

* 填充数组中的每一个元素的过程称为数组初始化。也就是说数组初始化就是为数组的每 一个元素赋值。

* 可以更新或修改数组元素的值，但不能删除数组元素。如果要删除功能，你可以将它的值 赋值为 0 或其它表示删除的值。

例子：

```rust
fn main() {
    let mut arr: [i32; 3] = [1, 2, 3];
    arr[0] = 4;
    println!("{:?}", arr);
}
```

遍历数组：

```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}
```

或者使用 `iter()` 方法：

```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    for i in arr.iter() {
        println!("{}", i);
    }
}
```

修改数组元素：

```rust
fn main() {
    let mut arr: [i32; 3] = [1, 2, 3];
    arr[0] = 4;
    println!("{:?}", arr);
}
```

## 切片

存储了数组的引用以及数组的长度，可以访问数组的一部分元素。

```rust
fn main() {
    let mut arr: [i32; 3] = [1, 2, 3];
    let slice = &mut arr[..];
    slice[0] = 4;
    println!("{:?}", arr);
}
```

切片的常用函数：

* `len()`：返回切片的长度。
* `is_empty()`：判断切片是否为空。
* `contains()`：判断切片是否包含某个元素。
* `repeat()`：重复slice指定次数
* `reverse()`：反转slice
* `join()`：将各元素压平（flatten）并通过指定的分隔符连接起来
* `swap()`：交换两个索引处的元素，如s.swap(1,3)
* `windows()`：以指定大小的窗口进行滚动迭代
* `starts_with()`：判断slice是否以某个slice开头

例子：

```rust
// 使用上述所有函数

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let slice = &arr[..];
    println!("len: {}", slice.len());
    println!("is_empty: {}", slice.is_empty());
    println!("contains: {}", slice.contains(&2));
    println!("repeat: {:?}", slice.repeat(2));
    println!("reverse: {:?}", slice.reverse());
    println!("join: {:?}", slice.join(","));
    println!("swap: {:?}", slice.swap(0, 2));
    println!("windows: {:?}", slice.windows(2));
    println!("starts_with: {:?}", slice.starts_with(&[1, 2]));
}
```

## 课后习题

给定一个整数数组 nums，返回一个数组 answer，使得 answer[i] 等于 nums 除之外 nums[i] 的所有元素的乘积。

任何前缀或后缀的乘积 nums 都保证适合 32 位整数。

您必须编写一个能够及时运行 O(n) 且无需使用除法运算的算法。

示例 1： 输入：nums = [1,2,3,4] 输出：[24,12,8,6]

示例 2： 输入：nums = [-1,1,0,-3,3] 输出：[0,0,9,0,0]

限制： 2 <= nums.length <= 105 -30 <= nums[i] <= 30

任何前缀或后缀的乘积 nums 都保证适合 32 位整数。

进阶：你能以 O(1) 额外空间复杂度解决这个问题吗？（输出数组不算作空间复杂度分析的额外空间。）

```rust
fn main() {
    let input = [1, 2, 3, 4, 5];
    let result = answer(&input);
    println!("{:?}", result);

    let result = optimized_answer(&input);
    println!("{:?}", result);
}

fn answer<const N: usize>(input: &[u32; N]) -> [u32; N] {
    let mut prefix_product = [1; N];
    let mut suffix_product = [1; N];

    for i in 1..N {
        prefix_product[i] = prefix_product[i - 1] * input[i - 1];
        suffix_product[N - i - 1] = suffix_product[N - i] * input[N - i];
    }

    let mut result = [0; N];
    for i in 0..N {
        result[i] = prefix_product[i] * suffix_product[i];
    }

    result
}

fn optimized_answer<const N: usize>(input: &[u32; N]) -> [u32; N] {
    let mut result = [1; N];

    for i in 1..N {
        result[i] = result[i - 1] * input[i - 1];
    }

    let mut suffix_product = 1;
    for i in (0..N).rev() {
        result[i] *= suffix_product;
        suffix_product *= input[i];
    }

    result
}
```