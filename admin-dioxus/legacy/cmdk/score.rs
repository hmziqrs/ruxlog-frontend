use std::collections::HashMap;

// Scoring constants
const SCORE_CONTINUE_MATCH: f32 = 1.0;
const SCORE_SPACE_WORD_JUMP: f32 = 0.9;
const SCORE_NON_SPACE_WORD_JUMP: f32 = 0.8;
const SCORE_CHARACTER_JUMP: f32 = 0.17;
const SCORE_TRANSPOSITION: f32 = 0.1;
const PENALTY_SKIPPED: f32 = 0.999;
const PENALTY_CASE_MISMATCH: f32 = 0.9999;
const PENALTY_NOT_COMPLETE: f32 = 0.99;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // tests for any “gap” character: \ / _ + . # " @ [ ( { &
    static ref IS_GAP_REGEX: Regex = Regex::new(r#"[\\/_+.#"@\[\(\{&]"#).unwrap();
    // same pattern; in Rust you don’t need a `/g` flag to find all matches,
    // just call `.find_iter()` or `.captures_iter()` to iterate/count
    static ref COUNT_GAPS_REGEX: Regex = Regex::new(r#"[\\/_+.#"@\[\(\{&]"#).unwrap();

    // tests for space or dash
    static ref IS_SPACE_REGEX: Regex = Regex::new(r#"[\s-]"#).unwrap();
    // same here—no extra flag needed
    static ref COUNT_SPACE_REGEX: Regex = Regex::new(r#"[\s-]"#).unwrap();
}


fn command_score_inner(
    string: &str,
    abbreviation: &str,
    lower_string: &str,
    lower_abbreviation: &str,
    string_index: usize,
    abbreviation_index: usize,
    memoized_results: &mut HashMap<String, f32>,
) -> f32 {
    if abbreviation_index == abbreviation.len() {
        if string_index == string.len() {
            return SCORE_CONTINUE_MATCH;
        }
        return PENALTY_NOT_COMPLETE;
    }

    let memoize_key = format!("{},{}", string_index, abbreviation_index);
    if let Some(&score) = memoized_results.get(&memoize_key) {
        return score;
    }

    let abbreviation_char = lower_abbreviation.chars().nth(abbreviation_index).unwrap();
    let mut curr_index = string_index;
    let mut high_score = 0.0;

    while let Some(pos) = lower_string[curr_index..].find(abbreviation_char) {
        let index = curr_index + pos;
        let mut score = command_score_inner(
            string,
            abbreviation,
            lower_string,
            lower_abbreviation,
            index + 1,
            abbreviation_index + 1,
            memoized_results,
        );

        if score > high_score {
            if index == string_index {
                score *= SCORE_CONTINUE_MATCH;
            } else if let Some(prev_char) = string.chars().nth(index.saturating_sub(1)) {
                if IS_GAP_REGEX.is_match(&prev_char.to_string()) {
                    score *= SCORE_NON_SPACE_WORD_JUMP;
                    if let Some(word_breaks) = COUNT_GAPS_REGEX
                        .find_iter(&string[string_index..index.saturating_sub(1)])
                        .count()
                        .checked_sub(0)
                    {
                        if string_index > 0 {
                            score *= PENALTY_SKIPPED.powi(word_breaks as i32);
                        }
                    }
                } else if IS_SPACE_REGEX.is_match(&prev_char.to_string()) {
                    score *= SCORE_SPACE_WORD_JUMP;
                    if let Some(space_breaks) = COUNT_SPACE_REGEX
                        .find_iter(&string[string_index..index.saturating_sub(1)])
                        .count()
                        .checked_sub(0)
                    {
                        if string_index > 0 {
                            score *= PENALTY_SKIPPED.powi(space_breaks as i32);
                        }
                    }
                } else {
                    score *= SCORE_CHARACTER_JUMP;
                    if string_index > 0 {
                        score *= PENALTY_SKIPPED.powi((index - string_index) as i32);
                    }
                }
            }

            if string.chars().nth(index) != abbreviation.chars().nth(abbreviation_index) {
                score *= PENALTY_CASE_MISMATCH;
            }
        }

        let can_transpose = (score < SCORE_TRANSPOSITION)
            && ((lower_string.chars().nth(index.saturating_sub(1))
                == lower_abbreviation.chars().nth(abbreviation_index + 1))
                || (lower_abbreviation.chars().nth(abbreviation_index + 1)
                    == lower_abbreviation.chars().nth(abbreviation_index)
                    && lower_string.chars().nth(index.saturating_sub(1))
                        != lower_abbreviation.chars().nth(abbreviation_index)));

        if can_transpose {
            let transposed_score = command_score_inner(
                string,
                abbreviation,
                lower_string,
                lower_abbreviation,
                index + 1,
                abbreviation_index + 2,
                memoized_results,
            );

            if transposed_score * SCORE_TRANSPOSITION > score {
                score = transposed_score * SCORE_TRANSPOSITION;
            }
        }

        if score > high_score {
            high_score = score;
        }

        curr_index = index + 1;
    }

    memoized_results.insert(memoize_key, high_score);
    high_score
}

fn format_input(string: &str) -> String {
    COUNT_SPACE_REGEX.replace_all(string.to_lowercase().as_str(), " ").to_string()
}

pub fn command_score(string: &str, abbreviation: &str, aliases: Option<&[String]>) -> f32 {
    let string = if let Some(aliases) = aliases {
        format!("{} {}", string, aliases.join(" "))
    } else {
        string.to_string()
    };

    let formatted_string = format_input(&string);
    let formatted_abbrev = format_input(abbreviation);
    
    command_score_inner(
        &string,
        abbreviation,
        &formatted_string,
        &formatted_abbrev,
        0,
        0,
        &mut HashMap::new(),
    )
}