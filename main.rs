use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use rand;

fn main() {
    println!("Hello");
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(Scoreboard { score: 0 })
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(check_for_death.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
    let mob_material = materials.add(Color::rgb(1.0, 0.5, 0.5).into());
    for _i in 1..10 {
        let rand_position = Vec3::new(
            rand::random::<f32>() * 200.0 - 100.0,
            rand::random::<f32>() * 200.0 - 100.0,
            0.0
        );
        commands.spawn((
            SpriteBundle {
                material: mob_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: Transform::from_translation(rand_position),
                ..Default::default()
            },
        ))
        .with(Heading {
            rot: 0.0,
        })
        .with(Alive {
            health: 100,
        })
        .with(Mob {});
    }
    commands.spawn((
        SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
    ))
    .with(Heading {
        rot: 0.0,
    })
    .with(Alive {
        health: 100,
    })
    .with(Player {});
}

fn check_for_death(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Handle<ColorMaterial>, &Alive)>
) {
    println!("test");
    for (e, material, alive) in query.iter_mut() {
        println!("{:?}", alive.health);
        if (alive.health <= 0) {
            materials.get_mut(material).map(|m| {
                m.color = Color::rgb(0.1, 0.1, 0.1).into();
            });
            commands.remove_one::<Alive>(e);
        }
    }
}

struct Scoreboard {
    score: u32,
}

struct Heading {
    rot: f32,
}

struct Alive {
    health: u32,
}

struct Mob {}

struct Player {}
