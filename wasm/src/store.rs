use wasm_bindgen::prelude::*;
use uuid::Uuid;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn getItem(key: JsValue) -> JsValue;  

    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn setItem(key: JsValue, item: JsValue);
}

fn set_item(id: &JsValue, key: JsValue, item: JsValue) { setItem(id + key, item); }
fn get_item(id: &JsValue, key: JsValue) -> Option<JsValue> { 
    let x = getItem(id + key);
    if x.is_null() { None } else { Some(x) }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct Day {
    pub date: i32,
    pub price: u32,
    pub tag: JsValue,
    pub comment: JsValue,
    pub id: JsValue,
}

#[wasm_bindgen]
impl Day {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { 
            id: Uuid::new_v4().to_string().into(), 
            ..Self::default() 
        }
    }

    pub fn new_with_date(date: i32) -> Self {
        Self { date, ..Self::new() }
    }

    pub fn save(&self) {
        let id = &self.id;
        set_item(id, "price".into(), self.price.into()); 
        set_item(id, "date".into(), self.date.into()); 
        set_item(id, "tag".into(), self.tag.clone());
        set_item(id, "comment".into(), self.comment.clone());
    }

    pub fn fetch(id: &JsValue) -> Option<Self> {
        Some(Day {
            price: get_item(id, "price".into())?.as_string()?.parse().ok()?,
            date: get_item(id, "date".into())?.as_string()?.parse().ok()?,
            tag: get_item(id, "tag".into())?,
            comment: get_item(id, "comment".into())?,
            id: id.clone(),
        })
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Row(pub bool, pub Day);

#[wasm_bindgen]
pub struct Stats { pub last_date: i32 }

#[wasm_bindgen]
pub struct Store {
    root: Option<JsValue> 
}

fn store(ns: &JsValue) -> Store {
    Store { root: get_item(ns, "root".into()) }
}

impl Iterator for Store {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        let root = &self.root.take()?;
        let tmp = Day::fetch(root);
        self.root = get_item(root, "next".into());
        tmp
    }
}

#[wasm_bindgen]
impl Store {

    pub fn all(ns: &JsValue) -> Option<Vec<Day>> {
        Some(store(ns).filter(|d| d.date >= 0).collect())
    }

    pub fn append(ns: &JsValue, day: Day) {
        get_item(ns, "root".into())
            .map(|root| set_item(&day.id, "next".into(), root));
        set_item(ns, "root".into(), day.id);
    }

    pub fn tags(ns: &JsValue) -> Option<Vec<JsValue>> {
        Some(Self::all(ns)?.into_iter().map(|x| x.tag).collect())
    }

    pub fn select(ns: &JsValue) -> Option<Vec<Row>> {
        let mut days = Self::all(ns)?;
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        Some(days.into_iter().scan(0, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(Row(is_next, x))
        }).collect())
    }

    pub fn stats(ns: &JsValue) -> Option<Stats> {
        let xs = Store::select(ns)?;
        let Row(_, x) = xs.first()?;
        Some(Stats { last_date: x.date })
    }
}


