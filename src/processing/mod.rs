//! Processing toolkit, including filter designer.
use std::str::FromStr;
use thiserror::Error;

mod item;
pub use item::{FilterItem, ItemError};

mod mask;
pub use mask::{Error as MaskError, MaskFilter, MaskOperand, Masking};

mod decim;
pub use decim::{Decimate, DecimationFilter, DecimationFilterType, Error as DecimationError};

impl Filter {
    /// Builds new [MaskFilter] from given specs
    pub fn mask(operand: MaskOperand, item: FilterItem) -> Self {
        Self::Mask(MaskFilter { operand, item })
    }
    /// Builds new [MaskFilter] with Equals operand
    /// from following [FilterItem] description
    pub fn equals(item: &str) -> Result<Self, ItemError> {
        let item = FilterItem::from_str(item)?;
        Ok(Self::mask(MaskOperand::Equals, item))
    }
    /// Builds new [MaskFilter] with !Equals operand
    /// from following [FilterItem] description
    pub fn not_equals(item: &str) -> Result<Self, ItemError> {
        let item = FilterItem::from_str(item)?;
        Ok(Self::mask(MaskOperand::NotEquals, item))
    }
    /// Builds new [MaskFilter] with GreaterThan operand
    /// from following [FilterItem] description
    pub fn greater_than(item: &str) -> Result<Self, ItemError> {
        let item = FilterItem::from_str(item)?;
        Ok(Self::mask(MaskOperand::GreaterThan, item))
    }
    /// Builds new [MaskFilter] with GreaterEquals operand
    /// from following [FilterItem] description
    pub fn greater_equals(item: &str) -> Result<Self, ItemError> {
        let item = FilterItem::from_str(item)?;
        Ok(Self::mask(MaskOperand::GreaterEquals, item))
    }
    /// Builds new [MaskFilter] with LowerEquals operand
    /// from following [FilterItem] description
    pub fn lower_equals(item: &str) -> Result<Self, ItemError> {
        let item = FilterItem::from_str(item)?;
        Ok(Self::mask(MaskOperand::LowerEquals, item))
    }
    /// Builds new [MaskFilter] with LowerThan operand
    /// from following [FilterItem] description
    pub fn lower_than(item: &str) -> Result<Self, ItemError> {
        let item = FilterItem::from_str(item)?;
        Ok(Self::mask(MaskOperand::LowerThan, item))
    }
}

impl From<MaskFilter> for Filter {
    fn from(mask: MaskFilter) -> Self {
        Self::Mask(mask)
    }
}

impl std::ops::Not for Filter {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::Mask(f) => Self::Mask(!f),
            _ => self.clone(), // does not apply
        }
    }
}

impl From<DecimationFilter> for Filter {
    fn from(decim: decim::DecimationFilter) -> Self {
        Self::Decimation(decim)
    }
}

impl std::str::FromStr for Filter {
    type Err = Error;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = content.split(':').collect();

        let identifier = items[0].trim();
        if identifier.eq("decim") {
            let offset = 6; //"decim:"
            Ok(Self::Decimation(DecimationFilter::from_str(
                content[offset..].trim(),
            )?))
        } else if identifier.eq("mask") {
            let offset = 5; //"mask:"
            Ok(Self::Mask(MaskFilter::from_str(content[offset..].trim())?))
        } else {
            // assume Mask (omitted identifier)
            if let Ok(f) = MaskFilter::from_str(content.trim()) {
                Ok(Self::Mask(f))
            } else {
                Err(Error::UnknownFilterType(content.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn from_str() {
        /*
         * MASK FILTER description
         */
        for descriptor in [
            "GPS",
            "=GPS",
            " != GPS",
            "G08, G09, G10",
            "=G08, G09, G10",
            "!= GPS, GAL",
            ">G08, G09",
            "iode",
            "iode,gps",
            "iode,crs,gps",
            "iode,crs",
            ">2020-01-14T00:31:55 UTC",
        ] {
            assert!(
                Filter::from_str(descriptor).is_ok(),
                "Filter::from_str failed on \"{}\"",
                descriptor
            );
        }
        /*
         * DECIMATION FILTER description
         */
        for desc in [
            "decim:10",
            "decim:10 min",
            "decim:1 hour",
            "decim:10 min:l1c",
            "decim:1 hour:L1C,L2C,L3C",
        ] {
            let filt = Filter::from_str(desc);
            assert!(filt.is_ok(), "Filter::from_str failed on \"{}\"", desc);
        }
        /*
         * SMOOTHING FILTER description
         */
        for desc in [
            "smooth:mov:10 min",
            "smooth:mov:1 hour",
            "smooth:mov:1 hour:l1c",
            "smooth:mov:10 min:clk",
            "smooth:hatch",
            "smooth:hatch:l1c",
        ] {
            let filt = Filter::from_str(desc);
            assert!(filt.is_ok(), "Filter::from_str failed on \"{}\"", desc);
        }
    }
}
