use std::{collections::HashSet, fs};

/* 
Eure Aufgabe: Ihr wollt einen Spam-Filter für eingehende Emails schreiben, basierend auf einer Liste bekannter Spam-Domains. Vervollständigt dafür die vorgegebenen Funktionen! Optional dürft ihr gerne die Unit-Tests nutzen.

- Die zu testenden Emails sowie die Liste der Spam-Domains sollen aus dem Ressourcen-Verzeichnis eingelesen werden.
- Die Textdateien dürfen nicht verändert werden!
- Mail-Adressen sollen genau dann als Spam erkannt werden, wenn ihre Domain-Endung mit einer Regel >übereinstimmt<.

Beispiel: 'fastmoney.biz' -> 'luckywinner9000@fastmoney.biz' ist Spam

Viel Spaß!
 */
fn is_spam(mail: &str, blacklist: &HashSet<String>) -> bool {
    todo!("true falls mail als Spam erkannt wurde")
}

fn load_blacklist(path: &str) -> Result<HashSet<String>, std::io::Error> {
    todo!("liefert Set von bekannten Spam-Domains aus 'resources\\blacklist.txt'")
}

fn main() {
    todo!("Finde alle 15 Spam-Adressen in 'resources\\emails.txt'")
}

/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_blacklist() {
        assert_eq!(blacklist, 11);
    }

    #[test]
    fn test_is_spam() {
        assert_eq!(spam_count, 15);
    }
} 
*/