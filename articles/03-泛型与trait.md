# 泛型与 trait：抽象但不牺牲性能

## 导语

泛型与 trait 是《Rust 程序设计语言》第 10 章上篇的核心，对应视频合集第 12 集（P12，约 8 分钟）。难处不在语法，而在它同时要求两件事：一份代码要覆盖多种类型，编译器又必须在编译期把这些类型逐一看清。习惯了动态类型或接口加虚表的读者，容易把泛型误当成「运行时再决定类型」，也容易把 trait 当成普通的继承接口。第 12 集从 00:51 的泛型热身，经 01:39 的约束报错、03:13 的单态化，到 05:15 的 where 子句，回答的正是：抽象如何被写进类型系统，而不是留到运行时。

## 一、从重复代码到泛型：把类型提升为参数

找出 `i32` 列表与 `char` 列表的最大值，逻辑一致，差别只在元素类型；与其为每种类型各写一份函数，不如把类型声明为参数 `T`。以下片段摘自 `ch12/examples/p12_s05.rs`（运行命令 `cargo run -p ch12 --example p12_s05`）：

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

多数情况下 `T` 由实参推断得出，`largest(&nums)` 会被推成 `i32`；需要显式指定或推断不出时，用 turbofish 写作 `largest::<char>(&chars)`，它跟在函数或方法名之后，而非类型名之后。类型参数同样可用于结构体与枚举：`Point<T>` 让两个字段共享同一类型，`Point<T, U>` 允许字段类型不同，标准库的 `Option<T>` 与 `Result<T, E>` 本身就是泛型枚举。为泛型类型写方法时，`impl` 之后要重新声明一次 `<T>`；方法还可以带自己独立的泛型参数，与类型上的 `T` 互不相干。

## 二、约束与单态化：泛型为何不付运行时代价

泛型并非「任意类型皆可」，函数体里用到的每个操作都要向编译器交代清楚。`T: PartialOrd + Copy` 称为 trait bound，加号读作「且」，意为两个约束须同时满足：`PartialOrd` 支持 `>` 比较，`Copy` 则是因为 `list[0]` 与 `for &item` 都要按值把元素复制出来。把同一函数用于 `Vec<String>` 时，`String` 能比较大小却不实现 `Copy`，编译器便以 E0277 点名 `String: Copy` 未满足。约束是写给编译器的承诺，缺一条都不行；若希望覆盖 `String`，可把返回类型改为 `&T`，或改用 `Clone` 显式克隆，从而绕开 `Copy`。

理解这一点，才能理解零成本抽象。编译器在编译期对每一个实际用到的具体类型各生成一份专用代码，这一过程称为单态化（monomorphization）。`ch12/examples/p12_s12.rs`（`cargo run -p ch12 --example p12_s12`）以注释把结果直接写了出来：

```rust
fn identity<T>(x: T) -> T { x }

fn main() {
    let a = identity(5i32);     // 生成一份 identity_i32
    let b = identity("hi");     // 再生成一份 identity_str
    println!("{a} {b}");
}
```

运行时既没有类型判断，也没有虚表查找，效率与手写多份函数相同；代价是编译时间略长、二进制略大。这也解释了报错为何能精确到「哪个 trait 未满足」：检查发生在编译期，而非运行时。

## 三、trait：描述「一个类型能做什么」

如果说泛型放宽了类型的范围，trait 就负责收紧能力。trait 是一组方法签名的集合，只描述实现类型具备什么行为；方法可仅有签名、由实现者补齐，也可带默认方法体。`Self` 指代当前实现类型，`&self`、`-> Self` 都由此而来。以下定义摘自 `ch12/examples/p12_s13.rs`（`cargo run -p ch12 --example p12_s13`）：

```rust
pub trait Summary {
    fn summarize(&self) -> String;

    fn default_hint(&self) -> String {
        String::from("（阅读更多…）")   // 默认实现
    }
}
```

实现者不写带默认体的方法即沿用默认版本，写了则以自己的实现为准；`p12_s14.rs` 的 `Tweet` 只补 `summarize`、`hint` 沿用默认。为类型实现 trait 写作 `impl Trait for Type`，并受孤儿规则约束：trait 与类型至少有一个在当前 crate 中定义，否则不同 crate 可能为同一外部类型实现同名 trait 而冲突。trait 还可直接充当泛型约束，`T: Summary` 即告知编译器 `T` 上可调用哪些方法。至此，泛型给出「这里可以是任意类型」，trait bound 补上「但必须会这些事」。

## 四、约束写在哪里：impl Trait、where 与关联类型的取舍

当函数只要求「任意实现了某 trait 的类型」，`&impl Summary` 是简洁写法；若需多个参数同类型，或约束较多，泛型加 trait bound 更精确。约束一多，把 `T:`、`U:` 全挤进尖括号会损害可读性，此时应移到 `where` 子句。`ch12/examples/p12_s20.rs`（`cargo run -p ch12 --example p12_s20`）是典型：

```rust
fn compare_and_print<T, U>(t: &T, u: &U) -> bool
where
    T: Display + PartialOrd,
    U: Debug + Clone,
{
    println!("t = {t}, u = {u:?}");
    true
}
```

`where` 与写在参数后完全等价，纯粹是可读性选择。再进一层是关联类型与泛型参数的取舍：当某 trait 对给定类型只有一种合理的相关类型时，应声明为关联类型（如迭代器 `Iterator` 的 `Item`），调用方无需额外指定；若同一类型可能需要多种实现，则保留泛型参数更合适。这一取舍与后续迭代器、trait 对象及生命周期的写法一脉相承。

## 常见坑

1. 把泛型理解为运行时携带类型信息。`T` 在编译期即被单态化为具体类型，运行时没有类型分支，也没有虚表查找。
2. 把 `+` 读成「或」。`T: PartialOrd + Copy` 表示「且」，两个约束须同时成立；E0277 会点名缺哪一条。
3. 认为「能比较大小」就足够。`list[0]`、`for &item` 还要求按值取出元素，`String` 因此不满足 `Copy`；改返回 `&T` 或改用 `Clone` 才是正解。
4. 把 turbofish 写错位置。`largest::<char>` 跟在函数名后，不跟在类型名后；它只在需要显式指定或推断失败时使用。
5. 忘记在 `impl` 后重新声明 `<T>`，或误以为方法对所有参数组合都存在。仅对 `Point<T, T>` 实现的 `x()`，在 `Point<i32, f64>` 上调用会报 E0599。
6. 忽视孤儿规则。为外部类型实现外部 trait 会被拒绝，须保证 trait 与类型至少一方是本地定义的。

## 配套资源

| 类别 | 视频定位 | 文件 | 命令 / 错误码 | 讲解点 |
| --- | --- | --- | --- | --- |
| 可运行示例 | P12 01:08 | `p12_s04.rs` | `cargo run -p ch12 --example p12_s04` | 推断不了就显式写 turbofish |
| 可运行示例 | P12 01:26 | `p12_s05.rs` | `cargo run -p ch12 --example p12_s05` | 泛型函数与 trait bound |
| 可运行示例 | P12 02:10 | `p12_s08.rs` | `cargo run -p ch12 --example p12_s08` | 结构体与枚举带类型参数 |
| 可运行示例 | P12 03:13 | `p12_s12.rs` | `cargo run -p ch12 --example p12_s12` | 单态化各生成一份 |
| 可运行示例 | P12 03:34 | `p12_s13.rs` | `cargo run -p ch12 --example p12_s13` | 定义共同行为与默认实现 |
| 可运行示例 | P12 03:44 | `p12_s14.rs` | `cargo run -p ch12 --example p12_s14` | 默认实现可被覆盖 |
| 可运行示例 | P12 05:15 | `p12_s20.rs` | `cargo run -p ch12 --example p12_s20` | where 子句 |
| 故意报错反例 | P12 01:39 | `p12_s06.rs` | `E0277` | 缺 Copy 约束 |
| 故意报错反例 | P12 02:22 | `p12_s09.rs` | `E0599` | 部分方法看类型 |
| 故意报错反例 | P12 04:58 | `p12_s19.rs` | `E0277` | 缺 Display 约束 |

反例不加入 Cargo workspace，可用 `rustc compile-fail/p12_s06.rs` 单独编译，观察对应错误码。

## 小结

泛型提供灵活性，trait 提供约束，编译器把两者一起变成专用代码——这是第 12 集要留下的主线。它与紧随其后的生命周期一集共同构成第 10 章，也与后续迭代器、trait 对象直接相连。建议对照视频合集按时间点回看关键镜头，再到配套代码仓库逐一运行 `ch12/examples/` 下的示例、编译 `compile-fail/` 下的反例，把 E0277 与 E0599 两类报错亲手复现一次；约束该不该写、写在何处，读报错往往比读定义更快建立直觉。
