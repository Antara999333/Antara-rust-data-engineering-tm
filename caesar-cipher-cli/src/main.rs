/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/


use caeser_cipher_cli::{decrypt, encrypt};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(long)]
    superencrypt: bool,

    /// Decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// Reverse the message (new argument)
    #[arg(long)]
    reverse: bool,

    /// The message to encrypt, decrypt, or reverse
    #[arg(long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}


// run it
fn main() {
    let args = Args::parse();
    if args.encrypt {
        println!("{}", superencrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else if args.reverse {
        // Reverse the message (for example, reverse a string "hello" to "olleh")
        let reversed_message: String = args.message.chars().rev().collect();
        println!("{}", reversed_message);
    } else {
        println!("Please specify either --superencrypt, --decrypt, or --reverse");
    }
}

