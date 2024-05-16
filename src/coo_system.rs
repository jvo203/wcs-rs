use crate::error::Error;
use crate::header::WCSHeader;
use crate::utils;

pub enum RadeSys {
    /// International Celestial Reference System
    ICRS,
    /// mean place, new (IAU 1984) system
    Fk5,
    /// mean place, old (Bessell-Newcomb) system
    Fk4,
    /// mean place, old system but without e-terms
    Fk4NoE,
    /// geocentric apparent place, IAU 1984 system
    GAPPT,
}

impl RadeSys {
    pub fn parse(header: &WCSHeader) -> Result<Self, Error> {
        let radesys = header.get_radesys()?;

        match radesys.as_str() {
            "ICRS" => Ok(RadeSys::ICRS),
            "FK5" => Ok(RadeSys::Fk5),
            "FK4" => Ok(RadeSys::Fk4),
            "FK4-NO-E" => Ok(RadeSys::Fk4NoE),
            "GAPPT" => Ok(RadeSys::GAPPT),
            _ => Err(Error::UnrecognizedRadeSys(radesys)),
        }
    }
}

pub enum CooSystem {
    EQUATORIAL,
    GALACTIC,
    ECLIPTIC,
    HELIOECLIPTIC,
    SUPERGALACTIC,
    CUSTOM { radesys: RadeSys, equinox: f64 },
}

impl CooSystem {
    pub fn parse(header: &WCSHeader) -> Result<Self, Error> {
        // wrap get_float("EQUINOX") in a Result<f64, Error>
        let equinox = match header.get_float("EQUINOX") {
            Some(Ok(equinox)) => Ok(equinox),
            _ => Err(Error::MandatoryWCSKeywordsMissing("EQUINOX")),
        };

        let radesys = RadeSys::parse(header);

        let coo_system = if let (Ok(radesys), Ok(equinox)) = (radesys, equinox) {
            // if there is a radesys take it into account
            CooSystem::CUSTOM { radesys, equinox }
        } else {
            let ctype1 = header.get_ctype(1)?;

            match ctype1.as_bytes()[0] {
                b'G' => CooSystem::GALACTIC,
                b'E' => CooSystem::ECLIPTIC,
                b'H' => CooSystem::HELIOECLIPTIC,
                b'S' => CooSystem::SUPERGALACTIC,
                _ => CooSystem::EQUATORIAL,
            }
        };

        Ok(coo_system)
    }
}
