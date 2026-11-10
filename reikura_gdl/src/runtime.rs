use std::time::{Duration, Instant};

pub struct Runtime {
    pub(crate) accumulator: u32,
    pub(crate) current_time: Instant,
    pub(crate) state: State,
    timer: Option<Instant>,
    wait_until: Option<Instant>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            current_time: Instant::now(),
            state: State::Run,
            timer: None,
            wait_until: None,
        }
    }

    pub fn set_wait_time(&mut self, duration: Duration) {
        self.wait_until = Some(self.current_time + duration);
    }

    pub fn get_timer_elapsed(&self) -> Option<i32> {
        let elapsed = self
            .current_time
            .saturating_duration_since(self.timer?)
            .as_millis() as i32;

        Some(elapsed)
    }

    pub fn start_timer(&mut self, start: Duration) {
        self.timer = Some(self.current_time + start);
    }

    pub fn stop_timer(&mut self) {
        self.timer = None;
    }

    pub fn is_wait_finished(&self) -> bool {
        self.wait_until
            .is_none_or(|until| self.current_time > until)
    }
}

pub enum State {
    Exit,
    Run,
    Wait,
    WaitClick,
    WaitText,
    WaitTransition,
    WaitVoice,
    WaitVideo,
}

impl State {
    pub fn exit(&mut self) {
        *self = Self::Exit;
    }

    pub fn run(&mut self) {
        *self = Self::Run;
    }

    pub fn wait(&mut self) {
        *self = Self::Wait;
    }

    pub fn wait_click(&mut self) {
        *self = Self::WaitClick;
    }

    pub fn wait_text(&mut self) {
        *self = Self::WaitText;
    }

    pub fn wait_voice(&mut self) {
        *self = Self::WaitVoice;
    }

    pub fn wait_video(&mut self) {
        *self = Self::WaitVideo;
    }
}
