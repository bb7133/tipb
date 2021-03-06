// This file is generated. Do not edit
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
pub struct ChecksumRequest {
    // message fields
    start_ts: ::std::option::Option<u64>,
    scan_on: ::std::option::Option<ChecksumScanOn>,
    algorithm: ::std::option::Option<ChecksumAlgorithm>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChecksumRequest {}

impl ChecksumRequest {
    pub fn new() -> ChecksumRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChecksumRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumRequest,
        };
        unsafe {
            instance.get(ChecksumRequest::new)
        }
    }

    // optional uint64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None;
    }

    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts.unwrap_or(0)
    }

    fn get_start_ts_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_ts
    }

    // optional .tipb.ChecksumScanOn scan_on = 2;

    pub fn clear_scan_on(&mut self) {
        self.scan_on = ::std::option::Option::None;
    }

    pub fn has_scan_on(&self) -> bool {
        self.scan_on.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scan_on(&mut self, v: ChecksumScanOn) {
        self.scan_on = ::std::option::Option::Some(v);
    }

    pub fn get_scan_on(&self) -> ChecksumScanOn {
        self.scan_on.unwrap_or(ChecksumScanOn::Table)
    }

    fn get_scan_on_for_reflect(&self) -> &::std::option::Option<ChecksumScanOn> {
        &self.scan_on
    }

    fn mut_scan_on_for_reflect(&mut self) -> &mut ::std::option::Option<ChecksumScanOn> {
        &mut self.scan_on
    }

    // optional .tipb.ChecksumAlgorithm algorithm = 3;

    pub fn clear_algorithm(&mut self) {
        self.algorithm = ::std::option::Option::None;
    }

    pub fn has_algorithm(&self) -> bool {
        self.algorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_algorithm(&mut self, v: ChecksumAlgorithm) {
        self.algorithm = ::std::option::Option::Some(v);
    }

    pub fn get_algorithm(&self) -> ChecksumAlgorithm {
        self.algorithm.unwrap_or(ChecksumAlgorithm::Crc64_Xor)
    }

    fn get_algorithm_for_reflect(&self) -> &::std::option::Option<ChecksumAlgorithm> {
        &self.algorithm
    }

    fn mut_algorithm_for_reflect(&mut self) -> &mut ::std::option::Option<ChecksumAlgorithm> {
        &mut self.algorithm
    }
}

impl ::protobuf::Message for ChecksumRequest {
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
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.scan_on, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.algorithm, 3, &mut self.unknown_fields)?
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
        if let Some(v) = self.start_ts {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.scan_on {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.algorithm {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_ts {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.scan_on {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.algorithm {
            os.write_enum(3, v.value())?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChecksumRequest {
    fn new() -> ChecksumRequest {
        ChecksumRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChecksumRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    ChecksumRequest::get_start_ts_for_reflect,
                    ChecksumRequest::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChecksumScanOn>>(
                    "scan_on",
                    ChecksumRequest::get_scan_on_for_reflect,
                    ChecksumRequest::mut_scan_on_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChecksumAlgorithm>>(
                    "algorithm",
                    ChecksumRequest::get_algorithm_for_reflect,
                    ChecksumRequest::mut_algorithm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumRequest>(
                    "ChecksumRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChecksumRequest {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_scan_on();
        self.clear_algorithm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChecksumRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChecksumResponse {
    // message fields
    checksum: ::std::option::Option<u64>,
    total_kvs: ::std::option::Option<u64>,
    total_bytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChecksumResponse {}

impl ChecksumResponse {
    pub fn new() -> ChecksumResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChecksumResponse {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumResponse,
        };
        unsafe {
            instance.get(ChecksumResponse::new)
        }
    }

    // optional uint64 checksum = 1;

    pub fn clear_checksum(&mut self) {
        self.checksum = ::std::option::Option::None;
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: u64) {
        self.checksum = ::std::option::Option::Some(v);
    }

    pub fn get_checksum(&self) -> u64 {
        self.checksum.unwrap_or(0)
    }

    fn get_checksum_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.checksum
    }

    // optional uint64 total_kvs = 2;

    pub fn clear_total_kvs(&mut self) {
        self.total_kvs = ::std::option::Option::None;
    }

    pub fn has_total_kvs(&self) -> bool {
        self.total_kvs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_kvs(&mut self, v: u64) {
        self.total_kvs = ::std::option::Option::Some(v);
    }

    pub fn get_total_kvs(&self) -> u64 {
        self.total_kvs.unwrap_or(0)
    }

    fn get_total_kvs_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_kvs
    }

    fn mut_total_kvs_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_kvs
    }

    // optional uint64 total_bytes = 3;

    pub fn clear_total_bytes(&mut self) {
        self.total_bytes = ::std::option::Option::None;
    }

    pub fn has_total_bytes(&self) -> bool {
        self.total_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_bytes(&mut self, v: u64) {
        self.total_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_total_bytes(&self) -> u64 {
        self.total_bytes.unwrap_or(0)
    }

    fn get_total_bytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_bytes
    }

    fn mut_total_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_bytes
    }
}

impl ::protobuf::Message for ChecksumResponse {
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
                    self.checksum = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_kvs = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_bytes = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.checksum {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.total_kvs {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.total_bytes {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.checksum {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.total_kvs {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.total_bytes {
            os.write_uint64(3, v)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChecksumResponse {
    fn new() -> ChecksumResponse {
        ChecksumResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChecksumResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "checksum",
                    ChecksumResponse::get_checksum_for_reflect,
                    ChecksumResponse::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_kvs",
                    ChecksumResponse::get_total_kvs_for_reflect,
                    ChecksumResponse::mut_total_kvs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_bytes",
                    ChecksumResponse::get_total_bytes_for_reflect,
                    ChecksumResponse::mut_total_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumResponse>(
                    "ChecksumResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChecksumResponse {
    fn clear(&mut self) {
        self.clear_checksum();
        self.clear_total_kvs();
        self.clear_total_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChecksumResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumScanOn {
    Table = 0,
    Index = 1,
}

impl ::protobuf::ProtobufEnum for ChecksumScanOn {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumScanOn> {
        match value {
            0 => ::std::option::Option::Some(ChecksumScanOn::Table),
            1 => ::std::option::Option::Some(ChecksumScanOn::Index),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChecksumScanOn] = &[
            ChecksumScanOn::Table,
            ChecksumScanOn::Index,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ChecksumScanOn>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumScanOn", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumScanOn {
}

impl ::protobuf::reflect::ProtobufValue for ChecksumScanOn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumAlgorithm {
    Crc64_Xor = 0,
}

impl ::protobuf::ProtobufEnum for ChecksumAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(ChecksumAlgorithm::Crc64_Xor),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChecksumAlgorithm] = &[
            ChecksumAlgorithm::Crc64_Xor,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ChecksumAlgorithm>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumAlgorithm", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumAlgorithm {
}

impl ::protobuf::reflect::ProtobufValue for ChecksumAlgorithm {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0echecksum.proto\x12\x04tipb\x1a\x14gogoproto/gogo.proto\"\xa4\x01\n\
    \x0fChecksumRequest\x12\x1f\n\x08start_ts\x18\x01\x20\x01(\x04R\x07start\
    TsB\x04\xc8\xde\x1f\0\x123\n\x07scan_on\x18\x02\x20\x01(\x0e2\x14.tipb.C\
    hecksumScanOnR\x06scanOnB\x04\xc8\xde\x1f\0\x12;\n\talgorithm\x18\x03\
    \x20\x01(\x0e2\x17.tipb.ChecksumAlgorithmR\talgorithmB\x04\xc8\xde\x1f\0\
    \"~\n\x10ChecksumResponse\x12\x20\n\x08checksum\x18\x01\x20\x01(\x04R\
    \x08checksumB\x04\xc8\xde\x1f\0\x12!\n\ttotal_kvs\x18\x02\x20\x01(\x04R\
    \x08totalKvsB\x04\xc8\xde\x1f\0\x12%\n\x0btotal_bytes\x18\x03\x20\x01(\
    \x04R\ntotalBytesB\x04\xc8\xde\x1f\0*&\n\x0eChecksumScanOn\x12\t\n\x05Ta\
    ble\x10\0\x12\t\n\x05Index\x10\x01*\"\n\x11ChecksumAlgorithm\x12\r\n\tCr\
    c64_Xor\x10\0B%\n\x15com.pingcap.tidb.tipbP\x01\xe0\xe2\x1e\x01\xc8\xe2\
    \x1e\x01\xd0\xe2\x1e\x01J\xb6\r\n\x06\x12\x04\0\0\x20\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0c\n\x08\n\x01\x08\
    \x12\x03\x04\0\"\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x04\0\"\n\x0c\n\x05\
    \x08\xe7\x07\0\x02\x12\x03\x04\x07\x1a\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\
    \x03\x04\x07\x1a\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x04\x07\x1a\
    \n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x04\x1d!\n\x08\n\x01\x08\x12\x03\
    \x05\0.\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x05\0.\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x05\x07\x13\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x05\x07\x13\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x05\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x05\x16-\n\t\n\x02\x03\0\x12\x03\
    \x07\x07\x1d\n\x08\n\x01\x08\x12\x03\t\0$\n\x0b\n\x04\x08\xe7\x07\x02\
    \x12\x03\t\0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\t\x07\x1c\n\r\n\
    \x06\x08\xe7\x07\x02\x02\0\x12\x03\t\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\
    \x02\0\x01\x12\x03\t\x08\x1b\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\t\
    \x1f#\n\x08\n\x01\x08\x12\x03\n\0(\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\n\
    \0(\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\n\x07\x20\n\r\n\x06\x08\xe7\
    \x07\x03\x02\0\x12\x03\n\x07\x20\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\
    \x12\x03\n\x08\x1f\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\n#'\n\x08\n\
    \x01\x08\x12\x03\x0b\0*\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x0b\0*\n\x0c\
    \n\x05\x08\xe7\x07\x04\x02\x12\x03\x0b\x07\"\n\r\n\x06\x08\xe7\x07\x04\
    \x02\0\x12\x03\x0b\x07\"\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\
    \x0b\x08!\n\x0c\n\x05\x08\xe7\x07\x04\x03\x12\x03\x0b%)\n\n\n\x02\x05\0\
    \x12\x04\r\0\x10\x01\n\n\n\x03\x05\0\x01\x12\x03\r\x05\x13\n\x0b\n\x04\
    \x05\0\x02\0\x12\x03\x0e\x04\x0e\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x0e\
    \x04\t\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x0e\x0c\r\n\x0b\n\x04\x05\0\
    \x02\x01\x12\x03\x0f\x04\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x0f\
    \x04\t\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x0f\x0c\r\n\n\n\x02\x05\x01\
    \x12\x04\x12\0\x14\x01\n\n\n\x03\x05\x01\x01\x12\x03\x12\x05\x16\n\x0b\n\
    \x04\x05\x01\x02\0\x12\x03\x13\x04\x12\n\x0c\n\x05\x05\x01\x02\0\x01\x12\
    \x03\x13\x04\r\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x13\x10\x11\n\n\n\
    \x02\x04\0\x12\x04\x16\0\x1a\x01\n\n\n\x03\x04\0\x01\x12\x03\x16\x08\x17\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x17\x04@\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x17\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x17\r\x13\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x17\x14\x1c\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x17\x1f\x20\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x17!?\n\x0f\n\
    \x08\x04\0\x02\0\x08\xe7\x07\0\x12\x03\x17\">\n\x10\n\t\x04\0\x02\0\x08\
    \xe7\x07\0\x02\x12\x03\x17\"6\n\x11\n\n\x04\0\x02\0\x08\xe7\x07\0\x02\0\
    \x12\x03\x17\"6\n\x12\n\x0b\x04\0\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x17#5\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x03\x12\x03\x179>\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x18\x04G\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\
    \x18\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x18\r\x1b\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x18\x1c#\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x18&'\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x18(F\n\x0f\n\x08\x04\0\
    \x02\x01\x08\xe7\x07\0\x12\x03\x18)E\n\x10\n\t\x04\0\x02\x01\x08\xe7\x07\
    \0\x02\x12\x03\x18)=\n\x11\n\n\x04\0\x02\x01\x08\xe7\x07\0\x02\0\x12\x03\
    \x18)=\n\x12\n\x0b\x04\0\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x03\x18*<\n\
    \x10\n\t\x04\0\x02\x01\x08\xe7\x07\0\x03\x12\x03\x18@E\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\x19\x04L\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x19\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x19\r\x1e\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03\x19\x1f(\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x19+\
    ,\n\x0c\n\x05\x04\0\x02\x02\x08\x12\x03\x19-K\n\x0f\n\x08\x04\0\x02\x02\
    \x08\xe7\x07\0\x12\x03\x19.J\n\x10\n\t\x04\0\x02\x02\x08\xe7\x07\0\x02\
    \x12\x03\x19.B\n\x11\n\n\x04\0\x02\x02\x08\xe7\x07\0\x02\0\x12\x03\x19.B\
    \n\x12\n\x0b\x04\0\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\x03\x19/A\n\x10\n\
    \t\x04\0\x02\x02\x08\xe7\x07\0\x03\x12\x03\x19EJ\n\n\n\x02\x04\x01\x12\
    \x04\x1c\0\x20\x01\n\n\n\x03\x04\x01\x01\x12\x03\x1c\x08\x18\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x1d\x04@\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\
    \x1d\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x1d\r\x13\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x1d\x14\x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x1d\x1f\x20\n\x0c\n\x05\x04\x01\x02\0\x08\x12\x03\x1d!?\n\x0f\n\x08\
    \x04\x01\x02\0\x08\xe7\x07\0\x12\x03\x1d\">\n\x10\n\t\x04\x01\x02\0\x08\
    \xe7\x07\0\x02\x12\x03\x1d\"6\n\x11\n\n\x04\x01\x02\0\x08\xe7\x07\0\x02\
    \0\x12\x03\x1d\"6\n\x12\n\x0b\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x1d#5\n\x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x03\x12\x03\x1d9>\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x03\x1e\x04A\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03\x1e\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x1e\r\x13\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x1e\x14\x1d\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\x1e\x20!\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\
    \x1e\"@\n\x0f\n\x08\x04\x01\x02\x01\x08\xe7\x07\0\x12\x03\x1e#?\n\x10\n\
    \t\x04\x01\x02\x01\x08\xe7\x07\0\x02\x12\x03\x1e#7\n\x11\n\n\x04\x01\x02\
    \x01\x08\xe7\x07\0\x02\0\x12\x03\x1e#7\n\x12\n\x0b\x04\x01\x02\x01\x08\
    \xe7\x07\0\x02\0\x01\x12\x03\x1e$6\n\x10\n\t\x04\x01\x02\x01\x08\xe7\x07\
    \0\x03\x12\x03\x1e:?\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1f\x04C\n\x0c\
    \n\x05\x04\x01\x02\x02\x04\x12\x03\x1f\x04\x0c\n\x0c\n\x05\x04\x01\x02\
    \x02\x05\x12\x03\x1f\r\x13\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1f\
    \x14\x1f\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1f\"#\n\x0c\n\x05\x04\
    \x01\x02\x02\x08\x12\x03\x1f$B\n\x0f\n\x08\x04\x01\x02\x02\x08\xe7\x07\0\
    \x12\x03\x1f%A\n\x10\n\t\x04\x01\x02\x02\x08\xe7\x07\0\x02\x12\x03\x1f%9\
    \n\x11\n\n\x04\x01\x02\x02\x08\xe7\x07\0\x02\0\x12\x03\x1f%9\n\x12\n\x0b\
    \x04\x01\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\x03\x1f&8\n\x10\n\t\x04\x01\
    \x02\x02\x08\xe7\x07\0\x03\x12\x03\x1f<A\
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
