use crate::args::Color;
use std::iter;

use colored::*;

use super::{Cell, CellType, DrawOptions, Header};

const LEFT_TOP: char = '┌';
const RIGHT_TOP: char = '┐';
const LEFT_BOTTOM: char = '└';
const RIGHT_BOTTOM: char = '┘';
const HORIZONTAL: char = '─';
const HORIZONTAL_DOWN: char = '┬';
const HORIZONTAL_UP: char = '┴';
const CROSS: char = '┼';
const VERTICAL: char = '│';
const VERTICAL_RIGHT: char = '┤';
const VERTICAL_LEFT: char = '├';
const NEW_LINE: char = '\n';

pub fn draw_table(headers: &[Header], rows: &Vec<Vec<Cell>>, options: DrawOptions) -> String {
    top_border(headers)
        .chain(header_row(headers, &options))
        .chain(rows.iter().flat_map(|row| {
            return row_seperator(headers)
                .chain(content_row(row, headers, &options));
        }))
        .chain(bottom_border(headers))
        .collect::<String>()
}

// Returns top border of table
// Example: ┌────────┬────────┬────────┐
fn top_border(headers: &[Header]) -> impl Iterator<Item = char> + '_ {
    border(headers, LEFT_TOP, HORIZONTAL_DOWN, RIGHT_TOP)
}

// Returns bottom border of table
// Example: └────────┴────────┴────────┘
fn bottom_border(headers: &[Header]) -> impl Iterator<Item = char> + '_ {
    border(headers, LEFT_BOTTOM, HORIZONTAL_UP, RIGHT_BOTTOM)
}

// Returns a seperator between rows
// Example: ├────────┼────────┼────────┤
fn row_seperator(headers: &[Header]) -> impl Iterator<Item = char> + '_ {
    border(headers, VERTICAL_LEFT, CROSS, VERTICAL_RIGHT)
}

// Returns a header row
// Example: │header1 │header2 │header3 │
fn header_row<'a>(headers: &'a [Header], options: &'a DrawOptions) -> impl Iterator<Item = char>  + 'a {
    iter::once(VERTICAL)
        .chain(headers.iter().flat_map(|header| {
            format_header(&header.name, header.max_width, &options.color)
                .chars()
                .chain(iter::once(VERTICAL))
                .collect::<Vec<_>>()
        }))
        .chain(iter::once(NEW_LINE))
}

// Returns a content row
// Example: │"value1"│"value2"│"value3"│
fn content_row<'a>(row: &'a Vec<Cell>, headers: &'a [Header], options: &'a DrawOptions) -> impl Iterator<Item = char> + 'a {
    iter::once(VERTICAL)
        .chain(headers.iter().enumerate().flat_map(|(i, header)| {
            format_cell(&row[i], header.max_width, &options.color)
                .chars()
                .chain(iter::once(VERTICAL))
                .collect::<Vec<_>>()
        }))
        .chain(iter::once(NEW_LINE))
}

fn border(headers: &[Header], left: char, mid: char, right: char) -> impl Iterator<Item = char> + '_ {
    iter::once(left) // Start with :left
        .chain(headers[..headers.len() - 1].iter().flat_map(move |header| {
            (0..header.max_width) // repeat ────────:mid for every header other than last
                .map(|_| HORIZONTAL)
                .chain(iter::once(mid))
        }))
        .chain((0..headers.last().unwrap().max_width).map(|_| HORIZONTAL)) // ──────── for last header
        .chain(iter::once(right)) // End with :right
        .chain(iter::once(NEW_LINE))
}

fn format_cell(cell: &Cell, width: usize, color: &Color) -> String {
    let padded = format!("{:<width$}", cell.content, width = width);

    match color {
        Color::Never => padded,
        Color::Auto => colorize(&padded, &cell.cell_type), // Fix with atty
        Color::Always => colorize(&padded, &cell.cell_type),
    }
}

fn format_header(s: &str, width: usize, color: &Color) -> String {
    let padded = format!("{:<width$}", s, width = width);

    match color {
        Color::Never => padded,
        Color::Auto => padded.blue().bold().to_string(),
        Color::Always => padded.blue().bold().to_string(),
    }
}

fn colorize(content: &str, cell_type: &CellType) -> String {
    match cell_type {
        CellType::Null => content.white().dimmed(),
        CellType::Bool => content.white(),
        CellType::Number => content.yellow(),
        CellType::String => content.green(),
        CellType::Collapsed => content.white().dimmed(),
    }
    .to_string()
}
