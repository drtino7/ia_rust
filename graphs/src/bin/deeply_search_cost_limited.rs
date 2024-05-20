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
    static ref SCORES: HashMap<&'static str, HashMap<&'static str, i32>> = {
        let mut map = HashMap::new();
        map.insert(
            "argentina",
            HashMap::from([("west", 1400), ("east", 200), ("north", 3300)]),
        );

        map.insert("chile", HashMap::from([("east", 1400), ("north", 2400)]));

        map.insert(
            "paraguay",
            HashMap::from([("south", 940), ("north", 1100), ("east", 1200),]),
        );

        map.insert("uruguay", HashMap::from([("west", 200), ("north", 940),]));

        map.insert(
            "bolivia",
            HashMap::from([
                ("south", 2600),
                ("north", 1550),
                ("east", 2400),
                ("west", 1100),
            ]),
        );

        map.insert(
            "peru",
            HashMap::from([
                ("east", 1550),
                ("south", 2400),
                ("north", 1180),
                ("west", 2800),
            ]),
        );

        map.insert(
            "brazil",
            HashMap::from([
                ("north", 3300),
                ("east", 3000),
                ("south", 940),
                ("west", 1550),
                ("northeast", 2500),
            ]),
        );

        map.insert(
            "venezuela",
            HashMap::from([("east", 1100), ("south", 3100), ("north", 1300),]),
        );

        map.insert(
            "colombia",
            HashMap::from([("west", 800), ("north", 3100), ("east", 1550), ("south", 1180),]),
        );

        map.insert("ecuador", HashMap::from([("south", 1180), ("east", 800),]));

        map.insert(
            "mexico",
            HashMap::from([("south", 1100), ("east", 2400), ("north", 700),]),
        );

        map.insert(
            "guatemala",
            HashMap::from([("north", 1100), ("east", 240), ("south", 420),]),
        );

        map.insert(
            "honduras",
            HashMap::from([("east", 350), ("south", 230), ("north", 420),]),
        );

        map.insert("costa_rica", HashMap::from([("north", 280)]));

        map.insert(
            "nicaragua",
            HashMap::from([("west", 350), ("south", 280),]),
        );

        map.insert("panama", HashMap::from([("west", 1550), ("north", 700),]));

        map.insert("united_states", HashMap::from([("south", 3100)]));

        map.insert("el_salvador", HashMap::from([("north", 230)]));

        map.insert(
            "nicaragua",
            HashMap::from([("west", 350), ("south", 280),]),
        );

        map.insert("belize", HashMap::from([("west", 240)]));

        map.insert(
            "guyana",
            HashMap::from([("south", 3300), ("west", 370),]),
        );

        map.insert(
            "suriname",
            HashMap::from([("east", 370), ("south", 3300),]),
        );

        map
    };
}

lazy_static! {
    static ref EXPLORED: Mutex<HashSet<&'static str>> = Mutex::new(HashSet::from(["argentina"]));
}
lazy_static! {
    static ref STATES: Mutex<HashSet<&'static str>> = Mutex::new(HashSet::from(["argentina"]));
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

fn get_score<'a>(node: &Box<Node>,action: &'a str) -> u16{

    if !&SCORES.contains_key(node.state) {
        panic!("State not found");
    }
    let state_map = SCORES.get(node.state).unwrap();
    match state_map.get(action) {
        Some(result) => *result as u16,
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


const INITIAL_STATE: &str = "argentina";

fn get_route(node: Box<Node>, objective: &'static str,mut limit: i32,mut t: u8) {
    if node.state == objective {
        println!("{:?}", node.route);
        println!("we reached the objective");
        std::process::exit(0);
    } else {

    

    if limit <= 0 {
        let root_node = Node {
        state: INITIAL_STATE,
        father: None,
        route: vec![],
    };
         let actions = TRAVELS
            .get(&root_node.state)
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
    for action in actions {
        let state = resolve(&root_node.state, action);
        if  !EXPLORED.lock().unwrap().contains(state) {
            get_route(Box::new(root_node.expand(action, state)), objective, 4500,t);
        }
    }

    limit = (t as i32)*4500;
    t = t + 1;
    FRONTIER.lock().unwrap().clear();
    EXPLORED.lock().unwrap().clear();
    
    get_route(Box::new(root_node), objective, limit,t);
        }
    }

        println!("{} {}", &node.state, limit);
        let actions = TRAVELS
            .get(&node.state)
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        println!("{:?}", &actions);
        
        for action in actions {
            let state = resolve(&node.state, action);
            if !EXPLORED.lock().unwrap().contains(&state) {
                FRONTIER.lock().unwrap().push(node.expand(action, state));
                EXPLORED.lock().unwrap().insert(state);
            }
        }
     
        let next = FRONTIER.lock().unwrap().pop();
        
        let next_clone = &next.clone().unwrap();
        let father = &next_clone.father.clone().unwrap();

         let actions = TRAVELS
            .get(father.state)
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

         for action in actions {
             if resolve(father.state, action) == next_clone.state{

                limit = limit - (get_score(father, action) as  i32);
             }
         }

        dbg!(limit);
        get_route(Box::new(next.unwrap()), objective, limit,t)

       

    }

fn main() {
    let root_node = Node {
        state: INITIAL_STATE,
        father: None,
        route: vec![],
    };

    let objective: &str = "united_states";
    get_route(Box::new(root_node), objective, 4500, 1);
}
