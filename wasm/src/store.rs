use wasm_bindgen::prelude::*;
use crate::opfs::{read, write};
use crate::linked;
use crate::day::{Day};

#[wasm_bindgen(getter_with_clone)]
pub struct Row(pub bool, pub Day);

#[wasm_bindgen]
pub struct Stats { pub last_date: i32 }

#[wasm_bindgen]
pub enum Sort { Asc, Desc }

#[wasm_bindgen]
pub struct Store {}

async fn store(ns: &JsValue) -> Vec<Day> {
    let mut result = vec![];
    for id in linked::collect_ids(ns, read).await { 
        result.push( Day::fetch(&id).await ); 
    }
    result
}

async fn append<T,F>(ns: JsValue, id: JsValue, read: T) 
    where
        T: Fn(JsValue, JsValue) -> F, 
        F: Future<Output = Result<JsValue, JsValue>> 
{
    if let Ok(root) = read(ns.clone(), "root".into()).await {
        let _ = write(&id, &"next".into(), &root).await;
    }
    let _ = write(&ns, &"root".into(), &id).await;
}

#[wasm_bindgen]
impl Store {

    async fn all(ns: &JsValue) -> Vec<Day> {
        Self::all_with(ns, |x| x.date >= 0).await
    }    

    async fn all_with<F: FnMut(&Day) -> bool>(ns: &JsValue, f: F) -> Vec<Day> {
        let mut v = store(ns).await; v.retain(f); v
    }

    fn sort(mut days: Vec<Day>, ordering: Sort) -> Vec<Day> {
        days.sort_by(match ordering {
            Sort::Asc => |x: &Day, y: &Day| x.date.cmp(&y.date),
            Sort::Desc => |x: &Day, y: &Day| y.date.cmp(&x.date),
        });
        days
    }

    async fn copy(ns: &JsValue, day: Day) -> Day {
        let x = day.r#move(); 
        Self::append(ns, &x.id).await; 
        x.save().await;
        return x;
    }

    // ui -- create new record
    pub async fn append(ns: &JsValue, id: &JsValue) 
    { append(ns.clone(), id.clone(), read).await }

    // ui -- prepare for rendering
    pub fn transform(days: Vec<Day>) -> Vec<Row> {
        days.into_iter().scan(-1, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(Row(is_next, x))
        }).collect()
    }

    // ui -- data for rendering
    pub async fn select(ns: &JsValue, ordering: Sort) -> Vec<Row> {
        let days = Self::sort(Self::all(ns).await, ordering);
        Self::transform(days) 
    }

    // ui -- every month records
    pub async fn regular(ns: &JsValue) -> Vec<Day> {
        Self::all_with(ns, |x| x.date == 0).await
    }

    // ui -- copy every month records
    pub async fn repeat_regular(ns: &JsValue, prev_ns: &JsValue) -> Vec<Day> {
        let mut result = vec![];
        for x in Self::regular(prev_ns).await {
            result.push(Self::copy(ns, x).await); 
        }
        result
    }

    // ui -- handy defaults values 
    pub async fn stats(ns: &JsValue) -> Option<Stats> {
        let mut days = Store::all_with(ns, |x| x.date > 0).await;
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        Some(Stats { last_date: days.first()?.date })
    }

    // ui -- monthly summary 
    pub async fn sum(ns: &JsValue) -> f32 {
        let days = Store::all(ns).await;
        days.into_iter().map(|x| x.price).sum::<f32>().round()
    }

    // ui -- list of tags (e.g. in slider) 
    pub async fn tags(ns: &JsValue) -> Vec<JsValue> {
        Self::all(ns).await.into_iter().map(|x| x.tag).collect()
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
}


