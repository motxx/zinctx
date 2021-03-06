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
//! Generated file from `src/zinctx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct Value {
    // message oneof groups
    pub ValueOneOf: ::std::option::Option<Value_oneof_ValueOneOf>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Value {
    fn default() -> &'a Value {
        <Value as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Value_oneof_ValueOneOf {
    intval(i32),
    stringval(::std::string::String),
}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    // int32 intval = 1;


    pub fn get_intval(&self) -> i32 {
        match self.ValueOneOf {
            ::std::option::Option::Some(Value_oneof_ValueOneOf::intval(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_intval(&mut self) {
        self.ValueOneOf = ::std::option::Option::None;
    }

    pub fn has_intval(&self) -> bool {
        match self.ValueOneOf {
            ::std::option::Option::Some(Value_oneof_ValueOneOf::intval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_intval(&mut self, v: i32) {
        self.ValueOneOf = ::std::option::Option::Some(Value_oneof_ValueOneOf::intval(v))
    }

    // string stringval = 2;


    pub fn get_stringval(&self) -> &str {
        match self.ValueOneOf {
            ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_stringval(&mut self) {
        self.ValueOneOf = ::std::option::Option::None;
    }

    pub fn has_stringval(&self) -> bool {
        match self.ValueOneOf {
            ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_stringval(&mut self, v: ::std::string::String) {
        self.ValueOneOf = ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(v))
    }

    // Mutable pointer to the field.
    pub fn mut_stringval(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(_)) = self.ValueOneOf {
        } else {
            self.ValueOneOf = ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(::std::string::String::new()));
        }
        match self.ValueOneOf {
            ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_stringval(&mut self) -> ::std::string::String {
        if self.has_stringval() {
            match self.ValueOneOf.take() {
                ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.ValueOneOf = ::std::option::Option::Some(Value_oneof_ValueOneOf::intval(is.read_int32()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.ValueOneOf = ::std::option::Option::Some(Value_oneof_ValueOneOf::stringval(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.ValueOneOf {
            match v {
                &Value_oneof_ValueOneOf::intval(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Value_oneof_ValueOneOf::stringval(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.ValueOneOf {
            match v {
                &Value_oneof_ValueOneOf::intval(v) => {
                    os.write_int32(1, v)?;
                },
                &Value_oneof_ValueOneOf::stringval(ref v) => {
                    os.write_string(2, v)?;
                },
            };
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

    fn new() -> Value {
        Value::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                "intval",
                Value::has_intval,
                Value::get_intval,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "stringval",
                Value::has_stringval,
                Value::get_stringval,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Value>(
                "Value",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Value {
        static instance: ::protobuf::rt::LazyV2<Value> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Value::new)
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.ValueOneOf = ::std::option::Option::None;
        self.ValueOneOf = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Value {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryRequest {
    // message fields
    pub address: ::std::string::String,
    pub method: ::std::string::String,
    pub arguments: ::std::collections::HashMap<::std::string::String, Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryRequest {
    fn default() -> &'a QueryRequest {
        <QueryRequest as ::protobuf::Message>::default_instance()
    }
}

impl QueryRequest {
    pub fn new() -> QueryRequest {
        ::std::default::Default::default()
    }

    // string address = 1;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string method = 2;


    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::std::string::String) {
        self.method = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method(&mut self) -> &mut ::std::string::String {
        &mut self.method
    }

    // Take field
    pub fn take_method(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.method, ::std::string::String::new())
    }

    // repeated .QueryRequest.ArgumentsEntry arguments = 3;


    pub fn get_arguments(&self) -> &::std::collections::HashMap<::std::string::String, Value> {
        &self.arguments
    }
    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    // Param is passed by value, moved
    pub fn set_arguments(&mut self, v: ::std::collections::HashMap<::std::string::String, Value>) {
        self.arguments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arguments(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Value> {
        &mut self.arguments
    }

    // Take field
    pub fn take_arguments(&mut self) -> ::std::collections::HashMap<::std::string::String, Value> {
        ::std::mem::replace(&mut self.arguments, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for QueryRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.method)?;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(wire_type, is, &mut self.arguments)?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        }
        if !self.method.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.method);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(3, &self.arguments);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        }
        if !self.method.is_empty() {
            os.write_string(2, &self.method)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(3, &self.arguments, os)?;
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

    fn new() -> QueryRequest {
        QueryRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "address",
                |m: &QueryRequest| { &m.address },
                |m: &mut QueryRequest| { &mut m.address },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "method",
                |m: &QueryRequest| { &m.method },
                |m: &mut QueryRequest| { &mut m.method },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "arguments",
                |m: &QueryRequest| { &m.arguments },
                |m: &mut QueryRequest| { &mut m.arguments },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryRequest>(
                "QueryRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryRequest {
        static instance: ::protobuf::rt::LazyV2<QueryRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryRequest::new)
    }
}

impl ::protobuf::Clear for QueryRequest {
    fn clear(&mut self) {
        self.address.clear();
        self.method.clear();
        self.arguments.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryResponse {
    // message fields
    pub output: ::protobuf::SingularPtrField<Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryResponse {
    fn default() -> &'a QueryResponse {
        <QueryResponse as ::protobuf::Message>::default_instance()
    }
}

impl QueryResponse {
    pub fn new() -> QueryResponse {
        ::std::default::Default::default()
    }

    // .Value output = 1;


    pub fn get_output(&self) -> &Value {
        self.output.as_ref().unwrap_or_else(|| <Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_output(&mut self, v: Value) {
        self.output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_output(&mut self) -> &mut Value {
        if self.output.is_none() {
            self.output.set_default();
        }
        self.output.as_mut().unwrap()
    }

    // Take field
    pub fn take_output(&mut self) -> Value {
        self.output.take().unwrap_or_else(|| Value::new())
    }
}

impl ::protobuf::Message for QueryResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.output {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.output)?;
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
        if let Some(ref v) = self.output.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.output.as_ref() {
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

    fn new() -> QueryResponse {
        QueryResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "output",
                |m: &QueryResponse| { &m.output },
                |m: &mut QueryResponse| { &mut m.output },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryResponse>(
                "QueryResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryResponse {
        static instance: ::protobuf::rt::LazyV2<QueryResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryResponse::new)
    }
}

impl ::protobuf::Clear for QueryResponse {
    fn clear(&mut self) {
        self.output.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallRequest {
    // message fields
    pub address: ::std::string::String,
    pub method: ::std::string::String,
    pub arguments: ::std::collections::HashMap<::std::string::String, Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CallRequest {
    fn default() -> &'a CallRequest {
        <CallRequest as ::protobuf::Message>::default_instance()
    }
}

impl CallRequest {
    pub fn new() -> CallRequest {
        ::std::default::Default::default()
    }

    // string address = 1;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string method = 2;


    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::std::string::String) {
        self.method = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method(&mut self) -> &mut ::std::string::String {
        &mut self.method
    }

    // Take field
    pub fn take_method(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.method, ::std::string::String::new())
    }

    // repeated .CallRequest.ArgumentsEntry arguments = 3;


    pub fn get_arguments(&self) -> &::std::collections::HashMap<::std::string::String, Value> {
        &self.arguments
    }
    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    // Param is passed by value, moved
    pub fn set_arguments(&mut self, v: ::std::collections::HashMap<::std::string::String, Value>) {
        self.arguments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arguments(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Value> {
        &mut self.arguments
    }

    // Take field
    pub fn take_arguments(&mut self) -> ::std::collections::HashMap<::std::string::String, Value> {
        ::std::mem::replace(&mut self.arguments, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for CallRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.method)?;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(wire_type, is, &mut self.arguments)?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        }
        if !self.method.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.method);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(3, &self.arguments);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        }
        if !self.method.is_empty() {
            os.write_string(2, &self.method)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(3, &self.arguments, os)?;
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

    fn new() -> CallRequest {
        CallRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "address",
                |m: &CallRequest| { &m.address },
                |m: &mut CallRequest| { &mut m.address },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "method",
                |m: &CallRequest| { &m.method },
                |m: &mut CallRequest| { &mut m.method },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "arguments",
                |m: &CallRequest| { &m.arguments },
                |m: &mut CallRequest| { &mut m.arguments },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CallRequest>(
                "CallRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CallRequest {
        static instance: ::protobuf::rt::LazyV2<CallRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CallRequest::new)
    }
}

impl ::protobuf::Clear for CallRequest {
    fn clear(&mut self) {
        self.address.clear();
        self.method.clear();
        self.arguments.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CallResponse {
    // message fields
    pub output: ::protobuf::SingularPtrField<Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CallResponse {
    fn default() -> &'a CallResponse {
        <CallResponse as ::protobuf::Message>::default_instance()
    }
}

impl CallResponse {
    pub fn new() -> CallResponse {
        ::std::default::Default::default()
    }

    // .Value output = 1;


    pub fn get_output(&self) -> &Value {
        self.output.as_ref().unwrap_or_else(|| <Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_output(&mut self, v: Value) {
        self.output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_output(&mut self) -> &mut Value {
        if self.output.is_none() {
            self.output.set_default();
        }
        self.output.as_mut().unwrap()
    }

    // Take field
    pub fn take_output(&mut self) -> Value {
        self.output.take().unwrap_or_else(|| Value::new())
    }
}

impl ::protobuf::Message for CallResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.output {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.output)?;
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
        if let Some(ref v) = self.output.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.output.as_ref() {
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

    fn new() -> CallResponse {
        CallResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "output",
                |m: &CallResponse| { &m.output },
                |m: &mut CallResponse| { &mut m.output },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CallResponse>(
                "CallResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CallResponse {
        static instance: ::protobuf::rt::LazyV2<CallResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CallResponse::new)
    }
}

impl ::protobuf::Clear for CallResponse {
    fn clear(&mut self) {
        self.output.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CallResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CallResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10src/zinctx.proto\"O\n\x05Value\x12\x18\n\x06intval\x18\x01\x20\x01\
    (\x05H\0R\x06intval\x12\x1e\n\tstringval\x18\x02\x20\x01(\tH\0R\tstringv\
    alB\x0c\n\nValueOneOf\"\xc2\x01\n\x0cQueryRequest\x12\x18\n\x07address\
    \x18\x01\x20\x01(\tR\x07address\x12\x16\n\x06method\x18\x02\x20\x01(\tR\
    \x06method\x12:\n\targuments\x18\x03\x20\x03(\x0b2\x1c.QueryRequest.Argu\
    mentsEntryR\targuments\x1aD\n\x0eArgumentsEntry\x12\x10\n\x03key\x18\x01\
    \x20\x01(\tR\x03key\x12\x1c\n\x05value\x18\x02\x20\x01(\x0b2\x06.ValueR\
    \x05value:\x028\x01\"/\n\rQueryResponse\x12\x1e\n\x06output\x18\x01\x20\
    \x01(\x0b2\x06.ValueR\x06output\"\xc0\x01\n\x0bCallRequest\x12\x18\n\x07\
    address\x18\x01\x20\x01(\tR\x07address\x12\x16\n\x06method\x18\x02\x20\
    \x01(\tR\x06method\x129\n\targuments\x18\x03\x20\x03(\x0b2\x1b.CallReque\
    st.ArgumentsEntryR\targuments\x1aD\n\x0eArgumentsEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12\x1c\n\x05value\x18\x02\x20\x01(\x0b2\x06\
    .ValueR\x05value:\x028\x01\".\n\x0cCallResponse\x12\x1e\n\x06output\x18\
    \x01\x20\x01(\x0b2\x06.ValueR\x06outputB\tZ\x07protos/b\x06proto3\
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
