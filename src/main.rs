use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InputManagerPlugin::<Action>::default()))
        .add_systems(Startup, startup)
        .add_systems(Update, gamepad_system)
        .run();
}

#[derive(Actionlike, Reflect, Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Action {
    DoStuff,
}

fn startup(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Action> {
        action_state: ActionState::default(),
        input_map: InputMap::default()
            .insert(
                Action::DoStuff,
                // This ought to prevent inputs in the range [-0.5, 0.5]
                SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.5),
            )
            .build(),
    });
}

fn gamepad_system(query: Query<&ActionState<Action>>) {
    let action_state = query.single();
    let left_stick_x = action_state.clamped_value(&Action::DoStuff);
    info!("{}", left_stick_x); // observe unexpected inputs in the range [-0.5, 0.5]
}
