#[derive(Eq, PartialEq)]
pub enum Input {
    LeftPress,
    LeftRelease,
    RightPress,
    RightRelease,
    SoftDropPress,
    SoftDropRelease,
    RotateLeft,
    RotateRight,
    HardDrop,
    Hold,
}

pub trait InputSource {
    fn inputs(&mut self) -> Vec<Input>;
}

#[derive(Eq, PartialEq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    SoftDropStarted,
    SoftDropStopped,
    HardDrop,
    RotateLeft,
    RotateRight,
    Hold,
}

pub trait InputActions {
    fn actions(&mut self, delta_time: f64) -> Vec<Action>;
}

pub struct DasInputActions<TInputSource: InputSource> {
    input_source: TInputSource,
    delayed_auto_shift: f64,
    auto_repeat_rate: f64,
    auto_shift_timer: f64,
    holding_left: bool,
    holding_right: bool,
}

impl<TInputSource: InputSource> DasInputActions<TInputSource> {
    pub fn new(input_source: TInputSource, delayed_auto_shift: f64, auto_repeat_rate: f64) -> Self {
        Self {
            input_source: input_source,
            delayed_auto_shift,
            auto_repeat_rate,
            auto_shift_timer: 0f64,
            holding_left: false,
            holding_right: false,
        }
    }

    fn handle_auto_shift_timer(&mut self, delta_time: f64) -> i32 {
        if self.auto_shift_timer > f64::EPSILON {
            self.auto_shift_timer -= delta_time;
        }

        let mut count = 0;

        while self.auto_shift_timer <= -f64::EPSILON {
            count += 1;
            self.auto_shift_timer += self.auto_repeat_rate;
        }

        count
    }
}

impl<TInputSource: InputSource> InputActions for DasInputActions<TInputSource> {
    fn actions(&mut self, delta_time: f64) -> Vec<Action> {
        let inputs = self.input_source.inputs();
        let mut actions = Vec::<Action>::new();
        if inputs.contains(&Input::LeftPress) {
            actions.push(Action::MoveLeft);
            self.holding_left = true;
            self.holding_right = false;
            self.auto_shift_timer = self.delayed_auto_shift;
        }
        if inputs.contains(&Input::LeftRelease) {
            self.holding_left = false;
        }
        if inputs.contains(&Input::RightPress) {
            actions.push(Action::MoveRight);
            self.holding_right = true;
            self.holding_left = false;
            self.auto_shift_timer = self.delayed_auto_shift;
        }
        if inputs.contains(&Input::RightRelease) {
            self.holding_right = false;
        }
        if inputs.contains(&Input::RotateLeft) {
            actions.push(Action::RotateLeft);
        }
        if inputs.contains(&Input::RotateRight) {
            actions.push(Action::RotateRight);
        }
        if inputs.contains(&Input::HardDrop) {
            actions.push(Action::HardDrop);
        }
        if inputs.contains(&Input::SoftDropPress) {
            actions.push(Action::SoftDropStarted);
        }
        if inputs.contains(&Input::SoftDropRelease) {
            actions.push(Action::SoftDropStopped);
        }
        if inputs.contains(&Input::Hold) {
            actions.push(Action::Hold);
        }
        if self.holding_left {
            let count = self.handle_auto_shift_timer(delta_time);
            for _ in 0..count {
                actions.push(Action::MoveLeft);
            }
        }
        if self.holding_right {
            let count = self.handle_auto_shift_timer(delta_time);
            for _ in 0..count {
                actions.push(Action::MoveRight);
            }
        }
        actions
    }
}
