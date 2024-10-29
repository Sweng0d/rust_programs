struct Stack<T> {
    elements: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {
    
    fn new() -> Self {
        Stack {
            elements: vec![]
        }
    }
    
    fn push (&mut self, value: T) {
        self.elements.push(value);
    }

    fn pop (&mut self) {
        self.elements.pop();
    }

    fn peek (&self) -> Option<&T> {
        let x = self.elements.len();
        if x == 0 {
            None
        } else {
            Some(&self.elements[x-1])
        }
    }

    fn show_elements(&self) {
        for element in &self.elements {
            println!("{:?}", element);
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(6.667);
    stack.push(5.196);
    stack.push(10.88);
    println!("Topo da pilha: {:?}", stack.peek()); // Mostra o topo da pilha
    stack.show_elements();
    stack.pop();
    println!("Topo da pilha após pop: {:?}", stack.peek()); // Mostra o novo topo da pilha
    stack.show_elements();


}
