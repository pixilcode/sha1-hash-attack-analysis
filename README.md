# SHA-1 Hash Attack

This project is an exploration of two attacks on the SHA-1 hash function. The
first attack is a collision attack, which is a type of attack where two
different inputs are found that produce the same hash value. The second attack
is a preimage attack, which is a type of attack where a second input is found
that produces the same hash value as a given input.

The main program is written in Rust, and the code is found in the `src`
directory. It repeatedly performs the attacks and outputs the results to a csv
file in the `results` directory. One file is produced for each attack. In order
to avoid overwriting previous results, a `.results_num` file is used to keep
track of the number of results files that have been produced.

The columns of the CSV output are:
* `bit_size`: The size of the hash value in bits
* `num_hashes`: The number of hashes that were performed in order to find the
  collision
* `collision_value_a`, `collision_value_b`: The two values that produce the
  collision

A graph of the results can be produced by sourcing `analysis.R` and calling the
`analyze_iteration` function. This can be done by executing the following in an
R session (assuming the working directory is the root of the project):

```R
# if not already installed, install the `tidyverse` and `svglite` packages
install.packages(c("tidyverse", "svglite"))

# source the analysis script
source("analysis.R")

# analyze the results for iteration 0001 (the iteration is the number in the
# results file name)
analyze_iteration("0001")
```

The resulting graphs will be saved as `results/xxxx_collision_graph.svg` and
`results/xxxx_preimage_graph.svg`, where `xxxx` is the iteration number. These
graphs show a boxplot for each bit_size that was tested. The boxplot shows the
distribution of the number of hashes that were performed in order to find a
collision or preimage. In addition, a line representing the median number of
hashes and a line representing the expected average number of hashes are shown.