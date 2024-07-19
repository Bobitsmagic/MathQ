use std::cmp::Ordering;

use super::math_type_name::MathTypeName;


pub struct MathType {
    pub type_name: MathTypeName,
    pub parameter: Vec<MathType>,
}

//Type, Parameter count, Parameters cmp
impl PartialOrd for MathType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let name_order = self.type_name.cmp(&other.type_name);
        if name_order != Ordering::Equal {
            return Some(name_order);
        }

        if self.parameter.len() != other.parameter.len() {
            return Some(self.parameter.len().cmp(&other.parameter.len()));
        }

        for i in 1..self.parameter.len() {
            let cmp = self.parameter[i].partial_cmp(&self.parameter[i - 1]);    

            if let Some(res) = cmp {
                if res == Ordering::Less {
                    return None;
                }
            }
            else {
                return None;
            }

            let cmp = other.parameter[i].partial_cmp(&other.parameter[i - 1]);    

            if let Some(res) = cmp {
                if res == Ordering::Less {
                    return None;
                }
            }
            else {
                return None;
            }
        }

        for i in 0..self.parameter.len() {
            let cmp = self.parameter[i].partial_cmp(&other.parameter[i]);    

            if let Some(res) = cmp {
                if res != Ordering::Equal {
                    return Some(res);
                }
            }
            else {
                return None;
            }
        }        

        Some(Ordering::Equal)
    }
}

impl PartialEq for MathType {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl MathType {
    pub fn new(type_name: MathTypeName, parameter: Vec<MathType>) -> MathType {

        let (min, max) = type_name.parameter_range();
        assert!(min <= parameter.len());
        assert!(max >= parameter.len());

        MathType {
            type_name,
            parameter,
        }
    }

    pub fn sort(&mut self) {
        for i in 0..self.parameter.len() {
            self.parameter[i].sort();
        }

        self.parameter.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    pub fn get_typst_string(&self) -> String {
        match self.type_name {
            MathTypeName::Undefined => "Undefined".to_string(),

            MathTypeName::Variable(ref name) => name.clone(),
            MathTypeName::NaturalNumber(value) => value.to_string(),
            MathTypeName::Sum => {
                if self.parameter.len() == 0 {
                    return "zero".to_string();
                }

                if self.parameter.len() == 1 {
                    return self.parameter[0].get_typst_string();
                }

                return self.parameter.iter().map(|x| {
                    if x.type_name.precedence() <= self.type_name.precedence() {
                        format!("({})", x.get_typst_string())
                    } else {
                        x.get_typst_string()
                    }
                
                }).collect::<Vec<String>>().join(" + ");
            },
            MathTypeName::FlipSign => "-".to_owned() + &self.parameter[0].get_typst_string(),
            
            MathTypeName::Product => {
                if self.parameter.len() == 0 {
                    return "one".to_string();
                }

                let mut string = "".to_string();

                for i in 0..self.parameter.len() {
                    let para = &self.parameter[i];
                    if para.type_name.precedence() <= self.type_name.precedence() {
                        string += &format!("({})", para.get_typst_string());
                    } else {
                        string += &para.get_typst_string();
                    }
                    
                    
                    if i < self.parameter.len() - 1 {
                        let pair = (self.parameter[i].type_name.clone(), self.parameter[i + 1].type_name.clone());
                        println!("{:?}", pair);
    
                        string += match pair {
                            (MathTypeName::NaturalNumber(_), MathTypeName::NaturalNumber(_)) => " dot ",
                            _ => " ",
                        };
                    }
                }

                return string;
            },
            
            MathTypeName::Reciprocal => {
                return format!("one / ({})", self.parameter[0].get_typst_string());
            },
        }
    }
}
