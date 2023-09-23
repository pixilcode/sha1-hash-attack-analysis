library(tidyverse)

analyze_results <- function(title, results, expected_fn) {
  # Box plot
  boxplot_plot <- geom_boxplot(
    data = results,
    mapping = aes(
      x = bit_size,
      y = num_hashes,
      group = cut_width(bit_size, 0.5)
    )
  )
  
  # Median plot
  results_summary <-
    group_by(results, bit_size) %>%
    summarize(median = median(num_hashes))
  median_plot <- geom_smooth(
    data = results_summary,
    mapping = aes(
      x = bit_size,
      y = median,
      color = "Actual"
    ),
    se = FALSE,
    size = 1
  )
  
  # Expected results plot
  expected_plot <- stat_function(
    fun = expected_fn,
    mapping = aes(
      color = "Expected"
    ),
    show.legend = TRUE,
    linetype = 2,
    size = 1
  )
  
  # Aesthetics
  labels <- labs(title = title, x = "Bit Size", y = "Number of Hashes")
  x_scale <- scale_x_continuous(breaks = results_summary$bit_size)
  y_scale <- scale_y_log10()
  legend_guide <- guides(color = guide_legend(
    title = NULL,
  ))
  
  final_plot <-
    ggplot() +
    boxplot_plot +
    median_plot +
    expected_plot +
    labels +
    x_scale +
    y_scale +
    legend_guide
  
  final_plot
}

save_plot <- function(plot, filename) {
  file_loc <- paste0("./results/", filename, ".svg")
  ggsave(
    filename = file_loc,
    plot = plot,
    device = "svg",
    width = 600,
    height = 375,
    units = "px",
    dpi = "screen"
  )
}

analyze_iteration <- function(iteration_num) {
  iteration <- formatC(iteration_num, width = 4, flag = "0")
  
  collision_title <- "Collision Attack Results"
  collision_results <- read_csv(paste0("./results/", iteration, "_collision_results.csv"))
  collision_expected_fn <- function(x) 2^(x/2)
  collision_plot <- analyze_results(collision_title, collision_results, collision_expected_fn)
  collision_file <- paste0(iteration, "_collision_graph")
  save_plot(collision_plot, collision_file)
  
  preimage_title <- "Pre-image Attack Results"
  preimage_results <- read_csv(paste0("./results/", iteration, "_preimage_results.csv"))
  preimage_expected_fn <- function(x) 2^x
  preimage_plot <- analyze_results(preimage_title, preimage_results, preimage_expected_fn)
  preimage_file <- paste0(iteration, "_preimage_graph")
  save_plot(preimage_plot, preimage_file)
}
