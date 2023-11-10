#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::borrow::Borrow;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SmartStorageItem {
    id: u64,
    name: String,
    description: String,
    location: String,
    created_at: u64,
    updated_at: Option<u64>,
    is_available: bool,
}

impl Storable for SmartStorageItem {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SmartStorageItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE_ITEM_STORAGE: RefCell<StableBTreeMap<u64, SmartStorageItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct SmartStorageItemPayload {
    name: String,
    description: String,
    location: String,
    is_available: bool,
}

#[ic_cdk::query]
fn get_smart_storage_item(id: u64) -> Result<SmartStorageItem, Error> {
    match _get_smart_storage_item(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_all_smart_storage_items() -> Vec<SmartStorageItem> {
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_available_smart_storage_items() -> Vec<SmartStorageItem> {
    // Assuming the type of STORAGE_ITEM_STORAGE is Ref<'_, BTreeMap<u64, SmartStorageItem, VirtualMemory<Rc<RefCell<Vec<u8>>>>>>
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, item)| item.is_available)
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn search_smart_storage_items(query: String) -> Vec<SmartStorageItem> {
    STORAGE_ITEM_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, item)| item.name.contains(&query) || item.description.contains(&query))
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn add_smart_storage_item(item: SmartStorageItemPayload) -> Option<SmartStorageItem> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let storage_item = SmartStorageItem {
        id,
        name: item.name,
        description: item.description,
        location: item.location,
        created_at: time(),
        updated_at: None,
        is_available: item.is_available,
    };
    do_insert_smart_storage_item(&storage_item);
    Some(storage_item)
}
#[ic_cdk::update]
fn update_smart_storage_item(id: u64, payload: SmartStorageItemPayload) -> Result<SmartStorageItem, Error> {
    match STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(mut item) => {
            item.name = payload.name;
            item.description = payload.description;
            item.location = payload.location;
            item.updated_at = Some(time());
            item.is_available = payload.is_available;
            
            // No need to call do_insert_smart_storage_item as the item is modified in place

            Ok(item.clone())
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update an item with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn is_item_available(id: u64) -> Result<bool, Error> {
    match _get_smart_storage_item(&id) {
        Some(item) => Ok(item.is_available),
        None => Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn mark_item_as_available(id: u64) -> Result<SmartStorageItem, Error> {
    match STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(mut item) => {
            item.is_available = true;
            do_insert_smart_storage_item(&item);
            Ok(item.clone())
        }
        None => Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn mark_item_as_unavailable(id: u64) -> Result<SmartStorageItem, Error> {
    if let Some(mut item) = STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        item.is_available = false;
        do_insert_smart_storage_item(&item);
        Ok(item.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("an item with id={} not found", id),
        })
    }
}

fn do_insert_smart_storage_item(item: &SmartStorageItem) {
    STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().insert(item.id, item.clone()));
}

#[ic_cdk::update]
fn delete_smart_storage_item(id: u64) -> Result<SmartStorageItem, Error> {
    match STORAGE_ITEM_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete an item with id={}. item not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

fn _get_smart_storage_item(id: &u64) -> Option<SmartStorageItem> {
    // Assuming MemoryId::new(1) is reserved for smart storage item storage
    let item_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)));
    StableBTreeMap::<u64, SmartStorageItem, Memory>::init(item_storage)
        .borrow()
        .get(id)
}

ic_cdk::export_candid!();
