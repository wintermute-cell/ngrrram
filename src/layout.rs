use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

pub enum Layout {
    Qwerty,
    Qwertz,
    Azerty,
    Dvorak,
    Colemak,
    ColemakDH,
}

pub struct KbEmulator {
    input_layout: HashMap<char, u8>,
    output_layout: HashMap<u8, char>,
}

impl KbEmulator {
    pub fn new(input: Layout, output: Layout) -> Self {
        let input_layout = get_layout(input);
        let output_layout = make_output_map(get_layout(output));

        KbEmulator {
            input_layout,
            output_layout,
        }
    }

    pub fn translate(&self, input: char) -> Option<char> {
        self.input_layout
            .get(&input)
            .map(|&v| self.output_layout[&v])
    }
}

fn make_output_map(layout: HashMap<char, u8>) -> HashMap<u8, char> {
    layout.into_iter().map(|(k, v)| (v, k)).collect()
}

fn get_layout(layout: Layout) -> HashMap<char, u8> {
    match layout {
        Layout::Qwerty => qwerty(),
        Layout::Qwertz => qwertz(),
        Layout::Azerty => azerty(),
        Layout::Dvorak => dvorak(),
        Layout::Colemak => colemak(),
        Layout::ColemakDH => colemak_dh(),
    }
}

pub fn get_layout_string(layout: &Layout) -> String {
    match layout {
        Layout::Qwerty => render_qwerty(),
        Layout::Qwertz => render_qwertz(),
        Layout::Azerty => render_azerty(),
        Layout::Dvorak => render_dvorak(),
        Layout::Colemak => render_colemak(),
        Layout::ColemakDH => render_colemak_dh(),
    }
}

fn render_map(map: &[char]) -> String {
    let mut rows = Vec::new();
    // Newline is used as a marker for rows of keys. It is safe since it is not a valid key.
    for (_, chunk) in &map.iter().chunk_by(|elt| **elt != '\n') {
        let row: Vec<char> = chunk.copied().collect();
        // chunk by will leave one element chunks of the newline character. Skip these.
        if row.len() > 1 {
            rows.push(row);
        }
    }

    // Now that the rows of keys have been collected they can be processed.
    // Having the rows pre-processed enables the logic below to use the row count
    // instead of hard coding anything.

    let mut lines: Vec<String> = Vec::new();

    // The first line is skipped because the number rows are not used or shown.
    for (row_index, row) in rows.iter().enumerate().skip(1) {
        // First row gets the "cap".
        if row_index == 1 {
            let line = "┌"
                .chars()
                .chain(
                    iter::repeat(['─', '─', '─', '┬'])
                        .take(row.len() - 1)
                        .flatten(),
                )
                .chain(['─', '─', '─', '┐'])
                .collect();
            lines.push(line);
        }

        let line: String = row
            .iter()
            .flat_map(|c| ['│', ' ', *c, ' '])
            .chain(['│'])
            .collect();
        lines.push(format!("{}{line}", " ".repeat(row_index - 1)));

        // Every row but the last one has connectors to the next row.
        let connector = if row_index == rows.len() - 1 {
            '─'
        } else {
            '┬'
        };
        let line: String = "└"
            .chars()
            .chain(
                iter::repeat([connector, '─', '─', '┴'])
                    .take(row.len() - 1)
                    .flatten(),
            )
            .chain([connector, '─', '─', '┘'])
            .collect();
        lines.push(format!("{}{line}", " ".repeat(row_index - 1)));
    }
    lines.join("\n")
}

fn make_keymap(map: &[char]) -> HashMap<char, u8> {
    map.iter()
        .enumerate()
        .map(|(index, key)| (*key, (index + 1) as u8))
        .collect()
}

macro_rules! layout {
    // layout name and array of keys
    ($func_name:ident, $keymap:expr) => {
        fn $func_name() -> HashMap<char, u8> {
            make_keymap(&$keymap)
        }

        paste::item! {
            fn [< render_ $func_name >] () -> String {
                render_map(&$keymap)
            }
        }
    };
}

layout! {
    qwerty,
    [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\n',
        'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
        'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '\n',
        'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\n',
        ' ',
    ]
}

// Keys that are not relevant and only on ANSI or ISO keyboards are not represented.

layout! {
    qwertz,
    [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'ß', '´', '\n',
        'q', 'w', 'e', 'r', 't', 'z', 'u', 'i', 'o', 'p', 'ü', '+', '\n',
        'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ö', 'ä', '\n',
        'y', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-', '\n',
        ' ',
    ]
}

layout! {
    azerty,
    [
        // yes, the French have numbers on the shift layer
        '&', 'é', '"', '\'', '(', '-', 'è', '_', 'ç', 'à', ')', '=', '\n',
        'a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '^', '$', '\n',
        'q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'ù', '\n',
        'w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '!', '\n',
        ' ',
    ]
}

layout! {
    dvorak,
    [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '[', ']', '\n',
        '\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l', '/', '=', '\n',
        'a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's', '-', '\n',
        ';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z', '\n',
        ' ',
    ]
}

layout! {
    colemak,
    [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\n',
        'q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';', '[', ']', '\n',
        'a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o', '\'', '\n',
        'z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/', '\n',
        ' ',
    ]
}

layout! {
    colemak_dh,
    [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\n',
        'q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';', '[', ']', '\n',
        'a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o', '\'', '\n',
        'z', 'x', 'c', 'd', 'v', 'k', 'h', ',', '.', '/', '\n',
        ' ',
    ]
}
