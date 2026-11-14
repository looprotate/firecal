//! Index build-date validation.

use super::*;
#[test]
fn date_requires_real_gregorian_day_and_cannot_inject_json() {
    for valid in ["2026-11-14", "2000-02-29", "2024-02-29"] {
        assert!(validate_date(valid).is_ok());
    }
    for invalid in [
        "1900-02-29",
        "2026-02-29",
        "2026-04-31",
        "2026-00-01",
        "2026-13-01",
        "2026-01-00",
        "0000-01-01",
        "2026-1-01",
        "2026-01-1",
        "2026-01-0\"",
        "2026-aa-bb",
    ] {
        assert!(validate_date(invalid).is_err());
    }
}
