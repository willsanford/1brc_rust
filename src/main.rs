use std::collections::HashMap;
use std::fs::read_to_string;

fn v1() {
    #[derive(Debug)]
    struct Data {
        count: i32,
        min: f32,
        max: f32,
        sum: f32,
    }

    let file = read_to_string("/home/wsanf/code/rs_1brc/data/measurements.txt").unwrap();

    let mut m: HashMap<&str, Data> = HashMap::new();

    for line in file.lines() {
        let (name, val_str) = line.split_once(';').unwrap();
        let val: f32 = val_str.parse::<f32>().unwrap();

        match m.get_mut(name) {
            Some(data) => {
                data.count += 1;
                data.min = data.min.min(val);
                data.max = data.max.max(val);
                data.sum += val;
            }
            None => {
                m.insert(
                    name,
                    Data {
                        count: 1,
                        min: val,
                        max: val,
                        sum: val,
                    },
                );
            }
        }
    }
    for (k, v) in m.iter() {
        println!("{k:?} | {v:?}");
    }
}
fn main() {
    v1();
}
