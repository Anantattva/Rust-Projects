// ।। ॐ नमः शिवाय ।। \\
// ॥ ॐ वामदेवाय नमः ॥ \\

///// ++ ACCOUNTING ENGINE ++ \\\\\
use std::collections::HashSet;
use std::time::Instant;

///// =========================== \\\\\
/// +++ Declaring Structs Layout +++ \\\
/// ++ Journal Structs ++ \\\
#[derive(Debug, Clone)]
pub struct EntryItem {
  name: String,
  amount: i32,
}
#[derive(Debug, Clone)]
pub struct JournalEntry {
  date: String,
  debits: Vec<EntryItem>,
  credits: Vec<EntryItem>,
  narration: String,
}
#[derive(Debug, Clone)]
pub struct Journal(Vec<JournalEntry>);

/// ++ Ledger Structs ++ \\\
#[derive(Debug, Clone)]
pub struct LedgerItem {
  date: String,
  account: String,
  amount: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum LedgerBalance {
  DEBIT(i32),
  CREDIT(i32),
  NIL,
}
#[derive(Debug, Clone)]
pub struct Ledger {
  name: String,
  debits: Vec<LedgerItem>,
  credits: Vec<LedgerItem>,
  balance: LedgerBalance,
}

/// ++ Trial Balance Structs ++ \\\
#[derive(Debug, Clone)]
pub struct TrialBalance {
  heading: String,
  date: String,
  debits: Vec<EntryItem>,
  debits_total: i32,
  credits: Vec<EntryItem>,
  credits_total: i32,
  is_balanced: bool,
}

///// ====================== \\\\\
//// +++ IMPLEMENTATIONS +++ \\\\
impl JournalEntry {
  // << validates if amounts sum match >> \\
  pub fn check_match(&self) -> bool {
    let debits_total: i32 = self.debits.iter().map(|d| d.amount).sum();
    let credits_total: i32 = self.credits.iter().map(|c| c.amount).sum();
    debits_total == credits_total
  }
  // << logs out amount mismatch error >> \\
  pub fn log_amount_error(&self) {
    let human_date: String = convert_entry_date(self.date.as_str());
    let debits_total: i32 = self.debits.iter().map(|d| d.amount).sum();
    let credits_total: i32 = self.credits.iter().map(|c| c.amount).sum();
    let diff: i32 = debits_total - credits_total;
    println!("Error spotted on entry dated {}:
  $ERROR_TYPE : <AMOUNT_MISMATCH>.
  $ERROR_MESSAGE: Debit accounts add upto {} while credit accounts add upto {}.
  $ERROR_POINT: A difference offset of {}.
  $FIX_SUGGESTION: Please, consider rectifying the amounts or fixing the accounts.", human_date, debits_total, credits_total, diff);
  }
}

impl Journal {
  // << validates entry's debit sum & credit sum matches >> \\
  pub fn validate_journal(&self) -> bool {
    self.0.iter()
      .all(|entry| JournalEntry::check_match(entry))
  }
  // << takes all names & collects them into a HashSet >> \\
  pub fn tokenize_entries(&self) -> HashSet<String> {
    let mut all_accounts = HashSet::new();
    self.0.iter()
      .for_each(|entry| {
        entry.debits.iter()
          .for_each(|debit| {
            all_accounts.insert(debit.name.clone());
          });
        entry.credits.iter()
          .for_each(|credit| {
            all_accounts.insert(credit.name.clone());
          });
      });
    all_accounts
  }
  // << generates ledger of one account head >> \\
  pub fn generate_ledger(&self, account: &str) -> Ledger {
    let mut all_debits: Vec<LedgerItem> = Vec::new();
    let mut all_credits: Vec<LedgerItem> = Vec::new();
    for entry in &self.0 {
      let is_present_in_debits: bool = entry.debits.iter().any(|i| i.name == account);
      let is_present_in_credits: bool = entry.credits.iter().any(|i| i.name == account);
      if is_present_in_debits {
        if let Some(first_credit) = entry.credits.first() {
          all_debits.push(LedgerItem {
            date: entry.date.clone(),
            account: first_credit.name.clone(),
            amount: first_credit.amount,              
          });
        }
      }      
      if is_present_in_credits {
        if let Some(first_debit) = entry.debits.first() {
          all_credits.push(LedgerItem {
            date: entry.date.clone(),
            account: first_debit.name.clone(),
            amount: first_debit.amount,              
          }); // declare input struct type before value too;
        }
      }      
    }
    let debits_total: i32 = all_debits.iter().map(|i| i.amount).sum();
    let credits_total: i32 = all_credits.iter().map(|i| i.amount).sum();
    let net_amount: i32 = debits_total - credits_total;
    let account_balance: LedgerBalance = match net_amount {
      n if n > 0 => LedgerBalance::DEBIT(n),
      n if n < 0 => LedgerBalance::CREDIT(n.abs()),
      _ => LedgerBalance::NIL,
    };
    Ledger {
      name: account.to_string(),
      debits: all_debits,
      credits: all_credits,
      balance: account_balance,
    }
  }
}

///// +++ TRIAL BALANCE GENERATION ++++ \\\\\
fn generate_trial_balance(all_ledgers: &Vec<Ledger>) -> TrialBalance {
  let mut all_debits: Vec<EntryItem> = Vec::new();
  let mut all_credits: Vec<EntryItem> = Vec::new();
  for ledger in all_ledgers {
    match ledger.balance {
      LedgerBalance::DEBIT(amount) => {
        all_debits.push(EntryItem {
          name: ledger.name.clone(),
          amount,
        });
      },
      LedgerBalance::CREDIT(amount) => {
        all_credits.push(EntryItem {
          name: ledger.name.clone(),
          amount,
        });
      },
      LedgerBalance::NIL => {},
    }
  }
  let debit_sum: i32 = all_debits.iter().map(|i| i.amount).sum();
  let credit_sum: i32 = all_credits.iter().map(|i| i.amount).sum();
  TrialBalance {
    heading: "TRIAL_BALANCE".to_string(),
    date: "31-03-2027".to_string(),
    debits: all_debits,
    debits_total: debit_sum,
    credits: all_credits,
    credits_total: credit_sum,
    is_balanced: debit_sum == credit_sum,
  }
}

///// ++ UNIFIED MASTER FUNCTION ++ \\\\\
fn accounting_master_pipeline(journal: Journal) {
  let start = Instant::now();
  let is_journal_valid: bool = Journal::validate_journal(&journal);
  if is_journal_valid {
    println!("Journal is valid. Proceeding further.");
    let all_accounts = Journal::tokenize_entries(&journal);
    let mut all_ledgers: Vec<Ledger> = Vec::new();
    all_accounts.iter()
      .for_each(|account| {
        let ledger: Ledger = Journal::generate_ledger(&journal, account);
        all_ledgers.push(ledger.clone());
        println!("{} A/c: {:#?}", ledger.name, ledger); // use {:?} for standard one-line debug, use {:#?} for cleaner, readable, multi-line debug;
      });
    let trial_balance = generate_trial_balance(&all_ledgers);
    println!("Trial Balance: {:#?}", trial_balance);
  } else {
    println!("Journal is invalid. Fix it first!");
  }
  let end = Instant::now();
  let time = (end - start).as_millis() as f32; // use .as_millis() for milliseconds time, use .as_secs_f32() for seconds time;
  println!("Time for one complete accounting process: {:.3}ms.", time);
}

///// ===================== \\\\\
//// ++ Helper Function ++ \\\\

fn convert_entry_date(date: &str) -> String {
  // << split date >> \\
  let parts: Vec<&str> = date.split('-').collect();
  if parts.len() != 3 {
    // << safety fallback >> \\
    return date.to_string(); // .as_str() converts String to &str; .to_string() does vice-verse
  }
  // << parse >> \\
  let day_num: u32 = parts[0].parse().unwrap_or(0);
  let month_num: usize = parts[1].parse().unwrap_or(1);
  let year = parts[2];
  // << match >> \\
  let day_str: String = match day_num {
    1 => "1st".to_string(),
    2 => "2nd".to_string(),
    3 => "3rd".to_string(),
    _ => format!("{}th", day_num), // the macro format!() returns a brand new String
  };
  let months: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ];
  let month_str: &str = months.get(month_num - 1).unwrap_or(&"Unknown");
  format!("{} {} {}", day_str, month_str, year)
}

///// ====================== \\\\\
/// ++ Sample Testing ++ \\\
fn get_imperfect_journal() -> Journal {
  Journal(vec![
    JournalEntry {
      date: "01-04-2027".to_string(),
      debits: vec![EntryItem { name: "Cash".to_string(), amount: 50000 }],
      credits: vec![EntryItem { name: "Capital".to_string(), amount: 50000 }],
      narration: "Started business with cash".to_string(),
    },
    JournalEntry {
      date: "02-04-2027".to_string(),
      debits: vec![EntryItem { name: "Bank".to_string(), amount: 20000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 20000 }],
      narration: "Cash deposited into bank".to_string(),
    },
    JournalEntry {
      date: "03-04-2027".to_string(),
      debits: vec![EntryItem { name: "Purchases".to_string(), amount: 10000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 10000 }],
      narration: "Goods purchased for cash".to_string(),
    },
    JournalEntry {
      date: "04-04-2027".to_string(),
      debits: vec![EntryItem { name: "Furniture".to_string(), amount: 5000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 5000 }],
      narration: "Bought furniture".to_string(),
    },
    JournalEntry {
      date: "05-04-2027".to_string(),
      debits: vec![EntryItem { name: "Rent Expense".to_string(), amount: 2000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 2000 }],
      narration: "Paid rent".to_string(),
    },
    JournalEntry {
      date: "06-04-2027".to_string(),
      debits: vec![EntryItem { name: "Salaries".to_string(), amount: 8000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 8000 }],
      narration: "Paid salaries".to_string(),
    },
    JournalEntry {
      date: "07-04-2027".to_string(),
      debits: vec![EntryItem { name: "Cash".to_string(), amount: 15000 }],
      credits: vec![EntryItem { name: "Sales".to_string(), amount: 15000 }],
      narration: "Cash sales".to_string(),
    },
    JournalEntry {
      date: "08-04-2027".to_string(),
      debits: vec![EntryItem { name: "Debtor A".to_string(), amount: 6000 }],
      credits: vec![EntryItem { name: "Sales".to_string(), amount: 6000 }],
      narration: "Credit sales to Debtor A".to_string(),
    },
    JournalEntry {
      date: "09-04-2027".to_string(),
      debits: vec![EntryItem { name: "Cash".to_string(), amount: 4000 }],
      credits: vec![EntryItem { name: "Debtor A".to_string(), amount: 4000 }],
      narration: "Received from Debtor A".to_string(),
    },
    JournalEntry {
      date: "10-04-2027".to_string(),
      debits: vec![EntryItem { name: "Electricity Expense".to_string(), amount: 1500 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 1500 }],
      narration: "Electricity bill paid".to_string(),
    },
    JournalEntry {
      date: "11-04-2027".to_string(),
      debits: vec![EntryItem { name: "Stationery".to_string(), amount: 500 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 500 }],
      narration: "Bought stationery".to_string(),
    },
    JournalEntry {
      date: "12-04-2027".to_string(),
      debits: vec![EntryItem { name: "Advertising".to_string(), amount: 3000 }],
      credits: vec![EntryItem { name: "Bank".to_string(), amount: 3000 }],
      narration: "Paid for advertising via bank".to_string(),
    },
    JournalEntry {
      date: "13-04-2027".to_string(),
      debits: vec![EntryItem { name: "Cash".to_string(), amount: 2000 }],
      credits: vec![EntryItem { name: "Commission".to_string(), amount: 2000 }],
      narration: "Commission received".to_string(),
    },
    JournalEntry {
      date: "14-04-2027".to_string(),
      debits: vec![EntryItem { name: "Creditor B".to_string(), amount: 4000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 4000 }],
      narration: "Paid to Creditor B".to_string(),
    },
    JournalEntry {
      date: "15-04-2027".to_string(),
      debits: vec![EntryItem { name: "Purchases".to_string(), amount: 7000 }],
      credits: vec![EntryItem { name: "Creditor C".to_string(), amount: 7000 }],
      narration: "Credit purchases from Creditor C".to_string(),
    },
    JournalEntry {
      date: "16-04-2027".to_string(),
      debits: vec![EntryItem { name: "Insurance".to_string(), amount: 1200 }],
      credits: vec![EntryItem { name: "Bank".to_string(), amount: 1200 }],
      narration: "Insurance premium paid".to_string(),
    },
    JournalEntry {
      date: "17-04-2027".to_string(),
      debits: vec![EntryItem { name: "Cash".to_string(), amount: 3500 }],
      credits: vec![EntryItem { name: "Sales".to_string(), amount: 3500 }],
      narration: "Cash sales".to_string(),
    },
    JournalEntry {
      date: "18-04-2027".to_string(),
      debits: vec![EntryItem { name: "Computer Equipment".to_string(), amount: 15000 }],
      credits: vec![EntryItem { name: "Bank".to_string(), amount: 15000 }],
      narration: "Bought computer equipment".to_string(),
    },
    JournalEntry {
      date: "19-04-2027".to_string(),
      debits: vec![EntryItem { name: "Traveling Expense".to_string(), amount: 800 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 800 }],
      narration: "Traveling expenses paid".to_string(),
    },
    JournalEntry {
      date: "20-04-2027".to_string(),
      debits: vec![EntryItem { name: "Drawings".to_string(), amount: 2000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 2000 }],
      narration: "Cash withdrawn for personal use".to_string(),
    },
    JournalEntry {
      date: "21-04-2027".to_string(),
      debits: vec![EntryItem { name: "Bank Charges".to_string(), amount: 150 }],
      credits: vec![EntryItem { name: "Bank".to_string(), amount: 150 }],
      narration: "Bank charges levied".to_string(),
    },
    JournalEntry {
      date: "22-04-2027".to_string(),
      debits: vec![EntryItem { name: "Cash".to_string(), amount: 1000 }],
      credits: vec![EntryItem { name: "Debtor A".to_string(), amount: 1000 }],
      narration: "Further amount received from Debtor A".to_string(),
    },
    JournalEntry {
      date: "23-04-2027".to_string(),
      debits: vec![EntryItem { name: "Repairs".to_string(), amount: 600 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 600 }],
      narration: "Repairs paid".to_string(),
    },
    JournalEntry {
      date: "24-04-2027".to_string(),
      debits: vec![EntryItem { name: "Postage".to_string(), amount: 250 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 250 }],
      narration: "Postage charges paid".to_string(),
    },
    // Deliberate Mismatch Entry (Entry 25)
    JournalEntry {
      date: "25-04-2027".to_string(),
      debits: vec![EntryItem { name: "Consulting Fee".to_string(), amount: 5000 }],
      credits: vec![EntryItem { name: "Cash".to_string(), amount: 4500 }], // Mismatch of 500
      narration: "Consulting fees paid with deliberate imbalance error".to_string(),
    },
  ])
}

fn get_perfect_journal() -> Journal {
  let mut entries = Vec::new();
  
  // Entry 1: Capital investment
  entries.push(JournalEntry {
    date: "01-04-2027".to_string(),
    debits: vec![EntryItem { name: "Cash".to_string(), amount: 100000 }],
    credits: vec![EntryItem { name: "Capital".to_string(), amount: 100000 }],
    narration: "Initial capital introduced".to_string(),
  });

  // Entry 2: Bank deposit
  entries.push(JournalEntry {
    date: "02-04-2027".to_string(),
    debits: vec![EntryItem { name: "Bank".to_string(), amount: 50000 }],
    credits: vec![EntryItem { name: "Cash".to_string(), amount: 50000 }],
    narration: "Cash deposited into bank".to_string(),
  });

  // Loop to generate 68 repeatable recurring standardized transactions (total 70 entries)
  for i in 1..=68 {
    let day = (i % 28) + 1;
    let month = ((i / 28) % 3) + 4; // Spans across April, May, June
    let date_str = format!("{:02}-{:02}-2027", day, month);
    
    let (d_name, c_name, amt, narr) = match i % 4 {
      0 => ("Purchases", "Cash", 1500, "Routine cash purchase"),
      1 => ("Cash", "Sales", 2500, "Routine cash sale"),
      2 => ("Rent Expense", "Bank", 1000, "Periodic utility/rent payment"),
      _ => ("Salaries", "Bank", 3000, "Periodic staff salary payout"),
    };

    entries.push(JournalEntry {
      date: date_str,
      debits: vec![EntryItem { name: d_name.to_string(), amount: amt }],
      credits: vec![EntryItem { name: c_name.to_string(), amount: amt }],
      narration: narr.to_string(),
    });
  }

  Journal(entries)
}


fn main() {
  println!("12-00-2007 = {}.",convert_entry_date("12-07-2007"));
  let test_journal_1 = get_imperfect_journal();
  let test_journal_2 = get_perfect_journal();
  accounting_master_pipeline(test_journal_1);
  accounting_master_pipeline(test_journal_2); // barely took 1ms; JS took 50ms for same; MONSTER PERFORMANCE 💀💀💀☠️☠️☠️
}

/*
 * @date
 * START: 20th August, 2026
 * FINISH: 21st August, 2026
 */

/*
 * @learning
 An enum (enumeration) defines a type that can be one of several different variants. Unlike enums in many other languages (which are just numbers), Rust enums are powerful algebraic data types—each variant can also store data inside it.

 * Why use it? It enforces type safety at compile time. Instead of using strings or magic numbers to represent a state (like "DEBIT", "CREDIT", or "NIL"), an enum ensures that a value must be one of the explicitly allowed options.
 */

/*

 * @learning
 * An attribute is metadata applied to a module, crate, struct, enum, or function. In Rust, they are written with a pound sign and brackets (#[attribute]) or an exclamation mark (#! [attribute] for whole modules).

 * hy use it? They tell the Rust compiler or other tools how to treat that piece of code (e.g., generating boilerplate code, enabling warnings, or formatting).

 * #[derive(...)] is an attribute macro that automatically generates code for you.
 * Debug allows you feedstock the enum into println!("{:?}", my_balance); for debugging.
 * Clone lets you duplicate the data safely.
 * PartialEq lets you compare two balances using == (e.g., balance1 == balance2).
 */