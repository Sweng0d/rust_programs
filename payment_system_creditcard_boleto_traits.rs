trait Pagamento {
    fn processar_pagamento(&self);
}

pub struct CreditCard {
    pub card_number: String,
    pub value: f64,
}

pub struct Boleto {
    pub codigo_barras: String,
    pub value: f64,
}

impl Pagamento for CreditCard {
    fn processar_pagamento(&self) {
        println!("Processing the payment of {} in the credit card number {}", self.value, self.card_number);
    }
}

impl Pagamento for Boleto {
    fn processar_pagamento(&self) { 
        println!("Processing the payment of {} in the boleto with código de barras: {}", self.value, self.codigo_barras);
    }
}

fn payment (pagamentos: Vec<&dyn Pagamento>) {
    for pagamento in pagamentos {
        pagamento.processar_pagamento();
    }

}

fn main() {
    let payment1 = CreditCard {
        card_number: "4981091000001991".to_string(),
        value: 150.0,
    };

    let payment2 = Boleto {
        codigo_barras: "9102198563778192".to_string(),
        value: 230.0,
    };

    let payment3 = CreditCard {
        card_number: "9102288657011999".to_string(),
        value: 950.0,
    };

    let pagamentos: Vec<&dyn Pagamento> = vec![&payment1, &payment2, &payment3];

    payment(pagamentos);

}
