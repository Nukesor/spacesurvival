use std::collections::HashMap;


pub struct Level {
    pub level: i32,
    pub resources: Vec<(String, i32)>,
}


pub struct Research {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub level: Vec<Level>,
}


lazy_static! {
    pub static ref RESEARCH_LIST: HashMap<String, Research> = {
        let mut m = HashMap::new();
        m.insert(String::from("Rofl"), Research {
            name: String::from("Rofl"),
            dependencies: None,
            level: vec![
                Level {
                    level: 1,
                    resources: vec![(String::from("Iron"), 100)],
                },
            ]
        });
        m.insert(String::from("Lol"), Research {
            name: String::from("Lol"),
            dependencies: Some(vec![String::from("Rofl")]),
            level: vec![
                Level {
                    level: 1,
                    resources: vec![(String::from("Water"), 200)],
                },
            ]
        });
        m
    };
}
