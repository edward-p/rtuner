pub mod ui;

use std::io::Write;

use pitch_detection::detector::{yin::YINDetector, PitchDetector};

const PITCH: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub fn frequency_to_note(freq: f64) -> (&'static str, i32, f64) {
    let step = 12.0 * f64::log2(freq / 261.6);
    let percentage = (step - step.round()) * 100.0;

    let mut octave_d = step.round() as i32;
    if octave_d < 0 {
        octave_d -= 12;
    }
    let octave = 4 + octave_d / 12;
    let mut index = step.round() as i32 % 12;
    if index < 0 {
        index += 12
    }

    (PITCH[index as usize], octave, percentage)
}

pub fn get_pitch(data: &[f32], sample_rate: usize) -> (String, f64) {
    let mut detector = YINDetector::new(data.len(), data.len() / 2);
    let pitch = detector.get_pitch(data, sample_rate, 0.3, 0.9);
    if let Some(p) = pitch {
        let note = frequency_to_note(p.frequency.into());
        std::io::stdout().flush().unwrap();
        // print!("\r{:>2}, {:>2}, {:>+6.2}%", note.0, note.1, note.2);
        (format!("{:>2}{:>2}", note.0, note.1), note.2)
    } else {
        // print!("\r\nNo note detected.")
        ("N/A".into(), 14.0)
    }
}
