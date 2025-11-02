fn main() {
    println!("Hello, world!");
    tasks::first::execute();
}

mod tasks {
    pub mod first {
        use std::collections::HashMap;

        pub fn execute() {
            let mut v: Vec<i32> = vec![2, 3, 1, 3];
            println!("orig vector {:?}", &v);

            let mut map = HashMap::new();
            let mut sum: f32 = 0.0;
            for i in &v {
                let tmp = *i;
                sum += tmp as f32;

                let c = map.entry(tmp).or_insert(0);
                *c += 1;
            }
            println!("avg is {}", sum / (v.len() as f32));

            v.sort();
            println!("after sort {:?}", &v);
            let median = v.len() / 2 - 1;
            println!("median is {}", &v[median]);

            let mut moda: Option<i32> = None;

            for (k, _) in map {
                match moda {
                    Some(m) => {
                        if m < k {
                            moda = Some(k);
                        }
                    }
                    None => moda = Some(k),
                }
            }
            println!("moda is {:?}", moda);
        }
    }
    pub mod second {
        
    }
}
