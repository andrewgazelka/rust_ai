use crate::search::Action::To;
use crate::search::RomaniaState::{Oradea, Zerind, Arad, Timisoara, Lugoj, Mehadia, Drobeta, Craiova, RimnicuVilcea, Sibiu, Fagarus, Pitesti, Bucharest, Giurgiu, Urzieni, Hirsova, Eforie, Vaslui, Iasi, Neamt};
use rand::{Rng};

#[derive(Debug)]
pub struct Transition<T> {
    to: T,
    step_cost: i32
}

pub trait SearchProblem<A,B> {
    fn initial_state(&self) -> &A; // 1
    fn actions(&self, state: &A) -> Vec<B>; // 2
    fn transition_model(&self, state: &A, action: &B) -> Transition<A>; // 3, 4
    fn is_goal(&self, state: &A) -> bool; // 4
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RomaniaState {
    Oradea,
    Zerind,
    Arad,
    Timisoara,
    Lugoj,
    Mehadia,
    Drobeta,
    Craiova,
    RimnicuVilcea,
    Sibiu,
    Fagarus,
    Pitesti,
    Bucharest,
    Giurgiu,
    Urzieni,
    Hirsova,
    Eforie,
    Vaslui,
    Iasi,
    Neamt
}

pub enum Action {
    To(RomaniaState)
}

pub struct Connection(RomaniaState, RomaniaState, i32);

pub struct RomaniaSearchProblem {
    connections: Vec<Connection>,
    goal: RomaniaState,
    initial_state: RomaniaState,
}

impl RomaniaSearchProblem {

    pub fn init() -> RomaniaSearchProblem {
        let connections = vec!(
            Connection(Oradea, Zerind, 71),
            Connection(Zerind, Arad, 75),
            Connection(Arad, Timisoara, 118),
            Connection(Timisoara, Lugoj, 70),
            Connection(Lugoj, Mehadia, 70),
            Connection(Mehadia, Drobeta, 75),
            Connection(Drobeta, Craiova, 120),
            Connection(RimnicuVilcea, Sibiu, 80),
            Connection(Sibiu, Oradea, 151),
            Connection(Sibiu, Arad, 150),
            Connection(Sibiu, Fagarus, 99),
            Connection(RimnicuVilcea, Pitesti, 97),
            Connection(Craiova, Pitesti, 138),
            Connection(Pitesti, Bucharest, 101),
            Connection(Bucharest, Giurgiu, 90),
            Connection(Bucharest, Urzieni, 85),
            Connection(Urzieni, Hirsova, 98),
            Connection(Hirsova, Eforie, 86),
            Connection(Urzieni, Vaslui, 142),
            Connection(Vaslui, Iasi, 92),
            Connection(Iasi, Neamt, 87)
        );

        return RomaniaSearchProblem {
            connections,
            goal: RomaniaState::Bucharest,
            initial_state: RomaniaState::Arad
        }
    }

    // TODO: move needed?
    fn find_connections(&self, state: RomaniaState) -> impl Iterator<Item=&Connection> {
        self.connections.iter().filter(move |x| x.0 == state || x.1 == state)
    }

    fn find_connection(&self, from: RomaniaState, to: RomaniaState) -> Option<&Connection> {
        self.connections.iter().find(move |x| {
            (x.0 == from && x.1 == to) || (x.1 == from && x.0 == to)
        })
    }
}

impl SearchProblem<RomaniaState, Action> for RomaniaSearchProblem {
    fn initial_state(&self) -> &RomaniaState {
        return &self.initial_state;
    }

    fn actions(&self, state: &RomaniaState) -> Vec<Action> {
        self.find_connections(state.clone()).map(|x| {
           if &x.0 == state{
               Action::To(x.1.clone())
           } else {
               Action::To(x.0.clone())
           }
        }).collect()
    }

    fn transition_model(&self, state: &RomaniaState, action: &Action) -> Transition<RomaniaState> {
        match action {
            To(to) => Transition {
                to: to.clone(),
                step_cost: self.find_connection(state.clone(), to.clone()).unwrap().2
            }
        }
    }

    fn is_goal(&self, state: &RomaniaState) -> bool {
        &self.goal == state
    }
}

#[derive(Debug)]
pub struct SearchSolver {
    path: Vec<RomaniaState>,
    cost: i32
}

impl SearchSolver {
    pub fn new() -> SearchSolver {
        SearchSolver {
            path: vec!(),
            cost: 0
        }
    }
    pub fn solve(&mut self, search_problem: &dyn SearchProblem<RomaniaState, Action>){
        let mut state = search_problem.initial_state().clone();
        self.path.push(state.clone()); // initial state
        let mut random = rand::thread_rng();
        loop {
            let actions = search_problem.actions(&state);
            let index = random.gen_range(0,actions.len());
            let chosen_action = &actions[index];
            let Transition {to , step_cost} = search_problem.transition_model(&state, chosen_action);
            self.cost+=step_cost;
            self.path.push(to.clone());
            if search_problem.is_goal(&to) {
                break;
            }
            state = to;
        }
        println!("solved {:?}", self);
    }
}
