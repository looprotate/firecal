//! Gregorian acquisition dates, including leap days, without timezone assumptions.

use crate::ParseError;
use firecal_types::MonthIndex;

pub(crate) fn parse(text: &str) -> Result<(MonthIndex, u8), ParseError> {
    let bytes = text.as_bytes();
    if bytes.len() != 10
        || bytes.get(4) != Some(&b'-')
        || bytes.get(7) != Some(&b'-')
        || !bytes
            .iter()
            .enumerate()
            .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
    {
        return Err(ParseError::InvalidDate);
    }
    let year: u16 = text
        .get(..4)
        .ok_or(ParseError::InvalidDate)?
        .parse()
        .map_err(|_| ParseError::InvalidDate)?;
    let month: u16 = text
        .get(5..7)
        .ok_or(ParseError::InvalidDate)?
        .parse()
        .map_err(|_| ParseError::InvalidDate)?;
    let day: u8 = text
        .get(8..)
        .ok_or(ParseError::InvalidDate)?
        .parse()
        .map_err(|_| ParseError::InvalidDate)?;
    let index = MonthIndex::new(year, month).map_err(|_| ParseError::InvalidDate)?;
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if day == 0 || day > days {
        return Err(ParseError::InvalidDate);
    }
    Ok((index, day))
}

#[cfg(test)]
mod tests;
