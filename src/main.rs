use std::collections::HashMap;

fn main() {
    let total_cost = 67f64;
    let mut ar = HashMap::new();

    // TODO: using web front-end to input name and kg
    ar.insert("Gong", 200.);
    ar.insert("Stephen", 140.);
    ar.insert("Sandy", 100.);
    ar.insert("Mei", 100.);

    let total_weight = sum(&ar);
    for (name, &kg) in ar.iter() {
        println!("{}\t:\t{}", &name, (total_cost / total_weight * kg).floor());
    }
}

fn sum(arr: &HashMap<&str, f64>) -> f64 {
    let mut s = 0f64;
    for (_, &kg) in arr.iter() {
        s += kg;
    }
    return s;
}
