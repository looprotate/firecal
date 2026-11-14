use super::{SeriesRows, write_series_csv};
use crate::QueryError;
use firecal_types::MonthIndex;

fn november_2000() -> MonthIndex {
    MonthIndex::from_index(10)
}

fn text(rows: &SeriesRows<'_>) -> String {
    let mut out = vec![0_u8; 512];
    let written = write_series_csv(november_2000(), rows, &mut out).unwrap();
    String::from_utf8(out[..written].to_vec()).unwrap()
}

#[test]
fn each_month_is_a_row_with_the_documented_decimals() {
    let rows = SeriesRows {
        value: &[5.0, 7.0],
        sigma: &[3.0, 12.0],
        median: &[20.0, 8.5],
        anomaly: &[-0.75, -0.1],
        unusual: &[true, false],
    };
    assert_eq!(
        text(&rows),
        "year,month,value,sigma,median,anomaly,unusual\n\
         2000,11,5,3.000,20.0,-0.750,true\n\
         2000,12,7,12.000,8.5,-0.100,false\n"
    );
}

#[test]
fn a_missing_number_is_an_empty_field() {
    let rows = SeriesRows {
        value: &[5.0],
        sigma: &[f64::NAN],
        median: &[f64::NAN],
        anomaly: &[f64::NAN],
        unusual: &[false],
    };
    assert_eq!(
        text(&rows),
        "year,month,value,sigma,median,anomaly,unusual\n2000,11,5,,,,false\n"
    );
}

#[test]
fn the_year_rolls_over_after_december() {
    let rows = SeriesRows {
        value: &[1.0; 3],
        sigma: &[0.0; 3],
        median: &[0.0; 3],
        anomaly: &[0.0; 3],
        unusual: &[false; 3],
    };
    let csv = text(&rows);
    assert!(csv.contains("\n2000,12,"));
    assert!(csv.ends_with("2001,1,1,0.000,0.0,0.000,false\n"));
}

#[test]
fn columns_of_different_length_are_an_error() {
    let rows = SeriesRows {
        value: &[1.0, 2.0],
        sigma: &[0.0],
        median: &[0.0, 0.0],
        anomaly: &[0.0, 0.0],
        unusual: &[false, false],
    };
    let mut out = [0_u8; 256];
    assert_eq!(
        write_series_csv(november_2000(), &rows, &mut out),
        Err(QueryError::OutputLengthMismatch)
    );
}

#[test]
fn no_months_is_an_error() {
    let rows = SeriesRows {
        value: &[],
        sigma: &[],
        median: &[],
        anomaly: &[],
        unusual: &[],
    };
    let mut out = [0_u8; 256];
    assert!(matches!(
        write_series_csv(november_2000(), &rows, &mut out),
        Err(QueryError::Types(_))
    ));
}

#[test]
fn a_buffer_one_byte_short_is_an_error_and_the_exact_size_fits() {
    let rows = SeriesRows {
        value: &[5.0],
        sigma: &[3.0],
        median: &[20.0],
        anomaly: &[0.0],
        unusual: &[false],
    };
    let needed = text(&rows).len();
    let mut exact = vec![0_u8; needed];
    assert_eq!(
        write_series_csv(november_2000(), &rows, &mut exact),
        Ok(needed)
    );
    let mut short = vec![0_u8; needed - 1];
    assert_eq!(
        write_series_csv(november_2000(), &rows, &mut short),
        Err(QueryError::BufferTooSmall)
    );
}
