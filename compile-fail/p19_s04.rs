// Rc 跨线程当场被拦
// 视频定位：P19 01:07
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: `Rc<i32>` cannot be sent between threads safely

use std::rc::Rc;
use std::thread;

let counter = Rc::new(0);
let handles: Vec<_> = (0..2)
    .map(|_| {
        let c = Rc::clone(&counter);
        thread::spawn(move || { let _ = *c; })
    })
    .collect();
for h in handles { h.join().unwrap(); }
