use genetic_algorithm_fn::function;
use genetic_algorithm_fn::solutions;
use genetic_algorithm_fn::test_functions;
use genetic_algorithm_traits::Individual;
use genetic_algorithm_traits::Population;

#[test]
fn test_end_to_end() {
    // Use the hartman function to test whether a realistic function can be maximized.
    let function_to_optimize =
        function::Function::new(|(x, y, z)| -test_functions::hartman_3_dimensional(x, y, z));

    // End-to-end test: does the error of the solution get down?
    let solutions = solutions::Solutions::random(50, -10.0..10.0);
    let max_fit_initial =
        solutions.get_n_fittest(1, &function_to_optimize)[0].fitness(&function_to_optimize);
    let optimized_solutions =
        solutions::evolve_population(solutions, 100, 20, &function_to_optimize, 0);
    let max_fit_optimized = optimized_solutions.get_n_fittest(1, &function_to_optimize)[0]
        .fitness(&function_to_optimize);

    // Assert after optimizing, the routes is fitter then before.
    assert!(max_fit_initial < max_fit_optimized);
}
