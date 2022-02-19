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
    x: f64,
    // Function value for `y`.
    y: f64,
    // Function value for `z`.
    z: f64,
}

/// Represent the Solution by Displaying `Solution(x-value, y-value, z-value)`.
impl fmt::Display for Solution {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Solution({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Compare Solutions by converting the floating points values to a 10 decimal
/// places representation as string - then compare the strings.
impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        f64_to_floating_point_precision_string(self.x)
            == f64_to_floating_point_precision_string(other.x)
            && f64_to_floating_point_precision_string(self.y)
                == f64_to_floating_point_precision_string(other.y)
            && f64_to_floating_point_precision_string(self.z)
                == f64_to_floating_point_precision_string(other.z)
    }
}
/// Does not need additional implementation, uses the `eq` function from
/// `PartialEq`.
impl Eq for Solution {}

/// To hash a solution, use the representation chosen designed in `fmt::Display`.
impl Hash for Solution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        f64_to_floating_point_precision_string(self.x).hash(state);
        f64_to_floating_point_precision_string(self.y).hash(state);
        f64_to_floating_point_precision_string(self.z).hash(state);
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
    /// let my_solution = solution::Solution::new(3.0, 4.0, 5.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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
    /// let random_solution = solution::Solution::random(3.0..10.0);
    /// ```
    //fn get_random_elem_from_range<T, R>(range: R) -> Option<T>
    pub fn random<R>(range: R) -> Self
    where
        R: SampleRange<f64> + Clone,
    {
        let x = match get_random_elem_from_range(range.clone()) {
            Some(value) => value,
            None => panic!("Your range is empty!"),
        };
        let y = match get_random_elem_from_range(range.clone()) {
            Some(value) => value,
            None => panic!("Your range is empty!"),
        };
        let z = match get_random_elem_from_range(range.clone()) {
            Some(value) => value,
            None => panic!("Your range is empty!"),
        };
        Solution { x: x, y: y, z: z }
    }
    /// Return the function arguments stored in a solution.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solution;
    /// let simple_solution = solution::Solution::new(1.0, 2.0, 3.0);
    /// assert_eq!(simple_solution.get_arguments(), (1.0, 2.0, 3.0))
    /// ```
    pub fn get_arguments(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
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
    /// let my_solution = solution::Solution::new(1.0, 2.0, 3.0);
    /// println!("Solution before mutation: {}, solution after mutation: {}", my_solution, my_solution.clone().mutate(1.0));
    /// ```
    fn mutate(self, prob: f32) -> Self {
        if get_random_elem_from_range(0.0..1.0).unwrap() > prob {
            // With probabilty (1-prop) don't do any mutation.
            return self;
        } else {
            // Sample a random factor to mutate the solutions with that is not 1.0
            // so that a value is mutated.
            let mut factor_to_mutate = get_random_elem_from_range(0.8..1.2).unwrap();
            while factor_to_mutate == 1.0 {
                factor_to_mutate = get_random_elem_from_range(0.8..1.2).unwrap();
            }
            // Remove mutuability.
            let factor_to_mutate = factor_to_mutate;
            // Sample the argument that we want to mutate.
            match get_random_elem_from_range(0..3).unwrap() {
                0 => Solution::new(self.x * factor_to_mutate, self.y, self.z),
                1 => Solution::new(self.x, self.y * factor_to_mutate, self.z),
                2 => Solution::new(self.x, self.y, self.z * factor_to_mutate),
                unexpected_value => {
                    panic!(
                        "When sampling on a range from 0-2, we cannot get {}.",
                        unexpected_value
                    );
                }
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
    /// let solution_to_crossover = solution::Solution::new(1.0, 2.0, 3.0);
    /// let solution_to_crossover_with = solution::Solution::new(3.0, 2.0, 1.0);
    /// println!("{}", solution_to_crossover.crossover(&solution_to_crossover_with));
    /// ```
    fn crossover(&self, other: &Solution) -> Self {
        Solution {
            x: average(self.x, other.x),
            y: average(self.y, other.y),
            z: average(self.z, other.z),
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
    /// let my_function = function::Function::new(|(x, y, z)| { x * y * z });
    /// let this_solution = solution::Solution::new(2.0, 3.0, 5.0);
    /// println!("{}", this_solution.fitness(&my_function));
    /// ```
    ///
    fn fitness(&self, function: &function::Function) -> f64 {
        function.get_function_value(self.get_arguments())
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

        #[test]
        fn test_constructor() {
            // Ensure the constructor is working.
            Solution::new(1.0, 2.0, 3.0);
        }
        #[test]
        fn test_display() {
            assert_eq!(
                format!("{}", Solution::new(1.1, 2.2, 3.3)),
                "Solution(1.1, 2.2, 3.3)",
            );
        }
        #[test]
        fn fitness() {
            assert_eq!(
                Solution::new(2.0, 3.0, 5.0)
                    .fitness(&function::Function::new(|(x, y, z)| { x * y * z })),
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
            fn equal_solutions() {
                assert!(Solution::new(1.0, 2.0, 3.0) == Solution::new(1.0, 2.0, 3.0));
            }
            #[test]
            fn non_equal_solutions() {
                assert!(!(Solution::new(1.0, 3.0, 3.0) == Solution::new(1.0, 2.0, 3.0)));
            }
            #[test]
            fn non_equal_before_rounding() {
                assert!(Solution::new(1.00000000001, 2.0, 3.0) == Solution::new(1.0, 2.0, 3.0));
            }
            #[test]
            fn non_equal_before_and_after_rounding() {
                assert!(!(Solution::new(1.0000000001, 2.0, 3.0) == Solution::new(1.0, 2.0, 3.0)));
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
                    _create_hash(Solution::new(1.0, 2.0, 3.0))
                        == _create_hash(Solution::new(1.0, 2.0, 3.0))
                );
            }
            #[test]
            fn hash_different_solution() {
                assert!(
                    !(_create_hash(Solution::new(1.0, 3.0, 3.0))
                        == _create_hash(Solution::new(1.0, 2.0, 3.0)))
                );
            }
            #[test]
            fn hash_different_solution_but_rounding_makes_them_similiar() {
                assert!(
                    _create_hash(Solution::new(1.00000000001, 2.0, 3.0))
                        == _create_hash(Solution::new(1.0, 2.0, 3.0))
                );
            }
        }
        mod test_mutate {
            use super::*;
            #[test]
            fn no_mutation_applied() {
                assert_eq!(
                    Solution::new(1.0, 2.0, 3.0).mutate(0.0),
                    Solution::new(1.0, 2.0, 3.0)
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
                let original_solution = Solution::new(1.0, 2.0, 3.0);
                let mutated_solution = original_solution.clone().mutate(1.0);
                // original solution and mutated_solution should be different for exactly
                // one function paramter.
                let original_parameters = original_solution.get_arguments();
                let mutated_parameters = mutated_solution.get_arguments();
                assert_eq!(
                    (original_parameters.0 == mutated_parameters.0) as usize
                        + (original_parameters.1 == mutated_parameters.1) as usize
                        + (original_parameters.2 == mutated_parameters.2) as usize,
                    2
                )
            }
        }
        mod test_crossover {
            use super::*;
            #[test]
            fn same_inidividual_result_in_same_individual() {
                let solution = Solution::new(1.0, 4.0, 7.0);
                assert_eq!(solution.crossover(&solution.clone()), solution);
            }
            #[test]
            fn average_correctly_applied() {
                let solution_to_crossover = Solution::new(12.0, 3.0, 9.0);
                let solution_to_crossover_with = Solution::new(7.0, 6.0, 13.0);
                assert_eq!(
                    solution_to_crossover.crossover(&solution_to_crossover_with),
                    Solution::new(9.5, 4.5, 11.0)
                );
            }
        }
        mod test_fitness {
            use super::*;
            #[test]
            fn simple_test() {
                let function_to_maximize = function::Function::new(|(x, y, z)| x * y * z);
                let solution = Solution::new(1.0, 4.0, 7.0);
                assert_eq!(solution.fitness(&function_to_maximize), 28.0);
            }
        }
    }
}
