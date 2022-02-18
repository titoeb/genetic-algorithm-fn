/// The inner part of the 3d hartman function that is loop over for
/// each compution
fn hartman_inner_function(idx_i: usize, x: f64, y: f64, z: f64) -> f64 {
    // A matrix, represented by the columns always use with on dimension of `x`
    let a_x = [3.0, 0.1, 3.0, 0.1];
    let a_y = [10.0, 10.0, 10.0, 10.0];
    let a_z = [30.0, 35.0, 30.0, 35.0];
    // P matrix, represented by the columns always use with on dimension of `x`
    let p_x = [0.3689, 0.4699, 0.1091, 0.0381];
    let p_y = [0.1170, 0.4387, 0.8732, 0.5743];
    let p_z = [0.2673, 0.7470, 0.5547, 0.8828];
    // final computation
    -(a_x[idx_i] * (x - p_x[idx_i]).powi(2)
        + a_y[idx_i] * (y - p_y[idx_i]).powi(2)
        + a_z[idx_i] * (z - p_z[idx_i]).powi(2))
}
/// The hartman function taken from https://www.sfu.ca/~ssurjano/hart3.html
pub fn hartman_3_dimensional(x: f64, y: f64, z: f64) -> f64 {
    -(1.0 * hartman_inner_function(0, x, y, z).exp()
        + 1.2 * hartman_inner_function(1, x, y, z).exp()
        + 3.0 * hartman_inner_function(2, x, y, z).exp()
        + 3.2 * hartman_inner_function(3, x, y, z).exp())
}

mod tests {
    use super::*;
    mod test_full_function {
        use super::*;
        #[test]
        fn origin() {
            assert_eq!(hartman_3_dimensional(0.0, 0.0, 0.0), -0.06797411659013469)
        }
        #[test]
        fn global_minima() {
            assert_eq!(
                hartman_3_dimensional(0.114614, 0.555649, 0.852547),
                -3.8627797869493365
            )
        }
    }
    mod test_inner_function {
        use super::*;
        #[test]
        fn origin_iteration_0() {
            assert_eq!(
                hartman_inner_function(0, 0.114614, 0.555649, 0.852547),
                -12.393535091668001
            )
        }
        #[test]
        fn origin_iteration_1() {
            assert_eq!(
                hartman_inner_function(1, 0.114614, 0.555649, 0.852547),
                -0.5392994225046004
            )
        }
        #[test]
        fn origin_iteration_2() {
            assert_eq!(
                hartman_inner_function(2, 0.114614, 0.555649, 0.852547),
                -3.6698626508680015
            )
        }
        #[test]
        fn origin_iteration_3() {
            assert_eq!(
                hartman_inner_function(3, 0.114614, 0.555649, 0.852547),
                -0.036097577544599975
            )
        }
    }
}
