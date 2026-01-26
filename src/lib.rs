//! This is a library that contains utility functions for calculating net income, necessary hours, elected deductions, and standard tax withholding given set of expenses and a hypothetical hourly wage or salary scenario. The idea is pretty much like the "Sample Paycheck" tool found in the [Paycom](https://www.paycom.com/software/employee-self-service/) employee portal, but aimed at having more functionality and customization.

pub mod withholdings;
pub mod deductions;
pub mod income;
pub mod utils;
pub mod expenses;

pub use crate::withholdings::*;
pub use crate::deductions::*;
pub use crate::income::*;
pub use crate::expenses::*;

