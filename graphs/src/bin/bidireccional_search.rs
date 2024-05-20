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
    static ref EXPLORED_1: Mutex<HashSet<Box<Node>>> = Mutex::new(HashSet::new());
}
lazy_static! {
    static ref EXPLORED_2: Mutex<HashSet<Box<Node>>> = Mutex::new(HashSet::new());
}

lazy_static! {
    static ref FRONTIER_1: Mutex<Vec<Node>> = Mutex::new(vec![]);
}
lazy_static! {
    static ref FRONTIER_2: Mutex<Vec<Node>> = Mutex::new(vec![]);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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


fn get_route(node_1: Box<Node>, node_2: Box<Node>,mut limit: u8,mut t: u8,mut flag: bool) {
    for node_explored_1 in EXPLORED_1.lock().unwrap().iter() { 
        
         for node_explored_2 in EXPLORED_2.lock().unwrap().iter() {
             if node_explored_2.state == node_explored_1.state{
                 println!("{} <-> {}",node_explored_2.state,node_explored_1.state);
                let state_1_route = &node_explored_1.route;
                let state_2_route = &node_2.route;
                dbg!("route is: {} -> {} ",state_1_route, state_2_route);
                std::process::exit(0);
             }
         }
    }   

    if limit == 0 {
        if flag == true{
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
            if  !EXPLORED_1.lock().unwrap().contains(&root_node) {
                get_route(Box::new(root_node.expand(action, root_node.state)),Box::from(node_2.clone()), 20,t,flag);
            }
        }

        let limit = t*20;
        t = t + 1;
        FRONTIER_1.lock().unwrap().clear();
        get_route(Box::new(root_node), Box::from(node_2.clone()), limit,t,flag);
        
    }else{

        let root_node = Node {
            state: "united_states",
            father: None,
            route: vec![],
        };
        let actions = TRAVELS.get(&root_node.state).unwrap().keys().cloned().collect::<Vec<_>>();
        for action in actions {
            if !EXPLORED_2.lock().unwrap().contains(&root_node) {
                get_route(Box::new(root_node.expand(action, root_node.state)),Box::from(node_2.clone()),20,t,flag)
            }
        }
        let limit = t*20;
        t = t + 1;
        FRONTIER_2.lock().unwrap().clear();
        get_route(Box::from(node_2.clone()), Box::new(root_node), limit,t,flag);
    }
        
    }
    
     let actions_1 =   TRAVELS
             .get(&node_1.state)
             .unwrap()
             .keys()
             .cloned()
             .collect::<Vec<_>>();

      let actions_2 =   TRAVELS
             .get(&node_2.state)
             .unwrap()
             .keys()
             .cloned()
             .collect::<Vec<_>>();


      for action_1 in actions_1 {
          
         let state = resolve(&node_1.state, action_1);
          let mut match_ = false;
          for node in EXPLORED_1.lock().unwrap().iter() {
                if node.state == state {
                    match_ = true;
                }
            }
         if match_ == false{
             FRONTIER_1.lock().unwrap().push(node_1.expand(action_1, state));
             EXPLORED_1.lock().unwrap().insert(Box::from(node_1.clone().expand(action_1,state)));
         }
      }
    let front_guard_1 = FRONTIER_1.lock().unwrap();
    drop(front_guard_1);

 let mut i = 0;
      for action_2 in actions_2 {
          let mut match_ = false;
        let state = resolve(&node_2.state, action_2);
            for node in EXPLORED_2.lock().unwrap().iter() {
                if state == node.state {
                    match_ = true;
                }
            }
          if match_ == false{
              i = i +1;
                FRONTIER_2.lock().unwrap().push(node_2.expand(action_2, state));
                 EXPLORED_2.lock().unwrap().insert(Box::from(node_2.clone().expand(action_2,state)));
             }
      }
    let front_guard_2 = FRONTIER_2.lock().unwrap();
    drop(front_guard_2);
    
      if flag == true {
          flag = false;
          let next = FRONTIER_1.lock().unwrap().pop();
          limit = limit - 1;
          get_route(Box::from(next.unwrap()), node_2, limit, t, flag);
      }
      else {
          flag = true;
          let next = FRONTIER_2.lock().unwrap().pop();
          limit = limit - 1;
          get_route(node_1, Box::from(next.unwrap()), limit, t, flag);
      }

}




fn main() {
    let root_node_1 = Node {
        state: INITIAL_STATE,
        father: None,
        route: vec![],
    };

    let objective: &str = "united_states";

    let root_node_2 = Node {
        state: objective,
        father: None,
        route: vec![],
    };
    get_route(Box::new(root_node_1), Box::new(root_node_2) , 10,1,true);
}
