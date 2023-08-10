use bevy::prelude::*;

fn main() {
   App::new()
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest()) // tbh, already forgot what this does
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Risk of Bevy".into(),
            resolution: (640.0, 480.0).into(),
            resizable: false,
            ..default()
          }),
          ..default()
        })
    )
    .run()
}
