use crate::models::{HasName, Posting, PostingType, Transaction};
use crate::CommonOpts;

pub fn filter(options: &CommonOpts, transaction: &Transaction<Posting>, posting: &Posting) -> bool {
    // Get what's needed
    let predicate = &options.query;
    let real = options.real;

    // Check for real postings
    if real {
        if let PostingType::Real = posting.kind {
        } else {
            return false;
        }
    }

    // Check for dates at the transaction level
    // todo should do this at the posting level
    if let Some(date) = options.end {
        if transaction.date.unwrap() >= date {
            return false;
        }
    }
    if let Some(date) = options.begin {
        if transaction.date.unwrap() < date {
            return false;
        }
    }
    return filter_predicate(predicate, posting);
}
pub fn filter_predicate(predicate: &Vec<String>, posting: &Posting) -> bool {
    let name = posting.account.get_name().to_lowercase();
    if predicate.len() == 0 {
        return true;
    }
    for pred in predicate {
        let p = pred.trim();
        if p.starts_with("%") {
            // look in the posting tags
            for tag in posting.tags.iter() {
                match tag.name.to_lowercase().find(&p.to_lowercase()[1..]) {
                    None => continue,
                    Some(_) => return true,
                }
            }
        } else {
            match name.find(&p.to_lowercase()) {
                None => continue,
                Some(_) => return true,
            }
        }
    }
    false
}
