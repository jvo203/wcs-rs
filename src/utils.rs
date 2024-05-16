use mapproj::LonLat;

/*pub unsafe fn string_to_keyword_type(keyword: &str) -> &[u8; 8] {
    keyword.as_bytes()[0..=7]
        .try_into()
        .expect("String has less than 8 characters")
}*/

pub fn angular_dist(lonlat1: LonLat, lonlat2: LonLat) -> f64 {
    let abs_diff_lon = (lonlat1.lon() - lonlat2.lon()).abs();

    (lonlat1.lat().sin() * lonlat2.lat().sin()
        + lonlat1.lat().cos() * lonlat2.lat().cos() * abs_diff_lon.cos())
    .acos()
}
