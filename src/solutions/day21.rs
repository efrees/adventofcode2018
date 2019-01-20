use std::collections::HashSet;

pub fn solve() {
    println!("Day 21");
    let mut halting_values = HashSet::new();
    let mut e: u64 = 0;
    loop {
        let mut b = e | 0x10000;
        e = 2024736;
        loop {
            let mut c = b & 0xFF;
            e += c;
            e = e & 0xFFFFFF;
            e *= 65899;
            e = e & 0xFFFFFF;
            if b < 256 {
                break;
            }

            c = 0;
            while 256 * c + 256 <= b {
                c += 1;
            }
            b = c;
        } // end loop
        if halting_values.len() == 0 {
            println!("Halting input (lower bound): {}", e);
        }

        // print out all, looking for last
        // if !halting_values.contains(&e) {
        //     println!("Halting input: {}", e);
        // }

        if e == 12284643 {
            println!("Halting input (upper bound): {}", e);
            break;
        }

        halting_values.insert(e);
    }
}
