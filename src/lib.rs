use std::fmt::Display;

// TODO: Make it so that a single trick can return multiple errors.

#[derive(Clone, Debug, Default)]
pub struct Spell {
    pub fragment: Fragment,
    pub sub_spells: Vec<Spell>,
}

#[derive(Clone, Debug, Default)]
pub enum Fragment {
    #[default]
    Void,
    Number(f64),
    Trick(Trick),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FragmentType {
    #[default]
    Void,
    Number,
    Trick,
}

#[derive(Clone, Debug)]
pub struct Trick {
    pub evaluate_func: fn(&[Fragment]) -> Result<Fragment, String>,
    pub type_checker: fn(&[FragmentType]) -> Option<String>,
    pub output_type: FragmentType,
}

impl Spell {
    pub fn execute(&self) -> Result<Fragment, String> {
        let mut inputs: Vec<Fragment> = vec![];
        for sub in &self.sub_spells {
            inputs.push(sub.execute()?);
        }
        match &self.fragment {
            Fragment::Void | Fragment::Number(_) => Ok(self.fragment.clone()),
            Fragment::Trick(trick) => (trick.evaluate_func)(&inputs),
        }
    }
    pub fn type_check(&self) -> Option<String> {
        self.type_check_rec(vec![])
    }
    pub fn type_check_rec(&self, adress: Vec<usize>) -> Option<String> {
        let mut errors: Vec<String> = self
            .sub_spells
            .iter()
            .enumerate()
            .filter_map(|(index, sub)| {
                let mut new_adress = adress.clone();
                new_adress.push(index);
                sub.type_check_rec(new_adress)
            })
            .collect();
        let Fragment::Trick(trick) = &self.fragment else {
            if errors.is_empty() {
                return None;
            } else {
                return Some(errors.join("\n"));
            }
        };
        let inputs: Vec<FragmentType> = self
            .sub_spells
            .iter()
            .map(|sub| sub.fragment.output_type())
            .collect();
        if let Some(new_error) = (trick.type_checker)(&inputs) {
            errors.insert(0, format!("at {}: {}", adress_to_str(&adress), new_error));
        }
        if errors.is_empty() {
            None
        } else {
            Some(errors.join("\n"))
        }
    }
}

impl From<Fragment> for FragmentType {
    fn from(value: Fragment) -> Self {
        match value {
            Fragment::Void => FragmentType::Void,
            Fragment::Number(_) => FragmentType::Number,
            Fragment::Trick(_) => FragmentType::Trick,
        }
    }
}

impl Fragment {
    pub fn output_type(&self) -> FragmentType {
        match self {
            Fragment::Trick(trick) => trick.output_type,
            _ => self.clone().into(),
        }
    }
}
impl Display for FragmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            FragmentType::Void => "Void",
            FragmentType::Number => "Number",
            FragmentType::Trick => "Trick",
        };
        f.pad(a)
    }
}

fn adress_to_str(adress: &[usize]) -> String {
    let a: Vec<String> = adress.iter().map(|ind| ind.to_string()).collect();
    format!("[{}]", a.join(","))
}

#[cfg(test)]
mod tests {

    #[test]
    fn testing() {}
}
