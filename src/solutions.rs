use crate::function::Function;
use crate::solution::Solution;
use genetic_algorithm_traits::{Individual, Population};
use rand::distributions::uniform::SampleRange;
use std::fmt;

use crossbeam_utils::thread;
use std::collections::HashSet;
use std::convert::From;
use std::time::Instant;

/// The `Solution` is the container for your current pool of `solution`'s.
#[derive(Debug, Clone, PartialEq)]
pub struct Solutions {
    /// The unique solutions that currently exist.
    solutions: HashSet<Solution>,
}
// Convert a Vector of solution's to a `Solutions`-object.
impl From<Vec<Solution>> for Solutions {
    /// Create a new Population from a vector of solutions.
    ///
    /// # Arguments
    ///
    /// * `solutions` - The solutions you collected so far and would like to put into your
    /// Solutions.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solutions;
    /// use genetic_algorithm_fn::solution;
    ///
    /// let my_solutions = solutions::Solutions::from(vec![
    ///     solution::Solution::new(1.0, 2.0, 3.0),
    ///     solution::Solution::new(1.0, 2.0, 4.0)
    /// ]);
    /// println!("Current solutions: {}", my_solutions);
    /// ```
    fn from(solution: Vec<Solution>) -> Self {
        Solutions {
            solutions: solution.into_iter().collect(),
        }
    }
}

/// Represent the Solution by displaying `Solutions([solution-1, solution-2]).
impl fmt::Display for Solutions {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Solutions([\n\t{}\n])",
            self.solutions
                .iter()
                .map(|solution| format!("{}", solution.to_string()))
                .collect::<Vec<String>>()
                .join(",\n\t")
        )
    }
}

impl Solutions {
    /// Create a pool of random solutions.
    ///
    /// # Arguments
    ///
    /// * `n_solutions` - The number of solutions your population should contain.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solutions;
    /// println!("{}", solutions::Solutions::random(5, 1.0..10.0));
    /// ```
    pub fn random<R>(n_solutions: usize, range: R) -> Self
    where
        R: SampleRange<f64> + Clone,
    {
        let mut routes = HashSet::new();

        while routes.len() < n_solutions {
            routes.insert(Solution::random(range.clone()));
        }

        Solutions { solutions: routes }
    }
}

impl<'a> Population<'a> for Solutions {
    type Individual = Solution;
    type IndividualCollection = std::collections::hash_set::Iter<'a, Solution>;

    /// Given your pool, compute the fitness of your individuals to solve the
    /// problem at hand.
    ///
    /// # Arguments
    ///
    /// * `n` - How many individuals to keep?
    /// * `function` - The distances between nodes that is neccessary to computes how well the route
    /// work in terms of the Function to maximize.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solutions;
    /// use genetic_algorithm_fn::function;
    /// use genetic_algorithm_traits::Population;
    ///
    /// let function_to_optimize = function::Function::new(|(x,y,z)| {x*y*z});
    /// let all_solutions = solutions::Solutions::random(30, 1.0..10.0);
    /// println!("Best 5 solutions: {}", all_solutions.get_fittest_population(5, &function_to_optimize));
    /// ```
    fn get_fittest_population(&self, n: usize, function: &Function) -> Solutions {
        Solutions::from(self.get_n_fittest(n, function))
    }
    /// Evolve your population.
    ///
    /// The evolution process consists of the following stages:
    /// 1) `crossover` between all 1,...,n solutions excluding the solution itself.
    /// 2) `mutate` is applied to all individuals.
    ///
    /// # Arguments
    ///
    /// * `mutate_prob` - The probabilty of an inviduals beeing mutated. Is applied via `individuals.mutate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solutions;
    /// use genetic_algorithm_traits::Population;
    ///
    /// let all_solutions = solutions::Solutions::random(2, 1.0..10.0);
    /// println!("The evolved invdividuals are {}", all_solutions.evolve(0.5));
    ///
    /// ```
    fn evolve(&self, mutate_prob: f32) -> Solutions {
        Solutions {
            solutions: HashSet::from_iter(self.evolve_individuals(mutate_prob).into_iter()),
        }
    }
    /// Iterate over the individuals of your population.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_fn::solutions;
    /// use genetic_algorithm_traits::Population;
    ///
    /// let all_solutions = solutions::Solutions::random(5, 1.0..10.0);
    /// all_solutions.iter().map(|solution| println!("{}", solution));
    /// ```
    fn iter(&'a self) -> std::collections::hash_set::Iter<Solution> {
        self.solutions.iter()
    }
}

/// Given an initial population evolve it for `n_generations` while keeping `size_generation`
/// individuals. The final population will be returned.
///
/// # Arguments
///
/// * `initial_population` - Your initial population that should be evolved.
/// * `n_generations` - How many times should your population be evolved?
/// * `size_generation` - How many individuals should be kept after evolving it.
/// * `distance_matrix` - The distance matrix on which the fitness will be computed on.
///
pub fn evolve_population(
    initial_population: Solutions,
    n_generations: usize,
    size_generation: usize,
    function: &Function,
    n_jobs: usize,
) -> Solutions {
    if n_jobs == 0 {
        // single-thread
        (0..n_generations).fold(initial_population, |pop, _| {
            pop.evolve(0.5)
                .get_fittest_population(size_generation, function)
        })
    } else {
        // multi-threaded execution
        thread::scope(|s| {
            let mut result = Vec::new();
            // Schedule the threads.
            for _ in 0..n_jobs {
                let this_population = initial_population.clone();
                result.push(s.spawn(move |_| -> Vec<Solution> {
                    (0..((n_generations / n_jobs) + 1))
                        .fold(this_population, |pop, _| {
                            pop.evolve(0.5)
                                .get_fittest_population(size_generation, function)
                        })
                        .get_n_fittest(size_generation, function)
                }))
            }
            // Collect the results from the tread-handles.
            Solutions::from(
                result
                    .into_iter()
                    .map(|thread| thread.join().unwrap())
                    .flatten()
                    .collect::<Vec<Solution>>(),
            )
        })
        .unwrap()
    }
}
/// Compute the time in milliseconds that it takes for a genetic algorithm to run.
///
/// # Arguments
///
/// * `n_generations` - How many generations should the algorithm evolve?
/// * `size_generation` - How many individuals should be selected at the end of each
/// evolution step.
/// * `dist_mat` - What is the distance matrix for your TSP.
///
/// ```
pub fn benchmark_population<R>(
    n_generations: usize,
    size_generation: usize,
    function: &Function,
    n_jobs: usize,
    sample_range: R,
) -> (u64, f64)
where
    R: SampleRange<f64> + Clone,
{
    // End-to-end test: does the error of the route get down?
    let before = Instant::now();
    let final_population = evolve_population(
        Solutions::random(size_generation, sample_range),
        n_generations,
        size_generation,
        function,
        n_jobs,
    );
    let duration = before.elapsed();
    let nanos = duration.subsec_nanos() as u64;
    (
        (1000 * 1000 * 1000 * duration.as_secs() + nanos) / (1000 * 1000),
        final_population.get_n_fittest(1, function)[0].fitness(function),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution;

    #[test]
    fn test_formatter() {
        assert_eq!(
            format!(
                "{}",
                Solutions::from(vec![solution::Solution::new(1.1, 2.2, 3.3),])
            ),
            "Solutions([\n\tSolution(1.1, 2.2, 3.3)\n])"
        )
    }
}
