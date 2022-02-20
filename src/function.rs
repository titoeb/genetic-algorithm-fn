use std::fmt;
/// Custom error that can occur with the Function class defined below.
#[derive(Debug, PartialEq)]
pub enum FunctionError {
    /// The Function could not compute the function value because the
    /// Vector provided does not have the right number of arguments.
    WrongNumberOfEntries {
        /// Expected number of arguments.
        expected_number_of_entries: usize,
        /// Actual number of arguments.
        actual_number_of_entries: usize,
    },
}
impl fmt::Display for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionError::WrongNumberOfEntries {
                expected_number_of_entries,
                actual_number_of_entries,
            } => write!(
                f,
                "Expected {} entries, but got {}",
                expected_number_of_entries, actual_number_of_entries
            ),
        }
    }
}

/// A representation of a f64 based distance matrix.
#[derive(Debug)]
pub struct Function {
    fun: fn(Vec<f64>) -> Result<f64, FunctionError>,
}

impl Function {
    /// Create a new function.
    ///
    /// # Arguments
    ///
    /// * `fun` - The function that should be computed in this struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::function;
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
    ///
    /// ```
    pub fn new(fun: fn(Vec<f64>) -> Result<f64, FunctionError>) -> Self {
        Function { fun }
    }
    /// Compute the function value for a Solution.
    ///
    /// # Arguments
    ///
    /// * `solution` - The solution for which the function value should be computed.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::function;
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
    /// println!("{}", function_to_optimize.get_function_value(vec![3.0, 4.0, 5.0]).unwrap());
    ///
    /// ```
    pub fn get_function_value(&self, function_values: Vec<f64>) -> Result<f64, FunctionError> {
        (self.fun)(function_values)
    }
}

#[cfg(test)]
mod test_distance_mat {
    use super::*;
    use crate::test_objects;
    #[test]
    fn test_constructor() {
        let _ = Function::new(|x| match x.len() {
            3 => Ok(x[0] * x[1] * x[2]),
            _ => Err(FunctionError::WrongNumberOfEntries {
                actual_number_of_entries: x.len(),
                expected_number_of_entries: 3,
            }),
        });
    }
    #[test]
    fn test_simple_computation() {
        let my_func = Function::new(test_objects::triple_multiplication());

        assert_eq!(my_func.get_function_value(vec![1.0, 2.0, 3.0]), Ok(6.0));
    }
    #[test]
    fn test_simple_computation_wrong_arguments() {
        let my_func = Function::new(test_objects::triple_multiplication());
        assert_eq!(
            my_func.get_function_value(vec![1.0, 2.0]),
            Err(FunctionError::WrongNumberOfEntries {
                expected_number_of_entries: 3,
                actual_number_of_entries: 2
            })
        );
    }
}
