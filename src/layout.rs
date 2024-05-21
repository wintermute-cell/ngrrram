use std::collections::HashMap;

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

        KbEmulator { input_layout, output_layout, }
    }

    pub fn translate(&self, input: char) -> Option<char> {
        self.input_layout.get(&input).map(|&v| self.output_layout[&v])
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
    let string_repr = match layout {
        Layout::Qwerty => {
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ Q │ W │ E │ R │ T │ Y │ U │ I │ O │ P │ [ │ ] │
└┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
 │ A │ S │ D │ F │ G │ H │ J │ K │ L │ ; │ ' │
 └┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
  │ Z │ X │ C │ V │ B │ N │ M │ , │ . │ / │
  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘".to_string()
        }
        Layout::Qwertz => {
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ Q │ W │ E │ R │ T │ Z │ U │ I │ O │ P │ Ü │ + │
└┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
 │ A │ S │ D │ F │ G │ H │ J │ K │ L │ Ö │ Ä │
 └┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
  │ Y │ X │ C │ V │ B │ N │ M │ , │ . │ - │
  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘".to_string()
        }
        Layout::Azerty => {
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ A │ Z │ E │ R │ T │ Y │ U │ I │ O │ P │ ^ │ $ │
└┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
 │ Q │ S │ D │ F │ G │ H │ J │ K │ L │ M │ ù │
 └┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
  │ W │ X │ C │ V │ B │ N │ , │ ; │ : │ ! │
  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘".to_string()
        }
        Layout::Dvorak => {
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ ' │ , │ . │ P │ Y │ F │ G │ C │ R │ L │ / │ = │
└┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
 │ A │ O │ E │ U │ I │ D │ H │ T │ N │ S │ - │
 └┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
  │ ; │ Q │ J │ K │ X │ B │ M │ W │ V │ Z │
  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘".to_string()
        }
        Layout::Colemak => {
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ Q │ W │ F │ P │ G │ J │ L │ U │ Y │ ; │ [ │ ] │
└┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
 │ A │ R │ S │ T │ D │ H │ N │ E │ I │ O │ ' │
 └┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
  │ Z │ X │ C │ V │ B │ K │ M │ , │ . │ / │
  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘".to_string()
        }
        Layout::ColemakDH => {
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ Q │ W │ F │ P │ B │ J │ L │ U │ Y │ ; │ [ │ ] │
└┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
 │ A │ R │ S │ T │ G │ M │ N │ E │ I │ O │ ' │
 └┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┴┬──┘
  │ Z │ X │ C │ D │ V │ K │ H │ , │ . │ / │
  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘".to_string()
        }
    };
    string_repr
}

// Returs the QWERTY layout, including symbols on the base layer.
fn qwerty() -> HashMap<char, u8> {
    let mut layout = HashMap::new();
    layout.insert('1', 1);
    layout.insert('2', 2);
    layout.insert('3', 3);
    layout.insert('4', 4);
    layout.insert('5', 5);
    layout.insert('6', 6);
    layout.insert('7', 7);
    layout.insert('8', 8);
    layout.insert('9', 9);
    layout.insert('0', 10);
    layout.insert('-', 11);
    layout.insert('=', 12);
    layout.insert('q', 13);
    layout.insert('w', 14);
    layout.insert('e', 15);
    layout.insert('r', 16);
    layout.insert('t', 17);
    layout.insert('y', 18);
    layout.insert('u', 19);
    layout.insert('i', 20);
    layout.insert('o', 21);
    layout.insert('p', 22);
    layout.insert('[', 23);
    layout.insert(']', 24);
    //layout.insert('\\',25); // we ignore this key since it is not relevant for layout emulation and only exists on ANSI keyboards
    layout.insert('a', 25);
    layout.insert('s', 26);
    layout.insert('d', 27);
    layout.insert('f', 28);
    layout.insert('g', 29);
    layout.insert('h', 30);
    layout.insert('j', 31);
    layout.insert('k', 32);
    layout.insert('l', 33);
    layout.insert(';', 34);
    layout.insert('\'',35);
    layout.insert('z', 36);
    layout.insert('x', 37);
    layout.insert('c', 38);
    layout.insert('v', 39);
    layout.insert('b', 40);
    layout.insert('n', 41);
    layout.insert('m', 42);
    layout.insert(',', 43);
    layout.insert('.', 44);
    layout.insert('/', 45);
    layout.insert(' ', 46);
    layout
}

fn qwertz() -> HashMap<char, u8> {
    let mut layout = HashMap::new();
    layout.insert('1', 1);
    layout.insert('2', 2);
    layout.insert('3', 3);
    layout.insert('4', 4);
    layout.insert('5', 5);
    layout.insert('6', 6);
    layout.insert('7', 7);
    layout.insert('8', 8);
    layout.insert('9', 9);
    layout.insert('0', 10);
    layout.insert('ß', 11);
    layout.insert('´', 12);
    layout.insert('q', 13);
    layout.insert('w', 14);
    layout.insert('e', 15);
    layout.insert('r', 16);
    layout.insert('t', 17);
    layout.insert('z', 18);
    layout.insert('u', 19);
    layout.insert('i', 20);
    layout.insert('o', 21);
    layout.insert('p', 22);
    layout.insert('ü', 23);
    layout.insert('+', 24);
    layout.insert('a', 25);
    layout.insert('s', 26);
    layout.insert('d', 27);
    layout.insert('f', 28);
    layout.insert('g', 29);
    layout.insert('h', 30);
    layout.insert('j', 31);
    layout.insert('k', 32);
    layout.insert('l', 33);
    layout.insert('ö', 34);
    layout.insert('ä', 35);
    //layout.insert('\'', 36); // we ignore this key since it is not relevant for layout emulation and only exists on ISO keyboards
    //layout.insert('<', 36); // we ignore this key since it is not relevant for layout emulation and only exists on ISO keyboards
    layout.insert('y', 36);
    layout.insert('x', 37);
    layout.insert('c', 38);
    layout.insert('v', 39);
    layout.insert('b', 40);
    layout.insert('n', 41);
    layout.insert('m', 42);
    layout.insert(',', 43);
    layout.insert('.', 44);
    layout.insert('-', 45);
    layout.insert(' ', 46);
    layout
}

fn azerty() -> HashMap<char, u8> {
    let mut layout = HashMap::new();
    // yes, the french have numbers on the shift layer
    layout.insert('&', 1); // '1' key on QWERTY
    layout.insert('é', 2); // '2' key on QWERTY
    layout.insert('"', 3); // '3' key on QWERTY
    layout.insert('\'', 4); // '4' key on QWERTY
    layout.insert('(', 5); // '5' key on QWERTY
    layout.insert('-', 6); // '6' key on QWERTY
    layout.insert('è', 7); // '7' key on QWERTY
    layout.insert('_', 8); // '8' key on QWERTY
    layout.insert('ç', 9); // '9' key on QWERTY
    layout.insert('à', 10); // '0' key on QWERTY
    layout.insert(')', 11); // '-' key on QWERTY
    layout.insert('=', 12); // '=' key on QWERTY
    layout.insert('a', 13); // 'q' key on QWERTY
    layout.insert('z', 14); // 'w' key on QWERTY
    layout.insert('e', 15);
    layout.insert('r', 16);
    layout.insert('t', 17);
    layout.insert('y', 18);
    layout.insert('u', 19);
    layout.insert('i', 20);
    layout.insert('o', 21);
    layout.insert('p', 22);
    layout.insert('^', 23); // '[' key on QWERTY
    layout.insert('$', 24); // ']' key on QWERTY
    //layout.insert('\\', 25); // we ignore this key since it is not relevant for layout emulation and only exists on ANSI keyboards
    layout.insert('q', 25); // 'a' key on QWERTY
    layout.insert('s', 26);
    layout.insert('d', 27);
    layout.insert('f', 28);
    layout.insert('g', 29);
    layout.insert('h', 30);
    layout.insert('j', 31);
    layout.insert('k', 32);
    layout.insert('l', 33);
    layout.insert('m', 34); // ';' key on QWERTY
    layout.insert('ù', 35); // '\'' key on QWERTY
    //layout.insert('*', 37); // we ignore this key since it is not relevant for layout emulation and only exists on ISO keyboards
    layout.insert('w', 36); // 'z' key on QWERTY
    layout.insert('x', 37);
    layout.insert('c', 38);
    layout.insert('v', 39);
    layout.insert('b', 40);
    layout.insert('n', 41);
    layout.insert(',', 42);
    layout.insert(';', 43);
    layout.insert(':', 44);
    layout.insert('!', 45);
    layout.insert(' ', 46);
    layout
}

fn dvorak() -> HashMap<char, u8> {
    let mut layout = HashMap::new();
    layout.insert('1', 1);
    layout.insert('2', 2);
    layout.insert('3', 3);
    layout.insert('4', 4);
    layout.insert('5', 5);
    layout.insert('6', 6);
    layout.insert('7', 7);
    layout.insert('8', 8);
    layout.insert('9', 9);
    layout.insert('0', 10);
    layout.insert('[', 11);
    layout.insert(']', 12);
    layout.insert('\'', 13);
    layout.insert(',', 14);
    layout.insert('.', 15);
    layout.insert('p', 16);
    layout.insert('y', 17);
    layout.insert('f', 18);
    layout.insert('g', 19);
    layout.insert('c', 20);
    layout.insert('r', 21);
    layout.insert('l', 22);
    layout.insert('/', 23);
    layout.insert('=', 24);
    layout.insert('a', 25);
    layout.insert('o', 26);
    layout.insert('e', 27);
    layout.insert('u', 28);
    layout.insert('i', 29);
    layout.insert('d', 30);
    layout.insert('h', 31);
    layout.insert('t', 32);
    layout.insert('n', 33);
    layout.insert('s', 34);
    layout.insert('-', 35);
    layout.insert(';', 36);
    layout.insert('q', 37);
    layout.insert('j', 38);
    layout.insert('k', 39);
    layout.insert('x', 40);
    layout.insert('b', 41);
    layout.insert('m', 42);
    layout.insert('w', 43);
    layout.insert('v', 44);
    layout.insert('z', 45);
    layout.insert(' ', 46);
    layout
}

fn colemak() -> HashMap<char, u8> {
    let mut layout = HashMap::new();
    layout.insert('1', 1);
    layout.insert('2', 2);
    layout.insert('3', 3);
    layout.insert('4', 4);
    layout.insert('5', 5);
    layout.insert('6', 6);
    layout.insert('7', 7);
    layout.insert('8', 8);
    layout.insert('9', 9);
    layout.insert('0', 10);
    layout.insert('-', 11);
    layout.insert('=', 12);
    layout.insert('q', 13);
    layout.insert('w', 14);
    layout.insert('f', 15);
    layout.insert('p', 16);
    layout.insert('g', 17);
    layout.insert('j', 18);
    layout.insert('l', 19);
    layout.insert('u', 20);
    layout.insert('y', 21);
    layout.insert(';', 22);
    layout.insert('[', 23);
    layout.insert(']', 24);
    layout.insert('a', 25);
    layout.insert('r', 26);
    layout.insert('s', 27);
    layout.insert('t', 28);
    layout.insert('d', 29);
    layout.insert('h', 30);
    layout.insert('n', 31);
    layout.insert('e', 32);
    layout.insert('i', 33);
    layout.insert('o', 34);
    layout.insert('\'',35);
    layout.insert('z', 36);
    layout.insert('x', 37);
    layout.insert('c', 38);
    layout.insert('v', 39);
    layout.insert('b', 40);
    layout.insert('k', 41);
    layout.insert('m', 42);
    layout.insert(',', 43);
    layout.insert('.', 44);
    layout.insert('/', 45);
    layout.insert(' ', 46);
    layout
}

fn colemak_dh() -> HashMap<char, u8> {
    let mut layout = HashMap::new();
    layout.insert('1', 1);
    layout.insert('2', 2);
    layout.insert('3', 3);
    layout.insert('4', 4);
    layout.insert('5', 5);
    layout.insert('6', 6);
    layout.insert('7', 7);
    layout.insert('8', 8);
    layout.insert('9', 9);
    layout.insert('0', 10);
    layout.insert('-', 11);
    layout.insert('=', 12);
    layout.insert('q', 13);
    layout.insert('w', 14);
    layout.insert('f', 15);
    layout.insert('p', 16);
    layout.insert('b', 17);
    layout.insert('j', 18);
    layout.insert('l', 19);
    layout.insert('u', 20);
    layout.insert('y', 21);
    layout.insert(';', 22);
    layout.insert('[', 23);
    layout.insert(']', 24);
    layout.insert('a', 25);
    layout.insert('r', 26);
    layout.insert('s', 27);
    layout.insert('t', 28);
    layout.insert('g', 29);
    layout.insert('m', 30);
    layout.insert('n', 31);
    layout.insert('e', 32);
    layout.insert('i', 33);
    layout.insert('o', 34);
    layout.insert('\'',35);
    layout.insert('z', 36);
    layout.insert('x', 37);
    layout.insert('c', 38);
    layout.insert('d', 39);
    layout.insert('v', 40);
    layout.insert('k', 41);
    layout.insert('h', 42);
    layout.insert(',', 43);
    layout.insert('.', 44);
    layout.insert('/', 45);
    layout.insert(' ', 46);
    layout
}


