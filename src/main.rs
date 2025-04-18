use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use bevy::input::keyboard::Key;
use bevy::utils::tracing::field::display;

mod stepping;

const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
const GAP_BETWEEN_PADDLE_AND_BORDER: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_PADDING: f32 = 10.0;

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const WALL_THICKNESS: f32 = 10.0;
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;


const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        .insert_resource(Score(0, 0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_left_paddle,
                move_right_paddle,
                check_for_collisions,
            ).chain(),
        )
        .add_systems(Update, update_scoreboard)
        .run();
}

#[derive(Component)]
struct FirstPaddle;

#[derive(Component)]
struct SecondPaddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Bundle)]
struct WallBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite: Sprite::from_color(WALL_COLOR, Vec2::ONE),
            transform: Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Resource)]
struct Score(usize, usize);

#[derive(Component)]
struct ScoreboardUi;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let first_paddle_x = LEFT_WALL + PADDLE_SIZE.x / 2.0 + GAP_BETWEEN_PADDLE_AND_BORDER;

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(first_paddle_x, 0.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        FirstPaddle,
        Collider,
    ));

    let second_paddle_x = RIGHT_WALL - PADDLE_SIZE.x / 2.0 - GAP_BETWEEN_PADDLE_AND_BORDER;

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(second_paddle_x, 0.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        SecondPaddle,
        Collider,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_STARTING_POSITION)
            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    commands
        .spawn((
            Text::new("Score "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn move_left_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Single<&mut Transform, With<FirstPaddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= 1.0;
    }

    let new_paddle_position =
        paddle.translation.y + direction * PADDLE_SPEED * time.delta_secs();

    let lower_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0;
    let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0;

    paddle.translation.y = new_paddle_position.clamp(lower_bound, upper_bound);
}

fn move_right_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<SecondPaddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();

    let lower_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0;
    let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0;

    paddle_transform.translation.y = new_paddle_position.clamp(lower_bound, upper_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let (player1_score, player2_score) = (score.0, score.1);
    *writer.text(*score_root, 1) = format!("{} : {}", player1_score, player2_score);
}

fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut query_set: ParamSet<(
        Single<(&mut Velocity, &mut Transform), With<Ball>>,
        Query<(Entity, &Transform), With<Collider>>,
    )>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, mut ball_transform) = query_set.p0().into_inner();
    let ball_position = ball_transform.translation.truncate();
    let mut velocity = ball_velocity.clone();
    let ball_bounds = BoundingCircle::new(ball_position, BALL_DIAMETER / 2.);
    let left_wall_hit = ball_collision(
        ball_bounds,
        Aabb2d::new(
            Vec2::new(LEFT_WALL, BOTTOM_WALL),
            Vec2::new(WALL_THICKNESS, TOP_WALL - BOTTOM_WALL),
        ),
    );
    let right_wall_hit = ball_collision(
        ball_bounds,
        Aabb2d::new(
            Vec2::new(RIGHT_WALL, BOTTOM_WALL),
            Vec2::new(WALL_THICKNESS, TOP_WALL - BOTTOM_WALL),
        ),
    );

    if let Some(collision) = left_wall_hit {
        collision_events.send_default();
        score.1 += 1;
        ball_transform.translation = BALL_STARTING_POSITION;
        return;
    } else if let Some(collision) = right_wall_hit {
        collision_events.send_default();
        score.0 += 1;
        ball_transform.translation = BALL_STARTING_POSITION;
        return;
    }

    for (collider_entity, collider_transform) in query_set.p1().iter() {
        let collision = ball_collision(
            ball_bounds,
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left =>  {
                    reflect_x = velocity.x > 0.0;
                },
                Collision::Right =>  {
                    reflect_x = velocity.x < 0.0;
                },
                Collision::Top => reflect_y = velocity.y < 0.0,
                Collision::Bottom => reflect_y = velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x = -velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -velocity.y;
            }
        }

    }
    *ball_velocity = velocity;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}