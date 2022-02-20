use genetic_algorithm_fn::function;
use genetic_algorithm_fn::solutions;
use genetic_algorithm_fn::test_functions;
use genetic_algorithm_traits::Individual;
use genetic_algorithm_traits::Population;

#[test]
fn test_end_to_end() {
    // Use the hartman function to test whether a realistic function can be maximized.
    let function_to_optimize = function::Function::new(|x| {
        Ok(-test_functions::hartman_3_dimensional(
            *x.get(0)
                .ok_or(function::FunctionError::WrongNumberOfEntries {
                    expected_number_of_entries: 3,
                    actual_number_of_entries: x.len(),
                })?,
            *x.get(1)
                .ok_or(function::FunctionError::WrongNumberOfEntries {
                    expected_number_of_entries: 3,
                    actual_number_of_entries: x.len(),
                })?,
            *x.get(2)
                .ok_or(function::FunctionError::WrongNumberOfEntries {
                    expected_number_of_entries: 3,
                    actual_number_of_entries: x.len(),
                })?,
        ))
    });

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
