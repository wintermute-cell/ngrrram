use crate::{layout, AppState, Args};
use crossterm::event::{self, KeyCode, KeyEventKind};

use rand::seq::SliceRandom;

// Generates a string of ngrams from the top ngrams in the state, based on the program args.
fn generate_lesson_string(args: &Args, state: &AppState) -> String {

    let mut lesson_string = String::new();
    let mut rng = rand::thread_rng();
    let mut ngrams = state.ngrams.clone();

    // 1. extract args.top many ngrams from state.ngrams
    ngrams.truncate(args.top as usize);

    // 2. randomly choose args.combi many ngrams from the top ngrams then chain ngrams A B C ...
    for _ in 0..args.combi {
        let ngram = ngrams.choose(&mut rng).unwrap();
        lesson_string.push_str(ngram);
        lesson_string.push(' ');
        // NOTE: this also causes the string to end with a space, 
        // which is actually pretty nice
    }

    // 4. repeat the chain args.rep times
    lesson_string.repeat(args.rep as usize)
}

// this function is called every time the game loop runs, before rendering the frame
pub fn run_game(args: &Args, state: &mut AppState, kb_emu: &mut layout::KbEmulator) -> Result<bool, Box<dyn std::error::Error>> {

    // CHECK FOR INPUT
    if state.current_lesson_string.len() > 0 {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                // quit on q or ctrl-c
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Esc
                || (key.modifiers == event::KeyModifiers::CONTROL && key.code == KeyCode::Char('c'))) {
                    return Ok(true);
                }

                else if key.kind == KeyEventKind::Press &&
                    ((key.code == KeyCode::Backspace && key.modifiers == event::KeyModifiers::ALT )
                    || (key.code == KeyCode::Char('h') && key.modifiers == event::KeyModifiers::CONTROL))
                {
                    if state.current_typed_string.len() > 0 {
                        // clear until last space
                        let mut last_space_index = 0;
                        for (i, c) in state.current_typed_string.chars().rev().enumerate() {
                            if i == 0 {
                                continue; // skip the most recent char, so we can delete a word even if we're currently on a space following a word
                            }
                            if c == ' ' {
                                last_space_index = state.current_typed_string.len() - i;
                                break;
                            }
                        }
                        state.current_typed_string.truncate(last_space_index);
                    } 
                }

                else if key.kind == KeyEventKind::Press && key.code == KeyCode::Backspace {
                    if state.current_typed_string.len() > 0 {
                        state.current_typed_string.pop();
                    }
                }

                else if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(mut c) => {
                            if state.use_emulation {
                                match kb_emu.translate(c) {
                                    Some(new_char) => c = new_char,
                                    None => {}
                                }
                            }

                            // check if its something between a-z or A-Z else skip
                            if (c.is_alphabetic() || c == ' ') && state.current_typed_string.len() < state.current_lesson_string.len() {
                                // if this is the first char of the lesson, start the timer
                                if state.current_typed_string.len() == 0 {
                                    state.wpm_start_time = std::time::Instant::now();
                                }
                                state.current_typed_string.push(c);
                                state.acc_key_hits += 1;
                                // check if the last typed char is correct, if not increment misses
                                match state.current_lesson_string.chars().nth(state.current_typed_string.len() - 1) {
                                    Some(lesson_char) => {
                                        if lesson_char != c {
                                            state.acc_key_misses += 1
                                        }
                                    }
                                    None => {}
                                }
                            }
                        }
                        // treat enter as space
                        KeyCode::Enter => {
                            state.current_typed_string.push(' ');
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if state.current_typed_string.len() as i64 == (state.current_lesson_string.len() as i64) - 1 { // we use i64 to avoid underflow
    }

    // CHECK IF LESSON IS SUCCESSFUL, GENERATE NEW LESSON
    if state.current_lesson_string == state.current_typed_string {

        if state.current_lesson_number > 0 {
            // Calculations are done like described here: 
            // https://www.typetolearn.app/knowledge-base/how-words-per-minute-and-accuracy-are-calculated/
            let elapsed_mins = state.wpm_start_time.elapsed().as_secs_f64() / 60.0;
            // we count every 5 chars as a word, but not including spaces since that would make smaller ngrams easier.
            let wpm = (state.current_typed_string.replace(" ", "").len() as f64 / 5.0) / elapsed_mins;
            let acc = (state.acc_key_hits as f64 / (state.acc_key_hits + state.acc_key_misses) as f64) * 100.0;
            state.acc_key_hits = 0;
            state.acc_key_misses = 0;

            // rounding to integers
            state.wpm_history.push(wpm as i32);
            state.acc_history.push(acc as i32);

            // calculate averages so far
            state.average_wpm = state.wpm_history.iter().sum::<i32>() / state.wpm_history.len() as i32;
            state.average_accuracy = state.acc_history.iter().sum::<i32>() / state.acc_history.len() as i32;

            if wpm as i32 >= state.need_wpm && acc as i32 >= state.need_acc {
                state.succeeded_lessons += 1;
            } else {
                state.failed_lessons += 1;
            }
        }

        state.current_lesson_number += 1;
        state.current_typed_string.clear();
        state.current_lesson_string = generate_lesson_string(args, state);
    }

    return Ok(false);
}
