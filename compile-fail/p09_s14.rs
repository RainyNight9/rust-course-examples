// use 只管当前作用域
// 视频定位：P09 03:47
// 期望错误：E0433 —— 本文件故意编译失败，不要加入 workspace。
// error[E0433]: failed to resolve: use of undeclared type `HashMap`

use std::collections::HashMap;

mod child {
    pub fn build() {
        // 父模块的 use 不会自动带进子模块
        let mut m = HashMap::new();
        m.insert("k", 1);
    }
}

fn main() {
    child::build();
}
