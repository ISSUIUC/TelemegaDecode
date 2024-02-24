use crate::transition::Transition;

pub struct StreamingBitSync {
    state: bool,
    alignment: f64,
    baud_width: f64,
}

impl StreamingBitSync {
    pub fn new(baud_width: f64) -> StreamingBitSync {
        StreamingBitSync {
            state: false,
            alignment: 0.0,
            baud_width
        }
    }

    pub fn feed(&mut self, sample: Transition, mut for_each: impl FnMut(bool)) {
        let old_state = self.state;
        let idx = sample.idx as f64;
        self.state = sample.new_state;

        if self.alignment == 0.0 {
            self.alignment = idx;
            return;
        } else if self.alignment > idx {
            return;
        }

        while idx > self.alignment {
            self.alignment += self.baud_width;
            for_each(old_state);

            if self.alignment - self.baud_width / 2.0 < idx && idx < self.alignment + self.baud_width / 2.0 {
                self.alignment = self.alignment * 0.8 + idx * 0.2;
                break;
            }
        }
    }
}
