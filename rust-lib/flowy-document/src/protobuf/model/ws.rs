// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `ws.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct WsDocumentData {
    // message fields
    pub id: ::std::string::String,
    pub ty: WsDataType,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WsDocumentData {
    fn default() -> &'a WsDocumentData {
        <WsDocumentData as ::protobuf::Message>::default_instance()
    }
}

impl WsDocumentData {
    pub fn new() -> WsDocumentData {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // .WsDataType ty = 2;


    pub fn get_ty(&self) -> WsDataType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = WsDataType::Acked;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: WsDataType) {
        self.ty = v;
    }

    // bytes data = 3;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for WsDocumentData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if self.ty != WsDataType::Acked {
            my_size += ::protobuf::rt::enum_size(2, self.ty);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.ty != WsDataType::Acked {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ty))?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> WsDocumentData {
        WsDocumentData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &WsDocumentData| { &m.id },
                |m: &mut WsDocumentData| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<WsDataType>>(
                "ty",
                |m: &WsDocumentData| { &m.ty },
                |m: &mut WsDocumentData| { &mut m.ty },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "data",
                |m: &WsDocumentData| { &m.data },
                |m: &mut WsDocumentData| { &mut m.data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WsDocumentData>(
                "WsDocumentData",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WsDocumentData {
        static instance: ::protobuf::rt::LazyV2<WsDocumentData> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WsDocumentData::new)
    }
}

impl ::protobuf::Clear for WsDocumentData {
    fn clear(&mut self) {
        self.id.clear();
        self.ty = WsDataType::Acked;
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WsDocumentData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WsDocumentData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum WsDataType {
    Acked = 0,
    Delta = 1,
}

impl ::protobuf::ProtobufEnum for WsDataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WsDataType> {
        match value {
            0 => ::std::option::Option::Some(WsDataType::Acked),
            1 => ::std::option::Option::Some(WsDataType::Delta),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WsDataType] = &[
            WsDataType::Acked,
            WsDataType::Delta,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<WsDataType>("WsDataType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for WsDataType {
}

impl ::std::default::Default for WsDataType {
    fn default() -> Self {
        WsDataType::Acked
    }
}

impl ::protobuf::reflect::ProtobufValue for WsDataType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08ws.proto\"Q\n\x0eWsDocumentData\x12\x0e\n\x02id\x18\x01\x20\x01(\t\
    R\x02id\x12\x1b\n\x02ty\x18\x02\x20\x01(\x0e2\x0b.WsDataTypeR\x02ty\x12\
    \x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data*\"\n\nWsDataType\x12\t\n\
    \x05Acked\x10\0\x12\t\n\x05Delta\x10\x01J\xb9\x02\n\x06\x12\x04\0\0\n\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x06\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x16\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x03\x04\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x03\x0b\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x03\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x16\n\x0c\n\
    \x05\x04\0\x02\x01\x06\x12\x03\x04\x04\x0e\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x04\x0f\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x14\x15\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04\x13\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\n\x0e\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x11\x12\n\n\n\x02\x05\0\x12\x04\
    \x07\0\n\x01\n\n\n\x03\x05\0\x01\x12\x03\x07\x05\x0f\n\x0b\n\x04\x05\0\
    \x02\0\x12\x03\x08\x04\x0e\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x08\x04\t\
    \n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x08\x0c\r\n\x0b\n\x04\x05\0\x02\x01\
    \x12\x03\t\x04\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\t\x04\t\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\t\x0c\rb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
