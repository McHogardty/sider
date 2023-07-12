use std::{collections::{HashMap, VecDeque, hash_map::Entry}, rc::Rc};

use chrono::{DateTime, Utc};

use crate::util::from_decimal_bytes;


#[derive(Debug, PartialEq)]
pub enum DBString {
    Integer(i64),
    String(Vec<u8>)
}

impl DBString {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::String(v) => v.to_vec(),
            Self::Integer(i) => i.to_string().into_bytes(),
        }
    }

    pub fn from_bytes(b: Vec<u8>) -> Self {
        Self::String(b)
    }

    pub fn incr_by(&mut self, n: i64) -> Result<i64, ()> {
        let mut i = match self {
            Self::Integer(i) => *i,
            Self::String(s) => {
                from_decimal_bytes(s)?
            }
        };


        i += n;

        *self = Self::Integer(i);

        Ok(i)
    }

}


/// The underlying database implementation.
/// 
/// Each key/value pair in the database is stored as a mapping from a String to a DBEntry.
/// A DB entry contains about the value, as well as the value itself.
/// 
/// The database also maintains a hashset of keys that point to values with expiry times.
/// This allows us to expire keys in the background on a regular basis, rather than only
/// expiring keys on access. This avoids the problem where the expiry is set for a key
/// and the key is never accessed again, as it would have otherwise not been removed from
/// the database.
/// 
/// The intention is that the database owns all data that is passed to it. If a value is to be
/// accessed, the database will return a non-mutable reference to that value. As an optimisation,
/// all keys and values are stored inside the Rc type. This allows us to store pointers to the
/// same key in both the backing hash map as well as the hash table of expiring entries. Similarly,
/// it allows us to return references to values inside the map without maintaining a reference to
/// the map itself, which is important for satisfying the borrow checking rules when doing things
/// like checking value expiry.
#[derive(Debug)]
pub struct DB {
    /// The money. This map stores all of the data that is stored in the database.
    map: HashMap<Rc<Vec<u8>>, DBEntry>,
    /// Maintains track of all of the key/value pairs in the map which have expiry
    /// values set.
    expiring_entries: HashMap<Rc<Vec<u8>>, DateTime<Utc>>,
}


#[derive(Debug)]
pub enum DBError {
    AlreadyExists,
    DoesNotExist,
    WrongType,
}


/// An entry in the database.
#[derive(Debug, PartialEq)]
pub enum DBEntry {
    Nil,
    String(DBString),
    List(VecDeque<Vec<u8>>),
}


impl DBEntry {
    pub fn get_string(&self) -> Result<&DBString, DBError> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(DBError::WrongType),
        }
    }

    pub fn get_mut_string(&mut self) -> Result<&mut DBString, DBError> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(DBError::WrongType),
        }
    }

    pub fn set_string(&mut self, v: DBString) {
        *self = Self::String(v);
    }

    pub fn get_list(&mut self) -> Result<&mut VecDeque<Vec<u8>>, DBError> {
        match self {
            Self::List(l) => Ok(l),
            _ => Err(DBError::WrongType)
        }
    }

    pub fn set_list(&mut self, l: VecDeque<Vec<u8>>) {
        *self = Self::List(l);
    }

    pub fn is_nil(&self) -> bool {
        self == &Self::Nil
    }
}

/// The ExpiryFlag enum is used to indicate expiry settings when setting a value in
/// the database.
#[derive(PartialEq)]
pub enum ExpiryFlag {
    /// The value does not expire.
    None,
    /// If the value already exists, keep the current TTL of the value. If the value
    /// does not exist, it will not expire.
    KeepTTL,
    /// A datetime in the future at which the value should expire.
    Some(DateTime<Utc>)
}

impl ExpiryFlag {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }
}


/// The ExistenceFlag enum is used to indicate whether or not existence conditions should
/// be asserted when setting a key/value pair.
#[derive(PartialEq, Eq)]
pub enum ExistenceFlag {
    /// Assert that the key does not already exist in the database.
    Nx,
    /// Assert that the key already exists in the database.
    Xx,
    /// Do not make any existence assertions when setting a key/value pair.
    None,
}

impl ExistenceFlag {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }
}


impl DB {
    /// Construct a new instance of the database. Should only be required on startup.
    pub fn new() -> Self {
        DB {
            map: HashMap::new(),
            expiring_entries: HashMap::new(),
        }
    }

    /// Determine whether or not a key exists in the database. Returns a boolean indicating
    /// whether or not this is the case.
    pub fn exists(&self, key: &Vec<u8>) -> bool {
        self.map.contains_key(key)
    }

    /// Delete a key from the database. Returns a boolean indicating whether or not the key
    /// actually existed.
    pub fn delete(&mut self, key: &Vec<u8>) -> bool {
        self.expiring_entries.remove(key);
        self.map.remove(key).is_some()
    }

    pub fn get(&mut self, key: &Vec<u8>) -> Option<&DBEntry> {
        if let Some(e) = self.expiring_entries.get(key) {
            if e <= &Utc::now() {
                self.expiring_entries.remove(key);
                self.map.remove(key);
                return None;
            }
        };

        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &Vec<u8>) -> Option<&mut DBEntry> {
        if let Some(e) = self.expiring_entries.get(key) {
            if e <= &Utc::now() {
                self.expiring_entries.remove(key);
                self.map.remove(key);
                return None;
            }
        };

        self.map.get_mut(key)

    }

    pub fn get_or_insert(&mut self, key: Vec<u8>, expiry: ExpiryFlag, existence_check: ExistenceFlag) -> Result<&mut DBEntry, DBError> {
        let k = Rc::new(key);

        match expiry {
            ExpiryFlag::KeepTTL => {},
            ExpiryFlag::None => {
                self.expiring_entries.remove(&k);
            },
            ExpiryFlag::Some(ex) => {
                self.expiring_entries.insert(Rc::clone(&k), ex);
            }
        }
        
        match self.map.entry(k) {
            Entry::Occupied(e) => {
                if existence_check == ExistenceFlag::Nx {
                    return Err(DBError::AlreadyExists);
                }

                return Ok(e.into_mut());
            },
            Entry::Vacant(e) => {
                if existence_check == ExistenceFlag::Xx {
                    return Err(DBError::DoesNotExist);
                }

                let v = DBEntry::Nil;

                Ok(e.insert(v))
            },
        }
    }

    pub fn expire_keys(&mut self) {
        let now = Utc::now();
        
        self.expiring_entries.retain(|k, e| {
            if *e < now {
                println!("Removing key from map {:?}", k);
                self.map.remove(k);
                false
            } else {
                true
            }
        });
    }
}