use anyhow::Result;
use atcoder_bingo_backend::crawler::problems::{get_problems, Problem};
use rand::prelude::SliceRandom;

pub const BINGO_SIZE: usize = 3;
const DIFF_DISTR: [(i32, i32); 5] = [
    (-10000, 600),
    (400, 1400),
    (1200, 2200),
    (2000, 3000),
    (2800, 10000),
];

#[tokio::main]
async fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    // Fetch problems and sort by difficulties.
    let mut problems = get_problems().await?;
    problems.sort_by_key(|problem| problem.difficulty);

    let mut bingos = Vec::with_capacity(DIFF_DISTR.len());
    for (lower_diff, upper_diff) in DIFF_DISTR {
        // The minimum index whose difficulty is no less than `lower_diff`.
        // Double the difficulties so that any problems doesn't match and we can detect the precise border.
        // Problems whose difficulty is in [lower_diff, upper_diff) is in [lower_index, upper_index).
        let lower_index = problems
            .binary_search_by_key(&(lower_diff * 2 + 1), |problem| problem.difficulty * 2)
            .unwrap_or_else(|i| i);
        let upper_index = problems
            .binary_search_by_key(&(upper_diff * 2 + 1), |problem| problem.difficulty * 2)
            .unwrap_or_else(|i| i);

        // choose problems from [lower_index, upper_index) randomly.
        let mut indices: Vec<usize> = (lower_index..upper_index).collect();
        let (chosen_indices, _) = indices.partial_shuffle(&mut rng, BINGO_SIZE * BINGO_SIZE);
        let bingo: Vec<Problem> = chosen_indices
            .iter_mut()
            .map(|index| problems[*index].clone())
            .collect();
        bingos.push(bingo);
    }

    for bingo in &bingos {
        for problem in bingo {
            println!("{} {}: {}", problem.problem_id, problem.title, problem.difficulty);
        }
        println!()
    }
    Ok(())
}
