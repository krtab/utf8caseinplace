use std::collections::HashMap;

use unic_ucd_name::Name;

type Map = HashMap<(usize, usize), Vec<char>>;
fn main() {
    let mut map_lower: Map = HashMap::new();
    let mut map_upper: Map = HashMap::new();
    let mut total = 0;
    for c in '\0'..=char::MAX {
        total += 1;
        let len_orig = c.len_utf8();
        let len_lower: usize = c.to_lowercase().map(|c| c.len_utf8()).sum();
        map_lower.entry((len_orig, len_lower)).or_default().push(c);
        let len_upper: usize = c.to_uppercase().map(|c| c.len_utf8()).sum();
        map_upper.entry((len_orig, len_upper)).or_default().push(c);
    }
    fn print_stats(map: Map, total: usize, to_up: bool) {
        for ((from, to), v) in map {
            print!(
                "{from} bytes -> {to} bytes: {} ({}%)",
                v.len(),
                100 * v.len() / total
            );

            if from != to {

                fn print_char(c: char) {
                    let name = Name::of(c);
                    print!("{c} (U+{:X})", c as u32);
                    if let Some(name) = name {
                        print!(" ({name})");
                    }
                }

                println!(": ");
                for c in v {
                    print!("\t");
                    print_char(c);
                    print!(" -> ");
                    if to_up {
                        for c in c.to_uppercase() {
                            print_char(c)
                        }
                    } else {
                        for c in c.to_lowercase() {
                            print_char(c)
                        }
                    }
                    println!();
                }
            }
            println!();
        }
    }
    println!("Lower case");
    println!("==========");
    print_stats(map_lower, total, false);
    println!("Upper case");
    println!("==========");
    print_stats(map_upper, total, true);
}
