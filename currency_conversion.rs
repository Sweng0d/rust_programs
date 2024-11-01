fn brl_to_usd (number: u32, usdbrl: f64) -> String {

    let mut new_number;
    new_number = number as f64 / usdbrl;
    let number_converted = new_number.to_string() + " dólares";
    number_converted


}

fn main() {
    let real_amount: u32 = 100;
    let usdbrl: f64 = 5.8273;

    let usd_converted = brl_to_usd(real_amount, usdbrl);

    println!("The real amount of: {}, converted to usd is {}.", real_amount, usd_converted);

}
