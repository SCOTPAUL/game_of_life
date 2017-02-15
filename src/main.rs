extern crate rustty;

use rustty::{Terminal, Cell, CellAccessor, Event, Color};
use std::time::Duration;

fn neighbors(accessor: &CellAccessor, i: isize, j: isize) -> Vec<&Cell>{
    let neighbor_indexes = vec![(i - 1, j - 1), (i, j - 1), (i, j + 1), (i + 1, j), (i - 1, j), (i - 1, j + 1), (i + 1, j - 1), (i + 1, j + 1)];
    let mut cells = Vec::with_capacity(8);

    for (x, y) in neighbor_indexes {
        if x < 0 || y < 0 {
            continue;
        }

        if let Some(cell) = accessor.get(x as usize, y as usize){
            cells.push(cell);
        }
    }

    cells
}

fn live_should_live(accessor: &CellAccessor, i: usize, j: usize) -> bool {
    let neighbours = neighbors(accessor, i as isize, j as isize);
    let mut alive_count = 0;

    for cell in neighbours {
        if cell.ch() == 'O' {
            alive_count += 1;
        }
    }

    alive_count == 2 || alive_count == 3
}

fn dead_should_live(accessor: &CellAccessor, i: usize, j: usize) -> bool {
    let neighbours = neighbors(accessor, i as isize, j as isize);

    let mut alive_count = 0;

    for cell in neighbours {
        if cell.ch() == 'O'{
            alive_count += 1;
        }
    }

    alive_count == 3
}

fn get_updates(term: &Terminal) -> Vec<Update>{
    let cols = term.cols();
    let rows = term.rows();
    let mut updates = vec![];


    for i in 0 .. cols {
        for j in 0 .. rows {
            if let Some(cell) = term.get(i, j) {
                if cell.ch() == 'O' {
                    if !live_should_live(term, i, j) {
                        updates.push(Update {x: i, y: j, alive: false});
                    }
                }
                else {
                    if dead_should_live(term, i, j) {
                        updates.push(Update {x: i, y: j, alive: true});
                    }
                }
            }
        }
    }

    updates
}


fn clear_non_live(term: &mut Terminal) {
    let cols = term.cols();
    let rows = term.rows();


    for i in 0 .. cols {
        for j in 0 .. rows {
            if let Some(cell) = term.get_mut(i, j) {
                cell.set_bg(Color::Default);
            }
        }
    }
}

struct CursorPos {
    x: usize,
    y: usize
}

fn select_loop(term: &mut Terminal){
    let cols = term.cols();
    let rows = term.rows();
    let mut cursor_pos = CursorPos {x: 0, y: 0};

    loop {
        {
            let mut current_cell = term.get_mut(cursor_pos.x, cursor_pos.y).unwrap();
            current_cell.set_bg(Color::Red);
        }
        let event = term.get_event(Duration::from_millis(80)).unwrap();
        if let Some(Event::Key(char_pressed)) = event {
            match char_pressed {
                'w' => if cursor_pos.y >= 1 {cursor_pos.y -= 1},
                's' => if cursor_pos.y < rows - 1 {cursor_pos.y += 1},
                'a' => if cursor_pos.x >= 1 {cursor_pos.x -= 1},
                'd' => if cursor_pos.x < cols - 1 {cursor_pos.x += 1;},
                ' ' => {term.get_mut(cursor_pos.x, cursor_pos.y).unwrap().set_ch('O');},
                'q' => break,
                _ => {}
            }
        }
        term.swap_buffers().unwrap();
        clear_non_live(term);
    }

    let mut current_cell = term.get_mut(cursor_pos.x, cursor_pos.y).unwrap();
    current_cell.set_bg(Color::Default);
}

struct Update {
    x: usize,
    y: usize,
    alive: bool
}


fn main() {
    let mut term = Terminal::new().expect("Can't get term");

    select_loop(&mut term);

    loop {
        term.swap_buffers().unwrap();

        let updates = get_updates(&term);

        if updates.is_empty() {
            return
        }

        for update in updates {
            if let Some(cell) = term.get_mut(update.x, update.y) {
                cell.set_ch(if update.alive {'O'} else {' '});
            }
        }

        let event = &term.get_event(Duration::from_millis(80)).unwrap();

        if let &Some(Event::Key(char_pressed)) = event {
            if char_pressed == 'q' {
                return
            }
        }
    }

}
