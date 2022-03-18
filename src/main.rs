mod api;

use anyhow::Result;
use api::problems::get_problems;

#[tokio::main]
async fn main() -> Result<()> {
    let problems = get_problems().await?;
    for problem in problems {
        println!(
            "{} [{}]: {}",
            problem.problem_id, problem.title, problem.difficulty
        );
    }

    Ok(())
}
