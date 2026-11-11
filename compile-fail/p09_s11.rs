// 字段默认私有
// 视频定位：P09 02:58
// 期望错误：E0616 —— 本文件故意编译失败，不要加入 workspace。
// error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.seasonal_fruit = String::from("mango");  // 私有字段
}
