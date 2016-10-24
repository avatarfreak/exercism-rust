use std::collections::HashMap;
pub struct School {
    grades: HashMap<usize, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn grades(&self) -> Vec<usize> {
        let mut grades: Vec<usize> = self.grades.keys().map(|&x| x).collect();
        grades.sort();
        grades
    }

    pub fn add(&mut self, grade: usize, name: &str) {
        let mut entry = self.grades.entry(grade).or_insert(vec![]);
        entry.push(name.to_string());
        entry.sort();
    }

    pub fn grade(&self, grade: usize) -> Option<&Vec<String>> {
        self.grades.get(&grade)
    }
}
