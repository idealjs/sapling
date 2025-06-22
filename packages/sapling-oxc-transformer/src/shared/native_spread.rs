use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref NON_SPREAD_NAMESPACES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("class");
        set.insert("style");
        set.insert("use");
        set.insert("prop");
        set.insert("attr");
        set.insert("bool");
        set
    };
}

pub fn can_native_spread(key: &str, check_namespaces: bool) -> bool {
    if check_namespaces && key.contains(':') {
        let namespace = key.split(':').next().unwrap();
        if NON_SPREAD_NAMESPACES.contains(namespace) {
            return false;
        }
    }
    key != "ref"
}
