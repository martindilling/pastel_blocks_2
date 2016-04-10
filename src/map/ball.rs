use map::Color;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub color: Option<Color>,
}

impl Ball {
    pub fn new(x: f64, y: f64) -> Self {
        Ball {
            x: x,
            y: y,
            color: None,
        }
    }

    pub fn update(&mut self, delta: f64) {
        // Move left
        // Why is this lagging :/
        self.x += -150.0 * delta;
        self.y += 0.0;
    }
}
