use std::error::Error;
use std::io::{self, stdout};
use std::time;
use crossterm::{
    event::{self, Event, KeyCode, read},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};
use rand::prelude::*;

pub struct App {
    pub data: Vec<u64>,
    pub should_quit: bool,
    pub finished_sorting: bool,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let data:Vec<u64> = init_data(&mut terminal);
    let mut app = App {data, should_quit: false, finished_sorting: true};

    terminal.draw(|f| ui(f, &app.data)).unwrap();
    //std::thread::sleep(time::Duration::from_millis(1000));
        
    visualize_sort(&mut terminal, &mut app);

    loop {
        terminal.draw(|f| ui(f, &app.data))?;
        update(&mut terminal, &mut app)?;
        if app.should_quit{
            break;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn init_data<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Vec<u64>{
    let window = terminal.backend().size().unwrap();
    let data_count = match window {
           _ => window.width - 2
    };
    rand_data(data_count.into())
}

pub fn visualize_sort<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App){
        quicksort(&mut app.data, terminal);
        app.finished_sorting = true;
}

pub fn quicksort<B: ratatui::backend::Backend>(arr: &mut [u64], terminal: &mut Terminal<B>) {
    _quicksort(arr, 0, (arr.len() - 1) as isize, terminal);
}

fn _quicksort<B: ratatui::backend::Backend>(arr: &mut [u64], left: isize, right: isize, terminal: &mut Terminal<B>) {
    //before doing any sorting, wait 25ms then draw the current data
    std::thread::sleep(time::Duration::from_millis(10));
    terminal.draw(|f| ui(f, &arr.to_vec())).unwrap();
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1, terminal);
        _quicksort(arr, partition_idx + 1, right, terminal);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

fn update<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            
            if key.kind == event::KeyEventKind::Press{
                match key.code {
                KeyCode::Char('q') => {app.should_quit = true; return  Ok(())},
                KeyCode::Char('r') => {
                    //println!("r pressed : sorting? {}", app.finished_sorting.clone());
                    app.data = init_data(terminal);
                    app.finished_sorting = false;
                    visualize_sort(terminal, app);
                      
                },
                _ => return Ok(()),
                }
            }
       }
    }
    Ok(())
}

fn rand_data(data_count: u32) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let size = data_count;
    let random_slice: Vec<u64> = (0..size).map(|_| rng.gen_range(1..1000)).collect::<Vec<u64>>();
    return random_slice;
}

fn ui(frame: &mut Frame, data: &Vec<u64>) {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Min(3),
        Constraint::Min(5),
                ]).split(frame.size());

    frame.render_widget(BarChart::default()
    .block(Block::default().title("quick sort").borders(Borders::ALL))
    .bar_width(1)
    .bar_gap(0)
    //.bar_style(Style::new().yellow().on_red())
    .value_style(Style::new().white().bold())
    //.label_style(Style::new().white())
    .data(BarGroup::default()
          .bars(&data.iter().map(|num| Bar::default().value(*num)).collect::<Vec<ratatui::widgets::Bar<'_>>>()))
    .max(1000), layout[1]);
    let p = Paragraph::new("Press 'r' to rerun the visualization, or 'q' to quit")
    .wrap(Wrap { trim: true})
    .style(Style::new().white().on_black())
    .alignment(Alignment::Left);

    let container = Block::default().title("algo-visualizer").borders(Borders::ALL);
    frame.render_widget(p.block(container), layout[0]);
}
