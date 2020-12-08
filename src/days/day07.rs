use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context};
mod day07_struct;
mod day07_parser;
use day07_struct::{BagTypeRegist, Rule};
use day07_parser as parser;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<usize> {

    let mut regist = BagTypeRegist::default();
    let mut rules: Vec<Rule> = Vec::new();

    for line in lines {
        let line = line.context("Failed to get line")?;
        let rule = match parser::parse_rule(&line, &mut regist) {
            Ok((_, rule)) => rule,
            Err(e) => {
                bail!("Failed to parse rule\nLine: {}\n{}", line, e);
            }
        };
        rules.push(rule);
    }

    let mut rules_matrix: Mat<u8> = Mat::new(regist.len(), regist.len());
    for rule in rules {
        let i = rule.bag_type;
        for element in rule.elements {
            let j = element.bag_type;
            *rules_matrix.get_mut(j, i).context("Tried to index outside m bounds")? = element.num as u8;
        }
    }

    let shiny_gold_type = regist.get("shiny gold").context("Did not find shiny gold in regist")?;

    Ok(answer_part_two(&rules_matrix, shiny_gold_type))
}

fn print_mat_u8(m: &Mat<u8>) {
    for i in 0..m.height {
        for j in 0..m.width {
            print!("{} ", m.get(j, i).unwrap());
        }
        print!("\n");
    }
}

#[derive(Clone)]
struct Mat<T> {
    data: Vec<T>,
    width: usize,
    height: usize
}

impl<T: Default + Clone> Mat<T> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height
        }
    }
}

impl<T> Mat<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if y * self.width + x > self.data.len() {
            return None;
        }

        Some(&self.data[y * self.width + x])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if y * self.width + x > self.data.len() {
            return None;
        }

        Some(&mut self.data[y * self.width + x])
    }
}

fn count_num_true(vec: &Vec<bool>) -> usize {
    let mut n = 0;
    for el in vec {
        if *el {
            n += 1;
        }
    }
    n
}

fn answer_part_one(rules: &Mat<u8>, shiny_gold_type: usize) -> usize {
    
    let mut can_contain_shiny_gold = vec![false; rules.height];
    let mut to_visit = vec![shiny_gold_type];
    
    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        for i in 0..rules.height {
            let num_current_contained_by_i = rules.get(current, i).unwrap().clone();
            if num_current_contained_by_i != 0 {
                can_contain_shiny_gold[i] = true;
                to_visit.push(i);
            }
        }
    }

    count_num_true(&can_contain_shiny_gold)
}

fn answer_part_two(rules: &Mat<u8>, shiny_gold_type: usize) -> usize {

    let mut m = rules.clone();
    remove_unwanted_edges(&mut m, shiny_gold_type);

    let sorted = topological_sort(m, shiny_gold_type);
    let mut num_contained_by_shiny_gold: Vec<usize> = vec![0; rules.width];
    num_contained_by_shiny_gold[shiny_gold_type] = 1;

    for i in sorted {
        for j in 0..rules.width {
            let e = rules.get(j, i).unwrap().clone();
            num_contained_by_shiny_gold[j] += num_contained_by_shiny_gold[i] * e as usize;
        }
    }

    let s: usize = num_contained_by_shiny_gold.iter().sum();
    s - 1
}

fn remove_unwanted_edges(graph: &mut Mat<u8>, start: usize) {
    // remove edges that do not have start as an ancestor
    let mut to_visit: Vec<usize> = Vec::new();
    let mut keep: Vec<bool> = vec![false; graph.width];
    to_visit.push(start);
    keep[start] = true;

    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        for j in 0..graph.width {
            let e = graph.get(j, current).unwrap();
            if *e > 0 {
                to_visit.push(j);
                keep[j] = true;
            }
        }
    }

    for i in 0..graph.height {
        if keep[i] {
            continue;
        }

        for j in 0..graph.width {
            let e = graph.get_mut(j, i).unwrap();
            *e = 0;
        }
    }
}

fn topological_sort(mut graph: Mat<u8>, start: usize) -> Vec<usize> {
    let mut sorted = Vec::new();
    let mut sources = vec![start];

    while !sources.is_empty() {
        let source = sources.pop().unwrap();
        sorted.push(source);
        for j in 0..graph.width {
            // for every j that has an edge from source
            let n = graph.get_mut(j, source).unwrap();
            if *n == 0 {
                continue;
            }

            // remove edge
            *n = 0;

            // if is source (no incoming edge), add to sources
            let mut has_incoming = false;
            for i in 0..graph.height {
                if *graph.get(j, i).unwrap() > 0 {
                    has_incoming = true;
                    break;
                }
            }
            if !has_incoming {
                sources.push(j);
            }
        }
    }

    for e in graph.data {
        assert!(e == 0);
    }

    sorted
}