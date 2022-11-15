use std::{
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize,
};

use rtuner::{get_pitch, ui};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    let input_device = host
        .default_input_device()
        .expect("failed to find input device");

    let mut config = input_device
        .default_input_config()
        .expect("Failed to get default input config")
        .config();
    config.channels = 1;
    config.buffer_size = BufferSize::Fixed(512);

    const BUFFER_SIZE: usize = 2048;

    let mut buffer = [0.0_f32; BUFFER_SIZE];
    let mut index = 0_usize;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = Arc::new(Mutex::new(ui::App::new()));
    let app1 = app.clone();

    let stream = input_device.build_input_stream(
        &config,
        move |data: &[f32], _input_callback_info: &cpal::InputCallbackInfo| {
            if index >= BUFFER_SIZE {
                let (note, error) = get_pitch(data, config.sample_rate.0 as usize);

                let mut bingding = app1.lock().unwrap();
                bingding.on_tick(note, error);

                index = 0;
            }
            let copy_len = (BUFFER_SIZE - index).min(data.len());
            buffer[index..index + copy_len].copy_from_slice(&data[..copy_len]);
            index += copy_len;
        },
        move |err| {
            // react to errors here.
            println!("{:?}", err);
        },
    )?;

    stream.play()?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);
    loop {
        let binding = app.lock().unwrap();
        terminal.draw(|f| ui::ui(f, &binding))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
