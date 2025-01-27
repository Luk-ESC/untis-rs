///
/// This example shows a basic usecase of searching for schools.
///

#[tokio::main]
async fn main() -> Result<(), untis::Error> {
    let schools = untis::schools::search("query").await?;

    for school in schools {
        println!("name: {}, server: {}", school.display_name, school.server);
    }

    Ok(())
}
