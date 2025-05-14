#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

fn setup() -> (HospitalContractClient<'static>, Address, Env, Address) {
    let env = Env::default();

    let contract_id = env.register(HospitalContract, ());
    let client = HospitalContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let admin_initializer = client.initialize(&admin);

    (client, admin, env, admin_initializer)
}

fn patient_data(env: &Env) -> (String, u64, String, Vec<String>, String) {
    env.mock_all_auths();

    let name = String::from_str(&env, "Anna");
    let date_of_birth: u64 = 17061999;
    let blood_type = String::from_str(&env, "A+");
    let allergies = vec![
        &env,
        String::from_str(&env, "peanut"),
        String::from_str(&env, "peanut"),
    ];
    let insurance_id = String::from_str(&env, "1S2D5");

    (name, date_of_birth, blood_type, allergies, insurance_id)
}

fn doctor_data(env: &Env) -> (String, String, String) {
    let doctor_name = String::from_str(&env, "Dr Divine");
    let specialization = String::from_str(&env, "neurologist");
    let license_number = String::from_str(&env, "DOC789");

    (doctor_name, specialization, license_number)
}

fn medical_test_data(env: &Env) -> (String, u64, String, String) {
    let test_type = String::from_str(&env, "Blood pressure");
    let test_date = env.ledger().timestamp();
    let results = String::from_str(&env, "120/80, Normal");
    let notes = String::from_str(&env, "Patient should continue his medication");

    (test_type, test_date, results, notes)
}

#[test]

pub fn test_register_get_update_patient() {
    let (client, admin, env, admin_initializer) = setup();

    assert_eq!(admin_initializer, admin);

    env.mock_all_auths();

    let (name, date_of_birth, blood_type, allergies, insurance_id) = patient_data(&env);

    // test register patient
    let patient_id = client.register_patient(
        &name,
        &date_of_birth,
        &blood_type,
        &allergies,
        &insurance_id,
    );

    assert_eq!(patient_id, 1);

    // test get patient
    let stored_patient = client.get_patient(&patient_id);

    assert_eq!(stored_patient.name, name);
    assert_eq!(stored_patient.active, true);

    // test update patient

    let updated_allergies = vec![
        &env,
        String::from_str(&env, "peanut"),
        String::from_str(&env, "peanut"),
        String::from_str(&env, "penicillin"),
    ];

    let updated_patient = client.update_patient(
        &patient_id,
        &name,
        &date_of_birth,
        &blood_type,
        &updated_allergies,
        &insurance_id,
    );

    assert_eq!(updated_patient.allergies.len(), 3);
    assert_eq!(
        updated_patient.allergies.get(2).unwrap(),
        updated_allergies.get(2).unwrap()
    );

    // Test listing patients
    let patients = client.list_patients();
    assert_eq!(patients.len(), 1);

    //test set patient inactive
    let inactive_patient = client.set_patient_active(&patient_id, &false);
    assert_eq!(inactive_patient.active, false);
}

#[test]
fn test_register_get_update_doctor() {
    let (client, _, env, _) = setup();
    env.mock_all_auths();
    let (doctor_name, specialization, license_number) = doctor_data(&env);

    //register doctor
    let doctor_id = client.register_doctor(&doctor_name, &specialization, &license_number);

    assert_eq!(doctor_id, 1);

    //get doctor
    let stored_doctor = client.get_doctor(&doctor_id);
    assert_eq!(stored_doctor.name, doctor_name);
    assert_eq!(stored_doctor.active, true);

    //update doctor
    let updated_specialization = String::from_str(&env, "Cardiologist");

    let updated_doctor = client.update_doctor(
        &doctor_id,
        &doctor_name,
        &updated_specialization,
        &license_number,
    );

    assert_eq!(updated_doctor.specialization, updated_specialization);

    // Test listing doctors
    let doctors = client.list_doctors();
    assert_eq!(doctors.len(), 1);

    // Test setting doctor inactive
    let inactive_doctor = client.set_doctor_active(&doctor_id, &false);
    assert_eq!(inactive_doctor.active, false);
}

#[test]
fn test_record_get_update_medical_test() {
    let (client, _, env, _) = setup();

    env.mock_all_auths();

    //get patient data
    let (name, date_of_birth, blood_type, allergies, insurance_id) = patient_data(&env);

    //get doctor data
    let (doctor_name, specialization, license_number) = doctor_data(&env);

    //get medical test data
    let (test_type, test_date, results, notes) = medical_test_data(&env);

    //register patient
    let patient_id = client.register_patient(
        &name,
        &date_of_birth,
        &blood_type,
        &allergies,
        &insurance_id,
    );

    //register doctor
    let doctor_id = client.register_doctor(&doctor_name, &specialization, &license_number);

    //test record medical test
    let test_id = client.record_medical_test(
        &patient_id,
        &doctor_id,
        &test_type,
        &test_date,
        &results,
        &notes,
    );

    assert_eq!(test_id, 1);

    // Test retrieve the medical test

    let test = client.get_medical_test(&test_id);
    assert_eq!(test.patient_id, patient_id);
    assert_eq!(test.doctor_id, doctor_id);

    // Test getting doctors test
    let doctor_test = client.get_medical_tests_by_doctor(&doctor_id);
    assert_eq!(doctor_test.len(), 1);
    assert_eq!(doctor_test.get(0).unwrap().id, test_id);

    // Test getting the patient tests
    let patient_test = client.get_medical_tests_for_patient(&patient_id);
    assert_eq!(patient_test.len(), 1);
    assert_eq!(patient_test.get(0).unwrap().id, test_id);

    //Test update medical test

    let updated_test_type = String::from_str(&env, "Culture");

    let updated_test = client.update_medical_test(
        &test_id,
        &patient_id,
        &doctor_id,
        &updated_test_type,
        &test_date,
        &results,
        &notes,
    );

    assert_eq!(updated_test.test_type, updated_test_type);
}

#[test]
#[should_panic(expected = "Patient is inactive")]

fn test_record_inactive_patient_medical_test() {
    let (client, _, env, _) = setup();

    env.mock_all_auths();

    //get patient data
    let (name, date_of_birth, blood_type, allergies, insurance_id) = patient_data(&env);

    //get doctor data
    let (doctor_name, specialization, license_number) = doctor_data(&env);

    //get medical test data
    let (test_type, test_date, results, notes) = medical_test_data(&env);

    //register patient
    let patient_id = client.register_patient(
        &name,
        &date_of_birth,
        &blood_type,
        &allergies,
        &insurance_id,
    );

    //register doctor
    let doctor_id = client.register_doctor(&doctor_name, &specialization, &license_number);

    // Set patient to inactive
    client.set_patient_active(&patient_id, &false);

    //Try to record

    client.record_medical_test(
        &patient_id,
        &doctor_id,
        &test_type,
        &test_date,
        &results,
        &notes,
    );
}

// patient not registered

#[test]
#[should_panic(expected = "Doctor is inactive")]

fn test_record_inactive_doctor_medical_test() {
    let (client, _, env, _) = setup();

    env.mock_all_auths();

    //get patient data
    let (name, date_of_birth, blood_type, allergies, insurance_id) = patient_data(&env);

    //get doctor data
    let (doctor_name, specialization, license_number) = doctor_data(&env);

    //get medical test data
    let (test_type, test_date, results, notes) = medical_test_data(&env);

    //register patient
    let patient_id = client.register_patient(
        &name,
        &date_of_birth,
        &blood_type,
        &allergies,
        &insurance_id,
    );

    //register doctor
    let doctor_id = client.register_doctor(&doctor_name, &specialization, &license_number);

    //set doctor to inactive
    client.set_doctor_active(&doctor_id, &false);

    //try to record medical test

    client.record_medical_test(
        &patient_id,
        &doctor_id,
        &test_type,
        &test_date,
        &results,
        &notes,
    );
}

#[test]
#[should_panic(expected = "Contract already initialized")]
fn test_initialize_already_initialized_contract() {
    let (client, admin, _, _) = setup();

    //try to initialize again

    client.initialize(&admin);
}

#[test]
#[should_panic(expected = "Patient not registered")]
fn test_get_unregistered_patient() {
    let (client, _, env, _) = setup();

    env.mock_all_auths();

    let unregistered_patient_id:u64 = 1;

    client.get_patient(&unregistered_patient_id);
}

#[test]
#[should_panic(expected = "Doctor not registered")]
fn test_get_unregistered_doctor() {
    let (client, _, env, _) = setup();

    env.mock_all_auths();

    let unregistered_doctor_id:u64 = 1;

    client.get_doctor(&unregistered_doctor_id);
}
