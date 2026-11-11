// 泛型方法破坏 dyn 兼容
// 视频定位：P21 02:43
// 期望错误：E0038 —— 本文件故意编译失败，不要加入 workspace。
// error[E0038]: the trait `Encoder` is not dyn compatible

trait Encoder {
    fn encode<T>(&self, value: &T);   // 泛型方法
    fn name(&self) -> String;
}
struct Json;
impl Encoder for Json {
    fn encode<T>(&self, _v: &T) {}
    fn name(&self) -> String { String::from("json") }
}

fn main() {
    let e: Box<dyn Encoder> = Box::new(Json);
    println!("{}", e.name());
}
