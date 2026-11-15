use super::{CsvHeader, ParseError};
use firecal_types::{MonthIndex, Satellite};

const HEADER: &[u8] =
    b"latitude,longitude,acq_date,satellite,instrument,confidence,frp,daynight,type";
const ROW: &str = "-33.5,-70.25,2024-02-29,Terra,MODIS,30,12.5,D,0";

fn parse(row: &str) -> Result<super::Hotspot, ParseError> {
    CsvHeader::parse(HEADER)
        .unwrap()
        .parse_record(row.as_bytes())
}

#[test]
fn modis_nominal_row_has_hand_checked_fields() {
    let record = parse(ROW).unwrap();
    assert_eq!(record.latitude.to_bits(), (-33.5_f64).to_bits());
    assert_eq!(record.longitude.to_bits(), (-70.25_f64).to_bits());
    assert_eq!(record.frp_megawatts.to_bits(), 12.5_f64.to_bits());
    assert_eq!(record.month, MonthIndex::from_index(289));
    assert_eq!(record.day, 29);
    assert_eq!(record.satellite, Satellite::Terra);
    assert!(record.daytime);
}

#[test]
fn headers_resolve_by_name_with_quotes_extra_column_bom_and_crlf() {
    let header = CsvHeader::parse(b"\xef\xbb\xbfextra,frp,confidence,instrument,satellite,acq_date,longitude,daynight,latitude\r\n").unwrap();
    let record = header
        .parse_record(b"\"unused, text\",12.5,h,VIIRS,NOAA-20,2024-02-29,-70.25,N,-33.5\r\n")
        .unwrap();
    assert_eq!(record.satellite, Satellite::Noaa20);
    assert!(!record.daytime);
    assert_eq!(record.frp_megawatts.to_bits(), 12.5_f64.to_bits());
}

#[test]
fn five_satellites_and_their_archive_aliases_are_supported() {
    for (name, instrument, confidence, expected) in [
        ("T", "MODIS", "80", Satellite::Terra),
        ("A", "MODIS", "100", Satellite::Aqua),
        ("Aqua", "MODIS", "79", Satellite::Aqua),
        ("N", "VIIRS", "n", Satellite::SuomiNpp),
        ("S-NPP", "VIIRS", "h", Satellite::SuomiNpp),
        ("Suomi NPP", "VIIRS", "n", Satellite::SuomiNpp),
        ("1", "VIIRS", "h", Satellite::Noaa20),
        ("NOAA-20", "VIIRS", "n", Satellite::Noaa20),
        ("N20", "VIIRS", "n", Satellite::Noaa20),
        ("2", "VIIRS", "n", Satellite::Noaa21),
        ("NOAA-21", "VIIRS", "h", Satellite::Noaa21),
        ("N21", "VIIRS", "h", Satellite::Noaa21),
    ] {
        let row = format!("0,0,2024-01-01,{name},{instrument},{confidence},0,D,0");
        assert_eq!(parse(&row).unwrap().satellite, expected, "{row}");
    }
}

#[test]
fn confidence_boundaries_are_kept_or_explicitly_filtered() {
    for (confidence, kept) in [
        ("0", false),
        ("29", false),
        ("30", true),
        ("79", true),
        ("80", true),
        ("100", true),
    ] {
        let row = ROW.replace(",30,", &format!(",{confidence},"));
        if kept {
            assert!(parse(&row).is_ok());
        } else {
            assert_eq!(parse(&row), Err(ParseError::LowConfidence));
        }
    }
    assert_eq!(
        parse(&ROW.replace("Terra,MODIS,30", "N,VIIRS,l")),
        Err(ParseError::LowConfidence)
    );
    for value in ["101", "-1", "30.5", "n", "NaN", ""] {
        assert_eq!(
            parse(&ROW.replace(",30,", &format!(",{value},"))),
            Err(ParseError::InvalidConfidence)
        );
    }
    assert_eq!(
        parse(&ROW.replace("Terra,MODIS,30", "N,VIIRS,80")),
        Err(ParseError::InvalidConfidence)
    );
}

#[test]
fn non_vegetation_is_filtered_and_unknown_type_is_rejected() {
    for kind in ["1", "2", "3"] {
        assert_eq!(
            parse(&ROW.replace(",D,0", &format!(",D,{kind}"))),
            Err(ParseError::NonVegetation)
        );
    }
    for kind in ["4", "-1", "x", ""] {
        assert_eq!(
            parse(&ROW.replace(",D,0", &format!(",D,{kind}"))),
            Err(ParseError::InvalidNumber)
        );
    }
}

#[test]
fn negative_frp_keeps_the_detection_with_zero_megawatts() {
    for negative in ["-1", "-0.3", "-0"] {
        let record = parse(&ROW.replace("12.5", negative)).unwrap();
        assert_eq!(
            record.frp_megawatts.to_bits(),
            0.0_f64.to_bits(),
            "{negative}"
        );
        assert_eq!(record.satellite, Satellite::Terra);
    }
}

#[test]
fn malformed_numeric_fields_are_rejected_even_on_filtered_rows() {
    for (source, bad) in [
        ("-33.5", "91"),
        ("-33.5", "-90.01"),
        ("-33.5", "NaN"),
        ("-70.25", "181"),
        ("-70.25", "-180.01"),
        ("-70.25", "inf"),
        ("12.5", "NaN"),
        ("12.5", "1e999"),
        ("12.5", "oops"),
        ("12.5", ""),
    ] {
        assert_eq!(
            parse(&ROW.replace(source, bad)),
            Err(ParseError::InvalidNumber),
            "{source} -> {bad}"
        );
    }
    assert_eq!(
        parse(&ROW.replace("-33.5", "91").replace(",30,", ",0,")),
        Err(ParseError::InvalidNumber)
    );
    for (lat, lon) in [("90", "180"), ("-90", "-180")] {
        assert!(parse(&ROW.replace("-33.5", lat).replace("-70.25", lon)).is_ok());
    }
}

#[test]
fn missing_duplicate_and_mismatched_columns_are_reported() {
    assert!(matches!(
        CsvHeader::parse(b"latitude,longitude"),
        Err(ParseError::MissingColumn)
    ));
    let duplicate = format!("{},frp", std::str::from_utf8(HEADER).unwrap());
    assert!(matches!(
        CsvHeader::parse(duplicate.as_bytes()),
        Err(ParseError::DuplicateColumn)
    ));
    assert_eq!(parse("1,2"), Err(ParseError::ColumnCount));
    assert_eq!(parse(&format!("{ROW},extra")), Err(ParseError::ColumnCount));
    assert_eq!(parse("\r\n"), Err(ParseError::EmptyRecord));
    assert_eq!(
        parse(&ROW.replace("Terra", "unknown")),
        Err(ParseError::InvalidSatellite)
    );
    assert_eq!(
        parse(&ROW.replace("MODIS", "VIIRS")),
        Err(ParseError::InvalidSatellite)
    );
    assert_eq!(
        parse(&ROW.replace(",D,", ",X,")),
        Err(ParseError::InvalidDaynight)
    );
    assert_eq!(
        parse(&ROW.replace("2024-02-29", "2023-02-29")),
        Err(ParseError::InvalidDate)
    );
}

#[test]
fn synthetic_ten_rows_keep_exactly_six_and_frp_sum_thirty_nine() {
    let fixture = include_str!("../../tests/fixtures/ten-hotspots.csv");
    let mut lines = fixture.lines();
    let header = CsvHeader::parse(lines.next().unwrap().as_bytes()).unwrap();
    let mut kept = Vec::new();
    let mut skipped = Vec::new();
    for row in lines {
        match header.parse_record(row.as_bytes()) {
            Ok(record) => kept.push(record),
            Err(error) => skipped.push(error),
        }
    }
    assert_eq!(
        kept.iter().map(|record| record.day).collect::<Vec<_>>(),
        [1, 2, 3, 4, 5, 6]
    );
    assert_eq!(kept.iter().filter(|record| record.daytime).count(), 4);
    assert_eq!(
        kept.iter()
            .map(|record| record.frp_megawatts)
            .sum::<f64>()
            .to_bits(),
        39.0_f64.to_bits()
    );
    assert_eq!(
        skipped,
        [
            ParseError::LowConfidence,
            ParseError::LowConfidence,
            ParseError::NonVegetation,
            ParseError::NonVegetation
        ]
    );
}
