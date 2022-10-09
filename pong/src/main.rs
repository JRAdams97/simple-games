use bevy::{
    prelude::*,
    time::FixedTimestep,
    window::PresentMode,
};
use bevy_prototype_lyon::prelude::*;

// Game constants
const BACKGROUND_COLOR: Color = Color::BLACK;
const DT: f32 = 1.0 / 60.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

// Entity constants
const PADDLE_SIZE: Vec3 = Vec3::new(8.0, 32.0, 0.0);
const PADDLE_SPD: f32 = 400.0;
const BALL_SIZE: Vec3 = Vec3::new(8.0, 8.0, 0.0);

#[derive(Component)]
struct P1;

#[derive(Component)]
struct P2;

#[derive(Component)]
struct Ball;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Active
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    #[bundle]
    spatial_bundle: SpatialBundle,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Pong!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state(GameState::Active)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::on_update(GameState::Active)
                .with_run_criteria(FixedTimestep::step(DT as f64))
                .with_system(p1_movement_system)
                .with_system(p2_movement_system)
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_system(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Paddles
    let paddle = shapes::Rectangle {
        extents: Vec2::new(16.0, 64.0),
        origin: Default::default(),
    };

    commands.spawn()
        .insert(P1)
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-(WINDOW_WIDTH / 2.0) + 64.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn()
                .insert_bundle(GeometryBuilder::build_as(
                    &paddle,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform::default(),
                ));
        });

    commands.spawn()
        .insert(P2)
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3::new((WINDOW_WIDTH / 2.0) - 64.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn()
                .insert_bundle(GeometryBuilder::build_as(
                    &paddle,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform::default(),
                ));
        });

    // Ball
    let ball = shapes::RegularPolygon {
        sides: 4,
        center: Default::default(),
        feature: RegularPolygonFeature::Radius(8.0),
    };

    commands.spawn()
        .insert(Ball)
        .insert_bundle(SpatialBundle {
            transform: Transform::default(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn()
                .insert_bundle(GeometryBuilder::build_as(
                    &ball,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform::default(),
                ));
        });
}

fn p1_movement_system(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<P1>>) {
    let mut direction = 0.0;
    let mut transform = query.single_mut();

    if input.pressed(KeyCode::A) {
        direction += 1.0;
    } else if input.pressed(KeyCode::D) {
        direction -= 1.0;
    }

    let new_pos = transform.translation.y + direction * PADDLE_SPD * DT;

    transform.translation.y = new_pos.clamp(-(WINDOW_HEIGHT / 2.0) + PADDLE_SIZE.y + 16.0,
                                            (WINDOW_HEIGHT / 2.0) - PADDLE_SIZE.y - 16.0);
}

fn p2_movement_system(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<P2>>) {
    let mut direction = 0.0;
    let mut transform = query.single_mut();

    if input.pressed(KeyCode::Left) {
        direction += 1.0;
    } else if input.pressed(KeyCode::Right) {
        direction -= 1.0;
    }

    let new_pos = transform.translation.y + direction * PADDLE_SPD * DT;

    transform.translation.y = new_pos.clamp(-(WINDOW_HEIGHT / 2.0) + PADDLE_SIZE.y + 16.0,
                                            (WINDOW_HEIGHT / 2.0) - PADDLE_SIZE.y - 16.0);
}