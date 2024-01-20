use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "tetris".into(),

                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, make_visible)
        .run();
}

fn make_visible(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
