use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
mod cam;
/// set up a simple 3D scene

fn make_mesh(
            positions: Vec<[f32; 3]>, 
            normals: Vec<[f32; 3]>, 
            uvs: Vec<[f32; 2]>, 
            indices: mesh::Indices
        )   -> bevy::prelude::Mesh {

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    //setting vertices
    let vertices = [
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];

    //let indices = mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);
    let indices = mesh::Indices::U32(vec![1, 2, 0]);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals:   Vec<[f32; 3]> = Vec::new();
    let mut uvs:       Vec<[f32; 2]> = Vec::new();

    //adding position, normal, uvs to their corresponding vectors
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    //Mesh made up 
    let mesh = make_mesh(positions, normals, uvs, indices);

    // add entities to the world
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1800.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 0.0),
        ..default()
    });
    // camera
    cam::spawn_camera(commands);
  
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(cam::pan_orbit_camera)
        .run();
}