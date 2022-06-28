use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// ---- constants
const PADDLE_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const BALL_COLOR: Color = Color::rgb(0.7, 0., 0.7);
const WALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

const LEVEL_WIDTH: f32 = 300.;
const LEVEL_HEIGHT: f32 = 100.;
const PADDLE_HEIGHT: f32 = 50.;
const WALL_HEIGHT: f32 = 10.;
const PADDLE_SIZE: Vec2 = const_vec2!([10., PADDLE_HEIGHT]);
const BALL_SIZE: Vec2 = const_vec2!([10., 10.]);
const WALL_SIZE: Vec2 = const_vec2!([LEVEL_WIDTH, WALL_HEIGHT]);
const BOUNDARY_X: f32 = LEVEL_WIDTH / 2. + 5.;
const BOUNDARY_Y: f32 = LEVEL_HEIGHT + 5.;
const PADDLE_MAX_Y: f32 = LEVEL_HEIGHT - PADDLE_HEIGHT / 2. + WALL_HEIGHT;

// ---- resources

// ---- Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
// ---- Components

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My Pong!".to_string(),
            width: 500.0,
            height: 300.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_level)
        .add_startup_system(setup_player)
        .add_startup_system(setup_ai)
        .add_startup_system(setup_ball)
        .add_system(movable_system)
        .add_system(player_movement_input)
        .add_system(ai_movement)
        .add_system(ball_reflection)
        .add_system(paddle_movement_constraint)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_level(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, BOUNDARY_Y + WALL_SIZE.y, 1.0),
                scale: Vec3::new(WALL_SIZE.x, WALL_SIZE.y, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Wall);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -(BOUNDARY_Y + WALL_SIZE.y), 1.0),
                scale: Vec3::new(WALL_SIZE.x, WALL_SIZE.y, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Wall);
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(BOUNDARY_X - PADDLE_SIZE.x / 2., 0.0, 1.0),
                scale: Vec3::new(PADDLE_SIZE.x, PADDLE_SIZE.y, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. });
}

fn setup_ai(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-(BOUNDARY_X - PADDLE_SIZE.x / 2.), 0.0, 1.0),
                scale: Vec3::new(PADDLE_SIZE.x, PADDLE_SIZE.y, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Ai)
        .insert(Velocity { x: 0., y: 0. });
}

fn setup_ball(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(BALL_SIZE.x, BALL_SIZE.y, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Ball)
        .insert(Velocity { x: 1., y: 1. });
}

fn player_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_velocities: Query<&mut Velocity, With<Player>>,
) {
    for mut player_velocity in player_velocities.iter_mut() {
        //reset velocity first
        player_velocity.x = 0.;
        player_velocity.y = 0.;
        if keyboard_input.pressed(KeyCode::Down) {
            player_velocity.y = -2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player_velocity.y = 2.;
        }
    }
}

fn paddle_movement_constraint(
    mut players: Query<&mut Transform, With<Player>>,
    mut ais: Query<&mut Transform, (With<Ai>, Without<Player>)>,
) {
    if let Some(mut player_transform) = players.iter_mut().next() {
        if collide(
            player_transform.translation,
            PADDLE_SIZE,
            Vec3::new(0.0, -(BOUNDARY_Y + WALL_SIZE.y), 1.0),
            WALL_SIZE,
        )
        .is_some()
        {
            player_transform.translation.y = -PADDLE_MAX_Y;
        }
        if collide(
            player_transform.translation,
            PADDLE_SIZE,
            Vec3::new(0.0, BOUNDARY_Y + WALL_SIZE.y, 1.0),
            WALL_SIZE,
        )
        .is_some()
        {
            player_transform.translation.y = PADDLE_MAX_Y;
        }
    }
    if let Some(mut ai_transform) = ais.iter_mut().next() {
        if collide(
            ai_transform.translation,
            PADDLE_SIZE,
            Vec3::new(0.0, -(BOUNDARY_Y + WALL_SIZE.y), 1.0),
            WALL_SIZE,
        )
        .is_some()
        {
            ai_transform.translation.y = -PADDLE_MAX_Y;
        }
        if collide(
            ai_transform.translation,
            PADDLE_SIZE,
            Vec3::new(0.0, BOUNDARY_Y + WALL_SIZE.y, 1.0),
            WALL_SIZE,
        )
        .is_some()
        {
            ai_transform.translation.y = PADDLE_MAX_Y;
        }
    }
}

fn ai_movement(
    balls: Query<&Transform, With<Ball>>,
    mut ais: Query<(&mut Velocity, &Transform), (With<Ai>, Without<Ball>)>,
) {
    if let Some((mut ai_velocity, ai_transform)) = ais.iter_mut().next() {
        if let Some(ball_transform) = balls.iter().next() {
            if ball_transform.translation.y <= ai_transform.translation.y {
                ai_velocity.y = -1.;
            } else {
                ai_velocity.y = 1.;
            }
        }
    }
}

fn movable_system(mut movable: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in movable.iter_mut() {
        transform.translation.y += velocity.y;
        transform.translation.x += velocity.x;
    }
}

fn ball_reflection(
    mut balls: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    players: Query<(&Velocity, &Transform), (With<Player>, Without<Ball>)>,
    ais: Query<(&Velocity, &Transform), (With<Ai>, Without<Ball>)>,
) {
    if let Some((mut ball_velocity, mut ball_transform)) = balls.iter_mut().next() {
        //check collision with player or ai
        if let Some((player_velocity, player_transform)) = players.iter().next() {
            if collide(
                player_transform.translation,
                PADDLE_SIZE,
                ball_transform.translation,
                BALL_SIZE,
            )
            .is_some()
            {
                ball_velocity.x *= -1.;

                if player_velocity.y > 0. {
                    ball_velocity.y += 0.05;
                } else if player_velocity.y < 0. {
                    ball_velocity.y -= 0.05;
                }

                //increase velocity when hit player each time
                ball_velocity.x *= 1.05;
                ball_velocity.y *= 1.05;

                //adjust ball transform
                ball_transform.translation.x = player_transform.translation.x - PADDLE_SIZE.x;
            }
        }
        if let Some((ai_velocity, ai_transform)) = ais.iter().next() {
            if collide(
                ai_transform.translation,
                PADDLE_SIZE,
                ball_transform.translation,
                BALL_SIZE,
            )
            .is_some()
            {
                ball_velocity.x *= -1.;

                if ai_velocity.y > 0. {
                    ball_velocity.y += 0.05;
                } else if ai_velocity.y < 0. {
                    ball_velocity.y -= 0.05;
                }

                //adjust ball transform
                ball_transform.translation.x = ai_transform.translation.x + PADDLE_SIZE.x;
            }
        }

        //also check boundary
        if ball_transform.translation.x > BOUNDARY_X || ball_transform.translation.x < -BOUNDARY_X {
            // reset pong position
            ball_transform.translation.x = 0.;
            ball_transform.translation.y = 0.;
            // reset velocity
            let temp_vec = Vec2::new(ball_velocity.x, ball_velocity.y);
            ball_velocity.x = temp_vec.normalize().x;
            ball_velocity.y = temp_vec.normalize().y;
        }

        if ball_transform.translation.y > BOUNDARY_Y || ball_transform.translation.y < -BOUNDARY_Y {
            ball_velocity.y *= -1.;

            //adjust ball transform
            if ball_transform.translation.y > BOUNDARY_Y {
                ball_transform.translation.y = BOUNDARY_Y;
            } else {
                ball_transform.translation.y = -BOUNDARY_Y;
            }
        }
    }
}
