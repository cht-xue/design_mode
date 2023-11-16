use f::Factory;
use factory_method_pattern as f;

/*
    实现工厂方法模式
 */
fn main() {
    // 通过老虎工厂创建老虎
    let tiger = f::TigerFactory::factory();
    tiger.to_string();

    let bird = f::BirdFactory::factory();
    bird.to_string();
}
