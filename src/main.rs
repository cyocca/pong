// mod ui;
mod ball;
use ball::BallPlugin;
// use ui::run;

// fn main() {
//     run();
// }

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    // App::new()
    //     .insert_resource(Msaa { samples: 4 })
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(ShapePlugin)
    //     .add_startup_system(setup_system)
    //     .run();

    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(setup)
        .run();
}

// fn setup_system(mut commands: Commands) {
//     let shape = shapes::RegularPolygon {
//         sides: 6,
//         feature: shapes::Circle
//         ..shapes::RegularPolygon::default()
//     };

//     let mut transform = Transform::default();
//     transform.translation.x = 5.0;
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     commands.spawn_bundle(GeometryBuilder::build_as(
//         &shape,
//         DrawMode::Outlined {
//             fill_mode: FillMode::color(Color::CYAN),
//             outline_mode: StrokeMode::new(Color::BLACK, 10.0),
//         },
//         transform,
//     ));
// }

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
