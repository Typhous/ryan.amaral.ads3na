// Importa VecDeque, que é uma fila eficiente
use std::collections::VecDeque;

// Criamos uma estrutura chamada Fila
struct Fila {
    dados: VecDeque<i32>, // Aqui guardamos os números da fila
}

// Implementamos as funcionalidades da fila
impl Fila {
    // Cria uma nova fila vazia
    fn new() -> Self {
        Fila {
            dados: VecDeque::new(), // Cria a fila interna vazia
        }
    }

    // Adiciona um valor no final da fila
    fn adicionar(&mut self, valor: i32) {
        self.dados.push_back(valor); // Adiciona no final
    }

    // Remove o valor da frente da fila
    fn remover(&mut self) -> Option<i32> {
        self.dados.pop_front() // Remove o primeiro (se houver)
    }

    // Mostra quem está na frente da fila, sem remover
    fn frente(&self) -> Option<&i32> {
        self.dados.front() // Retorna o primeiro elemento
    }

    // Verifica se a fila está vazia
    fn esta_vazia(&self) -> bool {
        self.dados.is_empty()
    }

    // Retorna quantos elementos há na fila
    fn tamanho(&self) -> usize {
        self.dados.len()
    }
}

// Função principal do programa
fn main() {
    // Cria uma nova fila
    let mut fila = Fila::new();
    println!("Fila criada!");

    // Adiciona elementos à fila
    fila.adicionar(10);
    fila.adicionar(20);
    fila.adicionar(30);
    println!("Adicionados 10, 20, 30");

    // Mostra quem está na frente
    println!("Frente da fila: {:?}", fila.frente()); // Deve mostrar Some(10)

    // Mostra o tamanho da fila
    println!("Tamanho atual: {}", fila.tamanho()); // Deve mostrar 3

    // Remove o primeiro da fila
    println!("Removendo da fila: {:?}", fila.remover()); // Remove o 10

    // Mostra a nova frente
    println!("Nova frente: {:?}", fila.frente()); // Deve mostrar Some(20)

    // Verifica se está vazia
    println!("Fila está vazia? {}", fila.esta_vazia()); // false
}
