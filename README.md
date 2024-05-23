This demonstrates a bug in leafwing-input-manager where gamepad axis thresholds are being ignored.

### Steps to Reproduce

1. `cargo run`
2. Connect gamepad
3. Move left gamepad joystick left/right slowly.
4. Observe values in the range `[-0.5, 0.5]` even though the code has set that as the threshold.
