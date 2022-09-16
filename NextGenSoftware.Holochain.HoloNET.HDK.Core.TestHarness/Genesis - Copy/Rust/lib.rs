#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
use holochain_entry_utils::HolochainEntry;

// see https://developer.holochain.org/api/0.0.50-alpha4/hdk/ for info on using the hdk library
impl HolochainEntry for SuperHolon {
    fn entry_type() -> String {
        String::from("super_holon")
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct SuperHolon {
super_test_string: String,
super_test_int: i32,
super_test_bool: bool
}

impl HolochainEntry for SuperTest {
    fn entry_type() -> String {
        String::from("super_test")
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct SuperTest {
test_string: String,
test_int: i32,
test_bool: bool
}


#[zome]
mod super_zome {
#[entry_def]
fn my_holon_def() -> ValidatingEntryType {
    entry!(
        name: "my_holon",
        description: "holon description",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MySuperHolon>| {
            Ok(())
        }
    )
}

#[zome_fn("hc_public")]
fn delete_holon(address: Address) -> ZomeApiResult<Address> {
    /*
    hdk::remove_link(&anchor_address()?, &address, "course_list", "")?;

    let students = get_students(address.clone())?;
    let course: Course = hdk::utils::get_as_type(address.clone())?;

    for student in students {
        hdk::remove_link(&student, &address, "student->course", "")?;
    }
    hdk::remove_link(&course.teacher_address, &address, "teacher->courses", "")?;
    */

    hdk::remove_entry(&address)
}

#[zome_fn("hc_public")]
fn update_my_entry(updated_entry: MyEntry, address: Address) -> ZomeApiResult<Address> {
    let mut my_entry: MyEntry = hdk::utils::get_as_type(address.clone())?;
    
    super_holon.super_test_string=updated_entry.super_test_string;
	super_holon.super_test_int=updated_entry.super_test_int;
	super_holon.super_test_bool=updated_entry.super_test_bool;


    let updated_address = hdk::update_entry(my_entry.clone().entry(), &address)?;
    Ok(updated_address)
}


#[zome_fn("hc_public")]
fn read_my_holon(address: Address) -> ZomeApiResult<MyEntry> {
    //hdk::get_entry(&address)
    hdk::utils::get_as_type(address)
}

#[zome_fn("hc_public")]
fn create_holon(holon: SuperHolon) -> ZomeApiResult<Address> {
    let holon = Entry::App("super_holon".into(), holon.into());
    let address = hdk::commit_entry(&holon)?;
    Ok(address)
}


#[entry_def]
fn my_holon_def() -> ValidatingEntryType {
    entry!(
        name: "my_holon",
        description: "holon description",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MySuperTest>| {
            Ok(())
        }
    )
}

#[zome_fn("hc_public")]
fn delete_holon(address: Address) -> ZomeApiResult<Address> {
    /*
    hdk::remove_link(&anchor_address()?, &address, "course_list", "")?;

    let students = get_students(address.clone())?;
    let course: Course = hdk::utils::get_as_type(address.clone())?;

    for student in students {
        hdk::remove_link(&student, &address, "student->course", "")?;
    }
    hdk::remove_link(&course.teacher_address, &address, "teacher->courses", "")?;
    */

    hdk::remove_entry(&address)
}

#[zome_fn("hc_public")]
fn update_my_entry(updated_entry: MyEntry, address: Address) -> ZomeApiResult<Address> {
    let mut my_entry: MyEntry = hdk::utils::get_as_type(address.clone())?;
    
    super_test.test_string=updated_entry.test_string;
	super_test.test_int=updated_entry.test_int;
	super_test.test_bool=updated_entry.test_bool;


    let updated_address = hdk::update_entry(my_entry.clone().entry(), &address)?;
    Ok(updated_address)
}


#[zome_fn("hc_public")]
fn read_my_holon(address: Address) -> ZomeApiResult<MyEntry> {
    //hdk::get_entry(&address)
    hdk::utils::get_as_type(address)
}

#[zome_fn("hc_public")]
fn create_holon(holon: SuperTest) -> ZomeApiResult<Address> {
    let holon = Entry::App("super_test".into(), holon.into());
    let address = hdk::commit_entry(&holon)?;
    Ok(address)
}


    
    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }
}
