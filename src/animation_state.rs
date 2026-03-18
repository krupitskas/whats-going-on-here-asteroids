#[derive(Clone, Copy, Default)]
pub struct AnimationState {
    pub frame_index: u32,
    pub frame_time: f32,
    pub finished: bool,
}

impl AnimationState {
    pub fn advance(&mut self, delta_time: f32, fps: f32, animation_count: u32, looping: bool) {
        if self.finished || animation_count == 0 || fps <= 0.0 {
            return;
        }

        let frame_duration = 1.0 / fps;
        self.frame_time += delta_time;

        while self.frame_time >= frame_duration {
            self.frame_time -= frame_duration;

            if self.frame_index + 1 >= animation_count {
                if looping {
                    self.frame_index = 0;
                } else {
                    self.frame_index = animation_count - 1;
                    self.finished = true;
                    self.frame_time = 0.0;
                    break;
                }
            } else {
                self.frame_index += 1;
            }
        }
    }
}
