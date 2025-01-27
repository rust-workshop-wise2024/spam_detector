use std::{collections::HashSet, fs};

const EMAIL_PATH : &str = "./resources/emails.txt";
const BLACKLIST_PATH : &str = "./resources/blacklist.txt";

fn is_spam(mail: &str, blacklist: &HashSet<String>) -> bool {
    for spam_domain in blacklist  {
        let temp = format!("@{}", spam_domain);
        if mail.ends_with(temp.as_str()) {
            return true;
        }
    }
    return false;
}

fn load_file(path: &str) -> Result<HashSet<String>, std::io::Error> {
    let contents = fs::read_to_string(path)?;

    let mut hashmap = HashSet::new();
    for line in contents.lines() {
        hashmap.insert(line.to_string());
    }

    Ok(hashmap)
}

fn main() {
    let emails = match load_file(&EMAIL_PATH){
        Ok(hash_set) => hash_set,
        Err(error) => panic!("{:?}", error),
    };

    let blacklist = match load_file(&BLACKLIST_PATH){
        Ok(hash_set) => hash_set,
        Err(error) => panic!("{:?}", error),
    };

    for email in emails {
        let res = is_spam(email.as_str(), &blacklist);
        if res {
            println!("Spam {:?}", email)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_blacklist() {
        let blacklist = match load_file(&BLACKLIST_PATH){
            Ok(hash_set) => hash_set,
            Err(error) => panic!("{:?}", error),
        };
        assert_eq!(blacklist.len(), 11);
    }

    #[test]
    fn test_load_email() {
        let email = match load_file(&EMAIL_PATH){
            Ok(hash_set) => hash_set,
            Err(error) => panic!("{:?}", error),
        };
        assert_eq!(email.len(), 100);
    }

    #[test]
    fn test_is_spam() {
        let mut test_blacklist: HashSet<String> = HashSet::new();
        test_blacklist.insert("scamalerts.biz".to_string());
        test_blacklist.insert("fishyclickbait.biz".to_string());

        assert_eq!(is_spam("scamalerts.biz", &test_blacklist), false);
        assert_eq!(is_spam("@scamalerts.biz", &test_blacklist), true);
        assert_eq!(is_spam("test@scamalerts.biz", &test_blacklist), true);
        assert_eq!(is_spam("test@totallyscam.blob", &test_blacklist), false);

    }

    #[test]
    fn test_all() {
        let blacklist = match load_file(&BLACKLIST_PATH){
            Ok(hash_set) => hash_set,
            Err(error) => panic!("{:?}", error),
        };

        let emails = match load_file(&EMAIL_PATH){
            Ok(hash_set) => hash_set,
            Err(error) => panic!("{:?}", error),
        };
        let mut counter = 0;
        for email in emails {
            let res = is_spam(email.as_str(), &blacklist);
            if res {
                counter += 1 ;
            }
        }

        assert_eq!(counter, 15);
    }
} 