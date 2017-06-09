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

use error::Error;

pub mod sys {
    extern crate libc;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    pub struct virStream {}

    #[allow(non_camel_case_types)]
    pub type virStreamPtr = *mut virStream;
}

#[link(name = "virt")]
extern "C" {
    fn virStreamSend(c: sys::virStreamPtr,
                     data: *const libc::c_char,
                     nbytes: libc::c_uint)
                     -> libc::c_int;
    fn virStreamRecv(c: sys::virStreamPtr,
                     data: *mut libc::c_char,
                     nbytes: libc::c_uint)
                     -> libc::c_int;
    fn virStreamFree(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamAbort(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamFinish(c: sys::virStreamPtr) -> libc::c_int;
}

#[derive(Debug, PartialEq)]
pub enum EventType {
    READABLE = 1,
    WRITABLE = 2,
    ERROR    = 3,
    HANGUP   = 4,
}

#[derive(Debug)]
pub struct Stream {
    ptr: Option<sys::virStreamPtr>,
}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Stream, code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }
}

impl Stream {
    pub fn new(ptr: sys::virStreamPtr) -> Stream {
        Stream { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virStreamPtr {
        self.ptr.unwrap()
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virStreamFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn finish(self) -> Result<(), Error> {
        unsafe {
            if virStreamFinish(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn abort(self) -> Result<(), Error> {
        unsafe {
            if virStreamAbort(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn send(&self, data: &str) -> Result<u32, Error> {
        unsafe {
            let ret = virStreamSend(self.as_ptr(),
                                    string_to_c_chars!(data),
                                    data.len() as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn recv(&self, size: u32) -> Result<String, Error> {
        unsafe {
            let mut data: [libc::c_char; 2048] = ['\0' as i8; 2048];
            let ret = virStreamRecv(self.as_ptr(), data.as_mut_ptr(), size as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(data.as_ptr()));
        }
    }
}
