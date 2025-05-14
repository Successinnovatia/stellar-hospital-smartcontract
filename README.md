# 🏥 Stellar Hospital Smart Contract

This project implements a hospital management smart contract using the [Stellar Soroban](https://soroban.stellar.org/docs) smart contract platform. It allows decentralized management of hospital records including patients, doctors, and medical tests.

---

## 📦 Features

- ✅ Register and retrieve **patients**
- 🧑‍⚕️ Register and retrieve **doctors**
- 🧪 Record and update **medical tests**
- 🔄 Update patient and doctor records
- 🔐 Access control using an admin account

---

## 🛠 Technologies

- 🦀 [Rust](https://www.rust-lang.org/)
- 🌌 [Soroban SDK](https://soroban.stellar.org/docs/software-development-kit)
- 📄 Soroban smart contracts

---

## 🧩 Contract Functions

### ✅ Initialization
```rust
fn initialize(admin: Address)
fn check_admin(env: &Env)
```

## Patient Management
``` rust
fn register_patient(...) -> u64
fn get_patient(id: u64) -> Patient
fn update_patient(...) -> Patient
fn list_patients(env: Env) -> Vec<Patient>
fn set_patient_active(env: Env, id: u64, active: bool) -> Patient

```

## Doctor Management

``` rust
fn register_doctor(...) -> u64
fn get_doctor(id: u64) -> Doctor
fn update_doctor(...) -> Doctor
fn set_doctor_active(env: Env, id: u64, active: bool) -> Doctor
fn list_doctors(env: Env) -> Vec<Doctor>
```

## Medical Test Management

```rust
fn record_medical_test(...) -> u64
fn get_medical_test(id: u64) -> MedicalTest
fn update_medical_test(...) -> MedicalTest
fn get_medical_tests_for_patient(env: Env, patient_id: u64) -> Vec<MedicalTest>
fn get_medical_tests_by_doctor(env: Env, doctor_id: u64) -> Vec<MedicalTest>
fn list_medical_tests(env: Env) -> Vec<MedicalTest>
fn get_test_statistics(env: Env) -> (u64, u64, u64)
```

🧪 Running Tests
Make sure you have Rust and the Soroban CLI installed.

1. Clone the repo:

```bash 
git clone https://github.com/Successinnovatia/stellar-hospital-smartcontract.git
cd stellar-hospital-smartcontract
```

2. Run tests:
```bash
cargo test
```

📂 Folder Structure
sstellar-hospital-smartcontract/
├── src/
│   ├── contract.rs        # Main smart contract logic
│   ├── storage.rs         # Storage key definitions
│   ├── types.rs           # Structs for Patient, Doctor, MedicalTest, etc.
│   └── test.rs            # Unit tests for contract functions
├── Cargo.toml             # Rust package configuration
├── README.md              # Project overview
└── .gitignore             # Git ignored files


👤 Author
- Divine Success (@Successinnovatia)

📄 License
MIT License. See LICENSE file for details.