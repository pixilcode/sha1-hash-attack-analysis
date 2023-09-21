mod attack;
mod bench;
mod hash;

fn main() {
    println!("Beginning!");
    let result = bench::run_collision_bench(8, 1);
    println!("Printing results:");
    println!("{:?}", result);
    println!("Done!");
}
