use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(
            Update,
            (update_people, greet_people, list_people_with_jobs).chain(),
        );
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        Name("Elaina Proctor".to_string()),
        Employed { job: Job::Doctor },
    ));
    commands.spawn((
        Person,
        Name("Renzo Hume".to_string()),
        Employed { job: Job::Engineer },
    ));
    commands.spawn((
        Person,
        Name("Zayna Nieves".to_string()),
        Employed { job: Job::Artist },
    ));
    commands.spawn((
        Person,
        Name("Corynn Shepherd".to_string()),
        Employed { job: Job::Musician },
    ));
}

fn list_people_with_jobs(query: Query<(&Name, &Employed), With<Person>>) {
    for (name, employed) in &query {
        println!("{} is a {:?}", name.0, employed.job);
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    Lawyer,
    Engineer,
    Teacher,
    Artist,
    Musician,
}
