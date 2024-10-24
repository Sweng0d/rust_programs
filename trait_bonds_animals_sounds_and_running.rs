pub trait Animal {
    fn som(&self) -> String;
}

pub trait Corredor: Animal {
    fn correr(&self) -> String;
}

struct Coelho;
struct Cavalo;
struct Peixe;

impl Animal for Coelho {
    fn som(&self) -> String {
        String::from("tchektchek")
    }
}

impl Animal for Cavalo {
    fn som(&self) -> String {
        String::from("poktipokti")
    }
}

impl Animal for Peixe {
    fn som(&self) -> String {
        String::from("fluflu")
    }
}

impl Corredor for Coelho {
    fn correr(&self) -> String {
        String::from("pulando em muitas quantidades")
    }
}

impl Corredor for Cavalo {
    fn correr(&self) -> String {
        String::from("pulando e sendo rapido com grande altura")
    }
}

impl Corredor for Peixe {
    fn correr(&self) -> String {
        String::from("navegando nos mares")
    }
}

fn main () {
    let coelho = Coelho;
    let cavalo = Cavalo;
    let peixe = Peixe;

    println!("O coelho faz {} e corre {}", coelho.som(), coelho.correr());
    println!("O cavalo faz {} e corre {}", cavalo.som(), cavalo.correr());
    println!("O peixe faz {} e corre {}", peixe.som(), peixe.correr());
}

