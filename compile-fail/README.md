# 故意编译失败示例

共 59 个。这些文件在课程里用来讲编译错误，**故意编译不过**，
不加入 Cargo workspace。想看报错可以在目录里单独执行：

```bash
rustc compile-fail/p05_s15.rs
```

| 文件 | 讲解点 | 视频定位 | 错误码 | 期望报错 |
| --- | --- | --- | --- | --- |
| [`p01_s07.rs`](p01_s07.rs) | 一个报错，胜过一次线上事故 | P01 02:28 | `E0382` | error[E0382]: borrow of moved value: `s1` |
| [`p01_s20.rs`](p01_s20.rs) | 同一份数据，两次可变借用 | P01 07:10 | `E0499` | error[E0499]: cannot borrow `v` as mutable more than once |
| [`p02_s11.rs`](p02_s11.rs) | 少写 main 会报 E0601 | P02 03:05 | `E0601` | error[E0601]: main function not found in crate |
| [`p03_s13.rs`](p03_s13.rs) | 漏一个分支就编译不过 | P03 03:58 | `E0004` | error[E0004]: non-exhaustive patterns: `Ordering::Equal` not covered |
| [`p03_s16.rs`](p03_s16.rs) | 不写 : u32 就推不出来 | P03 04:54 | `E0282` | error[E0282]: type annotations needed for the result of `parse` |
| [`p04_s04.rs`](p04_s04.rs) | 想改必须先说一声 | P04 01:06 | `E0384` | error[E0384]: cannot assign twice to immutable variable `x` |
| [`p04_s16.rs`](p04_s16.rs) | 参数不写类型不行 | P04 04:37 | `E0308` | error[E0308]: mismatched types, expected `i32`, found `&str` |
| [`p04_s18.rs`](p04_s18.rs) | 加分号，值就没了 | P04 05:15 | `E0308` | error[E0308]: mismatched types, expected `i32`, found `()` |
| [`p05_s15.rs`](p05_s15.rs) | let s2 = s1 之后 | P05 04:25 | `E0382` | error[E0382]: borrow of moved value: `s1` |
| [`p05_s16.rs`](p05_s16.rs) | 报错会告诉你在哪移动 | P05 04:41 | `E0382` | error[E0382]: borrow of moved value: `s1` |
| [`p05_s17.rs`](p05_s17.rs) | 取走一个字段也算移动 | P05 05:00 | `E0382` | error[E0382]: borrow of partially moved value: `u` |
| [`p06_s10.rs`](p06_s10.rs) | 三处必须对齐 | P06 03:01 | `E0596` | error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable |
| [`p06_s13.rs`](p06_s13.rs) | 两个可变引用撞一起 | P06 04:01 | `E0499` | error[E0499]: cannot borrow `v` as mutable more than once at a time |
| [`p06_s14.rs`](p06_s14.rs) | 一读一写也不行 | P06 04:22 | `E0502` | error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable |
| [`p06_s17.rs`](p06_s17.rs) | 返回值里没有可借的东西 | P06 05:16 | `—` | this function's return type contains a borrowed value, but there is no value for it to be borrowed from |
| [`p06_s18.rs`](p06_s18.rs) | 返回的引用没来源 | P06 05:27 | `E0106` | error[E0106]: missing lifetime specifier |
| [`p07_s07.rs`](p07_s07.rs) | 字段为什么报错 | P07 01:57 | `E0106` | error[E0106]: missing lifetime specifier |
| [`p07_s10.rs`](p07_s10.rs) | 更新语法的部分移动 | P07 02:49 | `E0382` | error[E0382]: borrow of moved value: `user1.username` |
| [`p07_s13.rs`](p07_s13.rs) | 打印结构体要Debug | P07 03:37 | `E0277` | error[E0277]: `Rectangle` doesn't implement `Debug` |
| [`p07_s16.rs`](p07_s16.rs) | 可变方法要声明 | P07 04:21 | `E0596` | error[E0596]: cannot borrow `rect` as mutable, as it is not declared as mutable |
| [`p07_s17.rs`](p07_s17.rs) | 谁会拿走实例 | P07 04:40 | `E0382` | error[E0382]: borrow of moved value: `rect` |
| [`p07_s21.rs`](p07_s21.rs) | 点号调不了关联函数 | P07 05:46 | `E0599` | error[E0599]: no method named `square` found for struct `Rectangle` in the current scope |
| [`p08_s11.rs`](p08_s11.rs) | 按值匹配会搬走数据 | P08 03:07 | `E0382` | error[E0382]: borrow of moved value: `name` |
| [`p08_s14.rs`](p08_s14.rs) | 漏一个分支就报错 | P08 03:52 | `E0004` | error[E0004]: non-exhaustive patterns: `Direction::Right` not covered |
| [`p08_s15.rs`](p08_s15.rs) | 分支类型要统一 | P08 04:13 | `E0308` | error[E0308]: `match` arms have incompatible types |
| [`p09_s08.rs`](p09_s08.rs) | 私有模块报 E0603 | P09 02:07 | `E0603` | error[E0603]: module `inner` is private |
| [`p09_s11.rs`](p09_s11.rs) | 字段默认私有 | P09 02:58 | `E0616` | error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private |
| [`p09_s14.rs`](p09_s14.rs) | use 只管当前作用域 | P09 03:47 | `E0433` | error[E0433]: failed to resolve: use of undeclared type `HashMap` |
| [`p09_s19.rs`](p09_s19.rs) | mod 报 E0583 | P09 05:10 | `E0583` | error[E0583]: file not found for module `garden` |
| [`p10_s08.rs`](p10_s08.rs) | 忘了标 mut | P10 02:11 | `E0596` | error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable |
| [`p10_s11.rs`](p10_s11.rs) | 引用没用完别 push | P10 02:54 | `E0502` | error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable |
| [`p10_s14.rs`](p10_s14.rs) | collect 要标注 | P10 03:46 | `E0282` | error[E0282]: type annotations needed |
| [`p10_s18.rs`](p10_s18.rs) | 两个 &str 不能加 | P10 04:53 | `E0369` | error[E0369]: cannot add `&str` to `&str` |
| [`p11_s10.rs`](p11_s10.rs) | 别忘了 Err 分支 | P11 02:52 | `E0004` | error[E0004]: non-exhaustive patterns: `Err(_)` not covered |
| [`p11_s13.rs`](p11_s13.rs) | E 得Debug才行 | P11 03:40 | `E0277` | error[E0277]: `MyError` doesn't implement `Debug` |
| [`p11_s16.rs`](p11_s16.rs) | 末尾别忘包 Ok | P11 04:27 | `E0308` | error[E0308]: mismatched types |
| [`p11_s18.rs`](p11_s18.rs) | void 里用不了 ? | P11 04:56 | `E0277` | error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` |
| [`p12_s06.rs`](p12_s06.rs) | 缺 Copy 约束 | P12 01:39 | `E0277` | error[E0277]: the trait bound `String: Copy` is not satisfied |
| [`p12_s09.rs`](p12_s09.rs) | 部分方法看类型 | P12 02:22 | `E0599` | error[E0599]: no method named `x` found for struct `Point<i32, f64>` in the current scope |
| [`p12_s19.rs`](p12_s19.rs) | 缺 Display | P12 04:58 | `E0277` | error[E0277]: the trait bound `Config: Display` is not satisfied |
| [`p13_s03.rs`](p13_s03.rs) | 返回局部变量必然失败 | P13 00:52 | `E0515` | error[E0515]: cannot return value referencing local variable `s` |
| [`p13_s05.rs`](p13_s05.rs) | 编译器看不出哪个还活着 | P13 01:32 | `E0106` | error[E0106]: missing lifetime specifier |
| [`p13_s07.rs`](p13_s07.rs) | 较短那方说了算 | P13 02:07 | `E0597` | error[E0597]: `inner` does not live long enough |
| [`p13_s18.rs`](p13_s18.rs) | impl 要先声明标记 | P13 05:22 | `E0261` | error[E0261]: use of undeclared lifetime name `'a` |
| [`p13_s21.rs`](p13_s21.rs) | 局部引用变不成静态 | P13 06:19 | `E0597` | error[E0597]: `s` does not live long enough |
| [`p14_s06.rs`](p14_s06.rs) | 自定义类型缺了什么 | P14 01:48 | `E0277` | error[E0277]: `Secret` doesn't implement `std::fmt::Debug` |
| [`p15_s07.rs`](p15_s07.rs) | 目标类型推不出 | P15 02:05 | `E0282` | error[E0282]: type annotations needed |
| [`p15_s16.rs`](p15_s16.rs) | 去掉就立刻报错 | P15 04:45 | `E0106` | error[E0106]: missing lifetime specifier |
| [`p16_s03.rs`](p16_s03.rs) | 第二次调用就锁死 | P16 00:51 | `E0308` | error[E0308]: mismatched types |
| [`p16_s10.rs`](p16_s10.rs) | 转移的可能是副本 | P16 02:58 | `E0382` | error[E0382]: borrow of moved value: `s` |
| [`p16_s17.rs`](p16_s17.rs) | 目标类型省不掉 | P16 04:59 | `E0282` | error[E0282]: type annotations needed |
| [`p18_s06.rs`](p18_s06.rs) | 不装箱算不出大小 | P18 01:50 | `E0072` | error[E0072]: recursive type `List` has infinite size |
| [`p19_s04.rs`](p19_s04.rs) | Rc 跨线程当场被拦 | P19 01:07 | `E0277` | error[E0277]: `Rc<i32>` cannot be sent between threads safely |
| [`p20_s17.rs`](p20_s17.rs) | future 往往是 !Unpin | P20 04:35 | `E0277` | error[E0277]: the trait bound `{async block@...}: Unpin` is not satisfied |
| [`p20_s20.rs`](p20_s20.rs) | 守护让 future 不 Send | P20 05:22 | `E0277` | error[E0277]: future cannot be sent between threads safely |
| [`p21_s04.rs`](p21_s04.rs) | 私有字段访问报错 | P21 01:09 | `E0616` | error[E0616]: field `balance` of struct `Account` is private |
| [`p21_s10.rs`](p21_s10.rs) | 泛型方法破坏 dyn 兼容 | P21 02:43 | `E0038` | error[E0038]: the trait `Encoder` is not dyn compatible |
| [`p21_s17.rs`](p21_s17.rs) | let 触发 E0005 | P21 04:39 | `E0005` | error[E0005]: refutable pattern in local binding: `None` not covered |
| [`p22_s05.rs`](p22_s05.rs) | unsafe 块里照样E0499 | P22 01:20 | `E0499` | error[E0499]: cannot borrow `v` as mutable more than once at a time |
