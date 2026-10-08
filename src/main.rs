//          ╭─────────────────────────────────────────────────────────╮
//          │                    LIFETIME ELISION                     │
//          ╰─────────────────────────────────────────────────────────╯

/*
 * INFO: RULE 1/ Each parameter that is a reference gets its own lifetime parameter.
 * INFO: RULE 2/ If there is exactly one input lifetime parameter, that lifetime is assigned
 * INFO: to all output lifetime parameters.
 * INFO: RULE 3/ If there are multiple input lifetime parameters, but one of them is &self or &mut self,
 * INFO: the lifetime of self is assigned to all output lifetime parameters => ⚠️ Apply to methods!
 */

fn main() {
    let str1 = "some_str";
    let str2 = "other_str2";

    // let received_str = return_str(&str1);
    let _received_str = return_str(&str1, &str2);
}

fn return_str<'a, 'b>(s1: &'a str, _s2: &'b str) -> &'a str {
    s1
}

// fn return_str(s1: &str) -> &str {
//     s1
// }

// fn return_str<'a>(s1: &'a str) -> &'a str {
//     s1
// }
