use std::iter;

const LEFT_TOP: char = '┌';
const RIGHT_TOP: char = '┐';
const LEFT_BOTTOM: char = '└';
const RIGHT_BOTTOM: char = '┘';
const HORIZONTAL: char = '─';
const HORIZONTAL_DOWN: char = '┬';
const HORIZONTAL_UP: char = '┴';
const CROSS: char = '┼';
const VERTICAL: char = '│';
const VERTICAL_LEFT: char = '┤';
const VERTICAL_RIGHT: char = '├';
const SPACE: char = ' ';
const NEW_LINE: char = '\n';

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub max_width: usize,
}

pub fn draw_box(height: usize, width: usize) -> String {
    let mut chars = row(width, LEFT_TOP, HORIZONTAL, RIGHT_TOP);
    chars.push(NEW_LINE);

    for _ in 1..height - 2 {
        chars.append(&mut row(width, VERTICAL, SPACE, VERTICAL));
        chars.push(NEW_LINE);
    }

    chars.append(&mut row(width, LEFT_BOTTOM, HORIZONTAL, RIGHT_BOTTOM));

    chars.iter().collect()
}

// TODO: Optimize later
pub fn draw_table(headers: &[Header], rows: &Vec<Vec<String>>) -> String {
    let mut table = String::from(LEFT_TOP);

    // Top
    for header in headers[..headers.len() - 1].iter() {
        table.push_str(
            &(0..header.max_width)
                .map(|_| HORIZONTAL)
                .chain(iter::once(HORIZONTAL_DOWN))
                .collect::<String>(),
        )
    }

    table.push_str(
        &(0..headers.last().unwrap().max_width)
            .map(|_| HORIZONTAL)
            .chain(iter::once(RIGHT_TOP))
            .collect::<String>(),
    );

    table.push(NEW_LINE);

    // Headers

    table.push(VERTICAL);


    for header in headers.iter() {
        table.push_str(&pad_left(&header.name, header.max_width));
        table.push(VERTICAL)
    }

    table.push(NEW_LINE);

    // Rows

    for row in rows.iter() {
        // Seperator

        table.push(VERTICAL_RIGHT);

        for header in headers[..headers.len() - 1].iter() {
            table.push_str(
                &(0..header.max_width)
                    .map(|_| HORIZONTAL)
                    .chain(iter::once(CROSS))
                    .collect::<String>(),
            )
        }
    
        table.push_str(
            &(0..headers.last().unwrap().max_width)
                .map(|_| HORIZONTAL)
                .chain(iter::once(VERTICAL_LEFT))
                .collect::<String>(),
        );

        // Row content
        table.push(NEW_LINE);

        table.push(VERTICAL);

        for (i, header) in headers.iter().enumerate() {
            table.push_str(&pad_left(&row[i], header.max_width));
            table.push(VERTICAL)
        }
    
        table.push(NEW_LINE);
    }

    // Bottom
    table.push(LEFT_BOTTOM);

    for header in headers[..headers.len() - 1].iter() {
        table.push_str(
            &(0..header.max_width)
                .map(|_| HORIZONTAL)
                .chain(iter::once(HORIZONTAL_UP))
                .collect::<String>(),
        )
    }

    table.push_str(
        &(0..headers.last().unwrap().max_width)
            .map(|_| HORIZONTAL)
            .chain(iter::once(RIGHT_BOTTOM))
            .collect::<String>(),
    );

    table.push(NEW_LINE);

    table
}

fn pad_left(s: &str, width: usize) -> String {
    format!("{:<width$}", s, width=width)
}

fn row(width: usize, start: char, mid: char, end: char) -> Vec<char> {
    iter::once(start)
        .chain((0..width - 2).map(move |_| mid))
        .chain(iter::once(end))
        .collect()
}
