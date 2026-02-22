use super::runtime_state::RuntimeState;

/// Detects whether accumulation reset is needed by comparing previous and current runtime state.
pub fn needs_reset(prev: &RuntimeState, current: &RuntimeState) -> bool {
    prev.quality != current.quality
        || prev.camera_preset_index != current.camera_preset_index
        || prev.dof != current.dof
}
