//! This module deals with parsing the projection specific keywords
//!
//! Zenithal, pseudo-cylindrical, cylindrical and conic projections
//! are supported. Bonne, Polyconic and cubic projections are not

use crate::header::WCSHeader;

use mapproj::{
    conic::{cod::Cod, coe::Coe, coo::Coo, cop::Cop},
    cylindrical::{car::Car, cea::Cea, cyp::Cyp, mer::Mer},
    pseudocyl::{ait::Ait, mol::Mol, par::Par, sfl::Sfl},
    zenithal::{
        air::Air,
        arc::Arc,
        azp::Azp,
        ncp::Ncp,
        sin::{Sin, SinSlant},
        stg::Stg,
        szp::Szp,
        tan::Tan,
        zea::Zea,
        zpn::Zpn,
    },
    CanonicalProjection, CenteredProjection, LonLat,
};

use super::Error;

pub trait WCSCanonicalProjection: CanonicalProjection {
    fn parse_proj(header: &WCSHeader) -> Result<CenteredProjection<Self>, Error>
    where
        Self: Sized,
    {
        let crval1 = header.get_float("CRVAL1  ").unwrap_or(Ok(0.0))?;
        let crval2 = header.get_float("CRVAL2  ").unwrap_or(Ok(0.0))?;

        let proj = Self::parse_internal_proj_params(header)?;

        let mut centered_proj = CenteredProjection::new(proj);
        let proj_center = LonLat::new(crval1.to_radians(), crval2.to_radians());
        centered_proj.set_proj_center_from_lonlat(&proj_center);

        Ok(centered_proj)
    }

    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error>
    where
        Self: Sized;
}

// Zenithal projections
impl WCSCanonicalProjection for Azp {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // mu given in spherical radii, default value: 0.0
        let mu = header.get_float("PV_1    ").unwrap_or(Ok(0.0))?;
        // gamma given in deg, default value: 0.0
        let gamma = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;

        let azp = Azp::from_params(mu, gamma.to_radians());

        Ok(azp)
    }
}

impl WCSCanonicalProjection for Szp {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // mu given in spherical radii, default value: 0.0
        let mu = header.get_float("PV_1    ").unwrap_or(Ok(0.0))?;
        // phi_c given in deg, default value: 0.0
        let phi_c = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;
        // theta_c given in deg, default value: 90.0
        let theta_c = header.get_float("PV_3    ").unwrap_or(Ok(90.0))?;

        let szp = Szp::from_params(mu, phi_c.to_radians(), theta_c.to_radians());

        Ok(szp)
    }
}

impl WCSCanonicalProjection for Tan {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Tan::new())
    }
}

impl WCSCanonicalProjection for Stg {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Stg::new())
    }
}

impl WCSCanonicalProjection for Sin {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Sin::new())
    }
}

impl WCSCanonicalProjection for SinSlant {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // xi dimensionless, default value: 0.0
        let xi = header.get_float("PV_1    ").unwrap_or(Ok(0.0))?;
        // eta dimensionless, default value: 0.0
        let eta = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;

        let sin_slant = SinSlant::new(-xi, eta);
        Ok(sin_slant)
    }
}

impl WCSCanonicalProjection for Arc {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Arc::new())
    }
}

impl WCSCanonicalProjection for Zpn {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        let mut coeffs = [
            "PV_0    ", "PV_1    ", "PV_2    ", "PV_3    ", "PV_4    ", "PV_5    ", "PV_6    ",
            "PV_7    ", "PV_8    ", "PV_9    ", "PV_10   ", "PV_11   ", "PV_12   ", "PV_13   ",
            "PV_14   ", "PV_15   ", "PV_16   ", "PV_17   ", "PV_18   ", "PV_19   ", "PV_20   ",
        ]
        .iter()
        .rev()
        .map(|key| header.get_float(key))
        .skip_while(|value| {
            if let Some(value) = value {
                // Skip if the value is not a float
                value.is_err()
            } else {
                // Skip if the key has not been found
                true
            }
        })
        .map(|value| value.unwrap_or(Ok(0.0)))
        .collect::<Result<Vec<f64>, _>>()?;

        coeffs.reverse();

        Zpn::from_params(coeffs).ok_or(Error::InitProjection(
            Zpn::NAME,
            "negative polynomial in [0, pi]",
        ))
    }
}

impl WCSCanonicalProjection for Zea {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Zea::new())
    }
}

impl WCSCanonicalProjection for Air {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // theta_b in deg, default value: 90.0
        let theta_b = header.get_float("PV_1    ").unwrap_or(Ok(90.0))?;

        let airy = Air::from_param(theta_b.to_radians());
        Ok(airy)
    }
}

impl WCSCanonicalProjection for Ncp {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        let ncp = Ncp::new();
        Ok(ncp)
    }
}

// Cylindrical projections
impl WCSCanonicalProjection for Cyp {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // mu given in spherical radii, default value: 1.0
        let mu = header.get_float("PV_1    ").unwrap_or(Ok(1.0))?;
        // lambda given in spherical radii, default value: 1.0
        let lambda = header.get_float("PV_2    ").unwrap_or(Ok(1.0))?;

        let cyp = Cyp::from_params(mu, lambda);
        Ok(cyp)
    }
}

impl WCSCanonicalProjection for Cea {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // lambda given in spherical radii, default value: 1.0
        let lambda = header.get_float("PV_1    ").unwrap_or(Ok(1.0))?;

        let cea = Cea::from_param(lambda);
        Ok(cea)
    }
}

impl WCSCanonicalProjection for Car {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Car::default())
    }
}

impl WCSCanonicalProjection for Mer {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Mer::default())
    }
}

// Pseudo-cylindrical projections
impl WCSCanonicalProjection for Sfl {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Sfl::default())
    }
}

impl WCSCanonicalProjection for Par {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Par::default())
    }
}

impl WCSCanonicalProjection for Mol {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Mol::default())
    }
}

impl WCSCanonicalProjection for Ait {
    fn parse_internal_proj_params(_: &WCSHeader) -> Result<Self, Error> {
        Ok(Ait::default())
    }
}

// Conic projections
impl WCSCanonicalProjection for Cop {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // theta_a given in deg, has no default value
        if let Some(theta_a) = header.get_float("PV_1    ") {
            let theta_a = theta_a?;

            // eta given in deg, default value: 0
            let eta = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;

            let cop = Cop::from_params(theta_a.to_radians(), eta.to_radians());
            Ok(cop)
        } else {
            Err(Error::InitProjection(
                Cop::NAME,
                "PV_1 = theta_a must be defined as it has no default value",
            ))
        }
    }
}

impl WCSCanonicalProjection for Coe {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // theta_a given in deg, has no default value
        if let Some(theta_a) = header.get_float("PV_1    ") {
            let theta_a = theta_a?;

            // eta given in deg, default value: 0
            let eta = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;

            let cop = Coe::from_params(theta_a.to_radians(), eta.to_radians());
            Ok(cop)
        } else {
            Err(Error::InitProjection(
                Coe::NAME,
                "PV_1 = theta_a must be defined as it has no default value",
            ))
        }
    }
}

impl WCSCanonicalProjection for Cod {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // theta_a given in deg, has no default value
        if let Some(theta_a) = header.get_float("PV_1    ") {
            let theta_a = theta_a?;

            // eta given in deg, default value: 0
            let eta = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;

            let cod = Cod::from_params(theta_a.to_radians(), eta.to_radians());
            Ok(cod)
        } else {
            Err(Error::InitProjection(
                Cod::NAME,
                "PV_1 = theta_a must be defined as it has no default value",
            ))
        }
    }
}

impl WCSCanonicalProjection for Coo {
    fn parse_internal_proj_params(header: &WCSHeader) -> Result<Self, Error> {
        // theta_a given in deg, has no default value
        if let Some(theta_a) = header.get_float("PV_1    ") {
            let theta_a = theta_a?;

            // eta given in deg, default value: 0
            let eta = header.get_float("PV_2    ").unwrap_or(Ok(0.0))?;

            let coo = Coo::from_params(theta_a.to_radians(), eta.to_radians());
            Ok(coo)
        } else {
            Err(Error::InitProjection(
                Coo::NAME,
                "PV_1 = theta_a must be defined as it has no default value",
            ))
        }
    }
}
