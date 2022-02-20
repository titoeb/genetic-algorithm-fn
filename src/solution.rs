use crate::function;
use core::ops::Add;
use genetic_algorithm_traits::Individual;
use rand::distributions::uniform::SampleRange;
use rand::Rng;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Get a random alement from a range.
///
/// # Arguments
///
/// * `range` - The range that should be sampled.
fn get_random_elem_from_range<T, R>(range: R) -> Option<T>
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform,
    R: SampleRange<T>,
{
    if !range.is_empty() {
        Some(rand::thread_rng().gen_range::<T, R>(range))
    } else {
        None
    }
}

/// Average two values.
/// # Arguments
///
/// * `value_1` - The fist value that should be part of the average.
/// * `value_2` - The second value that should be part of the average.
fn average<T>(value_1: T, value_2: T) -> f64
where
    T: Add<Output = T>,
    T: Into<f64>,
{
    let sum_as_float: f64 = (value_1 + value_2).into();
    sum_as_float / 2.0
}

/// Convert a floating point value into a string with 10 decimal places.
/// # Arguments
///
/// * `value` - The floating point value that should be converted.
fn f64_to_floating_point_precision_string(value: f64) -> String {
    // We use 10 digits as floating point precision.
    f64_to_rounded_string(value, 10)
}
/// Convert a floating point value into a string with `precision` decimal places.
/// # Arguments
///
/// * `value` - The floating point value that should be converted.
/// * `precion` - The number of decimal places in the representation.
fn f64_to_rounded_string(value: f64, precision: usize) -> String {
    format!("{:.*}", precision, value,)
}
/// The `Solution` is an individual for using genetic algorithm to approximate functions. It contains
/// the specific function values.
#[derive(Debug, Clone)]
pub struct Solution {
    // Function value for `x`.
    function_values: Vec<f64>,
}

/// Represent the Solution by Displaying `Solution(x-value, y-value, z-value)`.
impl fmt::Display for Solution {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Solution({:?})", self.function_values)
    }
}

/// Compare Solutions by converting the floating points values to a 10 decimal
/// places representation as string - then compare the strings.
impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.function_values.len() == other.function_values.len()
            && (self.function_values.is_empty()
                || self
                    .function_values
                    .iter()
                    .zip(other.function_values.iter())
                    .map(|(self_value, other_value)| {
                        f64_to_floating_point_precision_string(*self_value)
                            == f64_to_floating_point_precision_string(*other_value)
                    })
                    .all(|elem| elem))
    }
}
/// Does not need additional implementation, uses the `eq` function from
/// `PartialEq`.
impl Eq for Solution {}

/// To hash a solution, use the representation chosen designed in `fmt::Display`.
impl Hash for Solution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for single_function_value in &self.function_values {
            f64_to_floating_point_precision_string(*single_function_value).hash(state);
        }
    }
}

impl Solution {
    /// Create a new Solution based on function values x,y and z.
    ///
    /// # Arguments
    ///
    /// * `x` - The value of x that this solution represents.
    /// * `y` - The value of y that this solution represents.
    /// * `z` - The value of z that this solution represents.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solution;
    /// let my_solution = solution::Solution::new(vec![3.0, 4.0, 5.0]);
    /// ```
    pub fn new(function_values: Vec<f64>) -> Self {
        Self { function_values }
    }
    /// Create a random Solution with with values between or equal
    /// `min` .. `max`.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimal value of the function arguments in solution.
    /// * `max` - The maximal value of the function arguments in solution.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solution;
    /// let random_solution = solution::Solution::random(3.0..10.0, 3);
    /// ```
    //fn get_random_elem_from_range<T, R>(range: R) -> Option<T>
    pub fn random<R>(range: R, length: usize) -> Self
    where
        R: SampleRange<f64> + Clone,
    {
        Solution {
            function_values: (0..length)
                .map(|_| match get_random_elem_from_range(range.clone()) {
                    Some(value) => value,
                    // TODO: Don't use panic, but this function should return
                    // a result.
                    None => panic!("Your range is empty!"),
                })
                .collect(),
        }
    }
    /// Return the function arguments stored in a solution.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solution;
    /// let simple_solution = solution::Solution::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(simple_solution.get_arguments(), vec![1.0, 2.0, 3.0])
    /// ```
    pub fn get_arguments(&self) -> Vec<f64> {
        self.function_values.clone()
    }
}
impl<'a> Individual<'a> for Solution {
    // The Distance matrix is needed by the individuals to compute their fitness on.
    type IndividualCost = function::Function;
    /// Mutate the solution by multiplying a random function argument with a factor between
    /// 0.8-1.2
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which on of the function values will mutated.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solution;
    /// use genetic_algorithm_traits::Individual;
    ///
    /// let my_solution = solution::Solution::new(vec![1.0, 2.0, 3.0]);
    /// println!("Solution before mutation: {}, solution after mutation: {}", my_solution, my_solution.clone().mutate(1.0));
    /// ```
    fn mutate(self, prob: f32) -> Self {
        if get_random_elem_from_range(0.0..1.0).unwrap() > prob {
            // With probabilty (1-prop) don't do any mutation.
            self
        } else {
            // Sample a random factor to mutate the solutions with that is not 1.0
            // so that a value is mutated.
            let mut factor_to_mutate = get_random_elem_from_range(0.8..1.2).unwrap();
            while factor_to_mutate == 1.0 {
                factor_to_mutate = get_random_elem_from_range(0.8..1.2).unwrap();
            }
            // Remove mutuability.
            let factor_to_mutate_with = factor_to_mutate;
            // Sample the argument that we want to mutate.
            let idx_to_mutate = get_random_elem_from_range(0..self.function_values.len()).unwrap();
            Solution {
                function_values: self
                    .function_values
                    .iter()
                    .enumerate()
                    .map(|(idx, function_value)| {
                        if idx == idx_to_mutate {
                            function_value * factor_to_mutate_with
                        } else {
                            *function_value
                        }
                    })
                    .collect(),
            }
        }
    }
    /// Crossover one solution with another. For a lack of creativity, this is currently just taking
    /// the average of the two solutions.
    ///
    /// # Arguments
    ///
    /// * `other` - The other Solution you would like to crossover with.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_traits::Individual;
    /// use genetic_algorithm_fn::solution;
    ///
    /// let solution_to_crossover = solution::Solution::new(vec![1.0, 2.0, 3.0]);
    /// let solution_to_crossover_with = solution::Solution::new(vec![3.0, 2.0, 1.0]);
    /// println!("{}", solution_to_crossover.crossover(&solution_to_crossover_with));
    /// ```
    fn crossover(&self, other: &Solution) -> Self {
        if self.function_values.len() != other.get_arguments().len() {
            // TODO: Crossover should return an Option or Result not panic.
            panic!(
                "Cannot crossover a Solution with {} elements when the other solution has {} elements",
                self.function_values.len(),
                other.get_arguments().len()
            );
        }
        Solution {
            function_values: self
                .function_values
                .iter()
                .zip(other.function_values.iter())
                .map(|(self_function_value, other_function_value)| {
                    average(*self_function_value, *other_function_value)
                })
                .collect(),
        }
    }
    /// Compute the fitness of a Solution, that is the specific function value of the `Function`
    /// for the function arguments stored in `Solution`.
    ///
    /// # Arguments
    ///
    /// * `function` - The function that is should be used to compute of the function value of the
    /// solution's arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solution;
    /// use genetic_algorithm_fn::function;
    /// use genetic_algorithm_traits::Individual;
    ///
    /// let function_to_optimize = function::Function::new(
    ///     |x| match x.len() {
    ///         3 => Ok(x[0] * x[1] * x[2]),
    ///         _ => Err(function::FunctionError::WrongNumberOfEntries {
    ///             actual_number_of_entries: x.len(),
    ///             expected_number_of_entries: 3,
    ///         }),
    ///     }
    /// );
    /// let this_solution = solution::Solution::new(vec![2.0, 3.0, 5.0]);
    /// println!("{}", this_solution.fitness(&function_to_optimize));
    /// ```
    ///
    fn fitness(&self, function: &function::Function) -> f64 {
        function
            .get_function_value(self.function_values.clone())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_float_to_round_str {
        use super::*;

        #[test]
        fn no_rounding_fewer_digts() {
            assert_eq!(f64_to_rounded_string(1.57, 3), String::from("1.570"))
        }

        #[test]
        fn no_rounding_same_digts() {
            assert_eq!(f64_to_rounded_string(1.572, 3), String::from("1.572"))
        }

        #[test]
        fn actual_rounding() {
            assert_eq!(f64_to_rounded_string(2.38493, 2), String::from("2.38"))
        }
        #[test]
        fn integration_f64_to_floating_point_precision_string() {
            // The function used is tested anyways, so this is just an
            // integration making sure the function runs through.
            f64_to_floating_point_precision_string(0.1323);
        }
    }

    mod test_solution {
        use super::*;
        use crate::test_objects;
        #[test]
        fn test_constructor() {
            // Ensure the constructor is working.
            Solution::new(vec![1.0, 2.0, 3.0]);
        }
        #[test]
        fn test_display() {
            assert_eq!(
                format!("{}", Solution::new(vec![1.1, 2.2, 3.3])),
                "Solution([1.1, 2.2, 3.3])",
            );
        }
        #[test]
        fn fitness() {
            assert_eq!(
                Solution::new(vec![2.0, 3.0, 5.0]).fitness(&function::Function::new(
                    test_objects::triple_multiplication()
                )),
                30.0
            )
        }
        mod test_average {
            use super::*;
            #[test]
            fn test_int_average() {
                assert_eq!(average(1, 2), 1.5)
            }
            #[test]
            fn test_int_average_same_value() {
                assert_eq!(average(0, 0), 0.0)
            }
            #[test]
            fn test_float_average() {
                assert_eq!(average(1.0, 2.0), 1.5)
            }
            #[test]
            fn test_float_average_same_value() {
                assert_eq!(average(0.0, 0.0), 0.0)
            }
        }
        mod test_equality {
            use super::*;
            #[test]
            fn equal_solution_no_record() {
                assert!(Solution::new(Vec::<f64>::new()) == Solution::new(Vec::<f64>::new()));
            }

            #[test]
            fn equal_solutions() {
                assert!(Solution::new(vec![1.0, 2.0, 3.0]) == Solution::new(vec![1.0, 2.0, 3.0]));
            }
            #[test]
            fn non_equal_solutions() {
                assert!(
                    !(Solution::new(vec![1.0, 3.0, 3.0]) == Solution::new(vec![1.0, 2.0, 3.0]))
                );
            }
            #[test]
            fn non_equal_before_rounding() {
                assert!(
                    Solution::new(vec![1.00000000001, 2.0, 3.0])
                        == Solution::new(vec![1.0, 2.0, 3.0])
                );
            }
            #[test]
            fn non_equal_before_and_after_rounding() {
                assert!(
                    !(Solution::new(vec![1.0000000001, 2.0, 3.0])
                        == Solution::new(vec![1.0, 2.0, 3.0]))
                );
            }
            #[test]
            fn non_equal_solutions_different_length() {
                assert!(!(Solution::new(vec![1.0000000001]) == Solution::new(vec![1.0, 2.0, 3.0])));
            }
        }
        mod get_random_elem_from_range {
            use super::*;
            #[test]
            fn sample_int_range() {
                get_random_elem_from_range(0..10);
            }
            #[test]
            fn sample_float_range() {
                get_random_elem_from_range(0.0..1.0);
            }
            #[test]
            fn sample_empty_range() {
                assert_eq!(get_random_elem_from_range(0..0), None);
            }
        }
        mod test_hash {
            use super::*;
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            fn _create_hash(solution: Solution) -> u64 {
                let mut s = DefaultHasher::new();
                solution.hash(&mut s);
                s.finish()
            }
            #[test]
            fn hash_same_solution() {
                assert!(
                    _create_hash(Solution::new(vec![1.0, 2.0, 3.0]))
                        == _create_hash(Solution::new(vec![1.0, 2.0, 3.0]))
                );
            }
            #[test]
            fn hash_different_solution() {
                assert!(
                    !(_create_hash(Solution::new(vec![1.0, 3.0, 3.0]))
                        == _create_hash(Solution::new(vec![1.0, 2.0, 3.0])))
                );
            }
            #[test]
            fn hash_different_solution_but_rounding_makes_them_similiar() {
                assert!(
                    _create_hash(Solution::new(vec![1.00000000001, 2.0, 3.0]))
                        == _create_hash(Solution::new(vec![1.0, 2.0, 3.0]))
                );
            }
            #[test]
            fn hash_solutions_different_length() {
                assert!(
                    !(_create_hash(Solution::new(vec![1.00000000001, 2.0]))
                        == _create_hash(Solution::new(vec![1.0, 2.0, 3.0])))
                );
            }
        }
        mod test_mutate {
            use super::*;
            #[test]
            fn no_mutation_applied() {
                assert_eq!(
                    Solution::new(vec![1.0, 2.0, 3.0]).mutate(0.0),
                    Solution::new(vec![1.0, 2.0, 3.0])
                )
            }
            // Run the following test a few times.
            #[test]
            #[test]
            #[test]
            #[test]
            #[test]
            #[test]
            fn mutation_applied() {
                let original_solution = Solution::new(vec![1.0, 2.0, 3.0]);
                let mutated_solution = original_solution.clone().mutate(1.0);
                // original solution and mutated_solution should be different for exactly
                // one function paramter.
                let original_parameters = original_solution.get_arguments();
                let mutated_parameters = mutated_solution.get_arguments();
                assert_eq!(
                    original_parameters
                        .iter()
                        .zip(mutated_parameters.iter())
                        .map(
                            |(original_parameter, mutated_parameter)| (*original_parameter
                                == *mutated_parameter)
                                as usize
                        )
                        .sum::<usize>(),
                    2
                )
            }
        }
        mod test_crossover {
            use super::*;
            #[test]
            fn same_inidividual_result_in_same_individual() {
                let solution = Solution::new(vec![1.0, 4.0, 7.0]);
                assert_eq!(solution.crossover(&solution.clone()), solution);
            }
            #[test]
            fn average_correctly_applied() {
                let solution_to_crossover = Solution::new(vec![12.0, 3.0, 9.0]);
                let solution_to_crossover_with = Solution::new(vec![7.0, 6.0, 13.0]);
                assert_eq!(
                    solution_to_crossover.crossover(&solution_to_crossover_with),
                    Solution::new(vec![9.5, 4.5, 11.0])
                );
            }
            #[test]
            #[should_panic]
            fn crossover_solution_different_length() {
                let solution_to_crossover = Solution::new(vec![12.0, 3.0]);
                let solution_to_crossover_with = Solution::new(vec![7.0, 6.0, 13.0]);
                assert_eq!(
                    solution_to_crossover.crossover(&solution_to_crossover_with),
                    Solution::new(vec![9.5, 4.5, 11.0])
                );
            }
        }
        mod test_fitness {
            use super::*;
            #[test]
            fn simple_test() {
                let function_to_maximize =
                    function::Function::new(test_objects::triple_multiplication());
                let solution = Solution::new(vec![1.0, 4.0, 7.0]);
                assert_eq!(solution.fitness(&function_to_maximize), 28.0);
            }
        }
    }
}
