fn main() {
    println!("Hello, world!");
    tasks::second::execute();
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
        pub fn execute() {
            let input = "welcome to our world";
            let arr = input.split_whitespace();
            let mut output = String::new();
            for c in arr {
                let mut chars = c.chars();
                let tmp = chars.next().unwrap();
                if tmp == 'o' {
                    let hay = format!("{}-hay ", c);
                    output.push_str(&hay);
                } else {
                    let ay = format!("{}-{}ay ", chars.as_str(), tmp);
                    output.push_str(&ay);
                }
            }
            println!("{}", output);
        }
    }
    pub mod third {
        use std::{collections::HashMap, io};

        pub fn execute() {
            println!("welcome to our company CMD interface");
            println!("we have some main commands:");
            println!("'deps' shows all departmens");
            println!("'all' shows all empls with their deps");
            println!("'add -empl_name -dep_name' add new empl to existed dep");
            println!("'exit' close program");

            let deps = vec!["it", "account", "hr"];
            let mut all = HashMap::new();
            all.insert(String::from("alex_nesterov"), "it");

            loop {
                println!("please enter the command...");
                let mut cmd = String::new();

                io::stdin().read_line(&mut cmd).expect("incorrect input");

                let mut input = cmd.split_whitespace();
                let next = input.next();
                match next {
                    Some(val) => {
                        match val{
                            "deps" => println!("{:?}", &deps),
                            "all" => println!("{:?}", all),
                            "add" => {
                                let empl = input.next().unwrap();
                                let tar_dep = input.next().unwrap();

                                for dep in &deps {
                                    if *dep == tar_dep {
                                        all.insert(String::from(empl), tar_dep);
                                        break;
                                    }
                                }
                            },
                            "exit" => break,
                            _ => println!("incorrect input")
                        }
                    },
                    None => println!("incorrect input")
                }
            }
        }
    }
}
