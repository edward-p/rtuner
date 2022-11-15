use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

const Y_AXIS_BOUND: f64 = 30.0;

pub struct App {
    note_name: String,
    error: [f64; 10],
    p: usize,
    grid: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    pub fn new() -> App {
        let grid = (0..11).map(|i| (i as f64 * 0.1, 0.0)).collect();
        let note_name = "N/A".into();
        App {
            note_name,
            error: [0.0; 10],
            p: 0,
            grid,
            window: [0.0, 1.0],
        }
    }

    pub fn on_tick(&mut self, note_name: String, error: f64) {
        if note_name == "N/A" {
            return;
        }
        if self.note_name != note_name {
            self.p = 0;
            self.error = [0.0; 10];
        }
        self.note_name = note_name;
        if self.p > 9 {
            self.p = 0;
        }
        self.error[self.p] = error;
        self.p += 1;
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let avg_error = app.error.iter().sum::<f64>() / 10.0;
    let show_error = avg_error.min(Y_AXIS_BOUND - 1.0);

    let signal_data = [(0.1, show_error), (0.9, show_error)];
    let datasets = vec![
        Dataset::default()
            .name("Zero")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::DarkGray))
            .data(&app.grid),
        Dataset::default()
            .name("Signal")
            .marker(symbols::Marker::Block)
            .graph_type(GraphType::Line)
            .style(if avg_error.abs() < 2.5 {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .data(&signal_data),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "rtuner",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                // .title("X Axis")
                // .style(Style::default().fg(Color::Gray))
                // .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled(
                    format!("{}{:>+6.2}%", app.note_name, avg_error),
                    if avg_error.abs() < 2.5 {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ))
                .style(Style::default().fg(Color::DarkGray))
                .labels(vec![
                    Span::styled(
                        format!("-{}", Y_AXIS_BOUND),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("0"),
                    Span::styled(
                        format!("+{}", Y_AXIS_BOUND),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ])
                .bounds([-Y_AXIS_BOUND, Y_AXIS_BOUND]),
        );
    f.render_widget(chart, f.size());
}
