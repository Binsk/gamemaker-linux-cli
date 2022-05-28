use std::{io, thread, time::Duration};
use tui::{
	backend::CrosstermBackend, 
	widgets::{Block, Borders},
	layout::{Layout, Constraint, Direction},
	Terminal
};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

fn main() -> Result<(), io::Error>{

	// Create an example block:
	enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;	
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	terminal.draw(|f|{
		let size = f.size();
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(0)
			.constraints([
				Constraint::Length(size.height - 3),
				Constraint::Percentage(100)
			].as_ref())
			.split(f.size());

		let block = Block::default()
			.title("Console")
			.borders(Borders::ALL);
		f.render_widget(block, chunks[0]);
				
		let block = Block::default()
			.title("Input").
			borders(Borders::ALL);
		f.render_widget(block, chunks[1]);
	})?;

	// Wait 5 seconds and close:
	thread::sleep(Duration::from_millis(5000));

	// Return the terminal back to normal:
	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	
	terminal.show_cursor()?;
	Ok(())
}
