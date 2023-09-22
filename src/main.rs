mod attack;
mod bench;
mod hash;

fn main() {
    println!("Beginning!");

    let result = bench::run_collision_bench(31, 100);
    println!("Printing results:");
    println!("{:?}", result);

    let result = bench::run_preimage_bench(31, 100);
    println!("Printing results:");
    println!("{:?}", result);

    println!("Done!");
}
