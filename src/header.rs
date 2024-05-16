use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct WCSHeader {
    naxis1: u64,
    naxis2: u64,
    ctype1: String,
    ctype2: String,
    cards: HashMap<String, f64>,
}

const FITS_LINE_LENGTH: usize = 80;

impl WCSHeader {
    pub fn new(s: &str) -> Self {
        let mut cards = HashMap::new();
        let mut naxis1 = 0;
        let mut naxis2 = 0;
        let mut ctype1 = String::new();
        let mut ctype2 = String::new();

        let mut offset: usize = 0;

        while offset < s.len() {
            let line = &s[offset..offset + FITS_LINE_LENGTH];
            offset += FITS_LINE_LENGTH;

            // split the line into key and value by "= "
            let mut iter = line.split("= ");
            let key = iter.next();
            let value = iter.next();

            // continue if a key or a value are 'None'
            if key.is_none() || value.is_none() {
                continue;
            }

            let key = key.unwrap().trim();
            let value = value.unwrap();

            // remove an optional comment (starting with '/') from the value
            let value = value.split('/').next().unwrap().trim();

            match key {
                "NAXIS1" => naxis1 = value.parse().unwrap(),
                "NAXIS2" => naxis2 = value.parse().unwrap(),
                "CTYPE1" => ctype1 = value.to_string().replace("'", ""),
                "CTYPE2" => ctype2 = value.to_string().replace("'", ""),
                _ => {
                    if let Ok(value) = value.parse() {
                        cards.insert(key.to_string(), value);
                    }
                }
            }
        }

        WCSHeader {
            naxis1,
            naxis2,
            ctype1,
            ctype2,
            cards,
        }
    }

    pub fn get_naxisn(&self, idx: usize) -> Option<u64> {
        let value = match idx {
            1 => self.naxis1,
            2 => self.naxis2,
            _ => 0,
        };

        // check if value == 0
        if value > 0 {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_ctype(&self, idx: usize) -> Result<String, Error> {
        let value = match idx {
            1 => &self.ctype1,
            2 => &self.ctype2,
            _ => "",
        };

        if value.is_empty() {
            Err(Error::MandatoryWCSKeywordsMissing("CTYPE"))
        } else {
            Ok(value.to_string())
        }
    }

    pub fn get_float(&self, key: &str) -> Option<Result<f64, Error>> {
        if let Some(value) = self.cards.get(key.trim()) {
            Some(Ok(*value))
        } else {
            None
        }
    }
}
