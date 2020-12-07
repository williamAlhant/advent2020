type BagType = usize;

#[derive(Default)]
pub struct BagTypeRegist {
    index_to_str: Vec<String>,
    str_to_index: std::collections::HashMap<String, BagType>
}

pub struct NumBags {
    pub num: usize,
    pub bag_type: BagType
}

pub struct Rule {
    pub bag_type: BagType,
    pub elements: Vec<NumBags>
}

fn elements_to_str(elements: &Vec<NumBags>, regist: &BagTypeRegist) -> String {
    if elements.is_empty() {
        return "no other bags".to_string();
    }
    let mut s = String::new();
    for (i, e) in elements.iter().enumerate() {
        s.push_str(&format!("{} {} {}",
            e.num.to_string(),
            regist.index_to_str[e.bag_type],
            match e.num {
                1 => "bag",
                _ => "bags"
            }
        ));

        if i != elements.len() {
            s.push_str(", ");
        }
    }
    s
}

impl Rule {
    pub fn to_str(&self, regist: &BagTypeRegist) -> String {
        format!("{} bags contain {}.", 
            regist.index_to_str[self.bag_type], elements_to_str(&self.elements, regist))
    }
}