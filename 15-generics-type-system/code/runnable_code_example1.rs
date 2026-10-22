#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValue<K, V> {
    fn new(key: K, value: V) -> Self {
        KeyValue { key, value }
    }

    fn unpack(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K> KeyValue<K, f64> {
    fn is_positive_metric(&self) -> bool {
        self.value > 0.0
    }
}

fn find_min<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }

    let mut min = list[0];
    for &item in list.iter().skip(1) {
        if item < min {
            min = item;
        }
    }
    Some(min)
}

fn main() {

    let sensor = KeyValue::new("Temperature", 36.6);
    println!("Sensor Record: {:?}", sensor);
    println!("Is positive metric: {}", sensor.is_positive_metric());

    let user_flag = KeyValue::new(1001, true);
    println!("User Flag: {:?}", user_flag);
    let (uid, flag) = user_flag.unpack();
    println!("Unpacked -> UID: {}, Status: {}", uid, flag);

    let scores = [85, 92, 78, 90];
    let temps = [36.6, 37.2, 35.8, 38.0];

    println!("Min Score: {:?}", find_min(&scores));
    println!("Min Temp : {:?}", find_min(&temps));
}