use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref TRAVELS: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut map = HashMap::new();
        map.insert("argentina", HashMap::from([
            ("west", "chile"),
            ("east", "uruguay"),
            ("north", "bolivia"),
        ]));
        map.insert("chile", HashMap::from([
            ("east", "argentina"),
            ("north", "peru"),
        ]));
        map.insert("paraguay", HashMap::from([
            ("south", "argentina"),
            ("north", "bolivia"),
            ("east", "uruguay"),
        ]));
        map.insert("uruguay", HashMap::from([
            ("west", "argentina"),
            ("north", "paraguay"),
        ]));
        map.insert("bolivia", HashMap::from([
            ("south", "argentina"),
            ("north", "peru"),
        ]));
        map.insert("peru", HashMap::from([
            ("east", "bolivia"),
            ("south", "chile"),
        ]));
        map
    };
}

fn resolve<'a>(state: &'a str, action: &'a str) -> &'a str {
    if !&TRAVELS.contains_key(state) {
        panic!("State not found");
    }
    let state_map = TRAVELS.get(state).unwrap();
    match state_map.get(action) {
        Some(result) => {
            //println!("{} -> {}", state, result);
            result
        }
        None => panic!("Action not found"),
    }
}

#[derive(Debug,Clone)]
struct Node {
    state: &'static str,
    father: Option<Box<Node>>,
    route: Vec<&'static str>
}

impl Node {
    fn expand(&self, action: &'static str) -> Node {
        Node {
            state: resolve(&self.state, action),
            father: Some(Box::new(self.clone())),
            route: {
                let mut father_route = match &self.father {
                    Some(father) => {
                        self.route.clone()
                    }
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
fn get_route(
    mut node: Box<Node>,
    objective: &'static str,
) {
    if node.state == objective {
        println!("{:?}", node.route);
        println!("we reached the objective");
        std::process::exit(0);
    } else {
        let mut handles: Vec<_> = vec![];
        let actions = TRAVELS.get(&node.state).unwrap().keys().cloned().collect::<Vec<_>>();
        for action in actions {
            //node.route.push(resolve(&node.state, action));
            let son_node = node.expand(&action).clone();

            //println!("84: {:?}", node.route);
            let handle = std::thread::spawn(move|| {

                get_route(Box::new(son_node), objective);


            });
            handles.push(handle);

        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}


fn main() {

    let root_node = Node {
        state: "argentina",
        father: None,
        route: vec!["aleluya"],
    };
    let objective: &str = "peru";
    get_route(Box::new(root_node), objective);

}
