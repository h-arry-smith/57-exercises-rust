use std::io;
use std::io::Write;

struct DecisionNode {
    text: String,
    yes_branch: Option<Box<DecisionNode>>,
    no_branch: Option<Box<DecisionNode>>,
}

impl DecisionNode {
    fn new(text: &str, yes_branch: DecisionNode, no_branch: DecisionNode) -> Self {
        DecisionNode {
            text: text.to_string(),
            yes_branch: Some(Box::new(yes_branch)),
            no_branch: Some(Box::new(no_branch)),
        }
    }

    fn new_leaf(text: &str) -> Self {
        DecisionNode {
            text: text.to_string(),
            yes_branch: None,
            no_branch: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.yes_branch.is_none() && self.no_branch.is_none()
    }
}

fn main() {
    let clean_terminals = DecisionNode::new_leaf("Clean terminals and try again");
    let replace_cables = DecisionNode::new_leaf("Replace cables and try again");
    let replace_the_battery = DecisionNode::new_leaf("Replace the battery");
    let check_spark_plugs = DecisionNode::new_leaf("Check spark plug connections");
    let check_choke = DecisionNode::new_leaf("Check to ensure choke is opening and closing");
    let service = DecisionNode::new_leaf("Get it in for a service");
    let unknown = DecisionNode::new_leaf("Unknown problem!");

    let mut decision_tree = DecisionNode::new(
        "Is the car silent when you turn the key?",
        DecisionNode::new(
            "Are the battery terminals connected?",
            clean_terminals,
            replace_cables,
        ),
        DecisionNode::new(
            "Does the car make a clicking noise?",
            replace_the_battery,
            DecisionNode::new(
                "Does the car crank up but fail to start?",
                check_spark_plugs,
                DecisionNode::new(
                    "Does the engine start and then die?",
                    DecisionNode::new("Does your car have fuel injection?", service, check_choke),
                    unknown,
                ),
            ),
        ),
    );

    while !decision_tree.is_leaf() {
        match get_yesno(&decision_tree.text) {
            Answer::Yes => decision_tree = *decision_tree.yes_branch.unwrap(),
            Answer::No => decision_tree = *decision_tree.no_branch.unwrap(),
        }
    }

    println!("{}", decision_tree.text);
}

enum Answer {
    Yes,
    No,
}

fn get_yesno(question_text: &str) -> Answer {
    let mut answer = String::new();

    loop {
        print!("{question_text} (y/n) ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read a line.");

        match answer.to_lowercase().trim() {
            "y" => return Answer::Yes,
            "n" => return Answer::No,
            _ => continue,
        }
    }
}
