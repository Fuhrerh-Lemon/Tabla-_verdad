// Importando modulos
use std::io;
mod internals;

// Estructura para guardar la expresión establecida por el usuario
#[derive(Debug)]
struct Comando { expresion: String }

// Implementación referente a la estructura Comando
impl Comando {
    // Dentro tenemos 2 metodos, uno para recibir el valor o expresión
    pub fn nuevo(expresion: String) -> Self {
        Self {
            expresion
        }
    }
    // Segundo metodo envía el valor a verificar y transformarse a un operador
    // ejemplo: el usuario digita '!' es un string
    // con el segundo metodo será transformado a un operador ! negación en rust
    pub fn tabla(&self) {
        internals::mostrar(&self.expresion);
    }
}

// Función principal
fn main() {
    println!("Digite los valores a evaluar, ejemplo: !p | q & (!r -> q)");
    let mut valor_usuario = String::new();
    io::stdin().read_line(&mut valor_usuario)
        .expect("Error getting guess");
    println!("----------------------------------");
    // Creamos y guardamos el valor establecido por el usuario
    let valor = format!("{}", valor_usuario.trim());
    let expresion = Comando::nuevo(valor);
    // llamamos al metodo tabla
    expresion.tabla();

    println!("----------------------------------");
}