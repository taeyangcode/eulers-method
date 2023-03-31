use colored::Colorize;

struct StdinError {
    message: &'static str
}

fn get_stdin(invalid_input: StdinError) -> Result<f64, std::io::ErrorKind> {
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.pop(),
        Err(_) => {
            println!(invalid_input.message);
            return Err(std::io::ErrorKind::InvalidInput);
        }
    };
}

fn get_bounds() -> Result<(f64, f64), std::io::ErrorKind> {
    let get_bound = || -> Result<f64, std::io::ErrorKind> {
        let mut bound: String = String::new();
        match std::io::stdin().read_line(&mut bound) {
            Err(_) => {
                println!("An invalid entry was entered. Please try again.");
                return Err(std::io::ErrorKind::InvalidInput);
            },
            Ok(_) => bound.pop()
        };
        return match bound.parse::<f64>() {
            Ok(bound) => Ok(bound),
            Err(_) => {
                println!("Input could not be parsed to a floating point number. Please try again.");
                return Err(std::io::ErrorKind::InvalidData);
            }
        };
    };

    println!("Enter lower bound {}:", "a".italic());
    let lower_bound: f64 = get_bound()?;
    println!("Enter upper bound {}:", "b".italic());
    let upper_bound: f64 = get_bound()?;

    return Ok((lower_bound, upper_bound));
}

fn main() -> Result<(), std::io::ErrorKind> {
    let bounds: (f64, f64) = get_bounds()?;
    println!("{:?}", bounds);

    return Ok(());
}
