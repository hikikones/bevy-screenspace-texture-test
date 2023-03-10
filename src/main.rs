use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        render_resource::{AsBindGroup, ShaderRef},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "b62bb455-a72c-4b56-87bb-81e0554e234f"]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "screenspace_texture.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            ..default()
        },
        MainCamera,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 25000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let custom_material = custom_materials.add(CustomMaterial {
        texture: asset_server.load("level.png"),
    });

    // Plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 15.0,
            ..Default::default()
        })),
        material: custom_material.clone_weak(),
        ..default()
    });

    // Secret block
    commands.spawn(MaterialMeshBundle {
        transform: Transform::from_xyz(0.2, 0.2, -0.04).with_scale(Vec3::ONE + Vec3::Y * 2.0),
        mesh: meshes.add(Mesh::from(shape::Cube::new(0.3))),
        // material: standard_materials.add(Color::BLUE.into()),
        material: custom_material,
        ..default()
    });

    // Player
    commands
        .spawn((
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(-0.5, 0.0, -0.7),
                    scale: Vec3::ONE * 0.4,
                    rotation: Quat::from_rotation_y(-135.0_f32.to_radians()),
                },
                ..Default::default()
            },
            Player,
        ))
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: standard_materials.add(Color::WHITE.into()),
                transform: Transform {
                    translation: Vec3::Y,
                    ..Default::default()
                },
                ..Default::default()
            });

            let eye_mesh = meshes.add(shape::Icosphere::default().try_into().unwrap());
            let eye_left = Vec3::new(-0.2, 1.6, -0.4);
            let eye_right = Vec3::new(-eye_left.x, eye_left.y, eye_left.z);
            let eye_scale = Vec3::splat(0.15);

            builder.spawn(PbrBundle {
                mesh: eye_mesh.clone_weak(),
                material: standard_materials.add(Color::BLACK.into()),
                transform: Transform {
                    translation: eye_left,
                    scale: eye_scale,
                    ..Default::default()
                },
                ..Default::default()
            });
            builder.spawn(PbrBundle {
                mesh: eye_mesh,
                material: standard_materials.add(Color::BLACK.into()),
                transform: Transform {
                    translation: eye_right,
                    scale: eye_scale,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    mut player_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard.pressed(KeyCode::A) {
        horizontal -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        horizontal += 1.0;
    }
    if keyboard.pressed(KeyCode::W) {
        vertical -= 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        vertical += 1.0;
    }

    let camera = camera_q.single();
    let right = camera.right();
    let forward = right.cross(Vec3::Y);
    let movement = right * horizontal + forward * vertical;
    player_q.single_mut().translation += movement.normalize_or_zero() * time.delta_seconds();
}
