fn inverter_string(s: &str) -> String {
    // Convertendo a string em um vetor de caracteres, invertendo e coletando de volta para uma string
    s.chars().rev().collect()
}

fn main() {
    let texto = "Olá Mundo!";
    println!("Texto original: {}", texto);
    
    let texto_invertido = inverter_string(texto);
    println!("Texto invertido: {}", texto_invertido);
}
