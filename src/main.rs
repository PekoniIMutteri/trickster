use trickster::{Fragment, FragmentType, Spell, Trick};

fn main() {
    let a = to_spell("+2+2+").unwrap();
    if let Some(error) = a.type_check() {
        println!("Found {} errors:", error.split('\n').count());
        println!("{}", error);
    } else {
        println!("No errors");
    }
    //dbg!(a.execute().unwrap());
    //println!("Hello, world!");
}

fn to_spell(input: &str) -> Option<Spell> {
    if let Ok(num) = input.trim().parse::<f64>() {
        Some(Spell {
            fragment: Fragment::Number(num),
            sub_spells: vec![],
        })
    } else if input.trim().is_empty() {
        Some(Spell {
            fragment: Fragment::Void,
            sub_spells: vec![],
        })
    } else {
        let (lhs, rhs) = input.split_once('+')?;
        Some(Spell {
            fragment: Fragment::Trick(Trick {
                evaluate_func: add,
                type_checker: add_checker,
                output_type: FragmentType::Number,
            }),
            sub_spells: vec![to_spell(lhs)?, to_spell(rhs)?],
        })
    }
}

fn add_checker(inputs: &[FragmentType]) -> Option<String> {
    if inputs.len() < 2 {
        return Some(format!(
            "Wrong input count: Expected 2 or more inputs, got {}.",
            inputs.len()
        ));
    }
    for (index, other) in inputs.iter().enumerate() {
        if *other != FragmentType::Number {
            return Some(format!(
                "Wrong input type: input {} is {} instead of Number.",
                index + 1,
                other
            ));
        }
    }
    None
}
fn add(inputs: &[Fragment]) -> Result<Fragment, String> {
    if inputs.len() < 2 {
        return Err("Tried to do addition with too few arguments.".to_string());
    }
    let mut total = 0.0;
    for frag in inputs {
        if let Fragment::Number(num) = frag {
            total += num;
        } else {
            return Err("Tried to add something else than a number.".to_string());
        }
    }
    Ok(Fragment::Number(total))
}
