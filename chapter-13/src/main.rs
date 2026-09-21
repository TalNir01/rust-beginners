use std::collections::HashMap;

struct DNSCache {
    cache: HashMap<String, String>,
}

impl DNSCache {
    fn get_ip(&self, hostname: &str) -> Option<&String> {
        self.cache.get(hostname)
    }
}

fn main() {
    {
        println!("Lifetimes Example #1");
        let cache = DNSCache {
            cache: Default::default(),
        };
        dbg!(cache.get_ip("example.com")); // None

        let cache2 = DNSCache {
            cache: vec![(String::from("example2.com"), String::from("127.0.0.1"))]
                .into_iter()
                .collect(),
        };
        dbg!(cache2.get_ip("example2.com")); // 127.0.0.1
    }

    {
        println!("Example #2");
        // let addr = {
        //     let cache = DNSCache {
        //         cache: vec![(String::from("example2.com"), String::from("127.0.0.1"))]
        //             .into_iter()
        //             .collect(),
        //     };
        //     cache.get_ip("example.com")
        // };
        // dbg!(addr);
        // todo!();
    }

    {
        println!("Example #3");
        let cache: HashMap<String, String> =
            vec![(String::from("example2.com"), String::from("127.0.0.1"))]
                .into_iter()
                .collect();
        let addr = cache
            .get("example2.com")
            .unwrap_or_else(|| &String::from("default"));
        dbg!(addr);
    }
}
