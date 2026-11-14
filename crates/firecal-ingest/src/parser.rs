//! Header-directed FIRMS rows, with filtering distinct from malformed records.

use firecal_types::{MonthIndex, Satellite};

/// A nominal- or high-confidence vegetation hotspot kept for aggregation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hotspot {
    /// Latitude in degrees, between -90 and 90 inclusive.
    pub latitude: f64,
    /// Longitude in degrees, between -180 and 180 inclusive.
    pub longitude: f64,
    /// Acquisition month since January 2000.
    pub month: MonthIndex,
    /// Day within the acquisition month, validated against the calendar.
    pub day: u8,
    /// Satellite carrying the matching sensor.
    pub satellite: Satellite,
    /// Nonnegative, finite fire radiative power in megawatts.
    pub frp_megawatts: f64,
    /// Whether FIRMS marks the acquisition as daytime.
    pub daytime: bool,
}

/// A rejected row or header; intentional filtering has its own variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// The bytes are not UTF-8.
    InvalidUtf8,
    /// A quoted CSV field is unclosed or has text after its closing quote.
    InvalidCsv,
    /// A required header column is absent.
    MissingColumn,
    /// Two header columns have the same name.
    DuplicateColumn,
    /// A blank row is skipped without producing a hotspot.
    EmptyRecord,
    /// The record has a different number of columns from its header.
    ColumnCount,
    /// A required numeric field is invalid, non-finite, or outside its domain.
    InvalidNumber,
    /// The acquisition date is not a supported Gregorian YYYY-MM-DD date.
    InvalidDate,
    /// The satellite name or sensor is unknown, or the pair disagrees.
    InvalidSatellite,
    /// Confidence is not MODIS 0..100 or VIIRS l/n/h.
    InvalidConfidence,
    /// The daynight field is neither D nor N.
    InvalidDaynight,
    /// A valid low-confidence record is intentionally dropped.
    LowConfidence,
    /// A valid non-vegetation type (1, 2, 3) is intentionally dropped.
    NonVegetation,
}

/// Column names parsed once and reused for every row of one FIRMS file.
///
/// Columns are resolved by name. Extra columns are accepted; `type` is optional
/// because NRT CSVs omit it. In that case no source-type filter can be applied.
#[derive(Debug, Clone)]
pub struct CsvHeader {
    columns: Vec<String>,
}

impl CsvHeader {
    /// Reads a UTF-8 header, accepting a leading BOM and CRLF or LF endings.
    ///
    /// # Errors
    /// Returns a CSV/encoding error, [`ParseError::MissingColumn`] or
    /// [`ParseError::DuplicateColumn`]. Names are case-sensitive.
    pub fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        let columns: Vec<String> = crate::csv::fields(bytes)?
            .into_iter()
            .map(str::to_owned)
            .collect();
        for (index, column) in columns.iter().enumerate() {
            if columns.iter().take(index).any(|other| other == column) {
                return Err(ParseError::DuplicateColumn);
            }
        }
        for required in [
            "latitude",
            "longitude",
            "acq_date",
            "satellite",
            "instrument",
            "confidence",
            "frp",
            "daynight",
        ] {
            if !columns.iter().any(|name| name == required) {
                return Err(ParseError::MissingColumn);
            }
        }
        Ok(Self { columns })
    }

    /// Parses one row using this file's header, retaining nominal/high detections.
    ///
    /// MODIS accepts Terra/T and Aqua/A; VIIRS accepts N/S-NPP, 1/N20/NOAA-20,
    /// and 2/N21/NOAA-21. Dates and required fields are validated before filtering.
    ///
    /// # Errors
    /// Returns [`ParseError::EmptyRecord`], [`ParseError::LowConfidence`] or
    /// [`ParseError::NonVegetation`] for intentionally skipped rows. Other
    /// variants indicate malformed data and must not silently be counted.
    pub fn parse_record(&self, bytes: &[u8]) -> Result<Hotspot, ParseError> {
        let fields = crate::csv::fields(bytes)?;
        if fields.len() != self.columns.len() {
            return Err(ParseError::ColumnCount);
        }
        let field = |name| {
            self.columns
                .iter()
                .zip(&fields)
                .find_map(|(column, value)| (column == name).then_some(*value))
                .ok_or(ParseError::MissingColumn)
        };
        let latitude = number(field("latitude")?, -90.0, 90.0)?;
        let longitude = number(field("longitude")?, -180.0, 180.0)?;
        let frp_megawatts = number(field("frp")?, 0.0, f64::MAX)?;
        let (month, day) = crate::date::parse(field("acq_date")?)?;
        let satellite = satellite(field("satellite")?, field("instrument")?)?;
        let kept = confidence(field("confidence")?, satellite.is_viirs())?;
        let daytime = match field("daynight")? {
            "D" => true,
            "N" => false,
            _ => return Err(ParseError::InvalidDaynight),
        };
        let vegetation = match field("type") {
            Ok("0") | Err(ParseError::MissingColumn) => true,
            Ok("1" | "2" | "3") => false,
            _ => return Err(ParseError::InvalidNumber),
        };
        if !kept {
            return Err(ParseError::LowConfidence);
        }
        if !vegetation {
            return Err(ParseError::NonVegetation);
        }
        Ok(Hotspot {
            latitude,
            longitude,
            month,
            day,
            satellite,
            frp_megawatts,
            daytime,
        })
    }
}

fn number(text: &str, minimum: f64, maximum: f64) -> Result<f64, ParseError> {
    let value: f64 = text.parse().map_err(|_| ParseError::InvalidNumber)?;
    if !value.is_finite() || !(minimum..=maximum).contains(&value) {
        return Err(ParseError::InvalidNumber);
    }
    Ok(value)
}

fn satellite(name: &str, instrument: &str) -> Result<Satellite, ParseError> {
    match (name, instrument) {
        ("T" | "Terra", "MODIS") => Ok(Satellite::Terra),
        ("A" | "Aqua", "MODIS") => Ok(Satellite::Aqua),
        ("N" | "S-NPP" | "Suomi NPP", "VIIRS") => Ok(Satellite::SuomiNpp),
        ("1" | "N20" | "NOAA-20", "VIIRS") => Ok(Satellite::Noaa20),
        ("2" | "N21" | "NOAA-21", "VIIRS") => Ok(Satellite::Noaa21),
        _ => Err(ParseError::InvalidSatellite),
    }
}

fn confidence(text: &str, viirs: bool) -> Result<bool, ParseError> {
    if viirs {
        return match text {
            "l" => Ok(false),
            "n" | "h" => Ok(true),
            _ => Err(ParseError::InvalidConfidence),
        };
    }
    let value: u8 = text.parse().map_err(|_| ParseError::InvalidConfidence)?;
    if value > 100 {
        return Err(ParseError::InvalidConfidence);
    }
    Ok(value >= 30)
}

#[cfg(test)]
mod tests;
