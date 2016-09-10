use std::collections::BTreeMap;

pub fn transform(legacy: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    legacy.iter().flat_map(|(point, letters)| {
        letters.iter().map(move|letter| (letter.to_lowercase(), *point))
    }).collect::<BTreeMap<String, i32>>()
}
