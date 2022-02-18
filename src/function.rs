/// A representation of a f64 based distance matrix.
#[derive(Debug)]
pub struct Function {
    fun: fn((f64, f64, f64)) -> f64,
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
    /// let my_func = function::Function::new(|(x, y, z)| {x * y * z});
    ///
    /// ```
    pub fn new(fun: fn((f64, f64, f64)) -> f64) -> Self {
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
    /// let my_func = function::Function::new(|(x, y, z)| {x * y * z});
    /// println!("{}", my_func.get_function_value((3.0, 4.0, 5.0)));
    ///
    /// ```
    pub fn get_function_value(&self, function_values: (f64, f64, f64)) -> f64 {
        (self.fun)(function_values)
    }
}

#[cfg(test)]
mod test_distance_mat {
    use super::*;
    #[test]
    fn test_constructor() {
        let _ = Function::new(|(x, y, z)| x * y * z);
    }
    #[test]
    fn test_simple_computation() {
        let my_func = Function::new(|(x, y, z)| x * y * z);
        assert_eq!(my_func.get_function_value((1.0, 2.0, 3.0)), 6.0);
    }
}
