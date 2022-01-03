// importamos los modulos
mod var_values;
mod expression;
mod validar;
mod parse;
mod mostrar;

use validar::{ validar, Status };

// función que evaluará y verificará que la expresión digitada por el usuario
// sea correcta, sino enviará un error de compilación
pub fn mostrar(expresion: &str) {
    use Status::*;
    // Match (similar al switch) diversas opciones de errores a la hora de evaluar la expresión
    match validar(expresion) {
        Ok => (),
        Unexpected(i, ch) => {
            let mensaje_inicial = format!("Inesperado '{}' en \"", ch);
            let espacios = mensaje_inicial.len() + i;

            println!("{}{}\" en la posicion {}", mensaje_inicial, expresion, i);

            for _ in 0..espacios { print!(" ") }
            println!("^");
            return
        },
        ExpectedAtEnd(error) => {
            let mensaje_inicial = format!("Inesperado {} en final de \"{}", error, expresion);

            println!("{}\"", mensaje_inicial);

            for _ in 0..mensaje_inicial.len() { print!(" "); }
            println!("^");
            return
        },
        Msg(error) => {
            println!("{}", error);
            return
        }
    }

    // Una vez evaluada seran transformadas a un operador
    let (expr, var_val) = parse::parse(expresion);
    // Luego a imprimir la tabla
    mostrar::mostrar(expresion, &expr, var_val)
}