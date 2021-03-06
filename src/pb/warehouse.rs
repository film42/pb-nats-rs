// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `warehouse.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct Shipment {
    // message fields
    guid: ::protobuf::SingularField<::std::string::String>,
    address: ::protobuf::SingularField<::std::string::String>,
    price: ::std::option::Option<f64>,
    package_guid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Shipment {
    fn default() -> &'a Shipment {
        <Shipment as ::protobuf::Message>::default_instance()
    }
}

impl Shipment {
    pub fn new() -> Shipment {
        ::std::default::Default::default()
    }

    // optional string guid = 1;


    pub fn get_guid(&self) -> &str {
        match self.guid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_guid(&mut self) {
        self.guid.clear();
    }

    pub fn has_guid(&self) -> bool {
        self.guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guid(&mut self, v: ::std::string::String) {
        self.guid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guid(&mut self) -> &mut ::std::string::String {
        if self.guid.is_none() {
            self.guid.set_default();
        }
        self.guid.as_mut().unwrap()
    }

    // Take field
    pub fn take_guid(&mut self) -> ::std::string::String {
        self.guid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string address = 2;


    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional double price = 3;


    pub fn get_price(&self) -> f64 {
        self.price.unwrap_or(0.)
    }
    pub fn clear_price(&mut self) {
        self.price = ::std::option::Option::None;
    }

    pub fn has_price(&self) -> bool {
        self.price.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price(&mut self, v: f64) {
        self.price = ::std::option::Option::Some(v);
    }

    // optional string package_guid = 4;


    pub fn get_package_guid(&self) -> &str {
        match self.package_guid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_package_guid(&mut self) {
        self.package_guid.clear();
    }

    pub fn has_package_guid(&self) -> bool {
        self.package_guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_guid(&mut self, v: ::std::string::String) {
        self.package_guid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package_guid(&mut self) -> &mut ::std::string::String {
        if self.package_guid.is_none() {
            self.package_guid.set_default();
        }
        self.package_guid.as_mut().unwrap()
    }

    // Take field
    pub fn take_package_guid(&mut self) -> ::std::string::String {
        self.package_guid.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for Shipment {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.guid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.price = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.package_guid)?;
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
        if let Some(ref v) = self.guid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.address.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.price {
            my_size += 9;
        }
        if let Some(ref v) = self.package_guid.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.guid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.address.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.price {
            os.write_double(3, v)?;
        }
        if let Some(ref v) = self.package_guid.as_ref() {
            os.write_string(4, &v)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Shipment {
        Shipment::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guid",
                    |m: &Shipment| { &m.guid },
                    |m: &mut Shipment| { &mut m.guid },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    |m: &Shipment| { &m.address },
                    |m: &mut Shipment| { &mut m.address },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "price",
                    |m: &Shipment| { &m.price },
                    |m: &mut Shipment| { &mut m.price },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "package_guid",
                    |m: &Shipment| { &m.package_guid },
                    |m: &mut Shipment| { &mut m.package_guid },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Shipment>(
                    "Shipment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Shipment {
        static mut instance: ::protobuf::lazy::Lazy<Shipment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Shipment,
        };
        unsafe {
            instance.get(Shipment::new)
        }
    }
}

impl ::protobuf::Clear for Shipment {
    fn clear(&mut self) {
        self.guid.clear();
        self.address.clear();
        self.price = ::std::option::Option::None;
        self.package_guid.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Shipment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Shipment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShipmentRequest {
    // message fields
    guid: ::protobuf::RepeatedField<::std::string::String>,
    address: ::protobuf::RepeatedField<::std::string::String>,
    package_guid: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ShipmentRequest {
    fn default() -> &'a ShipmentRequest {
        <ShipmentRequest as ::protobuf::Message>::default_instance()
    }
}

impl ShipmentRequest {
    pub fn new() -> ShipmentRequest {
        ::std::default::Default::default()
    }

    // repeated string guid = 1;


    pub fn get_guid(&self) -> &[::std::string::String] {
        &self.guid
    }
    pub fn clear_guid(&mut self) {
        self.guid.clear();
    }

    // Param is passed by value, moved
    pub fn set_guid(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.guid = v;
    }

    // Mutable pointer to the field.
    pub fn mut_guid(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.guid
    }

    // Take field
    pub fn take_guid(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.guid, ::protobuf::RepeatedField::new())
    }

    // repeated string address = 2;


    pub fn get_address(&self) -> &[::std::string::String] {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    pub fn mut_address(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.address, ::protobuf::RepeatedField::new())
    }

    // repeated string package_guid = 3;


    pub fn get_package_guid(&self) -> &[::std::string::String] {
        &self.package_guid
    }
    pub fn clear_package_guid(&mut self) {
        self.package_guid.clear();
    }

    // Param is passed by value, moved
    pub fn set_package_guid(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.package_guid = v;
    }

    // Mutable pointer to the field.
    pub fn mut_package_guid(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.package_guid
    }

    // Take field
    pub fn take_package_guid(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.package_guid, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ShipmentRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.guid)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.package_guid)?;
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
        for value in &self.guid {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.address {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.package_guid {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.guid {
            os.write_string(1, &v)?;
        };
        for v in &self.address {
            os.write_string(2, &v)?;
        };
        for v in &self.package_guid {
            os.write_string(3, &v)?;
        };
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ShipmentRequest {
        ShipmentRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guid",
                    |m: &ShipmentRequest| { &m.guid },
                    |m: &mut ShipmentRequest| { &mut m.guid },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    |m: &ShipmentRequest| { &m.address },
                    |m: &mut ShipmentRequest| { &mut m.address },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "package_guid",
                    |m: &ShipmentRequest| { &m.package_guid },
                    |m: &mut ShipmentRequest| { &mut m.package_guid },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShipmentRequest>(
                    "ShipmentRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ShipmentRequest {
        static mut instance: ::protobuf::lazy::Lazy<ShipmentRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShipmentRequest,
        };
        unsafe {
            instance.get(ShipmentRequest::new)
        }
    }
}

impl ::protobuf::Clear for ShipmentRequest {
    fn clear(&mut self) {
        self.guid.clear();
        self.address.clear();
        self.package_guid.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShipmentRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShipmentRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Shipments {
    // message fields
    records: ::protobuf::RepeatedField<Shipment>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Shipments {
    fn default() -> &'a Shipments {
        <Shipments as ::protobuf::Message>::default_instance()
    }
}

impl Shipments {
    pub fn new() -> Shipments {
        ::std::default::Default::default()
    }

    // repeated .warehouse.Shipment records = 1;


    pub fn get_records(&self) -> &[Shipment] {
        &self.records
    }
    pub fn clear_records(&mut self) {
        self.records.clear();
    }

    // Param is passed by value, moved
    pub fn set_records(&mut self, v: ::protobuf::RepeatedField<Shipment>) {
        self.records = v;
    }

    // Mutable pointer to the field.
    pub fn mut_records(&mut self) -> &mut ::protobuf::RepeatedField<Shipment> {
        &mut self.records
    }

    // Take field
    pub fn take_records(&mut self) -> ::protobuf::RepeatedField<Shipment> {
        ::std::mem::replace(&mut self.records, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Shipments {
    fn is_initialized(&self) -> bool {
        for v in &self.records {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.records)?;
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
        for value in &self.records {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.records {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Shipments {
        Shipments::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Shipment>>(
                    "records",
                    |m: &Shipments| { &m.records },
                    |m: &mut Shipments| { &mut m.records },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Shipments>(
                    "Shipments",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Shipments {
        static mut instance: ::protobuf::lazy::Lazy<Shipments> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Shipments,
        };
        unsafe {
            instance.get(Shipments::new)
        }
    }
}

impl ::protobuf::Clear for Shipments {
    fn clear(&mut self) {
        self.records.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Shipments {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Shipments {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fwarehouse.proto\x12\twarehouse\"q\n\x08Shipment\x12\x12\n\x04guid\
    \x18\x01\x20\x01(\tR\x04guid\x12\x18\n\x07address\x18\x02\x20\x01(\tR\
    \x07address\x12\x14\n\x05price\x18\x03\x20\x01(\x01R\x05price\x12!\n\x0c\
    package_guid\x18\x04\x20\x01(\tR\x0bpackageGuid\"b\n\x0fShipmentRequest\
    \x12\x12\n\x04guid\x18\x01\x20\x03(\tR\x04guid\x12\x18\n\x07address\x18\
    \x02\x20\x03(\tR\x07address\x12!\n\x0cpackage_guid\x18\x03\x20\x03(\tR\
    \x0bpackageGuid\":\n\tShipments\x12-\n\x07records\x18\x01\x20\x03(\x0b2\
    \x13.warehouse.ShipmentR\x07records\
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
