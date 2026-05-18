use crate::expr::*;

pub fn eval(op: Expr) -> Option<f64> {
    match op {
        Expr::Integer(i) => Some(i.into()),
        Expr::UnaryMinus(expr) => Some((-(eval(*expr)?)).into()),
        Expr::BinOp { lhs, op, rhs } => {
            match op {
                Op::Add => Some(eval(*lhs)? + eval(*rhs)?),
                Op::Subtract => Some(eval(*lhs)? - eval(*rhs)?),
                Op::Multiply => Some(eval(*lhs)? * eval(*rhs)?),
                Op::Divide => {
                    let l_val = eval(*lhs)?;
                    let r_val = eval(*rhs)?;
                    if r_val == 0.0 {
                        None
                    } else {
                        Some(l_val / r_val)
                    }
                }
                Op::Modulo => {
                    let l_val = eval(*lhs)?;
                    let r_val = eval(*rhs)?;
                    if r_val == 0.0 {
                        None
                    } else {
                        Some(l_val % r_val)
                    }
                }
            }
        }
    }
}
