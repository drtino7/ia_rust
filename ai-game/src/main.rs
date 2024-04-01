use std::collections::HashMap;
use rand::Rng;
use std::process::exit;

#[derive(PartialEq, Eq, Debug, PartialOrd )]
struct State(u8);

trait Unwrap{
    fn unwrap(self) -> u8;
}

impl Unwrap for State{
    fn unwrap(self) -> u8 {
        self.0
    }
}

fn main() {
    let rand_: u8 = rand::thread_rng().gen_range(1..100);
    let mut perceptions = init_perceptions();

    let mut state = [0, 2];
    println!("rand numb is: {}",rand_);
    loop{
        let numb = smart_agent(&mut state, &mut perceptions);
        println!("try this numb: {}", numb);
        let result = play_game(numb, rand_).unwrap();
        state = [result, numb];
    }
}

fn play_game(numb: u8, rand_: u8) -> State{

    if numb < rand_{
        State(2)
    }
    else if numb > rand_{
        State(1)
    }
    else if numb == rand_{
        println!("easy win");
        exit(0);
    }
    else{
        State(3)
    }

}
// state[State, numb]
// perception<key, value >

fn smart_agent(state: &mut [u8;2], perceptions: &mut HashMap<u8, u8> ) -> u8{

    perceptions.insert(state[1],state[0]);

        let numb: u8 = {
            if perceptions.len() == 3{
                50
            }
            else{
                if state[0] == 1{
                    let mut var: &u8 = &0;
                    for perception in perceptions.iter(){
                        if perception.1 == &2{
                            if perception.0 < &state[1] && perception.0 > var {
                            var =  perception.0;

                            }
                            else{ continue; }
                        }
                    }
                    (((&state[1] - var) / 2 ) + var) as u8
                }
                else if state[0] == 2{
                    let mut var: &u8 = &100;
                    for perception in perceptions.iter(){
                        if perception.1 == &1{
                            if perception.0 > &state[1] && perception.0 < var {
                            var =  perception.0;
                            }
                            else{ continue; }
                        }
                    }
                    (var + &state[1]) / 2 as u8
                }
                else{
                    panic!()
                }
            }
        };
    
    numb

} 

fn init_perceptions() -> HashMap<u8, u8>{
    let mut perceptions: HashMap<u8, u8> = HashMap::new();
    perceptions.insert(0, 2);
    perceptions.insert(100, 1);
    perceptions
}
