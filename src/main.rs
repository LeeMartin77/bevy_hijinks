use bevy::prelude::*;
use bevy::math::Vec2;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(player_input_system.system())
        .add_system(movable_object_update_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(140.0, 140.0, 140.0).into()),
            transform: Transform::from_xyz(0.0, 150.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Player {})
        .insert(Thrust {
            thrust: 0.0,
            facing: 0.0,
        })
        .insert(Velocity {
            velocity: Vec3::new(150.0, 0.0, 0.0),
        })
        .insert(Gravity::Movable(MassRadius {
            radius: 10.0,
            mass: 0.001
        }));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
        .insert(Planet {
        })
        .insert(Gravity::Immovable(MassRadius {
            radius: 100.0,
            mass: 500.0 * (10f32).powi(14)
        }));
}

struct MassRadius {
    mass: f32,
    radius: f32
}

struct Velocity {
    velocity: Vec3
}

struct Thrust {
    thrust: f32,
    facing: f32
}

enum Gravity {
    Movable(MassRadius),
    Immovable(MassRadius)
}

struct Player {
}


struct Planet {
}

const PLAYER_ACCELERATION_RATE: f32 = 50.0;
const PLAYER_TURN_RATE: f32 = 10.0;

fn player_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Thrust)>,
) {
    if let Ok((_player, mut thrust)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            thrust.facing -= PLAYER_TURN_RATE * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Right) {
            thrust.facing += PLAYER_TURN_RATE * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Up) {
            thrust.thrust += PLAYER_ACCELERATION_RATE * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Down) {
            thrust.thrust -= PLAYER_ACCELERATION_RATE * time.delta_seconds();
            
        }
        if thrust.thrust < 0.0 {
            thrust.thrust = 0.0;
        }
    }
}

fn movable_object_update_system(
    time: Res<Time>,
    mut set: QuerySet<(
        Query<(&Gravity, &Transform, &Planet)>,
        Query<(&Gravity, &Thrust, &mut Transform, &mut Velocity)>
    )>
) {
    let mut planet_mass_radius = MassRadius { mass: 0.0, radius: 0.0 };
    let mut planet_translation = Vec3::new(0.0, 0.0, 0.0);
    if let Ok((planet_gravity, planet_transform, _planet)) = set.q0().single() {
        if let Gravity::Immovable(pmr) = planet_gravity {
            planet_mass_radius.mass = pmr.mass;
            planet_mass_radius.radius = pmr.radius;
            planet_translation.x = planet_transform.translation.x;
            planet_translation.y = planet_transform.translation.y;
        }
    }
    if let Ok((player_gravity, thrust, mut player_transform, mut velocity)) = set.q1_mut().single_mut() {
        if let Gravity::Movable(player_mass_radius) = player_gravity {
            //Some cheating is going on here:
            // - Only one planet
            // - Only one player
            // - Working on the assumption of the "gravity" type
            let distance_between_objects = distance_between_two_vec(planet_translation, player_transform.translation);
            if distance_between_objects <= (planet_mass_radius.radius + player_mass_radius.radius) {
                // player has crashed - time to restart;
                // Teeeechnically we should probably call this updating - but it will get hit after anyway
                return;
            }
            //We should definitely not be calcing this constantly.
            //https://en.wikipedia.org/wiki/Gravitational_constant
            let gravitational_constant: f32 = 6.674f32 * (10f32).powi(-11);
            let force = gravitational_constant * ((player_mass_radius.mass * planet_mass_radius.mass) / distance_between_objects.powi(2));
            //F=MA
            //A = F/M
            let acceleration = force / player_mass_radius.mass;
            //A = distance/time
            //distance = A*time
            let distance = acceleration * time.delta_seconds();
            let normalised_vector = vec_from_angle(angle_between_two_vec(planet_translation, player_transform.translation));
            velocity.velocity += normalised_vector * distance;
            if thrust.thrust > 0.0 {
                velocity.velocity += time.delta_seconds() * (thrust.thrust * vec_from_angle(thrust.facing));
            }
            player_transform.translation += time.delta_seconds() * velocity.velocity;
        }
    }
}

//Note: this all uses Vec3s, we only want to care about x and y though

fn vec_from_angle(angle: f32) -> Vec3 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec3::new(vx, vy, 0.0)
}


fn angle_between_two_vec(sourcevec: Vec3, targetvec: Vec3) -> f32 {
    let vec = sourcevec - targetvec;
    vec.x.atan2(vec.y)
}

fn distance_between_two_vec(vecone: Vec3, vectwo: Vec3) -> f32{
    let vec = vecone - vectwo;
    ((vec.x * vec.x) + (vec.y * vec.y)).abs().sqrt()
}