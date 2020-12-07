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

        if i < elements.len() - 1 {
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

impl BagTypeRegist {
    pub fn add(&mut self, bag_type: String) {
        self.index_to_str.push(bag_type.clone());
        self.str_to_index.insert(bag_type, self.index_to_str.len() - 1);
    }

    pub fn get_type(&self, s: &str) -> Option<BagType> {
        self.str_to_index.get(s).map(|x| x.clone())
    }
}

// impl BagTypeRegist {
//     pub fn test() -> Self {
//         let mut regist = BagTypeRegist::default();
//         regist.add("light red".to_string());
//         regist.add("bright white".to_string());
//         regist.add("muted yellow".to_string());
//         regist
//     }
// }