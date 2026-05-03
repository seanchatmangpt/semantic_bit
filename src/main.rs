mod access;
mod status8;
mod condition8;

use access::{AccessAttempt, AccessCondition, AccessField};
use status8::{Status8Field, Presence as StatusPresence};
use condition8::{Condition8Field};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("==================================================");
    println!(" THE SEMANTIC BIT: POLYMORPHIC SPINE INNOVATION ");
    println!("==================================================");
    println!();

    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("[MANUFACTURE] Entire spine generated from RDF graph.");
    println!("--------------------------------------------------");

    // 1. Check System Status (Status8)
    let status = Status8Field::empty()
        .with(Status8Field::OK)
        .with(Status8Field::RECEIPTED)
        .with(Status8Field::REPLAYABLE);

    println!("[STATUS] System State: {:#010b}", status.raw());
    if status.carries(Status8Field::OK) == StatusPresence::Present {
        println!("  -> System is OK and Replayable.");
    }

    // 2. Perform Access Admission
    let access_field = AccessField::empty()
        .with(AccessField::BADGE_PRESENT)
        .with(AccessField::BADGE_RECOGNIZED)
        .with(AccessField::HOLDER_ACTIVE)
        .with(AccessField::DOOR_ALLOWED)
        .with(AccessField::TIME_ALLOWED);

    let attempt = AccessAttempt::new(41_000_123, 17, epoch, access_field);
    let condition = attempt.select();

    match condition {
        AccessCondition::Grant => {
            println!("\n[ADMISSION] Access GRANTED.");
            
            // 3. Map to Decided Condition (Condition8)
            let decided = Condition8Field::empty().with(Condition8Field::OK);
            println!("[CONDITION] Decided Outcome: {:#010b} (OK)", decided.raw());
            
            // 4. Receipt
            let receipt = attempt.receipt(1);
            println!("\n[RECEIPT] Proof of Motion Emitted.");
            println!("     Badge: {}", receipt.badge_id);
            println!("     Raw Field: {:#010b}", receipt.access_raw);
        },
        _ => {
            println!("\n[ADMISSION] Access DENIED.");
        }
    }
    
    println!("\n[SUCCESS] Innovation complete: Meaning synchronized with Graph Law.");
}
