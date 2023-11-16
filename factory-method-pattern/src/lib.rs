/*!
 * # 实现一个简单的工厂方法模式
 */

/// 动物特征
pub trait Animals {
    fn to_string(&self);
}

/// 工厂特征
pub trait Factory {
    fn factory() -> Box<dyn Animals>;
}

/// 老虎
pub struct Tiger;

impl Animals for Tiger {

    fn to_string(&self) {
        println!("老虎");
    }
}
/// 老虎工厂
pub struct TigerFactory;

impl Factory for TigerFactory {

    fn factory() -> Box<dyn Animals> {
        Box::new(Tiger)
    }
}

/// 鸟
pub struct Bird;

impl Animals for Bird {
    fn to_string(&self) {
        println!("鸟");
    }
}

/// 鸟工厂
pub struct BirdFactory;

impl Factory for BirdFactory {
    fn factory() -> Box<dyn Animals> {
        Box::new(Bird)
    }
}

