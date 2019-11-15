mod bag;

fn main() {
    let mut BAG = bag::Bag::default();
    BAG.init();
    println!("Bag is: {:?}", BAG);
    println!("Score for z is: {}", BAG.score('z'));

    let rack = BAG.draw_tiles(7);
    println!("Rack is: {:?}", rack);
    println!("Bag is: {:?}", BAG.distribution);
}