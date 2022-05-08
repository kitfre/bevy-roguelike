use crate::input::{action_input_system, Action, ActionSystem, KeyBindings};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{App, CoreStage, Input, KeyCode, ParallelSystemDescriptorCoercion, Plugin};

#[derive(Default)]
pub(crate) struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KeyboardInput>()
            .init_resource::<Input<KeyCode>>()
            .init_resource::<Option<Action>>()
            .init_resource::<KeyBindings>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                action_input_system.label(ActionSystem),
            );
    }
}
