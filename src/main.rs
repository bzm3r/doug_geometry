use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_prototype_lyon::draw::{DrawMode, FillMode, StrokeMode};
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_prototype_lyon::shapes as LyonShapes;
use doug_geometry::decomp::tests::spiral_poly;
use doug_geometry::shapes::PointLike;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PanCamPlugin::default())
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let shape = LyonShapes::Polygon {
        points: spiral_poly(3)
            .points
            .iter()
            .map(|p| Vec2::new(p.x() as f32, p.y() as f32))
            .collect::<Vec<Vec2>>(),
        closed: true,
    };

    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    commands.spawn(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 0.1),
        },
        Transform::default(),
    ));
}
