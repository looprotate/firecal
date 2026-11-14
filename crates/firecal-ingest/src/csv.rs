//! Borrowed single-line CSV fields; doubled quotes remain escaped in unused fields.

use crate::ParseError;

pub(crate) fn fields(bytes: &[u8]) -> Result<Vec<&str>, ParseError> {
    let text = std::str::from_utf8(bytes).map_err(|_| ParseError::InvalidUtf8)?;
    let text = text
        .strip_prefix('\u{feff}')
        .unwrap_or(text)
        .trim_end_matches(['\r', '\n']);
    if text.trim().is_empty() {
        return Err(ParseError::EmptyRecord);
    }
    let mut result = Vec::new();
    let mut start = 0;
    let mut quoted = false;
    for (index, byte) in text.bytes().enumerate() {
        if byte == b'"' {
            quoted = !quoted;
        } else if byte == b',' && !quoted {
            result.push(field(
                text.get(start..index).ok_or(ParseError::InvalidCsv)?,
            )?);
            start = index + 1;
        } else if matches!(byte, b'\r' | b'\n') {
            return Err(ParseError::InvalidCsv);
        }
    }
    if quoted {
        return Err(ParseError::InvalidCsv);
    }
    result.push(field(text.get(start..).ok_or(ParseError::InvalidCsv)?)?);
    Ok(result)
}

fn field(text: &str) -> Result<&str, ParseError> {
    let text = text.trim();
    if let Some(inner) = text.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        let mut chars = inner.chars();
        while let Some(character) = chars.next() {
            if character == '"' && chars.next() != Some('"') {
                return Err(ParseError::InvalidCsv);
            }
        }
        return Ok(inner);
    }
    if text.contains('"') {
        return Err(ParseError::InvalidCsv);
    }
    Ok(text)
}

#[cfg(test)]
mod tests;
