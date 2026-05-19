
// use crossterm::event;
// use indicatif::{ProgressBar, ProgressStyle};
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;


// fn main() -> std::io::Result<()> {
//     ratatui::run(|mut terminal| {
//         loop {
//             terminal.draw(|frame| {
//                 frame.render_widget("Tomatillo Timer!!", frame.area())
//             })?;

//             if event::read()?.is_key_press() {
//                 break Ok::<(), std::io::Error>(());
//             }
//         }
//     })?; // <- THIS is the important part

//     let running = Arc::new(AtomicBool::new(true));
//     let r = running.clone();

//     // Use Control C to quit program, remove this later ///////////////////////
//     ctrlc::set_handler(move || {
//         r.store(false, Ordering::SeqCst);
//         println!("\n\nTimer stopped. Good work!");
//         std::process::exit(0);
//     })
//     .expect("Error setting Ctrl-C handler");
//     ///////////////////////////////////////////////////////////////////////////

//     let total_secs = 100u64;

//     let pb = ProgressBar::new(total_secs);

//     pb.set_style(
//         ProgressStyle::with_template("  {prefix:.bold}  [{bar:45.cyan/blue}]  {msg:.dim}")
//             .unwrap()
//             .progress_chars("━╸─"),
//     );

//     pb.set_prefix("pomodoro".to_string());

//     for elapsed in 0..=total_secs {
//         if !running.load(Ordering::SeqCst) {
//             pb.finish_and_clear();
//             return Ok(());
//         }

//         let remaining = total_secs - elapsed;
//         let mins = remaining / 60;
//         let secs = remaining % 60;

//         pb.set_position(elapsed);
//         pb.set_message(format!("{:02}:{:02} remaining", mins, secs));

//         if elapsed < total_secs {
//             thread::sleep(Duration::from_secs(1));
//         }
//     }

//     pb.finish_with_message("Done!".to_string());

//     println!("ratatui exited");

//     Ok(())
// }


use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
};
use std::io;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let total_secs = 100u64;

    let start = Instant::now();

    ratatui::run(|mut terminal| {
        loop {
            let elapsed = start.elapsed().as_secs();

            // Stop timer at 100 seconds
            let capped_elapsed = elapsed.min(total_secs);

            let remaining = total_secs - capped_elapsed;

            let mins = remaining / 60;
            let secs = remaining % 60;

            let progress = capped_elapsed as f64 / total_secs as f64;

            terminal.draw(|frame| {
                let gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title("Tomatillo Timer!")
                            .borders(Borders::ALL),
                    )
                    .gauge_style(
                        Style::default()
                            .fg(Color::Cyan)
                            .bg(Color::Black),
                    )
                    .ratio(progress)
                    .label(format!("{:02}:{:02} remaining", mins, secs));

                frame.render_widget(gauge, frame.area());
            })?;

            // Exit automatically when timer completes
            if capped_elapsed >= total_secs {
                break Ok::<(), io::Error>(());
            }

            // Handle keyboard input
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    // Exit only on Ctrl+C
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break Ok::<(), io::Error>(());
                    }
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    })?;

    println!("\nTimer stopped. Good work!");

    Ok(())
}