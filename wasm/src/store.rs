use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{JsString};
use crate::opfs::{read, read_in_worker, write_in_worker, noop_write};
use crate::local_storage::{read as fastread, write as fastwrite};
use crate::day::{Day};
use crate::provider::{Provider};

#[wasm_bindgen(getter_with_clone)]
pub struct FirstRecord(pub bool, pub Day);

#[wasm_bindgen(getter_with_clone)]
pub struct Tag(pub JsString, pub f32);

#[wasm_bindgen]
pub struct Stats { pub last_date: i32 }

#[wasm_bindgen]
pub enum Sort { Asc, Desc }

#[wasm_bindgen]
pub struct Store {}

#[wasm_bindgen]
impl Store {

    async fn all_with<F: FnMut(&Day) -> bool>(ns: JsString, f: F) -> Vec<Day> {
        let mut result = Provider{read, write: noop_write}.all(ns).await;
        result.retain(f); 
        result
    }    

    async fn all(ns: JsString) -> Vec<Day> {
        Self::all_with(ns, |x| x.date >= 0).await
    }    

    async fn all_fast(ns: JsString) -> Vec<Day> {
        Self::all_with_fast(ns, |x| x.date >= 0).await
    }    

    async fn all_with_fast<F: FnMut(&Day) -> bool>(ns: JsString, f: F) -> Vec<Day> {
        let mut result = Provider{read: fastread, write: fastwrite}.all(ns).await;
        result.retain(f); 
        result
    }

    async fn all_with_in_worker<F: FnMut(&Day) -> bool>(ns: JsString, f: F) -> Vec<Day> {
        let mut result = Provider{
            read: read_in_worker,
            write: write_in_worker,
        }.all(ns).await;
        result.retain(f); 
        result
    }    


    fn sort(mut days: Vec<Day>, ordering: Sort) -> Vec<Day> {
        days.sort_by(match ordering {
            Sort::Asc => |x: &Day, y: &Day| x.date.cmp(&y.date),
            Sort::Desc => |x: &Day, y: &Day| y.date.cmp(&x.date),
        });
        days
    }

    // ui -- create new record
    pub async fn append(ns: &JsString, id: &JsString) -> Result<(), JsString> {
        Provider{
            read: read_in_worker, 
            write: write_in_worker,
        
        }.append(ns.clone(), id.clone()).await 
    }

    // ui -- create new record in fast memory
    pub async fn append_fast(ns: &JsString, id: &JsString) -> Result<(), JsString> {
        Provider{read: fastread, write: fastwrite}.append(ns.clone(), id.clone()).await 
    }

    // ui -- prepare for rendering
    pub fn transform(days: Vec<Day>) -> Vec<FirstRecord> {
        days.into_iter().scan(-1, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(FirstRecord(is_next, x))
        }).collect()
    }

    // ui -- data for rendering
    pub async fn select(ns: &JsString, ordering: Sort) -> Vec<FirstRecord> {
        let days = Self::sort(Self::all(ns.clone()).await, ordering);
        Self::transform(days) 
    }    

    // ui -- data for (first fast) rendering
    pub async fn select_fast(ns: &JsString, ordering: Sort) -> Vec<FirstRecord> {
        let days = Self::sort(Self::all_fast(ns.clone()).await, ordering);
        Self::transform(days) 
    }

    // ui -- every month records
    pub async fn regular(ns: &JsString) -> Vec<Day> {
        Self::all_with(ns.clone(), |x| x.date == 0).await
    }

    async fn regular_in_worker(ns: &JsString) -> Vec<Day> {
        Self::all_with_in_worker(ns.clone(), |x| x.date == 0).await
    }

    // ui -- copy every month records
    pub async fn repeat_regular(ns: &JsString, prev_ns: &JsString) -> Vec<Day> {
        let mut result = vec![];
        for x in Self::regular_in_worker(prev_ns).await {
            result.push(
                Provider{
                    read: read_in_worker, 
                    write: write_in_worker
                }.copy(ns.clone(), x).await
            );
        }
        result
    }

    // ui -- handy defaults values 
    pub async fn stats(ns: &JsString) -> Option<Stats> {
        let mut days = Store::all_with(ns.clone(), |x| x.date > 0).await;
        days.sort_by_key(|x| core::cmp::Reverse(x.date));
        Some(Stats { last_date: days.first()?.date })
    }

    // ui -- monthly summary 
    pub async fn sum(ns: &JsString) -> f32 {
        let days = Store::all(ns.clone()).await;
        days.into_iter().map(|x| x.price).sum::<f32>()
    }

    // ui -- stats page
    pub async fn group_by(ns: &JsString) -> Vec<Tag> {
        let days = Store::all_with(ns.clone(), |x| x.date > 0).await;
        let mut map = BTreeMap::new();
        for day in days.into_iter() {
            map.entry(day.tag.as_string())
                .and_modify(|e| *e += day.price)
                .or_insert(day.price);
        }
        let mut v: Vec<_> = map.into_iter().filter_map(|(k,v)| Some(Tag(k?.into(),v))).collect();
        v.sort_by(|x,y| y.1.partial_cmp(&x.1).unwrap_or(core::cmp::Ordering::Less));
        return v;
    }

    // ui -- list of tags (e.g. in slider) 
    pub async fn tags(ns: &JsString) -> Vec<JsString> {
        Self::all(ns.clone()).await.into_iter().map(|x| x.tag).collect()
    }
}


#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
}
