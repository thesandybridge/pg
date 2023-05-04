use anyhow::Result;

pub fn percentage_of_growth(initial_value: f64, final_value: f64) -> Result<f64> {
    Ok(((final_value - initial_value) / initial_value) * 100.00)
}

pub fn parse_with_commas(input: &String) -> Result<f64> {
    let without_commas = input.replace(",", "");
    Ok(without_commas
       .parse::<f64>()
       .unwrap_or_else(|_| panic!("Invalid number format: {}", input))
    )
}
