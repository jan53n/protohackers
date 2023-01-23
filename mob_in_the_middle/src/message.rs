use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

const BOGUSCOIN: &str = "7YWHMfk9JZe0LM0g1ZauHuiSxhI";

fn eat_until(chars: &mut Peekable<Enumerate<Chars>>, predicate: fn(char) -> bool) -> Option<usize> {
    for (i, c) in chars.by_ref() {
        if predicate(c) {
            return Some(i);
        } else {
            continue;
        }
    }

    None
}

fn match_precond(c: &str, ci: usize) -> bool {
    let i: i32 = (ci as i32) - 1;

    if i < 0 {
        return true;
    }

    if let Some(valid) = c.get(i as usize..) {
        return valid.starts_with(' ');
    }

    false
}

fn match_postcond(c: &str, ci: usize) -> bool {
    if ci == c.len() {
        return true;
    }

    if let Some(valid) = c.get(ci + 1..) {
        return valid.starts_with(' ') || valid.starts_with('\n') || valid.is_empty();
    }

    true
}

fn eat_while(chars: &mut Peekable<Enumerate<Chars>>, predicate: fn(char) -> bool) -> Option<usize> {
    let mut last = None;

    for (i, c) in chars.by_ref() {
        if predicate(c) {
            last = Some(i);
            continue;
        } else {
            break;
        }
    }

    last
}

// yes, yes, i handwrote this, yeah, yeah, i know :(
pub fn rewrite_message_with_coin(message: String) -> String {
    let mut result = message.clone();
    let mut chars = message.chars().enumerate().peekable();

    while let Some(si) = eat_until(&mut chars, |x| x == '7') {
        if !match_precond(&message[..], si) {
            continue;
        }

        if let Some(li) = eat_while(&mut chars, |x| x.is_alphanumeric()) {
            if !match_postcond(&message[..], li) {
                continue;
            }

            let size = li - si;

            if (25..35).contains(&size) {
                let mut li = li + 1;
                if li > message.len() {
                    li = message.len();
                }

                result = result.replace(&message[si..li], BOGUSCOIN);
            }
        }
    }

    result
}

#[test]
fn should_match_preconditions() {
    assert_eq!(
        rewrite_message_with_coin("7hZv5BMgzUkjE28S3fc3QVGEHO".to_string()),
        format!("{BOGUSCOIN}")
    );

    assert_eq!(
        rewrite_message_with_coin("HELLO 7hZv5BMgzUkjE28S3fc3QVGEHO".to_string()),
        format!("HELLO {BOGUSCOIN}")
    );
}

#[test]
fn should_match_postconditions() {
    assert_eq!(
        rewrite_message_with_coin("7hZv5BMgzUkjE28S3fc3QVGEHO".to_string()),
        format!("{BOGUSCOIN}")
    );

    assert_eq!(
        rewrite_message_with_coin("HELLO 7hZv5BMgzUkjE28S3fc3QVGEHO easdasd".to_string()),
        format!("HELLO {BOGUSCOIN} easdasd")
    );
}

#[test]
fn should_fail_replace_with_bogus() {
    assert_eq!(
        rewrite_message_with_coin("Please pay the ticket price of 15 Boguscoins to one of these addresses: 7e8AZRUqJDuAKKVX0AVy7rumBw7nqo 7uI3b69cGkEVRcJQgHrmf5389sC8c 7YWHMfk9JZe0LM0g1ZauHuiSxhI".to_string()),
        format!("Please pay the ticket price of 15 Boguscoins to one of these addresses: {BOGUSCOIN} {BOGUSCOIN} {BOGUSCOIN}")
    );

    assert_eq!(
        rewrite_message_with_coin("a 7iKDZEwPZSq".to_string()),
        "a 7iKDZEwPZSq"
    );

    assert_eq!(
        rewrite_message_with_coin(
            "Please send the payment of 750 Boguscoins to 7aNs40WdhFyotsVWZ8Eaby9AxoV2AHfM72C"
                .to_string()
        ),
        format!("Please send the payment of 750 Boguscoins to {BOGUSCOIN}")
    );

    assert_eq!(
        rewrite_message_with_coin(
            "This is too long: 7Ryicv1IR6zGBDBKxgOoTsa8lLOKnlCc56d9".to_string()
        ),
        "This is too long: 7Ryicv1IR6zGBDBKxgOoTsa8lLOKnlCc56d9"
    );

    assert_eq!(
        rewrite_message_with_coin(
            "This is a product ID, not a Boguscoin: 79Q3XcLeao2AywS6ZESfQ6Klo9A-FkG9gFCIDABha2yyLEW8v2epb4Gumct-1234".to_string()
        ),
        "This is a product ID, not a Boguscoin: 79Q3XcLeao2AywS6ZESfQ6Klo9A-FkG9gFCIDABha2yyLEW8v2epb4Gumct-1234"
    );
}
