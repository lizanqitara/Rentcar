use ic_cdk::api::trap;
use ic_cdk::export::candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Car {
    id: u64,
    owner: Principal,
    model: String,
    location: String,
    price_per_day: u64,
    is_available: bool,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Reservation {
    id: u64,
    car_id: u64,
    renter: Principal,
    start_date: String,
    end_date: String,
}

struct RentCar {
    cars: HashMap<u64, Car>,
    reservations: HashMap<u64, Reservation>,
    next_car_id: u64,
    next_reservation_id: u64,
}

impl RentCar {
    fn new() -> Self {
        Self {
            cars: HashMap::new(),
            reservations: HashMap::new(),
            next_car_id: 1,
            next_reservation_id: 1,
        }
    }

    fn add_car(&mut self, owner: Principal, model: String, location: String, price_per_day: u64) -> u64 {
        let car = Car {
            id: self.next_car_id,
            owner,
            model,
            location,
            price_per_day,
            is_available: true,
        };
        self.cars.insert(self.next_car_id, car);
        self.next_car_id += 1;
        self.next_car_id - 1
    }

    fn list_cars(&self) -> Vec<Car> {
        self.cars.values().cloned().collect()
    }

    fn reserve_car(
        &mut self,
        renter: Principal,
        car_id: u64,
        start_date: String,
        end_date: String,
    ) -> Result<u64, String> {
        if let Some(car) = self.cars.get_mut(&car_id) {
            if car.is_available {
                car.is_available = false;
                let reservation = Reservation {
                    id: self.next_reservation_id,
                    car_id,
                    renter,
                    start_date,
                    end_date,
                };
                self.reservations.insert(self.next_reservation_id, reservation);
                self.next_reservation_id += 1;
                Ok(self.next_reservation_id - 1)
            } else {
                Err("Mobil tidak tersedia".to_string())
            }
        } else {
            Err("Mobil tidak ditemukan".to_string())
        }
    }
}

static mut RENTCAR: Option<RentCar> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        RENTCAR = Some(RentCar::new());
    }
}

#[ic_cdk::update]
fn add_car(model: String, location: String, price_per_day: u64) -> u64 {
    let caller = ic_cdk::caller();
    let app = unsafe { RENTCAR.as_mut().unwrap() };
    app.add_car(caller, model, location, price_per_day)
}

#[ic_cdk::query]
fn list_cars() -> Vec<Car> {
    let app = unsafe { RENTCAR.as_ref().unwrap() };
    app.list_cars()
}

#[ic_cdk::update]
fn reserve_car(car_id: u64, start_date: String, end_date: String) -> Result<u64, String> {
    let caller = ic_cdk::caller();
    let app = unsafe { RENTCAR.as_mut().unwrap() };
    app.reserve_car(caller, car_id, start_date, end_date)
}
