extern crate rustty;

use rustty::{Terminal, Cell, CellAccessor, Event};
use std::time::Duration;

fn neighbors(accessor: &CellAccessor, i: isize, j: isize) -> Vec<&Cell>{
    let neighbor_indexes = vec![(i - 1, j - 1), (i, j - 1), (i, j + 1), (i + 1, j), (i - 1, j), (i - 1, j + 1), (i + 1, j - 1), (i + 1, j + 1)];
    let mut cells = vec![];

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
            if let &Some(cell) = &term.get(i, j) {
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

struct Update {
    x: usize,
    y: usize,
    alive: bool
}


fn main() {
    let mut term = Terminal::new().expect("Can't get term");

    term[(51,5)] = Cell::with_char('O');
    term[(50,5)] = Cell::with_char('O');
    term[(50,6)] = Cell::with_char('O');
    term[(50,7)] = Cell::with_char('O');
    term[(49,6)] = Cell::with_char('O');

    loop {
        term.swap_buffers().unwrap();

        let updates = get_updates(&term);

        for update in updates {
            if let Some(cell) = term.get_mut(update.x, update.y) {
                cell.set_ch(if update.alive {'O'} else {' '});
            }
        }

        let event = &term.get_event(Duration::from_millis(100)).unwrap();

        if let &Some(Event::Key(char_pressed)) = event {
            if char_pressed == 'q' {
                return
            }
        }
    }

}
