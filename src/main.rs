use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    ecs::query::QueryFilter,
    prelude::*,
};
mod stepping;

const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
const GAP_BETWEEN_PADDLE_AND_BORDER: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const WALL_THICKNESS: f32 = 10.0;
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;


const SCOREBOARD_FONT_SIZE: f32 = 33.0;

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
                move_playerone_paddle,
                move_playertwo_paddle,
                check_for_score,
                check_for_collisions,
            ).chain(),
        )
        .add_systems(Update, update_scoreboard)
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct PlayerOne;

#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource)]
struct BounceSound(Handle<AudioSource>);

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
    asset_server: Res<AssetServer>,
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
        Paddle,
        PlayerOne,
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
        Paddle,
        PlayerTwo,
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

    let bounce_sound = asset_server.load("sound/ball_paddle.wav");
    commands.insert_resource(BounceSound(bounce_sound));

    commands
        .spawn((
            Text::new("Score: "),
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

fn move_paddle<F: QueryFilter>(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, F>,
    time: Res<Time>,
    up_key: KeyCode,
    down_key: KeyCode,
) {
    for mut transform in &mut query {
        let mut direction: Option<f32> = None;

        if keyboard_input.pressed(up_key) {
            direction = Some(1.0);
        }

        if keyboard_input.pressed(down_key) {
            direction = Some(-1.0);
        }

        if let Some(d) = direction {
            let new_paddle_position =
                transform.translation.y + d * PADDLE_SPEED * time.delta_secs();

            let lower_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0;
            let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0;

            transform.translation.y = new_paddle_position.clamp(lower_bound, upper_bound);
        }
    }
}

fn move_playerone_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&mut Transform, With<PlayerOne>>,
    time: Res<Time>,
) {
    move_paddle::<With<PlayerOne>>(keyboard_input, query, time, KeyCode::KeyW, KeyCode::KeyS);
}

fn move_playertwo_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&mut Transform, With<PlayerTwo>>,
    time: Res<Time>,
) {
    move_paddle::<With<PlayerTwo>>(keyboard_input, query, time, KeyCode::ArrowUp, KeyCode::ArrowDown);
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

fn check_for_score(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut score: ResMut<Score>,
) {
    let (mut transform, mut velocity) = ball_query.single_mut();

    let ball_x = transform.translation.x;
    let ball_radius = BALL_DIAMETER / 2.0;

    if ball_x - ball_radius < (LEFT_WALL + WALL_THICKNESS / 2.0) {
        score.0 += 1;
        reset_ball(&mut transform, &mut velocity, true);
    } else if ball_x + ball_radius > (RIGHT_WALL - WALL_THICKNESS / 2.0) {
        score.1 += 1;
        reset_ball(&mut transform, &mut velocity, false);
    }
}

fn reset_ball(transform: &mut Transform, velocity: &mut Velocity, to_left: bool) {
    transform.translation = BALL_STARTING_POSITION;

    let direction = if to_left {
        Vec2::new(-0.5, 0.5)
    } else {
        Vec2::new(0.5, 0.5)
    };

    velocity.0 = direction.normalize() * BALL_SPEED;
}

fn check_for_collisions(
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut commands: Commands,
    bounce_sound: Res<BounceSound>
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();

    for (_collider_entity, collider_transform) in &collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Reflect the ball's velocity when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // Reflect only if the velocity is in the opposite direction of the collision
            // This prevents the ball from getting stuck inside the bar
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            // Reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                commands.spawn((
                    AudioPlayer::new(bounce_sound.0.clone()),
                    PlaybackSettings::ONCE,
                ));
                ball_velocity.x = -ball_velocity.x;
            }

            // Reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }

    }
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