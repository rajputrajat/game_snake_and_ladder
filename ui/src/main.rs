use core_game::board::Entity;

fn main() {
    let entities = Entity::get_randomly(10, 5);
    println!("{:?}", entities);
}
