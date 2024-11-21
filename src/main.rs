use self::github::context::GithubContext;

mod github;

fn main() -> anyhow::Result<()> {

    dotenv::dotenv().ok();

    dbg!(std::env::var("CI"));

    let fruit = github::command::get_input("fruit")?;

    let context = GithubContext::new();

    println!("{context:#?}");

    println!("Hello, {fruit}!");

    debug!("Fruit was {fruit}");
    notice!("Fruit was {fruit}");
    warn!("Fruit was {fruit}");
    error!("Fruit was {fruit}");

    Ok(())
}
