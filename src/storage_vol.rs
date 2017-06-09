/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

#![allow(improper_ctypes)]

extern crate libc;

use std::str;

use connect::sys::virConnectPtr;
use storage_pool::sys::virStoragePoolPtr;
use stream::sys::virStreamPtr;

use connect::Connect;
use error::Error;
use storage_pool::StoragePool;
use stream::Stream;

pub mod sys {
    extern crate libc;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virStorageVol {}

    #[allow(non_camel_case_types)]
    pub type virStorageVolPtr = *mut virStorageVol;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    #[derive(Default)]
    pub struct virStorageVolInfo {
        pub kind: libc::c_int,
        pub capacity: libc::c_ulonglong,
        pub allocation: libc::c_ulonglong,
    }

    #[allow(non_camel_case_types)]
    pub type virStorageVolInfoPtr = *mut virStorageVolInfo;

}

#[link(name = "virt")]
extern "C" {
    fn virStorageVolCreateXML(p: virStoragePoolPtr,
                              xml: *const libc::c_char,
                              flags: libc::c_uint)
                              -> sys::virStorageVolPtr;
    fn virStorageVolCreateXMLFrom(p: virStoragePoolPtr,
                                  xml: *const libc::c_char,
                                  from: sys::virStorageVolPtr,
                                  flags: libc::c_uint)
                                  -> sys::virStorageVolPtr;
    fn virStorageVolLookupByName(p: virStoragePoolPtr,
                                 id: *const libc::c_char)
                                 -> sys::virStorageVolPtr;
    fn virStorageVolLookupByKey(c: virConnectPtr,
                                id: *const libc::c_char)
                                -> sys::virStorageVolPtr;
    fn virStorageVolLookupByPath(c: virConnectPtr,
                                 id: *const libc::c_char)
                                 -> sys::virStorageVolPtr;
    fn virStorageVolGetName(ptr: sys::virStorageVolPtr) -> *const libc::c_char;
    fn virStorageVolGetKey(ptr: sys::virStorageVolPtr) -> *const libc::c_char;
    fn virStorageVolGetPath(ptr: sys::virStorageVolPtr) -> *mut libc::c_char;
    fn virStorageVolDelete(ptr: sys::virStorageVolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStorageVolWipe(ptr: sys::virStorageVolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStorageVolWipePattern(ptr: sys::virStorageVolPtr,
                                algo: libc::c_uint,
                                flags: libc::c_uint)
                                -> libc::c_int;
    fn virStorageVolFree(ptr: sys::virStorageVolPtr) -> libc::c_int;
    fn virStorageVolGetXMLDesc(ptr: sys::virStorageVolPtr,
                               flags: libc::c_uint)
                               -> *mut libc::c_char;
    fn virStorageVolGetConnect(ptr: sys::virStorageVolPtr) -> virConnectPtr;
    fn virStorageVolResize(ptr: sys::virStorageVolPtr,
                           capacity: libc::c_ulonglong,
                           flags: libc::c_uint)
                           -> libc::c_int;

    fn virStorageVolGetInfo(ptr: sys::virStorageVolPtr,
                            info: sys::virStorageVolInfoPtr)
                            -> libc::c_int;
    fn virStorageVolGetInfoFlags(ptr: sys::virStorageVolPtr,
                                 info: sys::virStorageVolInfoPtr,
                                 flags: libc::c_uint)
                                 -> libc::c_int;
    fn virStorageVolDownload(ptr: sys::virStorageVolPtr,
                             stream: virStreamPtr,
                             offset: libc::c_ulonglong,
                             length: libc::c_ulonglong,
                             flags: libc::c_uint)
                             -> libc::c_int;
    fn virStorageVolUpload(ptr: sys::virStorageVolPtr,
                           stream: virStreamPtr,
                           offset: libc::c_ulonglong,
                           length: libc::c_ulonglong,
                           flags: libc::c_uint)
                           -> libc::c_int;
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateFlags {
    NORMAL = 0,
    PREALLOC_METADATA = 1,
    /// Perform a btrfs lightweight copy.
    REFLINK = 2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResizeFlags {
    NORMAL = 0,
    /// Force allocation of new size.
    ALLOCATE = 1,
    /// Size is relative to current.
    DELTA = 2,
    /// Allow decrease in capacity.
    SHRINK = 3,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WipeAlgorithm {
    ZERO = 0,
    NNSA = 1,
    DOD = 2,
    BSI = 3,
    GUTMANN = 4,
    SCHNEIER = 5,
    PFITZNER7 = 6,
    PFITZNER33 = 7,
    RANDOM = 8,
    TRIM = 9,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    FILE = 0,
    BLOCK = 1,
    DIR = 2,
    NETWORK = 3,
    NETDIR = 4,
    PLOOP = 5,
}

// TODO(sahid): Should provide a procedural macro
impl ::std::convert::From<u32> for Type {
    fn from(value: u32) -> Type {
        match value {
            0 => Type::FILE,
            1 => Type::BLOCK,
            2 => Type::DIR,
            3 => Type::NETWORK,
            4 => Type::NETDIR,
            5 => Type::PLOOP,
            unknow => panic!("Invalid Type provided: {:?}", unknow)
        }
    }
}

#[derive(Clone, Debug)]
pub struct StorageVolInfo {
    /// See: `virType` flags
    pub kind: Type,
    /// Logical size bytes.
    pub capacity: u64,
    /// Current allocation bytes
    pub allocation: u64,
}

impl StorageVolInfo {
    pub fn from_ptr(ptr: sys::virStorageVolInfoPtr) -> StorageVolInfo {
        unsafe {
            StorageVolInfo {
                kind: Type::from((*ptr).kind as u32),
                capacity: (*ptr).capacity as u64,
                allocation: (*ptr).allocation as u64,
            }
        }
    }
}

/// Provides APIs for the management of storage volumes.
///
/// See http://libvirt.org/html/libvirt-libvirt-storage.html
#[derive(Debug)]
pub struct StorageVol {
    ptr: Option<sys::virStorageVolPtr>,
}

impl Drop for StorageVol {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for StorageVol, code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }
}

impl StorageVol {
    pub fn new(ptr: sys::virStorageVolPtr) -> StorageVol {
        return StorageVol { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virStorageVolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virStorageVolGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn create_xml(pool: &StoragePool,
                      xml: &str,
                      flags: CreateFlags)
                      -> Result<StorageVol, Error> {
        unsafe {
            let ptr = virStorageVolCreateXML(pool.as_ptr(),
                                             string_to_c_chars!(xml),
                                             flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn create_xml_from(pool: &StoragePool,
                           xml: &str,
                           vol: &StorageVol,
                           flags: CreateFlags)
                           -> Result<StorageVol, Error> {
        unsafe {
            let ptr = virStorageVolCreateXMLFrom(pool.as_ptr(),
                                                 string_to_c_chars!(xml),
                                                 vol.as_ptr(),
                                                 flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn lookup_by_name(pool: &StoragePool, name: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = virStorageVolLookupByName(pool.as_ptr(), string_to_c_chars!(name));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn lookup_by_key(conn: &Connect, key: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = virStorageVolLookupByKey(conn.as_ptr(), string_to_c_chars!(key));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn lookup_by_path(conn: &Connect, path: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = virStorageVolLookupByPath(conn.as_ptr(), string_to_c_chars!(path));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virStorageVolGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_key(&self) -> Result<String, Error> {
        unsafe {
            let n = virStorageVolGetKey(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let n = virStorageVolGetPath(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = virStorageVolGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if virStorageVolDelete(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn wipe(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if virStorageVolWipe(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn wipe_pattern(&self, algo: WipeAlgorithm, flags: u32) -> Result<(), Error> {
        unsafe {
            if virStorageVolWipePattern(self.as_ptr(),
                                        algo as libc::c_uint,
                                        flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virStorageVolFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn resize(&self, capacity: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virStorageVolResize(self.as_ptr(),
                                          capacity as libc::c_ulonglong,
                                          flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_info(&self) -> Result<StorageVolInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virStorageVolInfo::default();
            let res = virStorageVolGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(StorageVolInfo::from_ptr(pinfo));
        }
    }

    pub fn get_info_flags(&self, flags: u32) -> Result<StorageVolInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virStorageVolInfo::default();
            let res = virStorageVolGetInfoFlags(self.as_ptr(), pinfo, flags as libc::c_uint);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(StorageVolInfo::from_ptr(pinfo));
        }
    }

    pub fn download(&self,
                    stream: &Stream,
                    offset: u64,
                    length: u64,
                    flags: u32)
                    -> Result<(), Error> {
        unsafe {
            let ret = virStorageVolDownload(self.as_ptr(),
                                            stream.as_ptr(),
                                            offset as libc::c_ulonglong,
                                            length as libc::c_ulonglong,
                                            flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn upload(&self,
                  stream: &Stream,
                  offset: u64,
                  length: u64,
                  flags: u32)
                  -> Result<(), Error> {
        unsafe {
            let ret = virStorageVolUpload(self.as_ptr(),
                                          stream.as_ptr(),
                                          offset as libc::c_ulonglong,
                                          length as libc::c_ulonglong,
                                          flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
