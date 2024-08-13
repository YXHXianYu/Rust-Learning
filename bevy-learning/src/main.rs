use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

// Plugins

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

// Resource

#[derive(Resource)]
struct GreetTimer(Timer);

// Components

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Systems

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Carol".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>, timer: ResMut<GreetTimer>) {
    if timer.0.just_finished() {
        for name in &query {
            println!("hello {}", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>, time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut name in &mut query {
            if &name.0[0..5] == "Alice" {
                name.0.push_str(" Romero");
                break;
            }
        }
    }
}
