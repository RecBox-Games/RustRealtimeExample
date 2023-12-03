
// represents some event resulting from an update (also called a tick)
// - add to this enum if you want to have events bubble up from elements lower
//   in your hierarchy to be handle by elements higher in the hierarchy
/*pub enum UpdateEvent {
    None,
    Finished,
}

impl UpdateEvent {
    pub fn is_finished(&self) -> bool {
        if let UpdateEvent::Finished = self {
            return true;
        }
        return false;
    }
}
 */

// represents the progress of some occurence (usually the movement of something
// across the screen)
pub struct Progression {
    // progress is a number 0.0 to 1.0 that represents how far along something
    // has progressed
    progress: f32,
    // per_tick is the amount of progress made per tick e.g. 1.0/60.0 if the
    // entire progression should take one second
    per_tick: f32,
}

impl Progression {

    // specify the total duration in seconds that this progression should last
    // WARNING: assumes 60 fps
    pub fn new(duration: f32) -> Self {
        Self {
            progress: 0.0,
            per_tick: 1.0/(60.0*duration),
        }
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    // progress by per_tick amount
    pub fn update(&mut self) {
        self.progress += self.per_tick;
    }

    pub fn is_done(&self) -> bool {
        self.progress >= 1.0
    }
}
