pub trait CalculosGeometricos {
    fn calcular_area (&self) -> u32;
    fn calcular_perimetro(&self) -> u32;
}

struct Quadrado {
    altura: u32,
    largura: u32,
}

struct Circulo {
    raio: u32,
}

impl CalculosGeometricos for Quadrado {
    fn calcular_area(&self) -> u32 {
        self.altura * self.largura
    }

    fn calcular_perimetro(&self) -> u32 {
        self.altura + self.altura + self.largura + self.largura
    }
}

impl CalculosGeometricos for Circulo {
    fn calcular_area(&self) -> u32 {
        3 * self.raio* self.raio
    }

    fn calcular_perimetro(&self) -> u32 {
        2 * 3 * self.raio
    }
}

fn main() {
    let quadrado1 = Quadrado {
        altura: 5,
        largura: 15,
    };

    let quadrado2 = Quadrado {
        altura: 10,
        largura: 20,
    };

    let circulo1 = Circulo {
        raio: 5,
    };

    let circulo2 = Circulo {
        raio: 10,
    };

    println!("The area of quadrado1 is {}", quadrado1.calcular_area());
    println!("The area of quadrado2 is {}", quadrado2.calcular_area());
    println!("The area of circulo1 is {}", circulo1.calcular_area());
    println!("The area of circulo2 is {}", circulo2.calcular_area());

    println!("The perimeter of quadrado1 is {}", quadrado1.calcular_perimetro());
    println!("The perimeter of quadrado2 is {}", quadrado2.calcular_perimetro());
    println!("The perimeter of circulo1 is {}", circulo1.calcular_perimetro());
    println!("The perimeter of circulo2 is {}", circulo2.calcular_perimetro());


}
