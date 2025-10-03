use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::store::{Day, Store};

fn local_storage() -> web_sys::Storage 
{ window().unwrap().local_storage().unwrap().unwrap() }

#[wasm_bindgen(start)]
fn start() {
    let _ = local_storage().clear();
    let ns = "2025:august".into();
    let prices = [23543, 420, 40, 456, 10, 200, 451];
    let tags = ["амазон", "рестораны", "продукты"];
    for x in 1..2 {
        let d = Day {
            date: x as i32 % 29 + 1,
            price: prices[x % prices.len()],
            tag: tags[x % tags.len()].into(),
            comment: "".into(),
            ..Day::new()
        };
        d.save();
        Store::append(&ns, d);
    }
}













