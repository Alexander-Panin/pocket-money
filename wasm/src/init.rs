use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::store::{Day, Store};

fn local_storage() -> web_sys::Storage 
{ window().unwrap().local_storage().unwrap().unwrap() }

#[wasm_bindgen(start)]
fn start() {
    return;
    let _ = local_storage().clear();
    let ns = "2025:august";
    let prices = [234.23, 42.0, 4.0, 45.6, 1.0, 2.0, 4.5];
    let tags = ["амазон", "рестораны", "продукты"];
    for x in 1..100 {
        let d = Day {
            date: x as i32 % 29 + 1,
            price: prices[x % prices.len()],
            tag: tags[x % tags.len()].to_string(),
            comment: "".to_owned(),
            ..Day::new()
        };
        d.save();
        Store::append(ns, &d);
    }
}













