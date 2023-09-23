library(tidyverse)

iteration <- "0000"

collision_results <- read_csv(paste0("./results/", iteration, "_collision_results.csv"))
preimage_results <- read_csv(paste0("./results/", iteration, "_preimage_results.csv"))

# Plot collision results as boxplots based on bit size with a logarithmic y-axis
collision_results %>%
  mutate(bit_size = as.factor(bit_size)) %>%
  ggplot(aes(x = bit_size, y = num_hashes)) +
  geom_boxplot() +
  labs(title = "Collision Results", x = "Bit Size", y = "Number of Hashes") +
  scale_y_log10() +
  stat_function(fun = function(x) 2^(x/2), color = "red")

# Plot preimage results as boxplots based on bit size with a logarithmic y-axis
# and a curve representing 2^x
preimage_results %>%
  mutate(bit_size = as.factor(bit_size)) %>%
  ggplot(aes(x = bit_size, y = num_hashes)) +
  geom_boxplot() +
  labs(title = "Preimage Results", x = "Bit Size", y = "Number of Hashes") +
  scale_y_log10() +
  stat_function(fun = function(x) 2^x, color = "red")
