fn main() {
    let mut largura = 7;
    let mut altura = 4;

    desenho(largura, altura);
}

fn desenho(largura: i32, altura: i32) {

    let mut altura_count = 0;

    'outer: loop {
        let mut largura_count = 0;
        'inner: loop {
            if largura > largura_count {
                print!("*");
                largura_count +=1;
            } else {
                break 'inner;
            }
        }

        if altura -1 > altura_count {
            println!("");
            altura_count +=1;
        } else {
            break 'outer;
        }
    }
}
