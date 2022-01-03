use super::var_values::VarValues;
use crate::internals::expression::Expr;

pub fn mostrar(original: &str, expresion: &Expr, mut var_valor: VarValues) {
    let mut tam_valor = vec![];
    for nombre in var_valor.names() {
        tam_valor.push(nombre.len());
        print!("{} ", nombre)
    }
    print!(" —  {}", original);
    println!();

    let mut tautologia = true;
    let mut contradiccion = true;
    let mut incluir = true;

    loop {
        for (espacio, valor) in tam_valor
            .iter()
            .zip(var_valor.values()) {
            if valor { print!("T"); } else { print!("F"); }
            for _ in 0..*espacio { print!(" "); }
        }

        if incluir { print!(" -  ") } else { print!(" —  ") };
        incluir = !incluir;

        if expresion.evaluar(&var_valor) {
            print!("T");
            contradiccion = false;
        } else {
            print!("F");
            tautologia = false;
        }
        println!();

        if !var_valor.advance() { break }
    }

    if tautologia && contradiccion {
        panic!("Tautología & Contradicción")
    } else if tautologia {
        println!("(Tautología)")
    } else if contradiccion {
        println!("(Contradicción)")
    }
}