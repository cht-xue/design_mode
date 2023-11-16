/*!
 * # 实现的一个简单工厂模式
 * 
 *  通过 factory 工厂函数可以创建 Tiger 和 Bird 实例
 */

/// 表示工厂可以创建的动物类型
pub enum Factory {
    Tiger,
    Bird
}

/// 表示动物的共同特征
pub trait Animals {
    fn to_string(&self);
}

/// 老虎
pub struct Tiger {
    name: String
}

impl Animals for Tiger {

    fn to_string(&self) {
        println!("name: {}\n",self.name);
    }
}

/// 鸟
pub struct  Bird {
    name: String
}

impl Animals for Bird {

    fn to_string(&self) {
        println!("name: {}\n",self.name);
    }
}

/// 工厂函数，根据传入的动物类型参数创建相应的动物实例
pub fn factory(animals_type: &Factory) -> Box<dyn Animals> {
    match animals_type {
        Factory::Tiger => Box::new(Tiger {
            name: String::from("老虎")
        }),
        Factory::Bird => Box::new(Bird {
            name: String::from("鸟")
        })
    }
}
