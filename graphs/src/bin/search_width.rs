use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref TRAVELS: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut map = HashMap::new();
        map.insert(
            "argentina",
            HashMap::from([("west", "chile"), ("east", "uruguay"), ("north", "bolivia")]),
        );
        map.insert(
            "chile",
            HashMap::from([("east", "argentina"), ("north", "peru")]),
        );
        map.insert(
            "paraguay",
            HashMap::from([
                ("south", "argentina"),
                ("north", "bolivia"),
                ("east", "brazil"),
            ]),
        );
        map.insert(
            "uruguay",
            HashMap::from([("west", "argentina"), ("north", "paraguay")]),
        );
        map.insert(
            "bolivia",
            HashMap::from([
                ("south", "argentina"),
                ("north", "peru"),
                ("east", "brazil"),
                ("west", "paraguay"),
            ]),
        );
        map.insert(
            "peru",
            HashMap::from([
                ("east", "bolivia"),
                ("south", "chile"),
                ("north", "ecuador"),
                ("west", "colombia"),
            ]),
        );
        map.insert(
            "brazil",
            HashMap::from([
                ("north", "colombia"),
                ("northeast", "guyana"),
                ("east", "suriname"),
                ("south", "uruguay"),
                ("west", "peru"),
            ]),
        );
        map.insert(
            "colombia",
            HashMap::from([
                ("north", "venezuela"),
                ("east", "brazil"),
                ("south", "peru"),
                ("west", "panama"),
            ]),
        );
        map.insert(
            "venezuela",
            HashMap::from([
                ("north", "colombia"),
                ("east", "guyana"),
                ("south", "brazil"),
            ]),
        );
        map.insert(
            "guyana",
            HashMap::from([("south", "brazil"), ("west", "suriname")]),
        );
        map.insert(
            "suriname",
            HashMap::from([("east", "guyana"), ("south", "brazil")]),
        );
        map.insert(
            "ecuador",
            HashMap::from([("south", "peru"), ("east", "colombia")]),
        );
        map.insert("french_guiana", HashMap::from([("south", "brazil")]));
        map.insert(
            "mexico",
            HashMap::from([
                ("north", "united_states"),
                ("south", "guatemala"),
                ("east", "belize"),
            ]),
        );
        map.insert(
            "guatemala",
            HashMap::from([
                ("north", "mexico"),
                ("east", "belize"),
                ("south", "honduras"),
            ]),
        );
        map.insert("belize", HashMap::from([("west", "guatemala")]));
        map.insert(
            "honduras",
            HashMap::from([
                ("north", "guatemala"),
                ("east", "nicaragua"),
                ("south", "el_salvador"),
            ]),
        );
        map.insert(
            "nicaragua",
            HashMap::from([("west", "honduras"), ("south", "costa_rica")]),
        );
        map.insert("el_salvador", HashMap::from([("north", "honduras")]));
        map.insert("costa_rica", HashMap::from([("north", "nicaragua")]));
        map.insert(
            "panama",
            HashMap::from([("west", "colombia"), ("north", "costa_rica")]),
        );
        map.insert("united_states", HashMap::from([("south", "mexico")]));
        map
    };
}

lazy_static! {
    static ref EXPLORED: Mutex<HashSet<&'static str>> = Mutex::new(HashSet::from(["argentina"]));
}

lazy_static! {
    static ref FRONTIER: Mutex<Vec<Node>> = Mutex::new(vec![]);
}

fn resolve<'a>(state: &'a str, action: &'a str) -> &'a str {
    if !&TRAVELS.contains_key(state) {
        panic!("State not found");
    }
    let state_map = TRAVELS.get(state).unwrap();
    match state_map.get(action) {
        Some(result) => result,
        None => panic!("Action not found"),
    }
}

#[derive(Debug, Clone)]
struct Node {
    state: &'static str,
    father: Option<Box<Node>>,
    route: Vec<&'static str>,
}

impl Node {
    fn expand(&self, action: &'static str, state: &'static str) -> Node {
        Node {
            state,
            father: Some(Box::new(self.clone())),
            route: {
                let mut father_route = match &self.father {
                    Some(_father) => self.route.clone(),
                    None => {
                        vec!["argentina"]
                    }
                };
                father_route.push(resolve(&self.state, action));
                father_route
            },
        }
    }
}
fn get_route(node: Box<Node>, objective: &'static str) {
    if node.state == objective {
        println!("{:?}", node.route);
        println!("we reached the objective");
        std::process::exit(0);
    } else {
        let actions = TRAVELS
            .get(&node.state)
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        
        for action in actions {
            let state = resolve(&node.state, action);
            if !EXPLORED.lock().unwrap().contains(&state) {
                FRONTIER.lock().unwrap().push(node.expand(action, state));
                EXPLORED.lock().unwrap().insert(state);
            }
        }
        let next = FRONTIER.lock().unwrap().remove(0);

        get_route(Box::new(next), objective)
        }
    }

fn main() {
    let root_node = Node {
        state: "argentina",
        father: None,
        route: vec![],
    };

    let objective: &str = "united_states";
    get_route(Box::new(root_node), objective);
}
