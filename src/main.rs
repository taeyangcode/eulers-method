use colored::Colorize;

mod io_error_messages {
    pub const INVALID_INPUT: &'static str = "Invalid input was supplied. Please try again.";
    pub const CANNOT_PARSE: &'static str = "Input could not be parsed as a 64 bit floating point number. Please try again.";

    pub const SMALLER_UPPER_BOUND: &'static str = "The upper bound cannot be smaller than the lower bound. Please try again.";

    pub const FLOAT_STEP_SIZE: &'static str = "The amount of steps must be a whole number. Please try again.";
    pub const NEGATIVE_ZERO_STEP_SIZE: &'static str = "The amount of steps cannot be zero or negative. Please try again.";

    pub const FLOAT_ROUND_PLACES: &'static str = "The amount of places must be a whole number. Please try again.";
    pub const NEGATIVE_ZERO_ROUND_PLACES: &'static str = "The amount of places cannot be zero or negative. Please try again.";

    pub const INVALID_DIFFERENTIAL_EXPRESSION: &'static str = "The differential expression you entered is either invalid, or conatins unparseable tokens. Please reference the README for more information regarding the specification and required format for a differential expression and try again.";
}

fn get_stdin<T>() -> Result<T, std::io::ErrorKind>
    where T: std::str::FromStr {
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.pop(),
        Err(_) => {
            println!("{}", io_error_messages::INVALID_INPUT.bright_red().bold());
            return Err(std::io::ErrorKind::InvalidInput);
        }
    };
    return match input.parse::<T>() {
        Ok(result) => Ok(result),
        Err(_) => {
            println!("{}", io_error_messages::CANNOT_PARSE.bright_red().bold());
            return Err(std::io::ErrorKind::InvalidData);
        }
    };
}

fn get_bounds() -> Result<(f64, f64), std::io::ErrorKind> {
    println!("Enter lower bound {}:", "a".italic());
    let lower_bound: f64 = get_stdin::<f64>()?;
    println!("Enter upper bound {}:", "b".italic());
    let upper_bound: f64 = get_stdin::<f64>()?;

    if upper_bound < lower_bound {
        println!("{}", io_error_messages::SMALLER_UPPER_BOUND.bright_red().bold());
        return Err(std::io::ErrorKind::InvalidData);
    }

    return Ok((lower_bound, upper_bound));
}

fn get_steps() -> Result<f64, std::io::ErrorKind> {
    println!("Enter the amount of steps {}:", "N".italic());
    let steps: f64 = get_stdin::<f64>()?;

    if steps.fract() != 0.0 {
        println!("{}", io_error_messages::FLOAT_STEP_SIZE.bright_red().bold());
        return Err(std::io::ErrorKind::InvalidData);
    }
    if steps <= 0.0 {
        println!("{}", io_error_messages::NEGATIVE_ZERO_STEP_SIZE.bright_red().bold());
        return Err(std::io::ErrorKind::InvalidData);
    }

    return Ok(steps);
}

fn round_to(number: f64, decimal_places: i32) -> f64 {
    let power: f64 = 10.0f64.powi(decimal_places);
    return (number * power).round() / power;
}

fn get_step_size(bounds: (f64, f64), steps: f64) -> Result<f64, std::io::ErrorKind> {
    println!("Enter the amount of decimal places should the step size be rounded to:");
    let round_places: f64 = get_stdin::<f64>()?;
    if round_places.fract() != 0.0 {
        println!("{}", io_error_messages::FLOAT_ROUND_PLACES.bright_red().bold());
        return Err(std::io::ErrorKind::InvalidData);
    }
    if round_places <= 0.0 {
        println!("{}", io_error_messages::NEGATIVE_ZERO_ROUND_PLACES.bright_red().bold());
        return Err(std::io::ErrorKind::InvalidData);
    }

    let step_size: f64 = (bounds.1 - bounds.0) / steps;
    return Ok(round_to(step_size, round_places as i32));
}

fn get_differential_expression() -> Result<meval::Expr, std::io::ErrorKind> {
    println!("Enter the differential expression to approximate:");
    let differential_expression: meval::Expr = match get_stdin::<String>()?.parse::<meval::Expr>() {
        Ok(differential_expression) => differential_expression,
        Err(_) => {
            println!("{}", io_error_messages::INVALID_DIFFERENTIAL_EXPRESSION.bright_red().bold());
            return Err(std::io::ErrorKind::InvalidData);
        }
    };

    return Ok(differential_expression);
}

fn get_initial_value() -> Result<f64, std::io::ErrorKind> {
    println!("Enter the initial value for the function at {}", "a".italic());
    let initial_value: f64 = get_stdin::<f64>()?;
    return Ok(initial_value);
}

fn compute_eulers_method() -> Result<(), std::io::ErrorKind> {
    let bounds: (f64, f64) = get_bounds()?;
    let steps: f64 = get_steps()?;
    let step_size: f64 = get_step_size(bounds, steps)?;

    let function_value: std::cell::Cell<f64> = std::cell::Cell::new(get_initial_value()?);
    let current_step: std::cell::Cell<f64> = std::cell::Cell::new(bounds.0);

    // let context: meval::Context = meval::Context::new();
    // context.func("e", |x| 

    let differential_expression = match get_differential_expression()?.bind2_with_context(meval::Context::new(), "x", "y") {
            Ok(result) => result,
            Err(_) => {
                println!(r#"
                    The differential expression could not be bound using the current context:
                    x = {}
                    y = {}
                "#, current_step.get(), function_value.get());
                return Err(std::io::ErrorKind::InvalidData);
            }
    };

    let mut current_index: i32 = 1;
    let expression_result: f64 = function_value.get() + step_size * differential_expression(current_step.get(), function_value.get());
    while current_index <= steps as i32 {
        println!(r#"
            w_{} = {}
            x_{} = {}
            y_{} = {}
        "#, current_index, expression_result, current_index - 1, current_step.get(), current_index - 1, function_value.get());

        current_step.set(current_step.get() + step_size);
        function_value.set(expression_result);

        current_index += 1;
    }

    return Ok(());
}

fn main() -> Result<(), std::io::ErrorKind> {
    compute_eulers_method()?;

    return Ok(());
}
