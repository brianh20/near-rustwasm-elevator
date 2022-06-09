use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub elevator: LazyOption<Elevator>,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Elevator {
    pub max_floors: u8,
    pub max_weight: u16,
    pub current_weight: u16,
    pub queue: Vec<(u8, u8)>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let (floors, weight) = get_random_floor_and_weight();
        let elevator = Elevator {
            max_floors: floors,
            max_weight: weight,
            current_weight: 0,
            queue: vec![],
        };
        let log_message = format!("Elevator created {:?}", elevator);
        env::log(log_message.as_bytes());
        Self {
            elevator: LazyOption::new(b"m", Some(&elevator)),
        }
    }

    pub fn get_queue(&mut self) {
        let log_message = format!("Current Queue is {:?}", self.elevator.get().unwrap().queue);
        env::log(log_message.as_bytes());
    }

    pub fn hop_on(&mut self) {
        let mut new_elevator = self.elevator.get().unwrap();

        new_elevator.queue.push(get_random_person(
            //&self.max_floors, &self.max_weight
        ));

        let log_message = format!("Current Queue is {:?}", new_elevator.queue);
        env::log(log_message.as_bytes());
        self.elevator.set(&new_elevator);
    }

    /// Reset to zero.
    pub fn flush(&mut self) {
        let mut elevator = self.elevator.get().unwrap();
        let mut queue: Vec<(u8, u8)> = elevator.queue.clone().into_iter().rev().collect();
        let mut stops = 0;
        let mut journey: Vec<u8> = vec![];

        // while people in queue
        while queue.len() > 0 {
            // load elevator until it is full
            while queue.len() > 0 && elevator.current_weight + queue[0].0 as u16 <= elevator.max_weight {
                let person = queue.pop().unwrap();
                elevator.current_weight += person.0 as u16;
                if !journey.contains(&person.1) {
                    journey.push(person.1);
                }
            }
            // count stops
            stops += journey.len() + 1;
            // flush elevator journey and weight
            journey = Vec::new();
            elevator.current_weight = 0;
        }

        let log_message = format!("Number of stops was {}", stops);
        env::log(log_message.as_bytes());

        let (floors, weight) = get_random_floor_and_weight();
        let empty_elevator = Elevator {
            max_floors: floors,
            max_weight: weight,
            current_weight: 0,
            queue: vec![],
        };
        let log_message = format!("Elevator created {:?}", empty_elevator);
        env::log(log_message.as_bytes());
        self.elevator.set(&empty_elevator);
    }
}

pub fn get_random_floor_and_weight() -> (u8, u16) {
    (10, 100)
}
pub fn get_random_person(//max_floors: &u8, max_weight: &u16
) -> (u8, u8) {
    let weight = 40;
    let floor = 1;
    (weight, floor)
}
