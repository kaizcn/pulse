// proto - bitdrift's client/server API definitions
// Copyright Bitdrift, Inc. All rights reserved.
//
// Use of this source code and APIs are governed by a source available license that can be found in
// the LICENSE file or at:
// https://polyformproject.org/wp-content/uploads/2020/06/PolyForm-Shield-1.0.0.txt

// This file is generated by rust-protobuf 4.0.0-alpha.0. Do not edit
// .proto file is parsed by protoc 27.3
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `pulse/config/inflow/v1/metric_generator.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_4_0_0_ALPHA_0;

// @@protoc_insertion_point(message:pulse.config.inflow.v1.MetricGeneratorConfig)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MetricGeneratorConfig {
    // message fields
    // @@protoc_insertion_point(field:pulse.config.inflow.v1.MetricGeneratorConfig.protocol)
    pub protocol: ::protobuf::MessageField<super::common::WireProtocol>,
    // @@protoc_insertion_point(field:pulse.config.inflow.v1.MetricGeneratorConfig.n_tasks)
    pub n_tasks: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:pulse.config.inflow.v1.MetricGeneratorConfig.batch_size)
    pub batch_size: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:pulse.config.inflow.v1.MetricGeneratorConfig.flush_interval)
    pub flush_interval: ::protobuf::MessageField<::protobuf::well_known_types::duration::Duration>,
    // special fields
    // @@protoc_insertion_point(special_field:pulse.config.inflow.v1.MetricGeneratorConfig.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MetricGeneratorConfig {
    fn default() -> &'a MetricGeneratorConfig {
        <MetricGeneratorConfig as ::protobuf::Message>::default_instance()
    }
}

impl MetricGeneratorConfig {
    pub fn new() -> MetricGeneratorConfig {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::WireProtocol>(
            "protocol",
            |m: &MetricGeneratorConfig| { &m.protocol },
            |m: &mut MetricGeneratorConfig| { &mut m.protocol },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "n_tasks",
            |m: &MetricGeneratorConfig| { &m.n_tasks },
            |m: &mut MetricGeneratorConfig| { &mut m.n_tasks },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "batch_size",
            |m: &MetricGeneratorConfig| { &m.batch_size },
            |m: &mut MetricGeneratorConfig| { &mut m.batch_size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::duration::Duration>(
            "flush_interval",
            |m: &MetricGeneratorConfig| { &m.flush_interval },
            |m: &mut MetricGeneratorConfig| { &mut m.flush_interval },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MetricGeneratorConfig>(
            "MetricGeneratorConfig",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MetricGeneratorConfig {
    const NAME: &'static str = "MetricGeneratorConfig";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.protocol)?;
                },
                16 => {
                    self.n_tasks = ::std::option::Option::Some(is.read_uint64()?);
                },
                24 => {
                    self.batch_size = ::std::option::Option::Some(is.read_uint64()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.flush_interval)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.protocol.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.n_tasks {
            my_size += ::protobuf::rt::uint64_size(2, v);
        }
        if let Some(v) = self.batch_size {
            my_size += ::protobuf::rt::uint64_size(3, v);
        }
        if let Some(v) = self.flush_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.protocol.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.n_tasks {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.batch_size {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.flush_interval.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MetricGeneratorConfig {
        MetricGeneratorConfig::new()
    }

    fn clear(&mut self) {
        self.protocol.clear();
        self.n_tasks = ::std::option::Option::None;
        self.batch_size = ::std::option::Option::None;
        self.flush_interval.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MetricGeneratorConfig {
        static instance: MetricGeneratorConfig = MetricGeneratorConfig {
            protocol: ::protobuf::MessageField::none(),
            n_tasks: ::std::option::Option::None,
            batch_size: ::std::option::Option::None,
            flush_interval: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MetricGeneratorConfig {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MetricGeneratorConfig").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MetricGeneratorConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetricGeneratorConfig {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n-pulse/config/inflow/v1/metric_generator.proto\x12\x16pulse.config.inf\
    low.v1\x1a#pulse/config/common/v1/common.proto\x1a\x1egoogle/protobuf/du\
    ration.proto\x1a\x17validate/validate.proto\"\x8c\x02\n\x15MetricGenerat\
    orConfig\x12J\n\x08protocol\x18\x01\x20\x01(\x0b2$.pulse.config.common.v\
    1.WireProtocolR\x08protocolB\x08\xfaB\x05\x8a\x01\x02\x10\x01\x12\x1c\n\
    \x07n_tasks\x18\x02\x20\x01(\x04H\0R\x06nTasks\x88\x01\x01\x12\"\n\nbatc\
    h_size\x18\x03\x20\x01(\x04H\x01R\tbatchSize\x88\x01\x01\x12J\n\x0eflush\
    _interval\x18\x04\x20\x01(\x0b2\x19.google.protobuf.DurationR\rflushInte\
    rvalB\x08\xfaB\x05\xaa\x01\x02*\0B\n\n\x08_n_tasksB\r\n\x0b_batch_sizeb\
    \x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::common::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::duration::file_descriptor().clone());
            deps.push(super::validate::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MetricGeneratorConfig::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
