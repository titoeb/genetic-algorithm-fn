use crate::function;
pub fn triple_multiplication() -> fn(Vec<f64>) -> Result<f64, function::FunctionError> {
    |x| match x.len() {
        3 => Ok(x[0] * x[1] * x[2]),
        _ => Err(function::FunctionError::WrongNumberOfEntries {
            actual_number_of_entries: x.len(),
            expected_number_of_entries: 3,
        }),
    }
}
