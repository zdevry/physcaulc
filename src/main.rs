use std::collections::HashMap;

pub mod eval;
pub mod f64plus;
pub mod parse;
pub mod rational;
pub mod utils;
pub mod value;

fn main() {
    let s = "1[kg]";

    // dbg!(parse::lex(s));

    let result = parse::lex(s).and_then(parse::parse);

    // dbg!(result);

    // match result {
    //     Ok(node) => {
    //         println!("{}", node.repr());
    //     }
    //     Err(e) => {
    //         dbg!(e);
    //     }
    // }

    let env = eval::Environment {
        consts: HashMap::from([("x".into(), rational::Rational::new(2, 3).into())]),
        evaluators: HashMap::new(),
        units: HashMap::from([(
            "kg".into(),
            eval::ConversionValue {
                factor: 1.,
                dim: value::SIDimension {
                    mass: 1.into(),
                    ..Default::default()
                },
            },
        )]),
    };
    let args = HashMap::<String, value::Value>::new();

    match result {
        Ok(node) => {
            dbg!(node.eval(&env, &args));
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
