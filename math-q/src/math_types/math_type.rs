use std::{char::ParseCharError, cmp::Ordering, iter::Sum};

use crate::math_types::{math_type, typst_symbols};

use super::math_type_name::MathTypeName;


#[derive(Clone, Eq)]
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

        //Check if the parameters are sorted
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

pub fn natural_number(value: u128) -> MathType {
    MathType::new(MathTypeName::NaturalNumber(value), vec![])
}
pub fn variable(name: &str) -> MathType {
    MathType::new(MathTypeName::Function(name.to_string()), vec![])
}
pub fn function(name: &str, parameter: Vec<MathType>) -> MathType {
    MathType::new(MathTypeName::Function(name.to_string()), parameter)
}
pub fn sum(parameter: Vec<MathType>) -> MathType {
    MathType::new(MathTypeName::Sum, parameter)
}
pub fn flip_sign(parameter: MathType) -> MathType {
    MathType::new(MathTypeName::FlipSign, vec![parameter])
}
pub fn product(parameter: Vec<MathType>) -> MathType {
    MathType::new(MathTypeName::Product, parameter)
}
pub fn power(base: MathType, exponent: MathType) -> MathType {
    MathType::new(MathTypeName::Power, vec![base, exponent])
}
pub fn reciprocal(parameter: MathType) -> MathType {
    power(parameter, flip_sign(natural_number(1)))
}
pub fn fraction(numerator: MathType, denominator: MathType) -> MathType {
    MathType::new(MathTypeName::Product, vec![numerator, reciprocal(denominator)])
}

pub fn exponential(exponent: MathType) -> MathType {
    MathType::new(MathTypeName::Exp, vec![exponent])
}

pub fn logarithm(parameter: MathType) -> MathType {
    MathType::new(MathTypeName::LogN, vec![parameter])
}

impl MathType {
    pub fn new(type_name: MathTypeName, parameter: Vec<MathType>) -> MathType {

        let (min, max) = type_name.parameter_range();

        // println!("{:?}: {}", type_name, parameter.len());

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

        if self.type_name.is_commutative() {
            self.parameter.sort_by(|a, b| a.partial_cmp(b).expect(format!("Comparison error on sorting parameters [{}] and [{}]", a.get_typst_string(), b.get_typst_string()).as_str()));
        }
    }

    pub fn is_variable(&self) -> bool {
        // println!("Is function: {}", matches!(self.type_name, MathTypeName::Function(_)));

        // println!("Parameter count: {}", self.parameter.len());

        return matches!(self.type_name, MathTypeName::Function(_)) && self.parameter.len() == 0;
    }

    pub fn flatten(&self) -> MathType {
        match self.type_name {
            MathTypeName::Product => {
                let para = self.parameter.iter().map(|x| x.flatten()).collect::<Vec<MathType>>();

                let mut new_para = vec![];

                for i in 0..para.len() {
                    if para[i].type_name == MathTypeName::Product {
                        new_para.append(&mut para[i].parameter.clone());
                    } else {
                        new_para.push(para[i].clone());
                    }
                }

                return MathType::new(self.type_name.clone(), new_para);
            },

            MathTypeName::Sum => {
                let para = self.parameter.iter().map(|x| x.flatten()).collect::<Vec<MathType>>();

                let mut new_para = vec![];

                for i in 0..para.len() {
                    if para[i].type_name == MathTypeName::Sum {
                        new_para.append(&mut para[i].parameter.clone());
                    } else {
                        new_para.push(para[i].clone());
                    }
                }

                return MathType::new(self.type_name.clone(), new_para);
            },

            _ => MathType::new(self.type_name.clone(), self.parameter.iter().map(|x| x.flatten()).collect()),
        }
    }
    pub fn expand(&self) -> MathType {
        match self.type_name {
            MathTypeName::Product => {
                let para = self.parameter.iter().map(|x| x.expand()).collect::<Vec<MathType>>();

                fn backtrack_sums(para: &Vec<MathType>, index: usize, current: &mut Vec<MathType>, list: &mut Vec<MathType>) {
                    if index == para.len() {
                        list.push(product(current.clone()));
                        return;
                    }

                    if para[index].type_name == MathTypeName::Sum {
                        for i in 0..para[index].parameter.len() {
                            current.push(para[index].parameter[i].clone());
                            backtrack_sums(para, index + 1, current, list);
                            current.pop();
                        }
                    } else {
                        current.push(para[index].clone());
                        backtrack_sums(para, index + 1, current, list);
                        current.pop();
                    }
                }

                let mut new_para = Vec::new(); 
                backtrack_sums(&para, 0, &mut Vec::new(), &mut new_para);

                return sum(new_para);
            },

            _ => MathType::new(self.type_name.clone(), self.parameter.iter().map(|x| x.expand()).collect()),
        }
    }

    /*
    a + a => 2 * a
    a + a + -a => a

     */
    pub fn combine(&self) -> MathType {
        match self.type_name {
            MathTypeName::Sum => {
                let mut para = self.parameter.iter().map(|x| x.combine()).collect::<Vec<MathType>>();

                for i in 0..para.len() {
                    para[i].sort();
                }

                let mut new_para = vec![];
                let mut collected = vec![false; para.len()];
                for i in 0..para.len() {
                    let mut count = 1;

                    if collected[i] {
                        continue;
                    }

                    for j in i + 1..para.len() {
                        if para[i] == para[j]{
                            count += 1;
                            collected[j] = true;
                        }
                        else {
                            if para[i] == flip_sign(para[j].clone()) {
                                count -= 1;
                                collected[j] = true;
                            }
                        }
                    }

                    if count == 1 {
                        new_para.push(para[i].clone());
                    } else {
                        new_para.push(product(vec![natural_number(count), para[i].clone()]));
                    }
                }

                return MathType::new(self.type_name.clone(), new_para);
            },

            MathTypeName::Product => {
                let mut para = self.parameter.iter().map(|x| x.combine()).collect::<Vec<MathType>>();

                for i in 0..para.len() {
                    para[i].sort();
                }

                let mut new_para = vec![];
                let mut collected = vec![false; para.len()];
                for i in 0..para.len() {
                    let mut count = 1;

                    if collected[i] {
                        continue;
                    }

                    for j in i + 1..para.len() {
                        if para[i] == para[j]{
                            count += 1;
                            collected[j] = true;
                        }
                        else {
                            if para[i] == reciprocal(para[j].clone()) {
                                count -= 1;
                                collected[j] = true;
                            }
                        }
                    }

                    if count == 1 {
                        new_para.push(para[i].clone());
                    } else {
                        new_para.push(product(vec![natural_number(count), para[i].clone()]));
                    }
                }

                return MathType::new(self.type_name.clone(), new_para);
            }

            _ => MathType::new(self.type_name.clone(), self.parameter.iter().map(|x| x.combine()).collect()),
        }
    }

    /*
    0 + a => a
    2 + a + 3 => 5 + a
    sum[] => 0
    sum[a] => a

    0 * a => 0
    1 * a => a
    2 * a * 3 => 6 * a
    product[] => 1
    product[a] => a
     */
    pub fn reduce_neutral(&self) -> MathType {

        match self.type_name {
            MathTypeName::Sum => {
                let mut sum = vec![];

                let mut const_sum = 0;
                for i in 0..self.parameter.len() {
                    let reduced = self.parameter[i].reduce_neutral();
                    if matches!(reduced.type_name, MathTypeName::NaturalNumber(_)) {
                        match reduced.type_name {
                            MathTypeName::NaturalNumber(value) => const_sum += value,
                            _ => unreachable!(),
                        }

                        continue;
                    }

                    sum.push(reduced);
                }

                if const_sum != 0 {
                    sum.insert(0, natural_number(const_sum));
                }

                if sum.len() == 0 {
                    return MathType::new(MathTypeName::NaturalNumber(0), vec![]);
                }

                if sum.len() == 1 {
                    return sum[0].clone();
                }

                return MathType::new(MathTypeName::Sum, sum);
            },
            MathTypeName::Product => {
                let mut prod = vec![];
                let mut const_prod = 1;
                for i in 0..self.parameter.len() {
                    let reduced = self.parameter[i].reduce_neutral();
                    if matches!(reduced.type_name, MathTypeName::NaturalNumber(_)) {
                        match reduced.type_name {
                            MathTypeName::NaturalNumber(value) => const_prod *= value,
                            _ => unreachable!(),
                        }

                        continue;
                    }

                    prod.push(reduced);
                }

                if const_prod == 0 {
                    return natural_number(0);
                }

                if const_prod != 1 {
                    prod.insert(0, natural_number(const_prod));
                }

                if prod.len() == 0 {
                    return MathType::new(MathTypeName::NaturalNumber(1), vec![]);
                }

                if prod.len() == 1 {
                    return prod[0].clone();
                }

                return MathType::new(MathTypeName::Product, prod);
            },

            MathTypeName::FlipSign => {
                let reduced = self.parameter[0].reduce_neutral();
                if reduced.type_name == MathTypeName::NaturalNumber(0) {
                    return natural_number(0);
                }

                if reduced.type_name == MathTypeName::FlipSign {
                    return reduced.parameter[0].clone();
                }

                return MathType::new(MathTypeName::FlipSign, vec![reduced]);
            }
            _ => MathType::new(self.type_name.clone(), self.parameter.iter().map(|x| x.reduce_neutral()).collect()),
        }
    }

    pub fn get_derivative(&self, delta_var: &MathType) -> MathType {
        assert!(delta_var.is_variable(), "Derivative can only be calculated for variables");

        return match self.type_name {
            MathTypeName::Function(_) => if self.type_name == delta_var.type_name {
                MathType::new(MathTypeName::NaturalNumber(1), vec![])
            } else {
                sum((0..self.parameter.len()).map(|i| {
                    let mut parameter = self.parameter.clone();
                    parameter[i] = delta_var.clone();
    
                    return product(vec![
                        self.parameter[i].get_derivative(delta_var),
                        variable(&format!("d/d{}", delta_var.get_typst_string())).clone(),
                        MathType::new(self.type_name.clone(), parameter)
                    ]);
                }).collect())
            },

            MathTypeName::NaturalNumber(_) => MathType::new(MathTypeName::NaturalNumber(0), vec![]),
            MathTypeName::Sum =>  MathType::new(MathTypeName::Sum, self.parameter.iter().map(|x| x.get_derivative(delta_var)).collect()),
            MathTypeName::FlipSign => MathType::new(MathTypeName::FlipSign, vec![self.parameter[0].get_derivative(delta_var)]),
            MathTypeName::Product => {
                let mut sum = vec![];
                for i in 0..self.parameter.len() {
                    let mut prod = vec![];
                    for j in 0..self.parameter.len() {
                        if i == j {
                            prod.push(self.parameter[j].get_derivative(delta_var));
                        } else {
                            prod.push(self.parameter[j].clone());
                        }
                    }
                    sum.push(MathType::new(MathTypeName::Product, prod));
                }
                return MathType::new(MathTypeName::Sum, sum);
            },
            MathTypeName::Exp => {
                return product(vec![
                    self.clone(), 
                    self.parameter[0].get_derivative(delta_var),
                ]);
            }
            MathTypeName::LogN => {
                return product(vec![
                    self.parameter[0].get_derivative(delta_var),
                    reciprocal(self.parameter[0].clone()), 
                ]);
            }
            MathTypeName::Power => {
                let base = self.parameter[0].clone();
                let exponent = self.parameter[1].clone();

                return exponential(product(vec![exponent, logarithm(base)])).get_derivative(delta_var);
            }
            MathTypeName::Undefined => MathType::new(MathTypeName::Undefined, vec![]) 
        };
    }

    pub fn get_string(&self) -> String {
        match self.type_name {
            MathTypeName::Undefined => "Undefined".to_string(),
            MathTypeName::NaturalNumber(value) => value.to_string(),
            MathTypeName::Sum => {
                return format!("+({})", self.parameter.iter().map(|x| {
                    if x.type_name.precedence() <= self.type_name.precedence() {
                        format!("({})", x.get_string())
                    } else {
                        x.get_string()
                    }
                
                }).collect::<Vec<String>>().join(", "));
            },
            MathTypeName::FlipSign => format!("-({})", self.parameter[0].get_string()),
            
            MathTypeName::Product => {
                return format!("*({})", self.parameter.iter().map(|x| {
                    if x.type_name.precedence() <= self.type_name.precedence() {
                        format!("({})", x.get_string())
                    } else {
                        x.get_string()
                    }
                
                }).collect::<Vec<String>>().join(", "));
            },

            MathTypeName::Function(ref name) => {
                if self.parameter.len() == 0 {
                    return name.clone();
                }

                return format!("{}({})", name, self.parameter.iter().map(|x| x.get_string()).collect::<Vec<String>>().join(", "));
            }

            MathTypeName::Exp => {
                return format!("exp({})", self.parameter[0].get_string());
            }

            MathTypeName::LogN => {
                return format!("ln({})", self.parameter[0].get_string());
            }

            MathTypeName::Power => {
                return format!("pow({}, {})", self.parameter[0].get_string(), self.parameter[1].get_string());
            }
        }
    }

    pub fn get_typst_string(&self) -> String {
        match self.type_name {
            MathTypeName::Undefined => "Undefined".to_string(),
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
                        // println!("{:?}", pair);
    
                        string += match pair {
                            (MathTypeName::NaturalNumber(_), MathTypeName::NaturalNumber(_)) => " dot ",
                            _ => " ",
                        };
                    }
                }

                return string;
            },

            MathTypeName::Function(ref name) => {
                if self.parameter.len() == 0 {
                    return name.clone();
                }

                return format!("{}({})", name, self.parameter.iter().map(|x| x.get_typst_string()).collect::<Vec<String>>().join(", "));
            }

            MathTypeName::Exp => {
                return format!("e^({})", self.parameter[0].get_typst_string());
            }

            MathTypeName::LogN => {
                return format!("ln({})", self.parameter[0].get_typst_string());
            }

            MathTypeName::Power => {
                return format!("{}^({})", self.parameter[0].get_typst_string(), self.parameter[1].get_typst_string());
            }   
        }
    }
}
