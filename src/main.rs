mod bag;

fn main() {
    let mut BAG = bag::Bag::default();
    BAG.init();
    println!("Score for z is: {}", BAG.score('z'));

    let rack = BAG.draw_tiles(7);
    println!("Rack is: {:?}", rack);
}