pub struct Track {
    pub title: String,
    pub duration: u32,
    cursor: u32,
}

impl Track {
    pub fn new(title: String, duration: u32) -> Self {
        Self {
            title,
            duration,
            cursor: 0,
        }
    }
}

pub struct Player {
    play_list: Vec<Track>,
    curr_track: usize,
    _volume: u8,
}

impl Player {
    pub fn next_track(&mut self) {
        self.curr_track = (self.curr_track + 1) % self.play_list.len();
    }

    pub fn prev_track(&mut self) {
        self.curr_track = (self.play_list.len() + self.curr_track - 1) % self.play_list.len();
    }

    pub fn play(&mut self) {
        self.track_mut().cursor = 10;
    }

    pub fn pause(&mut self) {
        self.track_mut().cursor = 43;
    }

    pub fn rewind(&mut self) {
        self.track_mut().cursor = 0;
    }

    pub fn track(&self) -> &Track {
        &self.play_list[self.curr_track]
    }

    fn track_mut(&mut self) -> &mut Track {
        &mut self.play_list[self.curr_track]
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            play_list: vec![
                Track::new("Track 1".into(), 180),
                Track::new("Track 2".into(), 165),
                Track::new("Track 3".into(), 197),
                Track::new("Track 4".into(), 205),
            ],
            curr_track: 0,
            _volume: 25,
        }
    }
}
