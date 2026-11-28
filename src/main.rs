use reticuli::{diagnostics::profiler::Profiler, rendering::ansi::terminal::Terminal};
use std::io::Result;

fn main() -> Result<()> {
    let mut profiler = Profiler::default();
    let mut terminal = Terminal::new()?;
    profiler.bench(&mut terminal, 1_000)?;
    drop(terminal);

    println!("Reticuli");
    for average in profiler.averages() {
        println!("{}: {}us", average.0, average.1.as_micros());
    }

    Ok(())
}
