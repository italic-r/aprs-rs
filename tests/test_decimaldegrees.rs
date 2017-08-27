extern crate aprs;
use aprs::decimaldegrees;

#[test]
fn test_dec2dms() {
    assert_eq!(decimaldegrees::decimal2dms(37.773972_f32), (37_f32, 46_f32, 26.297607_f32));
    assert_eq!(decimaldegrees::decimal2dms(-122.431297_f32), (-122_f32, 25_f32, 52.670288_f32));
}

#[test]
fn test_dec2dm() {
    assert_eq!(decimaldegrees::decimal2dm(37.773972_f32), (37_f32, 46.438293_f32));
    assert_eq!(decimaldegrees::decimal2dm(-122.431297_f32), (-122_f32, 25.877838_f32));
}

#[test]
fn test_dms2decimal() {
    assert_eq!(decimaldegrees::dms2decimal(37_f32, 46_f32, 26.2992_f32), 37.773972_f32);
    assert_eq!(decimaldegrees::dms2decimal(-122_f32, 25_f32, 52.6692_f32), -122.431297_f32);
}

#[test]
fn test_dm2decimal() {
    assert_eq!(decimaldegrees::dm2decimal(37_f32, 46.438293_f32), 37.773972_f32);
    assert_eq!(decimaldegrees::dm2decimal(-122_f32, 25.877838_f32), -122.431297_f32);
}
