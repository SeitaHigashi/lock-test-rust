use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

mod data;

use data::{Data, Key};

fn main() {
    let map: Arc<Mutex<HashMap<Key, Vec<Data>>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut rng = rand::thread_rng();
    let a: Vec<Key> = (0..50000000)
        .map(|_| Key {
            id: (rng.gen::<u8>() / 4).to_string(),
            obj_type: (rng.gen::<u8>() / 4).to_string(),
            ldn: (rng.gen::<u8>() / 4).to_string(),
        })
        .collect();

    a.into_par_iter().progress().for_each(|n| {
        let map_c = Arc::clone(&map);
        let mut data = map_c.lock().unwrap();
        let list = data.entry(n).or_default();
        let mut rng = rand::thread_rng();
        list.push(Data {
            time: rng.gen::<u16>().to_string(),
            value: rng.gen::<u16>().to_string(),
        });
    });

    let b = map.lock().unwrap();

    b.par_iter()
        .progress_count(b.len() as u64)
        .for_each(|(k, v)| {
            let filepath = k.make_filepath();
            let Ok(mut wtr) = csv::Writer::from_path(filepath.clone()) else {return};
            v.iter().for_each(|d| {
                wtr.serialize(d).expect("serialize failed");
            });
            wtr.flush().expect(&format!("flush failed : {}", filepath));
        });
}
