use crate::style::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use crate::style::values::computed::{ComputeContext, ToComputedValue, ValueDefault};
use crate::style::values::specified;

/// Computed value of a `width`.
#[derive(Clone, Copy, Debug)]
pub struct Width {
    size: LengthPercentageOrAuto,
}

impl Width {
    pub fn initial_value() -> Width {
        Width {
            size: LengthPercentageOrAuto::Auto,
        }
    }
}

impl ToComputedValue for specified::Width {
    type ComputedValue = Width;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        let computed_lp_auto: LengthPercentageOrAuto = match self {
            // TODO: I think we repeat computing the value of specified::LengthPercentageOrAuto a lot...eventually consider
            // simply implementing `ToComputedValue` for specified::LengthPercentageOrAuto.
            specified::Width::LengthPercentageOrAuto(lp_auto) => match lp_auto {
                specified::LengthPercentageOrAuto::Auto => LengthPercentageOrAuto::Auto,
                specified::LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
                    specified::LengthPercentage::Length(no_calc_length) => {
                        no_calc_length.to_computed_value(context).into()
                    }
                    specified::LengthPercentage::Percentage(percentage) => {
                        LengthPercentageOrAuto::LengthPercentage(LengthPercentage::Percentage(
                            percentage.clone(),
                        ))
                    }
                },
            },
        };

        Width {
            size: computed_lp_auto,
        }
    }
}

impl ValueDefault for specified::Width {
    type ComputedValue = Width;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        Width::initial_value()
    }
}