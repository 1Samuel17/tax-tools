/// Module for handling payroll withholdings.

pub enum Withholding {
    Federal(f32), // percentage
    SocialSecurity(f32), // percentage
    Medicare(f32), // percentage
}

pub struct Withholdings {
    pub withholdings: Vec<Withholding>,
}

impl Withholdings {
    pub fn total_withholdings_per_pay_period(&self, gross_pay_period_income: f32) -> f32 {
        self.withholdings.iter().map(|withholding| {
            match withholding {
                Withholding::Federal(percentage) |
                Withholding::SocialSecurity(percentage) |
                Withholding::Medicare(percentage) => gross_pay_period_income * (percentage / 100.0),
            }
        }).sum()
    }
}

