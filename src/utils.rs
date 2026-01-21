/// Utility constants for payroll calculations.

pub const WEEKS_PER_YEAR: u32 = 52; // potential working weeks in a year
pub const STANDARD_HOURS_PER_WEEK: u32 = 40; // standard full-time hours
pub const OVERTIME_MULTIPLIER: f32 = 1.5; // time and a half
pub const PAID_TIME_OFF_WEEKS_PER_YEAR: u32 = 3; // overtime not possible during PTO
pub const PAY_PERIOD_LENGTH_IN_WEEKS: u32 = 2; // bi-weekly pay periods