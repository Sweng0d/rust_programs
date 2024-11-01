struct Celsius {
    temp: f64,
}

struct Fahrenheit {
    temp: f64
}

impl From<&Celsius> for Fahrenheit {
    fn from(c: &Celsius) -> Self {
        Fahrenheit {
            temp: (c.temp * 9.0 / 5.0) + 32.0,
        }

    }
}

impl From<&Fahrenheit> for Celsius {
    fn from(f: &Fahrenheit) -> Self {
        Celsius {
            temp: (f.temp - 32.0) / 1.8,
        }
    }
}

fn main() {
    //Celsius to Fahrenheit

    let temp_c = Celsius {
        temp: 50.19,
    };

    let temp_f: Fahrenheit = (&temp_c).into();

    println!("Temperature in Celsius is {} and converted to Fahrenheit is {}", temp_c.temp, temp_f.temp);

    let temp_f2 = Fahrenheit {
        temp: 99.0,
    };    

    let temp_c2: Celsius = (&temp_f2).into();

    println!("Temperature in Fahrenheit is {} and converted to Celsius is {}", temp_f2.temp, temp_c2.temp);

}
