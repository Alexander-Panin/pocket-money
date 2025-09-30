use wasm_bindgen::prelude::*;
use web_sys::{window};
use uuid::Uuid;

fn local_storage() -> web_sys::Storage 
{ window().unwrap().local_storage().unwrap().unwrap() }

fn set_item(id: &str, key: &str, item: &str) 
{ let _ = local_storage().set_item(&format!("{id}:{key}"), item); }

fn get_item(id: &str, key: &str) -> Option<String> 
{ local_storage().get_item(&format!("{id}:{key}")).unwrap() }

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct Day {
    pub date: i32,
    pub price: f32,
    pub tag: String,
    pub comment: String,
    pub id: String,
}

#[wasm_bindgen]
impl Day {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { id: Uuid::new_v4().to_string(), ..Self::default() }
    }

    pub fn new_with_date(date: i32) -> Self {
        Self { date, ..Self::new() }
    }

    pub fn save(&self) {
        let (id, price, date) = (&self.id, self.price, self.date); 
        set_item(id, "price", &price.to_string());
        set_item(id, "date", &date.to_string());
        set_item(id, "tag", &self.tag);
        set_item(id, "comment", &self.comment);
    }

    pub fn fetch(id: &str) -> Self {
        Day {
            price: get_item(id, "price").and_then(|x| x.parse().ok()).unwrap(),
            date: get_item(id, "date").and_then(|x| x.parse().ok()).unwrap(),
            tag: get_item(id, "tag").unwrap(),
            comment: get_item(id, "comment").unwrap(),
            id: id.to_owned()
        } 
    }
}

#[wasm_bindgen]
pub struct Store {}

#[wasm_bindgen(getter_with_clone)]
pub struct Row(pub bool, pub Day);

#[wasm_bindgen]
pub struct Stats { pub last_date: i32 }

#[wasm_bindgen]
impl Store {

    fn all(ns: &str) -> Option<Vec<Day>> {
        let mut root = get_item(ns, "root")?;
        let mut result = vec![Day::fetch(&root)];
        while let Some(next) = get_item(&root, "next") {
            result.push(Day::fetch(&next));
            root = next;
        }
        result.retain(|x| x.date >= 0);
        Some(result)
    }

    pub fn append(ns: &str, day: &Day) {
        get_item(ns, "root")
            .map(|root| set_item(&day.id, "next", &root));
        set_item(ns, "root", &day.id);
    }

    pub fn tags(ns: &str) -> Option<Vec<String>> {
        let mut tags: Vec<_> = Self::all(ns)?.into_iter().map(|x| x.tag).collect();
        tags.sort();
        tags.dedup();
        Some(tags)
    }

    pub fn select(ns: &str) -> Option<Vec<Row>> {
        let mut days = Self::all(ns)?;
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        Some(days.into_iter().scan(0, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(Row(is_next, x))
        }).collect())
    }

    pub fn stats(ns: &str) -> Option<Stats> {
        let xs = Store::select(ns)?;
        let Row(_, x) = xs.first()?;
        Some(Stats { last_date: x.date })
    }
}


