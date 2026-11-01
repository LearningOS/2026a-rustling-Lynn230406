// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 检查矩形尺寸是否正确
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 填 rect.width
        assert_eq!(rect.height, 20); // 填 rect.height
    }

    // 核心魔法：加上这个属性！
    #[test]
    #[should_panic]
    fn negative_width() {
        // 这里应该 panic，所以测试通过
        let _rect = Rectangle::new(-10, 10);
    }

    // 核心魔法：加上这个属性！
    #[test]
    #[should_panic]
    fn negative_height() {
        // 这里应该 panic，所以测试通过
        let _rect = Rectangle::new(10, -10);
    }
}
