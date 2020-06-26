extern crate miniscript;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::str::FromStr;

use miniscript::DummyKey;
use miniscript::Miniscript;

lazy_static! {
    static ref MULTI_WRAP_PK_RE: Regex = Regex::new("([a-z]+)c:pk_k\\(").unwrap();
    static ref MULTI_WRAP_PKH_RE: Regex = Regex::new("([a-z]+)c:pk_h\\(").unwrap();
}

fn do_test(data: &[u8]) {
    let s = String::from_utf8_lossy(data);
    if let Ok(desc) = Miniscript::<DummyKey>::from_str(&s) {
        let output = desc.to_string();

        let normalize_aliases = MULTI_WRAP_PK_RE.replace_all(&s, "$1:pk(");
        let normalize_aliases = MULTI_WRAP_PKH_RE.replace_all(&normalize_aliases, "$1:pkh(");
        let normalize_aliases = normalize_aliases
            .replace("c:pk_k(", "pk(")
            .replace("c:pk_h(", "pkh(");

        assert_eq!(normalize_aliases.to_lowercase(), output.to_lowercase());
    }
}

#[cfg(feature = "afl")]
extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    afl::read_stdio_bytes(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use]
extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}
