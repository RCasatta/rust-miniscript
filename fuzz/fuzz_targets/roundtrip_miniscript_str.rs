extern crate miniscript;
extern crate regex;

use std::str::FromStr;

use miniscript::{re, DummyKey, Miniscript};

fn do_test(data: &[u8], re: &re::Regexes) {
    let s = String::from_utf8_lossy(data);
    if let Ok(desc) = Miniscript::<DummyKey>::from_str(&s) {
        let output = desc.to_string();

        let normalize_aliases = re.multi_wrap_pk_re.replace_all(&s, "$1:pk(");
        let normalize_aliases = re
            .multi_wrap_pkh_re
            .replace_all(&normalize_aliases, "$1:pkh(");
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
    let regexes = re::compile();
    afl::read_stdio_bytes(|data| {
        do_test(&data, &regexes);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use]
extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    let regexes = re::compile();
    loop {
        fuzz!(|data| {
            do_test(data, &regexes);
        });
    }
}
