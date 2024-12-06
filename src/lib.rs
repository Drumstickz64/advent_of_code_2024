use std::{fmt, str::FromStr};

pub mod template;

// Use this file to add helper functions and additional modules.
pub fn parse_number<T>(input: &str) -> Result<(&str, T), &str>
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    let mut curr = 0;
    while input.as_bytes()[curr].is_ascii_digit() {
        curr += 1;
    }

    if curr == 0 {
        return Err(input);
    }

    Ok((&input[curr..], input[..curr].parse().unwrap()))
}

pub fn match_pat<'input>(input: &'input str, pat: &str) -> Result<&'input str, &'input str> {
    if !input.starts_with(pat) {
        return Err(input);
    }

    Ok(&input[pat.len()..])
}
