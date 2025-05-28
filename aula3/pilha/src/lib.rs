// Definição da estrutura de Pilha (Stack)
pub struct Stack<T> {
    // A implementação será adicionada após criar os testes
    elementos: Vec<T>,
    capacidade_maxima: Option<usize>,
}

// A implementação será adicionada após criar os testes
impl<T> Stack<T> {
    // A implementação será adicionada após criar os testes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pilha_vazia() {
        let pilha: Stack<i32> = Stack::nova();
        assert!(pilha.esta_vazia());
    }

    #[test]
    fn nova_pilha_com_capacidade() {
        let pilha: Stack<i32> = Stack::nova_com_capacidade(5);
        assert!(pilha.esta_vazia());
        assert_eq!(pilha.tamanho(), 0);
    }