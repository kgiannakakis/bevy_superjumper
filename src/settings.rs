use std::{error::Error, fs};

pub const HIGHSCORE_COUNT: usize = 5;

const SETTINGS_FILE: &str = ".superjumper";

pub struct Settings {
    pub sound_enabled: bool,
    pub high_scores: [u32; HIGHSCORE_COUNT],
}

impl Default for Settings {
    fn default() -> Self {
        const DEFAULT_HIGHSCORES: [u32; HIGHSCORE_COUNT] = [100, 80, 50, 30, 10];

        let mut high_scores: [u32; HIGHSCORE_COUNT] = [0; HIGHSCORE_COUNT];
        for (i, score) in DEFAULT_HIGHSCORES.iter().enumerate() {
            high_scores[i] = *score;
        }

        Settings {
            sound_enabled: true,
            high_scores,
        }
    }
}

pub fn read_settings() -> Settings {
    if let Ok(settings) = read_settings_file() {
        return settings;
    }
    Settings::default()
}

fn read_settings_file() -> Result<Settings, Box<dyn Error>> {
    let contents = fs::read_to_string(SETTINGS_FILE)?;
    let mut sound_enabled = false;
    let mut high_scores: [u32; HIGHSCORE_COUNT] = [0; HIGHSCORE_COUNT];
    let mut high_score_index: usize = 0;

    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            match line {
                "true" => sound_enabled = true,
                "false" => sound_enabled = false,
                _ => Err("Invalid file")?,
            }
        } else if i > 0 && i <= HIGHSCORE_COUNT {
            if let Ok(score) = line.parse::<u32>() {
                high_scores[i - 1] = score;
                high_score_index = i;
            }
        } else if !line.is_empty() {
            Err("Invalid file")?
        }
    }

    if high_score_index != HIGHSCORE_COUNT {
        Err("Invalid file")?
    }

    Ok(Settings {
        sound_enabled,
        high_scores,
    })
}

pub fn write_sound_setting(sound_enabled: bool) {
    let mut data = format!("{}\n", sound_enabled);

    for score in read_settings().high_scores {
        data.push_str(&format!("{}\n", score));
    }

    let _ = fs::write(SETTINGS_FILE, data);
}

pub fn write_high_scores(high_scores: [u32; HIGHSCORE_COUNT]) {
    let mut data = format!("{}\n", read_settings().sound_enabled);

    for score in high_scores {
        data.push_str(&format!("{}\n", score));
    }

    let _ = fs::write(SETTINGS_FILE, data);
}
