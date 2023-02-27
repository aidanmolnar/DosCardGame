// Taken directly from bevy animated shader example: https://github.com/bevyengine/bevy/blob/main/examples/shader/animate_shader.rs

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<BackgroundMaterial>::default())
            .add_startup_system(add_quad_system);
    }
}

fn add_quad_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 3556., y: 2000. }))),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(BackgroundMaterial {}),
        ..default()
    });
    //     meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 3556., y: 2000. }))),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     GlobalTransform::default(),
    //     BackgroundMaterial {},
    //     Visibility::default(),
    //     ComputedVisibility::default(),
    // ));
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct BackgroundMaterial {}

impl Material for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\background.wgsl".into()
    }
}
