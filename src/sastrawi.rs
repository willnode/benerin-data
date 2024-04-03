use fancy_regex::{Captures, Regex};

pub fn get_prefix_precedence_matches() -> Vec<Regex> {
    let regex_rules = vec![r"^be(.*)lah$", r"^be(.*)an$", r"i$", r"is$"];

    regex_rules
        .iter()
        .map(|x| Regex::new(x).expect("error compile"))
        .collect()
}

pub struct Ds {
    pub debugr: String,
    pub regex: Regex,
    pub mutation: Box<dyn Fn(Captures<'_>) -> String>,
}

impl Ds {
    pub fn new(regex: &str, mutation: Box<dyn Fn(Captures<'_>) -> String>) -> Ds {
        Ds {
            debugr: regex.to_owned(),
            regex: Regex::new(regex).expect("error compile"),
            mutation,
        }
    }
}

pub fn get_suffix_matches() -> Vec<Vec<Ds>> {
    vec![
        vec![
            // Remove Inflectional particle.
            Ds::new(r"^(.*?)-?(lah|kah|tah|pun)$", Box::new(|x| x[1].to_owned())),
        ],
        vec![
            // Remove Inflectional Possessive Pronoun.
            Ds::new(r"^(.*?)-?(ku|mu|nya)$", Box::new(|x| x[1].to_owned())),
        ],
        vec![
            // Remove derivational suffix.
            Ds::new(
                r"^(.*?)(is|isme|isasi|i|kan|an)$",
                Box::new(|x| x[1].to_owned()),
            ),
        ],
    ]
}

pub fn get_prefix_matches() -> Vec<Vec<Ds>> {
    vec![
        vec![
            // Remove Plain Prefix
            Ds::new(r"^(di|ke|se)(.*)$", Box::new(|x| x[2].to_owned())),
        ],
        vec![
            // Rule 1a : berV -> ber-V
            Ds::new(r"^ber([aiueo].*)$", Box::new(|x| x[1].to_owned())),
            // Rule 1b : berV -> be-rV
            Ds::new(r"^ber([aiueo].*)$", Box::new(|x| "r".to_owned() + &x[1])),
        ],
        vec![
            // Rule 2 : berCAP -> ber-CAP where C != 'r' AND P != 'er'
            Ds::new(
                r"^ber([bcdfghjklmnpqrstvwxyz])([a-z])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r" || x[3].starts_with("er") {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2] + &x[3]
                    }
                }),
            ),
        ],
        vec![
            // Rule 2 : berCAP -> ber-CAP where C != 'r' AND P != 'er'
            Ds::new(
                r"^ber([bcdfghjklmnpqrstvwxyz])([a-z])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r" || x[3].starts_with("er") {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2] + &x[3]
                    }
                }),
            ),
        ],
        vec![
            // Rule 3: berCAerV -> ber-CAerV where C != 'r'
            Ds::new(
                r"^ber([bcdfghjklmnpqrstvwxyz])([a-z])er([aiueo].*)$",
                Box::new(|x| {
                    // pani!("{} {} {}", &x[1], &x[2], &x[3]);
                    if &x[1] == "r" {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2] + "er" + &x[3]
                    }
                }),
            ),
        ],
        vec![
            // Rule 4: belajar -> bel-ajar
            Ds::new(r"^belajar$", Box::new(|_| "ajar".to_owned())),
        ],
        vec![
            // Rule 5: beC1erC2 -> be-C1erC2 where C1 != 'r'
            Ds::new(
                r"^be([bcdfghjklmnpqstvwxyz])(er[bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 6a: terV -> ter-V
            Ds::new(r"^ter([aiueo].*)$", Box::new(|x| x[1].to_owned())),
            // Rule 6b: terV -> te-rV
            Ds::new(r"^ter([aiueo].*)$", Box::new(|x| "r".to_owned() + &x[1])),
        ],
        vec![
            // Rule 7: terCerv -> ter-CerV where C != 'r'
            Ds::new(
                r"^ter([bcdfghjklmnpqrstvwxyz])er([aiueo].*)$",
                Box::new(|x| {
                    if &x[1] == "r" {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + "er" + &x[2]
                    }
                }),
            ),
        ],
        vec![
            // Rule 8: terCP -> ter-CP where C != 'r' and P != 'er'
            Ds::new(
                r"^ter([bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r" || x[2].starts_with("er") {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2]
                    }
                }),
            ),
        ],
        vec![
            // Rule 9: te-C1erC2 -> te-C1erC2 where C1 != 'r'
            Ds::new(
                r"^te([bcdfghjklmnpqrstvwxyz])er([bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r" {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + "er" + &x[2] + &x[3]
                    }
                }),
            ),
        ],
        vec![
            // Rule 10: me{l|r|w|y}V -> me-{l|r|w|y}V
            Ds::new(
                r"^me([lrwy])([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 11: mem{b|f|v} -> mem-{b|f|v}
            Ds::new(r"^mem([bfv])(.*)$", Box::new(|x| x[1].to_owned() + &x[2])),
        ],
        vec![
            // Rule 12: mempe{r|l} -> mem-pe{r|l}
            Ds::new(r"^mempe(.*)$", Box::new(|x| "pe".to_owned() + &x[1])),
        ],
        vec![
            // Rule 13a: mem{rV|V} -> me-m{rV|V}
            Ds::new(
                r"^mem([aiueo])(.*)$",
                Box::new(|x| "m".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 13b: mem{rV|V} -> me-p{rV|V}
            Ds::new(
                r"^mem([aiueo])(.*)$",
                Box::new(|x| "p".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 14: men{c|d|j|s|t|z} -> men-{c|d|j|s|t|z}
            Ds::new(
                r"^men([cdjstz])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2]),
            ),
        ],
        vec![
            // Rule 15a: men{V} -> me-n{V}
            Ds::new(
                r"^men([aiueo])(.*)$",
                Box::new(|x| "n".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 15b: men{V} -> me-t{V}
            Ds::new(
                r"^men([aiueo])(.*)$",
                Box::new(|x| "t".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 16: meng{g|h|q|k} -> meng-{g|h|q|k}
            Ds::new(r"^meng([ghqk])(.*)$", Box::new(|x| x[1].to_owned() + &x[2])),
        ],
        vec![
            // Rule 17a: mengV -> meng-V
            Ds::new(
                r"^meng([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2]),
            ),
            // Rule 17b: mengV -> meng-kV
            Ds::new(
                r"^meng([aiueo])(.*)$",
                Box::new(|x| "k".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 17c: mengV -> mengV- where V = 'e'
            Ds::new(r"^menge(.*)$", Box::new(|x| x[1].to_owned())),
            // Rule 17d: mengV -> me-ngV
            Ds::new(
                r"^meng([aiueo])(.*)$",
                Box::new(|x| "ng".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 18a: menyV -> me-nyV to stem menyala -> nyala
            Ds::new(
                r"^meny([aiueo])(.*)$",
                Box::new(|x| "ny".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 18b: menyV -> meny-sV
            Ds::new(
                r"^meny([aiueo])(.*)$",
                Box::new(|x| "s".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 19: mempV -> mem-pV where V != 'e'
            Ds::new(
                r"^memp([abcdfghijklmopqrstuvwxyz])(.*)$",
                Box::new(|x| "p".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 20: pe{w|y}V -> pe-{w|y}V
            Ds::new(
                r"^pe([wy])([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 21a: perV -> per-V
            Ds::new(r"^per([aiueo])(.*)$", Box::new(|x| x[1].to_owned() + &x[2])),
            // Rule 21b: perV -> pe-rV
            Ds::new(
                r"^pe(r[aiueo])(.*)$",
                Box::new(|x| "r".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 23 : perCAP -> per-CAP where C != 'r' AND P != 'er'
            Ds::new(
                r"^per([bcdfghjklmnpqrstvwxyz])([a-z])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r" || x[3].starts_with("er") {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2] + &x[3]
                    }
                }),
            ),
        ],
        vec![
            // Rule 24 : perCAerV -> per-CAerV where C != 'r'
            Ds::new(
                r"^per([bcdfghjklmnpqrstvwxyz])([a-z])er([aiueo])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r" {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2] + "er" + &x[3] + &x[4]
                    }
                }),
            ),
        ],
        vec![
            // Rule 25 : pem{b|f|v} -> pem-{b|f|v}
            Ds::new(r"^pem([bfv])(.*)$", Box::new(|x| x[1].to_owned() + &x[2])),
        ],
        vec![
            // Rule 26a : pem{rV|V} -> pe-m{rV|V}
            Ds::new(
                r"^pem([aiueo])(.*)$",
                Box::new(|x| "m".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 26b : pem{rV|V} -> pe-p{rV|V}
            Ds::new(
                r"^pem([aiueo])(.*)$",
                Box::new(|x| "p".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 27 modified by Prasasto Adi : pen{c|d|j|s|t|z} -> pen-{c|d|j|s|t|z}
            Ds::new(
                r"^pen([cdjstz])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2]),
            ),
        ],
        vec![
            // Rule 28a : pen{V} -> pe-n{V}
            Ds::new(
                r"^pen([aiueo])(.*)$",
                Box::new(|x| "n".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 28b : pen{V} -> pe-t{V}
            Ds::new(
                r"^pen([aiueo])(.*)$",
                Box::new(|x| "t".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 29 modified by ECS : pengC -> peng-C
            Ds::new(
                r"^peng([bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2]),
            ),
        ],
        vec![
            // Rule 30a : pengV -> peng-V
            Ds::new(
                r"^peng([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2]),
            ),
            // Rule 30b : pengV -> peng-kV
            Ds::new(
                r"^peng([aiueo])(.*)$",
                Box::new(|x| "k".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 30c : peng-V -> pengV- where V = 'e'
            Ds::new(r"^penge(.*)$", Box::new(|x| x[1].to_owned())),
        ],
        vec![
            // Rule 31a : penyV -> pe-nyV
            Ds::new(
                r"^peny([aiueo])(.*)$",
                Box::new(|x| "ny".to_owned() + &x[1] + &x[2]),
            ),
            // Rule 31b : penyV -> peny-sV
            Ds::new(
                r"^peny([aiueo])(.*)$",
                Box::new(|x| "s".to_owned() + &x[1] + &x[2]),
            ),
        ],
        vec![
            // Rule 32 : pelV -> pe-lV except pelajar -> ajar
            Ds::new(
                r"^pe(l[aiueo])(.*)$",
                Box::new(|x| {
                    if &x[1] == "l" && &x[2] == "ajar" {
                        "ajar".to_owned()
                    } else {
                        x[1].to_owned() + &x[2]
                    }
                }),
            ),
        ],
        vec![
            // Rule 34 : peCP -> pe-CP where C != {r|w|y|l|m|n} and P != 'er'
            Ds::new(
                r"^pe([bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| {
                    if &x[1] == "r"
                        || &x[1] == "w"
                        || &x[1] == "y"
                        || &x[1] == "l"
                        || &x[1] == "m"
                        || &x[1] == "n"
                        || x[2].starts_with("er")
                    {
                        "".to_owned()
                    } else {
                        x[1].to_owned() + &x[2]
                    }
                }),
            ),
        ],
        vec![
            // Rule 35 : terC1erC2 -> ter-C1erC2 where C1 != {r}
            Ds::new(
                r"^ter([bcdfghjkpqstvxz])(er[bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 36 : peC1erC2 -> pe-C1erC2 where C1 != {r|w|y|l|m|n}
            Ds::new(
                r"^pe([bcdfghjkpqstvxz])(er[bcdfghjklmnpqrstvwxyz])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 37a : CerV -> CerV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])(er[aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
            // Rule 37b : CerV -> CV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])er([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 38a : CelV -> CelV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])(el[aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
            // Rule 38b : CelV -> CV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])el([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 39a : CemV -> CemV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])(em[aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
            // Rule 39b : CemV -> CV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])em([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 40a : CinV -> CinV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])(in[aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
            // Rule 40b : CinV -> CV
            Ds::new(
                r"^([bcdfghjklmnpqrstvwxyz])in([aiueo])(.*)$",
                Box::new(|x| x[1].to_owned() + &x[2] + &x[3]),
            ),
        ],
        vec![
            // Rule 41 : kuA -> ku-A
            Ds::new(r"^ku(.*)$", Box::new(|x| x[1].to_owned())),
        ],
        vec![
            // Rule 42 : kauA -> kau-A
            Ds::new(r"^kau(.*)$", Box::new(|x| x[1].to_owned())),
        ],
    ]
}
