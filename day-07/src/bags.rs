use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Bag {
    adjective: String,
    color: String,
}

impl Bag {
    pub fn new(description: &str) -> Bag {
        let parts: Vec<&str> = description.split(" ").collect();
        return Bag {
            adjective: parts[0].to_string(),
            color: parts[1].to_string(),
        };
    }
}

pub struct BagRules {
    contained_by: HashMap<Bag, HashMap<Bag, usize>>,
    contains: HashMap<Bag, HashSet<Bag>>,
}

impl BagRules {
    pub fn new() -> BagRules {
        return BagRules {
            contained_by: HashMap::new(),
            contains: HashMap::new(),
        };
    }

    pub fn add_rule(&mut self, sentence: &str) {
        let parts: Vec<&str> = sentence.split(" bags contain ").collect();
        let container_bag: Bag = Bag::new(parts[0]);
        let contents_string = parts[1];
        let mut contents: HashMap<Bag, usize> = HashMap::new();

        if !contents_string.starts_with("no") {
            let contains_parts: Vec<&str> = contents_string.split(", ").collect();
            for bag_string in contains_parts {
                let bag_string_parts: Vec<&str> = bag_string.split(' ').collect();
                if let [number_string, adjective, color, _] = &bag_string_parts[..] {
                    let contained_bag = Bag {
                        adjective: adjective.to_string(),
                        color: color.to_string(),
                    };
                    let number = number_string.parse::<usize>().unwrap();
                    contents.insert(contained_bag.clone(), number);

                    match self.contains.get_mut(&contained_bag) {
                        Some(container_bags) => {
                            container_bags.insert(container_bag.clone());
                        }
                        None => {
                            let mut container_bags: HashSet<Bag> = HashSet::new();
                            container_bags.insert(container_bag.clone());
                            self.contains.insert(contained_bag.clone(), container_bags);
                        }
                    }
                }
            }
        }

        self.contained_by.insert(container_bag, contents);
    }

    fn bags_directly_contained_by(&self, bag: &Bag) -> Option<&HashMap<Bag, usize>> {
        return self.contained_by.get(bag);
    }

    fn bags_that_directly_contain(&self, bag: &Bag) -> Option<&HashSet<Bag>> {
        return self.contains.get(bag);
    }

    pub fn bags_that_could_contain(&self, bag: &Bag) -> HashSet<Bag> {
        let mut containers: HashSet<Bag> = HashSet::new();
        let mut bags_to_check: VecDeque<Bag> = VecDeque::new();

        bags_to_check.push_back(bag.clone());

        while let Some(bag_to_check) = bags_to_check.pop_front() {
            if let Some(direct_containers) = self.bags_that_directly_contain(&bag_to_check) {
                for container in direct_containers.iter() {
                    containers.insert(container.clone());
                    bags_to_check.push_back(container.clone());
                }
            }
        }

        return containers;
    }

    // this is recursive, and we're just hoping that the problem is such that we
    // won't blow up the stack because we have a resonably small depth
    pub fn num_bags_inside(&self, bag: &Bag) -> usize {
        let bags_directly_inside = self.bags_directly_contained_by(bag);
        return match bags_directly_inside {
            None => 0,
            Some(bags_directly_inside) => bags_directly_inside
                .iter()
                .map(|(bag, count)| count + count * self.num_bags_inside(bag))
                .sum(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_empty_bag_rule() {
        let mut bag_rules = BagRules::new();
        bag_rules.add_rule("faded blue bags contain no other bags.");

        let faded_blue_bag = Bag::new("faded blue");

        assert_eq!(bag_rules.contained_by.contains_key(&faded_blue_bag), true);

        let value = bag_rules.contained_by.get(&faded_blue_bag).unwrap();
        assert_eq!(value.len(), 0);
    }

    #[test]
    fn test_add_single_bag_rule() {
        let mut bag_rules = BagRules::new();
        bag_rules.add_rule("bright white bags contain 1 shiny gold bag.");

        let bright_white_bag = Bag::new("bright white");
        let shiny_gold_bag = Bag::new("shiny gold");

        assert_eq!(bag_rules.contained_by.contains_key(&bright_white_bag), true);

        let value = bag_rules.contained_by.get(&bright_white_bag).unwrap();
        assert_eq!(value.len(), 1);

        assert_eq!(value.contains_key(&shiny_gold_bag), true);
        assert_eq!(value.get(&shiny_gold_bag), Some(&1));

        assert_eq!(bag_rules.contains.contains_key(&shiny_gold_bag), true);
        let containers = bag_rules.contains.get(&shiny_gold_bag).unwrap();
        assert_eq!(containers.len(), 1);
        assert_eq!(containers.contains(&bright_white_bag), true);
    }

    #[test]
    fn test_add_multiple_bag_rule() {
        let mut bag_rules = BagRules::new();
        bag_rules.add_rule("dark orange bags contain 3 bright white bags, 4 muted yellow bags.");

        let dark_orange_bag = Bag::new("dark orange");
        let bright_white_bag = Bag::new("bright white");
        let muted_yellow_bag = Bag::new("muted yellow");

        assert_eq!(bag_rules.contained_by.contains_key(&dark_orange_bag), true);

        let value = bag_rules.contained_by.get(&dark_orange_bag).unwrap();
        assert_eq!(value.len(), 2);

        assert_eq!(value.contains_key(&bright_white_bag), true);
        assert_eq!(value.get(&bright_white_bag), Some(&3));
        assert_eq!(value.contains_key(&muted_yellow_bag), true);
        assert_eq!(value.get(&muted_yellow_bag), Some(&4));

        assert_eq!(bag_rules.contains.contains_key(&bright_white_bag), true);
        assert_eq!(bag_rules.contains.contains_key(&muted_yellow_bag), true);

        let muted_yellow_containers = bag_rules.contains.get(&muted_yellow_bag).unwrap();
        assert_eq!(muted_yellow_containers.len(), 1);
        assert_eq!(muted_yellow_containers.contains(&dark_orange_bag), true);

        let bright_white_containers = bag_rules.contains.get(&bright_white_bag).unwrap();
        assert_eq!(bright_white_containers.len(), 1);
        assert_eq!(bright_white_containers.contains(&dark_orange_bag), true);
    }

    #[test]
    fn test_bags_that_directly_contain() {
        let mut bag_rules = BagRules::new();
        bag_rules.add_rule("bright white bags contain 1 shiny gold bag.");
        bag_rules.add_rule("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.");
        let shiny_gold_bag = Bag::new("shiny gold");

        let bags_containing_gold = bag_rules
            .bags_that_directly_contain(&shiny_gold_bag)
            .unwrap();
        assert_eq!(bags_containing_gold.len(), 2);
        assert_eq!(
            bags_containing_gold.contains(&Bag::new("bright white")),
            true
        );
        assert_eq!(
            bags_containing_gold.contains(&Bag::new("muted yellow")),
            true
        );
    }

    #[test]
    fn test_bags_that_could_contain() {
        let mut bag_rules = BagRules::new();

        let lines: Vec<&str> = r#"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "#
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect();

        for line in lines {
            bag_rules.add_rule(line);
        }

        let shiny_gold_bag = Bag::new("shiny gold");
        let possible_containers = bag_rules.bags_that_could_contain(&shiny_gold_bag);

        assert_eq!(possible_containers.len(), 4);
        let expected_containers = vec!["bright white", "muted yellow", "dark orange", "light red"];

        for expected_container in expected_containers {
            assert_eq!(
                possible_containers.contains(&Bag::new(expected_container)),
                true
            );
        }
    }

    #[test]
    fn num_bags_inside() {
        let mut bag_rules = BagRules::new();
        let lines = get_test_lines();

        for line in lines {
            bag_rules.add_rule(line);
        }

        let shiny_gold_bag = Bag::new("shiny gold");

        assert_eq!(bag_rules.num_bags_inside(&shiny_gold_bag), 32);
    }

    fn get_test_lines() -> Vec<&'static str> {
        return r#"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "#
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect();
    }
}
