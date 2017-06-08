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

pub mod sys {
    extern crate libc;

    use std;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    #[derive(Copy)]
    pub struct virTypedParameter {
        pub field: [libc::c_char; 80usize],
        pub typed: libc::c_int,
        pub value: libc::c_ulonglong,
    }

    impl std::clone::Clone for virTypedParameter {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl std::default::Default for virTypedParameter {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    }

    #[allow(non_camel_case_types)]
    pub type virTypedParameterPtr = *mut virTypedParameter;
}

virt_enum! {
    TypedParameterType {
        /// Int
        Int -> 1,
        /// Uint
        Uint -> 2,
        /// Llong
        Llong -> 3,
        /// Ullong
        Ullong -> 4,
        /// Double
        Double -> 5,
        /// Boolean
        Boolean -> 6,
        /// String
        String -> 7,
    }
}
