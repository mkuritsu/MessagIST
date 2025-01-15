use std::{
    env,
    fs::{self, File},
    io::Write,
    process::ExitCode,
    str::FromStr,
};

use cryptolib::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
    RsaPrivateKey, RsaPublicKey,
};
use rand::rngs::OsRng;

enum Operation {
    Help,
    Protect,
    Check,
    Unprotect,
    HashPassword,
    VerifyPassword,
    GenSecretKey,
    GenRSAKeyPair,
    EncryptWithPubKey,
    DecryptWithPrivKey,
    Exit,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_str = s.to_lowercase();
        match lower_str.as_str() {
            "help" => Ok(Operation::Help),
            "protect" => Ok(Operation::Protect),
            "check" => Ok(Operation::Check),
            "unprotect" => Ok(Operation::Unprotect),
            "hash-password" => Ok(Operation::HashPassword),
            "verify-password" => Ok(Operation::VerifyPassword),
            "gen-secret-key" => Ok(Operation::GenSecretKey),
            "gen-rsa-keypair" => Ok(Operation::GenRSAKeyPair),
            "encrypt-key-with-pub-key" => Ok(Operation::EncryptWithPubKey),
            "decrypt-key-with-priv-key" => Ok(Operation::DecryptWithPrivKey),
            "exit" => Ok(Operation::Exit),
            _ => Err(()),
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(">> Too few arguments! No operation provided.");
        help_menu(&args);
        return ExitCode::FAILURE;
    }
    let operation = match Operation::from_str(&args[1]) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid operation!");
            return ExitCode::FAILURE;
        }
    };
    match operation {
        Operation::Help => help_menu(&args),
        Operation::Protect => protect(&args),
        Operation::Check => check(&args),
        Operation::Unprotect => unprotect(&args),
        Operation::HashPassword => hash_password(&args),
        Operation::VerifyPassword => verify_password(&args),
        Operation::GenSecretKey => gen_secret_key(&args),
        Operation::GenRSAKeyPair => gen_rsa_keypair(&args),
        Operation::EncryptWithPubKey => encrypt_key_with_pub_key(&args),
        Operation::DecryptWithPrivKey => decrypt_key_with_priv_key(&args),
        Operation::Exit => ExitCode::SUCCESS,
    }
}

fn help_menu(args: &[String]) -> ExitCode {
    println!(">> Usage: {} <operation>", args[0]);
    println!(">> Operations:");
    println!("  ↳ help");
    println!("  ↳ protect <input file> <key file> <output file>");
    println!("  ↳ check <doc> <key file>");
    println!("  ↳ unprotect <input file> <key file> <output file>");
    println!("  ↳ hash-password <password>");
    println!("  ↳ verify-password <password> <hash>");
    println!("  ↳ gen-secret-key <output file>");
    println!("  ↳ gen-rsa-keypair <bits> <priv key output file> <pub key output file>");
    println!("  ↳ encrypt-key-with-pub-key <key file> <pub key file> <output file>");
    println!("  ↳ dencrypt-key-with-priv-key <key file> <priv key file> <output file>");
    ExitCode::SUCCESS
}

fn protect(args: &[String]) -> ExitCode {
    if args.len() < 5 {
        eprintln!("Invalid arguments for protect!");
        return ExitCode::FAILURE;
    }
    let file_path = &args[2];
    let secret_key_path = &args[3];
    let output_path = &args[4];
    let data = match std::fs::read(file_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read data file '{file_path}': {e}");
            return ExitCode::FAILURE;
        }
    };
    let secret_key = match std::fs::read(secret_key_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read encryption key file '{secret_key_path}': {e}");
            return ExitCode::FAILURE;
        }
    };
    let (ciphertext, nonce) = match cryptolib::protect(&data, &secret_key) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to protect document: {e}");
            return ExitCode::FAILURE;
        }
    };
    let bytes = cryptolib::utils::join_cipher_nonce(&ciphertext, &nonce);
    let doc_file_name = output_path;
    let mut protected_file = match File::create(&doc_file_name) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to create encrypted document file {e}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = protected_file.write_all(&bytes) {
        eprintln!("Failed to write encrypted document file: {e}");
        return ExitCode::FAILURE;
    }
    println!("Encrypted document file {} created", doc_file_name);
    ExitCode::SUCCESS
}

fn check(args: &[String]) -> ExitCode {
    if args.len() < 4 {
        println!("Invalid arguments for check!");
        return ExitCode::FAILURE;
    }
    let file_path = &args[2];
    let secret_key_path = &args[3];
    let data = match std::fs::read(file_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read input file: {e}");
            return ExitCode::FAILURE;
        }
    };
    let secret_key = match std::fs::read(secret_key_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read secret key file: {e}");
            return ExitCode::FAILURE;
        }
    };
    let (ciphertext, nonce) = cryptolib::utils::separate_cipher_nonce(&data);
    match cryptolib::check(ciphertext, &secret_key, nonce) {
        true => println!("The integrity of {file_path} passed the verification :D"),
        false => println!("The integrity of {file_path} failed the verification :("),
    }
    ExitCode::SUCCESS
}

fn unprotect(args: &[String]) -> ExitCode {
    if args.len() < 5 {
        println!("Invalid arguments for unprotect!");
        return ExitCode::FAILURE;
    }
    let file_path = &args[2];
    let secret_key_path = &args[3];
    let output_path = &args[4];
    let Ok(data) = std::fs::read(file_path) else {
        println!("Failed to read data file!");
        return ExitCode::FAILURE;
    };
    let Ok(secret_key) = std::fs::read(secret_key_path) else {
        println!("Failed to read encryption key file!");
        return ExitCode::FAILURE;
    };
    let (ciphertext, nonce) = cryptolib::utils::separate_cipher_nonce(&data);
    let Ok(data) = cryptolib::unprotect(ciphertext, &secret_key, nonce) else {
        println!("Failed to decrypt input document!");
        return ExitCode::FAILURE;
    };
    let Ok(mut output_file) = File::create(output_path) else {
        println!("Failed to create output file!");
        return ExitCode::FAILURE;
    };
    if let Err(_) = output_file.write_all(&data) {
        println!("Failed to write document file!");
        return ExitCode::FAILURE;
    };
    println!("File unprotected written to {output_path}");
    ExitCode::SUCCESS
}

fn hash_password(args: &[String]) -> ExitCode {
    if args.len() < 3 {
        println!("Invalid arguments for hash-password!");
        return ExitCode::FAILURE;
    }
    let password = &args[2];
    let Ok(hash) = cryptolib::hash_password(password) else {
        println!("Password failed to hash!");
        return ExitCode::FAILURE;
    };
    println!("> Input password: {password}");
    println!("> Hash: {hash}");
    ExitCode::SUCCESS
}

fn verify_password(args: &[String]) -> ExitCode {
    if args.len() < 4 {
        println!("Invalid arguments for verify-password");
        return ExitCode::FAILURE;
    }
    let password = &args[2];
    let hash = &args[3];
    println!("> Input password: {password}");
    println!("> Input hash: {hash}");
    if cryptolib::verify_hashed_password(password, hash) {
        println!("> Password verification: ✓ SUCCESS");
    } else {
        println!("> Password verification: ✗ FAILED");
    }
    ExitCode::SUCCESS
}

fn gen_secret_key(args: &[String]) -> ExitCode {
    if args.len() < 3 {
        println!("Invalid arguments for gen-aes-key!");
        return ExitCode::FAILURE;
    }
    let output_file = &args[2];
    let key = cryptolib::generate_secret_key();
    match fs::write(output_file, key) {
        Ok(_) => {
            println!("Key written to file!");
            ExitCode::SUCCESS
        }
        Err(_) => {
            eprintln!("Failed to write key to file!");
            ExitCode::FAILURE
        }
    }
}

fn gen_rsa_keypair(args: &[String]) -> ExitCode {
    if args.len() < 5 {
        eprintln!("Invalid arguments for gen-rsa-keypair");
        return ExitCode::FAILURE;
    }
    let bits = &args[2];
    let priv_file = &args[3];
    let pub_file = &args[4];
    let bits = match usize::from_str(bits) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse bits to number: {e}");
            return ExitCode::FAILURE;
        }
    };
    let priv_key = match RsaPrivateKey::new(&mut OsRng::default(), bits) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to generate RSA private key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let pub_key = RsaPublicKey::from(&priv_key);
    if let Err(e) = priv_key.write_pkcs8_pem_file(priv_file, LineEnding::default()) {
        eprintln!("Failed to write RSA private key to file: {e}");
        return ExitCode::FAILURE;
    }
    if let Err(e) = pub_key.write_public_key_pem_file(pub_file, LineEnding::default()) {
        eprintln!("Failed to write RSA private key to file: {e}");
        return ExitCode::FAILURE;
    }
    println!("Keys written to files!");
    ExitCode::SUCCESS
}

fn encrypt_key_with_pub_key(args: &[String]) -> ExitCode {
    if args.len() < 5 {
        println!("Invalid arguments for encrypt-key-with-pub-key!");
        return ExitCode::FAILURE;
    }
    let key_path = &args[2];
    let pub_key_path = &args[3];
    let output_file = &args[4];
    let secret_key = match fs::read(key_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read secret key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let pubk_pem = match fs::read_to_string(pub_key_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read public key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let pub_key = match RsaPublicKey::from_public_key_pem(&pubk_pem) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse RSA public key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let key_encripted = match cryptolib::encrypt_key_with_pub_key(&secret_key, &pub_key) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to encrypt secret key: {e}");
            return ExitCode::FAILURE;
        }
    };
    match fs::write(output_file, key_encripted) {
        Ok(_) => {
            println!("Key written to file!");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to write key to file: {e}");
            ExitCode::FAILURE
        }
    }
}

fn decrypt_key_with_priv_key(args: &[String]) -> ExitCode {
    if args.len() < 5 {
        println!("Invalid arguments for decrypt-key-with-priv-key!");
        return ExitCode::FAILURE;
    }
    let key_path = &args[2];
    let priv_key_path = &args[3];
    let output_file = &args[4];
    let secret_key = match fs::read(key_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read secret key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let privk_pem = match fs::read_to_string(priv_key_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read private key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let priv_key = match RsaPrivateKey::from_pkcs8_pem(&privk_pem) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse RSA private key: {e}");
            return ExitCode::FAILURE;
        }
    };
    let key_decrypted = match cryptolib::decrypt_key_with_priv_key(&secret_key, &priv_key) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to encrypt secret key: {e}");
            return ExitCode::FAILURE;
        }
    };
    match fs::write(output_file, key_decrypted) {
        Ok(_) => {
            println!("Key written to file!");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to write key to file: {e}");
            ExitCode::FAILURE
        }
    }
}
