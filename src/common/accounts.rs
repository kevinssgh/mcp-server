use anyhow::{Result, anyhow};
use ethers::signers::{LocalWallet, MnemonicBuilder, Signer};
use ethers::types::Address;
use std::collections::HashMap;

/// Anvil's default mnemonic phrase
const ANVIL_MNEMONIC: &str = "test test test test test test test test test test test junk";

pub struct Account {
    pub(crate) wallet: LocalWallet,
    pub(crate) derivation_path: String,
}

pub struct Accounts {
    pub(crate) wallets: HashMap<Address, Account>,
    pub(crate) mnemonic: String,
}

impl Accounts {
    /// Create accounts from Anvil's default mnemonic
    pub fn new() -> Result<Self> {
        Self::from_mnemonic(ANVIL_MNEMONIC, 10)
    }

    /// Create accounts from a custom mnemonic
    pub fn from_mnemonic(mnemonic: &str, count: u32) -> Result<Self> {
        let mut wallets = HashMap::new();

        for i in 0..count {
            let derivation_path = format!("m/44'/60'/0'/0/{i}");

            let wallet = MnemonicBuilder::<ethers::signers::coins_bip39::English>::default()
                .phrase(mnemonic)
                .derivation_path(&derivation_path)?
                .build()
                .map_err(|e| anyhow!("Failed to build wallet from mnemonic: {}", e))?;

            let address = wallet.address();
            let account = Account {
                wallet,
                derivation_path: derivation_path.clone(),
            };

            wallets.insert(address, account);
        }

        Ok(Self {
            wallets,
            mnemonic: mnemonic.to_string(),
        })
    }

    /// Get wallet by address
    pub fn get_wallet(&self, address: &Address) -> Option<&LocalWallet> {
        self.wallets.get(address).map(|account| &account.wallet)
    }

    /// Get Default wallet
    pub fn default_wallet(&self) -> Option<&LocalWallet> {
        let addresses = self.addresses();
        if !addresses.is_empty() {
            let first_address = addresses[0];
            return self.get_wallet(&first_address);
        }
        None
    }

    /// Get all addresses
    pub fn addresses(&self) -> Vec<Address> {
        self.wallets.keys().cloned().collect()
    }

    /// Print all account information
    /// (TESTING ONLY)
    #[allow(dead_code)]
    pub fn print_accounts(&self) {
        println!("Accounts derived from mnemonic:");
        println!("Mnemonic: {}", self.mnemonic);
        println!();

        let mut addresses: Vec<_> = self.wallets.iter().collect();
        addresses.sort_by_key(|(_, account)| &account.derivation_path);

        for (i, (address, account)) in addresses.iter().enumerate() {
            println!("Account #{i}: {address}");
            println!(
                "  Private Key: 0x{}",
                hex::encode(account.wallet.signer().to_bytes())
            );
            println!("  Derivation Path: {}", account.derivation_path);
            println!();
        }
    }
}

impl Default for Accounts {
    fn default() -> Self {
        Self::new().expect("Failed to create default accounts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anvil_accounts_creation() {
        let accounts = Accounts::new().unwrap();
        assert_eq!(accounts.wallets.len(), 10);

        // Test that we can get addresses
        let addresses = accounts.addresses();
        assert_eq!(addresses.len(), 10);

        // Test that all addresses are unique
        let mut unique_addresses = addresses.clone();
        unique_addresses.sort();
        unique_addresses.dedup();
        assert_eq!(unique_addresses.len(), 10);

        accounts.print_accounts();
    }

    #[test]
    fn test_get_wallet() {
        let accounts = Accounts::new().unwrap();
        let addresses = accounts.addresses();
        let first_address = addresses[0];

        let wallet = accounts.get_wallet(&first_address);
        assert!(wallet.is_some());
        assert_eq!(wallet.unwrap().address(), first_address);
    }
}
