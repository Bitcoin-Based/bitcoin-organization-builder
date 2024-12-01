use crate::wallet::{create_wallet, add_xpub_interactive, XpubManager};
use std::io::{self, Write};

pub fn run(manager: &mut XpubManager) {
    // Display the welcome message once
    display_welcome_message();

    loop {
        // Display options
        display_menu();
        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Creating wallet...");
                create_wallet(manager);
            }
            "2" => {
                println!("Listing stored xpubs...");
                manager.list_xpubs();
            }
            "3" => {
                println!("Adding an external xpub...");
                add_xpub_interactive(manager);
            }
            "4" => {
                println!("Generating addresses...");
                generate_addresses_workflow(manager);
            }
            "5" => {
                println!("Validating xpub...");
                validate_xpub_workflow(manager);
            }
            "6" | "exit" => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, 3, 4, 5, 6, or 'exit'."),
        }
    }
}

fn display_welcome_message() {
    println!("\n==============================");
    println!(" Welcome to Bitcoin Organization Builder (BoB)");
    println!("==============================");
}

fn display_menu() {
    println!("\nSelect an option:");
    println!("==============================");
    println!("[1] Create Wallet");
    println!("[2] List Stored Xpubs");
    println!("[3] Add an External Xpub");
    println!("[4] Generate Addresses");
    println!("[5] Validate Xpub");
    println!("[6/exit] Exit");
}

/// Workflow for generating addresses
fn generate_addresses_workflow(manager: &mut XpubManager) {
    if manager.is_empty() {
        println!("No xpubs stored. Please add an xpub first.");
        return;
    }

    println!("Stored xpubs:");
    for (index, xpub) in manager.iter_xpubs().enumerate() {
        println!("[{}] {}", index, xpub);
    }

    // Prompt user to select an xpub
    print!("\nEnter the index of the xpub to generate addresses for: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index: usize = match input.trim().parse() {
        Ok(idx) => idx,
        Err(_) => {
            println!("Invalid index. Returning to menu.");
            return;
        }
    };

    let xpub = match manager.get_xpub_by_index(index) {
        Some(x) => x,
        None => {
            println!("Index out of bounds. Returning to menu.");
            return;
        }
    };

    println!("\nUsing standard derivation path: m/0'/0'");
    println!("Generating the first 10 addresses:");
    for i in 0..10 {
        if let Some(address) = manager.generate_address(xpub, i) {
            println!("[{}] {}", i, address);
        } else {
            println!("[{}] Failed to generate address.", i);
        }
    }
}


/// Workflow for validating xpubs
fn validate_xpub_workflow(manager: &mut XpubManager) {
    print!("\nEnter xpub to validate: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let xpub = input.trim();

    if XpubManager::validate_xpub(xpub) {
        println!("Valid Xpub!");

        print!("\nDo you want to save this xpub? (yes/no): ");
        io::stdout().flush().unwrap();
        let mut save_input = String::new();
        io::stdin().read_line(&mut save_input).unwrap();

        if save_input.trim().eq_ignore_ascii_case("yes") {
            if manager.add_xpub(xpub) {
                println!("xpub added successfully!");
            }
        }
    } else {
        println!("Invalid Xpub.");
    }
}
