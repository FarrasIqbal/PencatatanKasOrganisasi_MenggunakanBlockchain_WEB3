#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct KasRecord {
    pub id: u64,
    pub tx_type: String,     // Contoh: "Pemasukan" atau "Pengeluaran"
    pub amount: String,      // Jumlah uang (dalam format String untuk simplifikasi workshop)
    pub description: String, // Keterangan transaksi
}

// Storage key untuk data kas
const LEDGER_DATA: Symbol = symbol_short!("LEDGER");

#[contract]
pub struct OrgLedgerContract;

#[contractimpl]
impl OrgLedgerContract {
    // Fungsi untuk melihat semua riwayat transaksi kas secara transparan
    pub fn get_transactions(env: Env) -> Vec<KasRecord> {
        return env.storage().instance().get(&LEDGER_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk mencatat transaksi baru (permanen di blockchain)
    pub fn add_transaction(env: Env, tx_type: String, amount: String, description: String) -> String {
        let mut ledger: Vec<KasRecord> = env.storage().instance().get(&LEDGER_DATA).unwrap_or(Vec::new(&env));
        
        let new_tx = KasRecord {
            id: env.prng().gen::<u64>(),
            tx_type: tx_type,
            amount: amount,
            description: description,
        };
        
        ledger.push_back(new_tx);
        env.storage().instance().set(&LEDGER_DATA, &ledger);
        
        return String::from_str(&env, "Transaksi kas berhasil dicatat di blockchain!");
    }

    // Fungsi untuk membatalkan/menghapus rekam transaksi jika ada salah input
    pub fn void_transaction(env: Env, id: u64) -> String {
        let mut ledger: Vec<KasRecord> = env.storage().instance().get(&LEDGER_DATA).unwrap_or(Vec::new(&env));

        for i in 0..ledger.len() {
            if ledger.get(i).unwrap().id == id {
                ledger.remove(i);
                env.storage().instance().set(&LEDGER_DATA, &ledger);
                return String::from_str(&env, "Transaksi berhasil dibatalkan (void)");
            }
        }

        return String::from_str(&env, "ID Transaksi tidak ditemukan")
    }
}

mod test;