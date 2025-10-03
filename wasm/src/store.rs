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
fn get_item(id: &JsValue, key: JsValue) -> JsValue { getItem(id + key) }

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default, Debug)]
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
            price: get_item(id, "price".into()).as_string()?.parse().ok()?,
            date: get_item(id, "date".into()).as_string()?.parse().ok()?,
            tag: get_item(id, "tag".into()),
            comment: get_item(id, "comment".into()),
            id: id.clone(),
        })
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

    fn all(ns: &JsValue) -> Option<Vec<Day>> {
        let mut root = get_item(&ns, "root".into());
        let mut result = vec![Day::fetch(&root)?];
        loop {
            let next = get_item(&root, "next".into());
            if next.is_null() { break; }
            result.push(Day::fetch(&next)?);
            root = next;
        }
        result.retain(|x| x.date >= 0);
        Some(result)
    }

    pub fn append(ns: &JsValue, day: Day) {
        let id = day.id;
        let root = get_item(ns, "root".into());
        if !root.is_null() { set_item(&id, "next".into(), root); }
        set_item(ns, "root".into(), id);
    }

    pub fn tags(ns: &JsValue) -> Option<Vec<JsValue>> {
        let tags: Vec<_> = Self::all(ns)?.into_iter().map(|x| x.tag).collect();
        // tags.sort();
        // tags.dedup();
        Some(tags)
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


