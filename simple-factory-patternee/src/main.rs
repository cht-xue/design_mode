use simple_factory_patternee as s;

/*
 * 实现一个简单工厂模式
 */
fn main() {

    // 通过工厂创建动物实例
    let tiger = s::factory(&s::Factory::Tiger);
    tiger.to_string();

    let bird = s::factory(&s::Factory::Bird);
    bird.to_string();

}
