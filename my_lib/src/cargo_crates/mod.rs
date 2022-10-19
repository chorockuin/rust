/* 
cargo doc --open 하면
web page로 모듈들에 정의된 구조체/함수들 목록들과 사용법 등을 볼 수 있다
아래와 같이 /// 으로 주석을 작성하면 doxygen처럼 web page에 설명과 예제를 더 보여줄 수 있다
*/

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Purple
    }
}

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub fn sample() {
    let red = kinds::PrimaryColor::Red;
    let yellow = kinds::PrimaryColor::Yellow;
    utils::mix(red, yellow);
}
