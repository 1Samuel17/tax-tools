//! A CLI tool for estimating paycheck net income and withholdings in order to compare against a given set of living expenses
//! This tool will allow users to input their employment scenario, including hourly rate, hours worked per week, filing status, pretax deductions, posttax deductions, and monthly expenses. The tool will then calculate the user's estimated net paycheck and compare it against their monthly expenses to help them understand their financial situation.
//! 
//! The tool utilizes the `clap` crate for command-line argument parsing, and the `anyhow` crate for error handling. The core logic for paycheck calculation and comparison will be implemented in a separate module, which will be imported into the main CLI application.
//! 
//! There is only 1 command for this CLI that will start the interactive dialogue for the user to be guided through inputting their employment scenario, deductions, and expenses. After confirming the inputs, the tool will perform the paycheck calculation and display the results, including the weekly net paycheck and a comparison of monthly income vs expenses.
//! 
//! The main components of the tool include:
//! - A `main` function that serves as the entry point for the CLI application, handling command-line arguments and orchestrating the flow of the application.
//! - An `interaction` module that contains functions for gathering user input, confirming inputs, and converting inputs into the appropriate data structures for paycheck calculation.

use anyhow::Result;
use clap::{Parser, Subcommand};
use paycheck_utils::interaction::*;

#[derive(Parser, Debug)]
#[command(name = "check-paycheck")]
#[command(
    author, 
    version = "0.1.0", 
    about = "A cli tool for estimating paycheck net income and withholdings in order to compare against a given set of living expenses", 
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// start a dialogue to input employment scenario, deductions, and expenses
    Start,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => { 
            
            // Start the interactive dialogue to receive user input for employment scenario, deductions, and expenses
            println!("\n{:-^100}"," CHECK-PAYCHECK CLI TOOL: ");

            println!("\n{:^100}","A CLI tool for estimating paycheck net income and withholdings");
            println!("{:^100}", "in order to compare against a given set of living expenses.");
            println!("\n{:-^100}","-");

            // create a scenario from user inputs
            let scenario = get_user_input();
            
            // Perform paycheck calculation based on confirmed inputs
            let net_paycheck = scenario.calculate_net_paycheck();
            let comparison = scenario.compare_monthly_expenses_to_monthly_income();

            // Display the calculated paycheck details

            println!("\n{:^100}", "--- Paycheck Calculation Results ---");
            println!("\nWeekly Net Paycheck: ${:.2}", net_paycheck);
            println!("Monthly Net Income: ${:.2}\nTotal Monthly Expenses: ${:.2}\nDifference: ${:.2}\n", comparison.0, comparison.1, comparison.2);

            // Restart or exit based on user choice (future implementation)

        }
    }

    Ok(())
}