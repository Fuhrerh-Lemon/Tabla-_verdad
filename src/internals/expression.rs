use super::var_values::VarValues;

pub enum BinOpType { And, Or, Implies, Iff }

pub enum Expr {
    Var(String),
    BinOp(BinOpType, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

// Metodos lógicos
use fns::*;
mod fns {
    #![allow(dead_code)]
    pub fn and(l: bool, r: bool) -> bool { l && r }
    pub fn or(l: bool, r: bool) -> bool { l || r }
    pub fn not(_: bool, r: bool) -> bool { !r }
    pub fn implies(l: bool, r: bool) -> bool { !l || r }
    pub fn iff(l: bool, r: bool) -> bool { l == r }
}

impl Expr {
    pub fn evaluar(&self, var_valor: &VarValues) -> bool {
        use Expr::*;
        use BinOpType::*;
        match self { // Match evaluará si es un operador o expresión
            Var(v) => var_valor.get_value(v),
            Not(e) => !e.evaluar(var_valor),
            BinOp(ty, detras, delante) => {
                let func = match ty {
                    And => and,
                    Or => or,
                    Implies => implies,
                    Iff => iff,
                };

                func(detras.evaluar(var_valor), delante.evaluar(var_valor))
            }
        }
    }
}