use std::collections::LinkedList;

fn main() {
    // Crie uma lista ligada vazia
    let mut frases = LinkedList::new();
    
    // TODO: Adicione 10 frases motivacionais à lista

    frases.push_back("A VIDA É BELA");
    frases.push_back("VIDA AGORA");
    frases.push_back("NÃO DEIXA PASSAR AS OPORTUNIDADES");
    frases.push_back("SOMOS FORTES");
    frases.push_back("DEIXA ACONTECER NATURALMENTE");
    frases.push_back("FOCO, FORÇA E FÉ");
    frases.push_back("UM DIA APÓS O OUTRO DIA");
    frases.push_back("UM COPO VAZIO NÃO É UM COPO MEIO CHEIO");
    frases.push_back("A VIDA PASSA, MAS ELE NÃO");
    frases.push_back("UMA VEZ FLAMENGO SEMPRE FLAMENGO!");


    // TODO: Acesse e imprima o terceiro elemento
    
    if let Some(frase) = frases.iter().nth(2) {
        println!("A terceira frase é: {}", frase);
    } else {
        println!("A lista não tem três frases.");
    }

    // TODO: Imprima o tamanho total da lista

    println!("A quantidade de frases é: {}",frases.len());
}