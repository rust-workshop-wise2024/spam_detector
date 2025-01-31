use std::{collections::HashSet, fs};

fn is_spam(mail: &str, blacklist: &HashSet<String>) -> bool {
    // geht nicht wegen domain parts fish-bait!
    // if blacklist.iter().any(|blacklisted| mail.contains(blacklisted)) { // shorthand anonyme funktion!
    //     return true;
    // }
    let mail_parts = mail.split('@').collect::<Vec<&str>>();
    let mail_domain = mail_parts.get(1).unwrap_or_else(|| mail_parts.last().unwrap());
    // println!("{:?}", mail_domain);

    for b in blacklist {
        // println!("{:?} == {:?} ? {:?}", b, mail_domain, b == mail_domain);
        if b == mail_domain {
            return true;
        }
    }
    return false;
}

fn load_blacklist(path: &str) -> Result<HashSet<String>, std::io::Error> {
    let domains = fs::read_to_string(path)?;
    let mut blacklist = HashSet::new();
    
    for line in domains.lines() {
        blacklist.insert(line.trim().to_string());
    }
    Ok(blacklist)
}

fn main() {
    let blacklist = load_blacklist("resources\\blacklist.txt").expect("Unable to load blacklist from fs");
    println!("Loaded {} blacklisted domains", blacklist.len());


    // TODO exception handling
    let content: String = fs::read_to_string("resources\\emails.txt").expect("");

    for line in content.lines() {
        if is_spam(line, &blacklist) {
            println!("Spam-Mail gefunden: {:?}", line)
        }
                  
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mail_list_len() {
        let fi = fs::read_to_string("resources\\emails.txt").expect("");
        let all_mails = fi.split("\n").collect::<Vec<&str>>();
        assert_eq!(all_mails.len(), 100);
    }

    #[test]
    fn test_is_spam() {
        let blacklist = load_blacklist("resources\\blacklist.txt").expect("Unable to load blacklist from fs");
        let binding = fs::read_to_string("resources\\emails.txt").expect("");
        let all_mails = binding.split("\r\n").collect::<Vec<&str>>();
        let spam_count = all_mails.iter()
            .filter(|m| is_spam(m, &blacklist))
            .count();
        assert_eq!(spam_count, 15);
    }
}
