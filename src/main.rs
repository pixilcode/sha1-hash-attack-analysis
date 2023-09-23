use std::io::Write;
use std::path::Path;

use bench::BenchResult;

mod attack;
mod bench;
mod hash;

const BIT_SIZES: [usize; 5] = [8, 11, 16, 19, 24];

const COLLISION_SAMPLE_SIZE: usize = 10000;
const PREIMAGE_SAMPLE_SIZE: usize = 100;

fn main() {
    println!("starting benchmarks...");

    let next_results_num = get_next_results_num();

    let collision_results: Vec<_> = BIT_SIZES
        .iter()
        .inspect(|&bit_size| print!("running collision benchmark for bit size {} ... ", bit_size))
        .flat_map(|&bit_size| bench::run_collision_bench(bit_size, COLLISION_SAMPLE_SIZE))
        .inspect(|_| println!("complete!"))
        .collect();

    let file_path = format!("results/collision_results_{next_results_num:04}.csv");
    print!("writing results to file {} ... ", file_path);
    write_to_file(file_path, collision_results);
    println!("complete!");

    let preimage_results: Vec<_> = BIT_SIZES
        .iter()
        .inspect(|&bit_size| println!("running preimage benchmark for bit size {} ... ", bit_size))
        .flat_map(|&bit_size| bench::run_preimage_bench(bit_size, PREIMAGE_SAMPLE_SIZE))
        .inspect(|_| println!("complete!"))
        .collect();

    let file_path = format!("results/preimage_results_{next_results_num:04}.csv");
    print!("writing results to file {} ... ", file_path);
    write_to_file(file_path, preimage_results);
    println!("complete!");

    println!("benchmarks complete!");
}

// write a vector of BenchResults to a csv file
fn write_to_file(file_path: impl AsRef<Path>, results: Vec<BenchResult>) {
    // open a file for writing
    let mut file = std::fs::File::create(file_path).unwrap();

    // write headers
    writeln!(file, "bit_size,num_hashes").unwrap();

    // write results
    for result in results {
        writeln!(file, "{},{}", result.bit_size, result.num_hashes).unwrap();
    }
}

// read a file, `.results_num`, from the `results` directory and return the number
// if it exists, otherwise return 0
fn get_next_results_num() -> usize {
    let results_dir = Path::new("results");
    let results_num_file = results_dir.join(".results_num");

    if !results_dir.exists() {
        std::fs::create_dir(results_dir).unwrap();
    }

    if results_num_file.exists() {
        let results_num = std::fs::read_to_string(&results_num_file)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        std::fs::write(&results_num_file, (results_num + 1).to_string()).unwrap();
        results_num
    } else {
        std::fs::write(&results_num_file, "1").unwrap();
        0
    }
}
