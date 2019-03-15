/*
 * Copyright 2019 OysterPack Inc.
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

// This file is generated by rust-protobuf 2.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub ulid: ::protobuf::SingularPtrField<ULID>,
    pub futures_version: Request_Futures,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    // .oysterpack_trust_grpc.protos.foo.ULID ulid = 1;

    pub fn clear_ulid(&mut self) {
        self.ulid.clear();
    }

    pub fn has_ulid(&self) -> bool {
        self.ulid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ulid(&mut self, v: ULID) {
        self.ulid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ulid(&mut self) -> &mut ULID {
        if self.ulid.is_none() {
            self.ulid.set_default();
        }
        self.ulid.as_mut().unwrap()
    }

    // Take field
    pub fn take_ulid(&mut self) -> ULID {
        self.ulid.take().unwrap_or_else(|| ULID::new())
    }

    pub fn get_ulid(&self) -> &ULID {
        self.ulid.as_ref().unwrap_or_else(|| ULID::default_instance())
    }

    // .oysterpack_trust_grpc.protos.foo.Request.Futures futures_version = 2;

    pub fn clear_futures_version(&mut self) {
        self.futures_version = Request_Futures::ONE;
    }

    // Param is passed by value, moved
    pub fn set_futures_version(&mut self, v: Request_Futures) {
        self.futures_version = v;
    }

    pub fn get_futures_version(&self) -> Request_Futures {
        self.futures_version
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        for v in &self.ulid {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ulid)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.futures_version, 2, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.ulid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.futures_version != Request_Futures::ONE {
            my_size += ::protobuf::rt::enum_size(2, self.futures_version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ulid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.futures_version != Request_Futures::ONE {
            os.write_enum(2, self.futures_version.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ULID>>(
                    "ulid",
                    |m: &Request| { &m.ulid },
                    |m: &mut Request| { &mut m.ulid },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Request_Futures>>(
                    "futures_version",
                    |m: &Request| { &m.futures_version },
                    |m: &mut Request| { &mut m.futures_version },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_ulid();
        self.clear_futures_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Request_Futures {
    ONE = 0,
    THREE = 1,
}

impl ::protobuf::ProtobufEnum for Request_Futures {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Request_Futures> {
        match value {
            0 => ::std::option::Option::Some(Request_Futures::ONE),
            1 => ::std::option::Option::Some(Request_Futures::THREE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Request_Futures] = &[
            Request_Futures::ONE,
            Request_Futures::THREE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Request_Futures", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Request_Futures {
}

impl ::std::default::Default for Request_Futures {
    fn default() -> Self {
        Request_Futures::ONE
    }
}

impl ::protobuf::reflect::ProtobufValue for Request_Futures {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub ulid: ::protobuf::SingularPtrField<ULID>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    // .oysterpack_trust_grpc.protos.foo.ULID ulid = 1;

    pub fn clear_ulid(&mut self) {
        self.ulid.clear();
    }

    pub fn has_ulid(&self) -> bool {
        self.ulid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ulid(&mut self, v: ULID) {
        self.ulid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ulid(&mut self) -> &mut ULID {
        if self.ulid.is_none() {
            self.ulid.set_default();
        }
        self.ulid.as_mut().unwrap()
    }

    // Take field
    pub fn take_ulid(&mut self) -> ULID {
        self.ulid.take().unwrap_or_else(|| ULID::new())
    }

    pub fn get_ulid(&self) -> &ULID {
        self.ulid.as_ref().unwrap_or_else(|| ULID::default_instance())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        for v in &self.ulid {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ulid)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.ulid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ulid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ULID>>(
                    "ulid",
                    |m: &Response| { &m.ulid },
                    |m: &mut Response| { &mut m.ulid },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_ulid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ULID {
    // message fields
    pub ulid_1: u64,
    pub ulid_2: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ULID {
    pub fn new() -> ULID {
        ::std::default::Default::default()
    }

    // uint64 ulid_1 = 1;

    pub fn clear_ulid_1(&mut self) {
        self.ulid_1 = 0;
    }

    // Param is passed by value, moved
    pub fn set_ulid_1(&mut self, v: u64) {
        self.ulid_1 = v;
    }

    pub fn get_ulid_1(&self) -> u64 {
        self.ulid_1
    }

    // uint64 ulid_2 = 2;

    pub fn clear_ulid_2(&mut self) {
        self.ulid_2 = 0;
    }

    // Param is passed by value, moved
    pub fn set_ulid_2(&mut self, v: u64) {
        self.ulid_2 = v;
    }

    pub fn get_ulid_2(&self) -> u64 {
        self.ulid_2
    }
}

impl ::protobuf::Message for ULID {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ulid_1 = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ulid_2 = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.ulid_1 != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ulid_1, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ulid_2 != 0 {
            my_size += ::protobuf::rt::value_size(2, self.ulid_2, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ulid_1 != 0 {
            os.write_uint64(1, self.ulid_1)?;
        }
        if self.ulid_2 != 0 {
            os.write_uint64(2, self.ulid_2)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ULID {
        ULID::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ulid_1",
                    |m: &ULID| { &m.ulid_1 },
                    |m: &mut ULID| { &mut m.ulid_1 },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ulid_2",
                    |m: &ULID| { &m.ulid_2 },
                    |m: &mut ULID| { &mut m.ulid_2 },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ULID>(
                    "ULID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ULID {
        static mut instance: ::protobuf::lazy::Lazy<ULID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ULID,
        };
        unsafe {
            instance.get(ULID::new)
        }
    }
}

impl ::protobuf::Clear for ULID {
    fn clear(&mut self) {
        self.clear_ulid_1();
        self.clear_ulid_2();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ULID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ULID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tfoo.proto\x12\x20oysterpack_trust_grpc.protos.foo\x1a\x19google/prot\
    obuf/any.proto\"\xc0\x01\n\x07Request\x12:\n\x04ulid\x18\x01\x20\x01(\
    \x0b2&.oysterpack_trust_grpc.protos.foo.ULIDR\x04ulid\x12Z\n\x0ffutures_\
    version\x18\x02\x20\x01(\x0e21.oysterpack_trust_grpc.protos.foo.Request.\
    FuturesR\x0efuturesVersion\"\x1d\n\x07Futures\x12\x07\n\x03ONE\x10\0\x12\
    \t\n\x05THREE\x10\x01\"F\n\x08Response\x12:\n\x04ulid\x18\x01\x20\x01(\
    \x0b2&.oysterpack_trust_grpc.protos.foo.ULIDR\x04ulid\"4\n\x04ULID\x12\
    \x15\n\x06ulid_1\x18\x01\x20\x01(\x04R\x05ulid1\x12\x15\n\x06ulid_2\x18\
    \x02\x20\x01(\x04R\x05ulid22\xb4\x03\n\x03Foo\x12`\n\x05unary\x12).oyste\
    rpack_trust_grpc.protos.foo.Request\x1a*.oysterpack_trust_grpc.protos.fo\
    o.Response\"\0\x12m\n\x10client_streaming\x12).oysterpack_trust_grpc.pro\
    tos.foo.Request\x1a*.oysterpack_trust_grpc.protos.foo.Response\"\0(\x01\
    \x12m\n\x10server_streaming\x12).oysterpack_trust_grpc.protos.foo.Reques\
    t\x1a*.oysterpack_trust_grpc.protos.foo.Response\"\00\x01\x12m\n\x0ebidi\
    _streaming\x12).oysterpack_trust_grpc.protos.foo.Request\x1a*.oysterpack\
    _trust_grpc.protos.foo.Response\"\0(\x010\x01B\x02H\x01b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
