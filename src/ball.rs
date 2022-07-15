use bevy::math::Vec3;
use std::time::Duration;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use std::thread::sleep;


#[derive(Component)]
struct Ball;

// struct SpawnTimer(Timer);

/// Spawns the pong ball
// fn spawn_ball(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
// fn draw_ball(mut commands: Commands, time: Res<Time>) {
fn spawn_ball(mut commands: Commands) {
    // if !timer.0.tick(time.delta()).just_finished() {
    //     return;
    // }

    // commands.remove_resource::<ShapeBundle>();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 5.,
            ..Default::default()
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 1.),
        },
        // Transform {
        //     translation: Vec3::new(time.seconds_since_startup() as f32 * 10., 0., 0.),
        //     ..Default::default()
        // },
        // Transform::from_translation(Vec3::new(-400., 200., 1.))
        Transform::default()
    )).insert(Ball);
}

// Moves the ball forward
// fn move_ball(time: Res<Time>, mut query: Query<&mut Transform>) {
//     for mut transform in query.iter_mut() {
//         transform.translation.x -= time.delta_seconds() * 10.;
//     }
// }
fn move_ball(mut query: Query<(&Ball, &mut Transform)>, keyboard: Res<Input<KeyCode>>) {
    // for (_player, mut transform) in query.iter_mut() {
    //     // transform.translation.x += time.delta_seconds() * 200.;
    //     transform.translation.x -= time.delta_seconds() * 10.;
    //     transform.translation.y += time.delta_seconds() * 10.;
    //     // println!("Updated transform: {}", transform.translation)
    // }

    let (_ball, mut transform) = query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 0.5;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 0.5;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 0.5;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += 0.5;
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
            // .add_system(draw_ball)
            .add_startup_system(spawn_ball)
            .add_system(move_ball);
    }
}
