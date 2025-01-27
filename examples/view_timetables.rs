///
/// This example shows how you can access a list of your school's teachers and how
/// you can view the timetables of other people/assets (i.e. rooms). Note that your
/// school likely has restrictions in place for accessing other timetables.
///

#[tokio::main]
async fn main() -> Result<(), untis::Error> {
    // Log in by specifying the school's details and credentials manually.
    let mut client =
        untis::Client::login("server.webuntis.com", "SchoolName", "username", "password").await?;

    // Retrieve a list of teachers at the user's school.
    let teachers = client.teachers().await?;

    for teacher in teachers {
        // Retrieve the teacher's timetable for the current week.
        let mut timetable = client
            .timetable_current_week(&teacher.id, &untis::ElementType::Teacher)
            .await?;

        timetable.sort_unstable_by_key(|lesson| (lesson.date, lesson.start_time));

        println!(
            "{} {}'s schedule this week:",
            teacher.first_name, teacher.last_name
        );

        for lesson in timetable {
            println!(
                "{}, {}-{}",
                weekday(lesson.date),
                *lesson.start_time,
                *lesson.end_time
            )
        }

        println!();
    }

    Ok(())
}

fn weekday(date: untis::Date) -> String {
    date.format("%A").to_string()
}
