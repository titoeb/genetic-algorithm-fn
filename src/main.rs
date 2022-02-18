use genetic_algorithm_fn::function;
use genetic_algorithm_fn::solutions;
use genetic_algorithm_fn::test_functions;

fn main() {
    let initial_params_range = -150.0..150.0;
    let function_to_optimize =
        function::Function::new(|(x, y, z)| -test_functions::hartman_3_dimensional(x, y, z));
    // Single-threaded test
    for n_generations in (10..=510).step_by(250) {
        for size_generation in (10..=40).step_by(10) {
            let (run_time, minimal_loss) = solutions::benchmark_population(
                n_generations,
                size_generation,
                &function_to_optimize,
                0,
                initial_params_range.clone(),
            );
            println!(
                "n_generations: {}, size_generation: {}, time: {} ms, maximal function value: {}",
                n_generations, size_generation, run_time, minimal_loss
            );
        }
    }
    // Multi-threaded test
    println!("Running multi-threaded computation!");
    let n_jobs = 8;
    for n_generations in (10..=1100).step_by(750) {
        for size_generation in (10..=80).step_by(10) {
            let (execution_time, maximal_function_value) = solutions::benchmark_population(
                n_generations,
                size_generation,
                &function_to_optimize,
                n_jobs,
                initial_params_range.clone(),
            );
            println!(
                "n_generations: {}, size_generation: {}, time: {} ms, maximal function value: {:.8}, n_jobs: {}",
                n_generations, size_generation, execution_time, maximal_function_value, n_jobs
            );
        }
    }
}
