// proto - bitdrift's client/server API definitions
// Copyright Bitdrift, Inc. All rights reserved.
//
// Use of this source code and APIs are governed by a source available license that can be found in
// the LICENSE file or at:
// https://polyformproject.org/wp-content/uploads/2020/06/PolyForm-Shield-1.0.0.txt

// This file is generated by rust-protobuf 4.0.0-alpha.0. Do not edit
// .proto file is parsed by protoc 29.3
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `pulse/config/processor/v1/cardinality_limiter.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_4_0_0_ALPHA_0;

// @@protoc_insertion_point(message:pulse.config.processor.v1.CardinalityLimiterConfig)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CardinalityLimiterConfig {
    // message fields
    // @@protoc_insertion_point(field:pulse.config.processor.v1.CardinalityLimiterConfig.buckets)
    pub buckets: u32,
    // @@protoc_insertion_point(field:pulse.config.processor.v1.CardinalityLimiterConfig.rotate_after)
    pub rotate_after: ::protobuf::MessageField<::protobuf::well_known_types::duration::Duration>,
    // message oneof groups
    pub limit_type: ::std::option::Option<cardinality_limiter_config::Limit_type>,
    // special fields
    // @@protoc_insertion_point(special_field:pulse.config.processor.v1.CardinalityLimiterConfig.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CardinalityLimiterConfig {
    fn default() -> &'a CardinalityLimiterConfig {
        <CardinalityLimiterConfig as ::protobuf::Message>::default_instance()
    }
}

impl CardinalityLimiterConfig {
    pub fn new() -> CardinalityLimiterConfig {
        ::std::default::Default::default()
    }

    // .pulse.config.processor.v1.CardinalityLimiterConfig.GlobalLimit global_limit = 1;

    pub fn global_limit(&self) -> &cardinality_limiter_config::GlobalLimit {
        match self.limit_type {
            ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(ref v)) => v,
            _ => <cardinality_limiter_config::GlobalLimit as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_global_limit(&mut self) {
        self.limit_type = ::std::option::Option::None;
    }

    pub fn has_global_limit(&self) -> bool {
        match self.limit_type {
            ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_global_limit(&mut self, v: cardinality_limiter_config::GlobalLimit) {
        self.limit_type = ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_global_limit(&mut self) -> &mut cardinality_limiter_config::GlobalLimit {
        if let ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(_)) = self.limit_type {
        } else {
            self.limit_type = ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(cardinality_limiter_config::GlobalLimit::new()));
        }
        match self.limit_type {
            ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_global_limit(&mut self) -> cardinality_limiter_config::GlobalLimit {
        if self.has_global_limit() {
            match self.limit_type.take() {
                ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(v)) => v,
                _ => panic!(),
            }
        } else {
            cardinality_limiter_config::GlobalLimit::new()
        }
    }

    // .pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLimit per_pod_limit = 2;

    pub fn per_pod_limit(&self) -> &cardinality_limiter_config::PerPodLimit {
        match self.limit_type {
            ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(ref v)) => v,
            _ => <cardinality_limiter_config::PerPodLimit as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_per_pod_limit(&mut self) {
        self.limit_type = ::std::option::Option::None;
    }

    pub fn has_per_pod_limit(&self) -> bool {
        match self.limit_type {
            ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_per_pod_limit(&mut self, v: cardinality_limiter_config::PerPodLimit) {
        self.limit_type = ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_per_pod_limit(&mut self) -> &mut cardinality_limiter_config::PerPodLimit {
        if let ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(_)) = self.limit_type {
        } else {
            self.limit_type = ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(cardinality_limiter_config::PerPodLimit::new()));
        }
        match self.limit_type {
            ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_per_pod_limit(&mut self) -> cardinality_limiter_config::PerPodLimit {
        if self.has_per_pod_limit() {
            match self.limit_type.take() {
                ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(v)) => v,
                _ => panic!(),
            }
        } else {
            cardinality_limiter_config::PerPodLimit::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, cardinality_limiter_config::GlobalLimit>(
            "global_limit",
            CardinalityLimiterConfig::has_global_limit,
            CardinalityLimiterConfig::global_limit,
            CardinalityLimiterConfig::mut_global_limit,
            CardinalityLimiterConfig::set_global_limit,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, cardinality_limiter_config::PerPodLimit>(
            "per_pod_limit",
            CardinalityLimiterConfig::has_per_pod_limit,
            CardinalityLimiterConfig::per_pod_limit,
            CardinalityLimiterConfig::mut_per_pod_limit,
            CardinalityLimiterConfig::set_per_pod_limit,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buckets",
            |m: &CardinalityLimiterConfig| { &m.buckets },
            |m: &mut CardinalityLimiterConfig| { &mut m.buckets },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::duration::Duration>(
            "rotate_after",
            |m: &CardinalityLimiterConfig| { &m.rotate_after },
            |m: &mut CardinalityLimiterConfig| { &mut m.rotate_after },
        ));
        oneofs.push(cardinality_limiter_config::Limit_type::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CardinalityLimiterConfig>(
            "CardinalityLimiterConfig",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CardinalityLimiterConfig {
    const NAME: &'static str = "CardinalityLimiterConfig";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.limit_type = ::std::option::Option::Some(cardinality_limiter_config::Limit_type::GlobalLimit(is.read_message()?));
                },
                18 => {
                    self.limit_type = ::std::option::Option::Some(cardinality_limiter_config::Limit_type::PerPodLimit(is.read_message()?));
                },
                24 => {
                    self.buckets = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rotate_after)?;
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
        if self.buckets != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.buckets);
        }
        if let Some(v) = self.rotate_after.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.limit_type {
            match v {
                &cardinality_limiter_config::Limit_type::GlobalLimit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cardinality_limiter_config::Limit_type::PerPodLimit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.buckets != 0 {
            os.write_uint32(3, self.buckets)?;
        }
        if let Some(v) = self.rotate_after.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.limit_type {
            match v {
                &cardinality_limiter_config::Limit_type::GlobalLimit(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &cardinality_limiter_config::Limit_type::PerPodLimit(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
            };
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

    fn new() -> CardinalityLimiterConfig {
        CardinalityLimiterConfig::new()
    }

    fn clear(&mut self) {
        self.limit_type = ::std::option::Option::None;
        self.limit_type = ::std::option::Option::None;
        self.buckets = 0;
        self.rotate_after.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CardinalityLimiterConfig {
        static instance: CardinalityLimiterConfig = CardinalityLimiterConfig {
            buckets: 0,
            rotate_after: ::protobuf::MessageField::none(),
            limit_type: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CardinalityLimiterConfig {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CardinalityLimiterConfig").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CardinalityLimiterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CardinalityLimiterConfig {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CardinalityLimiterConfig`
pub mod cardinality_limiter_config {

    #[derive(Clone,PartialEq,Debug)]
    // @@protoc_insertion_point(oneof:pulse.config.processor.v1.CardinalityLimiterConfig.limit_type)
    pub enum Limit_type {
        // @@protoc_insertion_point(oneof_field:pulse.config.processor.v1.CardinalityLimiterConfig.global_limit)
        GlobalLimit(GlobalLimit),
        // @@protoc_insertion_point(oneof_field:pulse.config.processor.v1.CardinalityLimiterConfig.per_pod_limit)
        PerPodLimit(PerPodLimit),
    }

    impl ::protobuf::Oneof for Limit_type {
    }

    impl ::protobuf::OneofFull for Limit_type {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CardinalityLimiterConfig as ::protobuf::MessageFull>::descriptor().oneof_by_name("limit_type").unwrap()).clone()
        }
    }

    impl Limit_type {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Limit_type>("limit_type")
        }
    }
    // @@protoc_insertion_point(message:pulse.config.processor.v1.CardinalityLimiterConfig.GlobalLimit)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct GlobalLimit {
        // message fields
        // @@protoc_insertion_point(field:pulse.config.processor.v1.CardinalityLimiterConfig.GlobalLimit.size_limit)
        pub size_limit: u32,
        // special fields
        // @@protoc_insertion_point(special_field:pulse.config.processor.v1.CardinalityLimiterConfig.GlobalLimit.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a GlobalLimit {
        fn default() -> &'a GlobalLimit {
            <GlobalLimit as ::protobuf::Message>::default_instance()
        }
    }

    impl GlobalLimit {
        pub fn new() -> GlobalLimit {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(1);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "size_limit",
                |m: &GlobalLimit| { &m.size_limit },
                |m: &mut GlobalLimit| { &mut m.size_limit },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GlobalLimit>(
                "CardinalityLimiterConfig.GlobalLimit",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for GlobalLimit {
        const NAME: &'static str = "GlobalLimit";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.size_limit = is.read_uint32()?;
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
            if self.size_limit != 0 {
                my_size += ::protobuf::rt::uint32_size(1, self.size_limit);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.size_limit != 0 {
                os.write_uint32(1, self.size_limit)?;
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

        fn new() -> GlobalLimit {
            GlobalLimit::new()
        }

        fn clear(&mut self) {
            self.size_limit = 0;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static GlobalLimit {
            static instance: GlobalLimit = GlobalLimit {
                size_limit: 0,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for GlobalLimit {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("CardinalityLimiterConfig.GlobalLimit").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for GlobalLimit {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for GlobalLimit {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    // @@protoc_insertion_point(message:pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLimit)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct PerPodLimit {
        // message fields
        // @@protoc_insertion_point(field:pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLimit.default_size_limit)
        pub default_size_limit: u32,
        // message oneof groups
        pub override_limit_location: ::std::option::Option<per_pod_limit::Override_limit_location>,
        // special fields
        // @@protoc_insertion_point(special_field:pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLimit.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a PerPodLimit {
        fn default() -> &'a PerPodLimit {
            <PerPodLimit as ::protobuf::Message>::default_instance()
        }
    }

    impl PerPodLimit {
        pub fn new() -> PerPodLimit {
            ::std::default::Default::default()
        }

        // string vrl_program = 2;

        pub fn vrl_program(&self) -> &str {
            match self.override_limit_location {
                ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(ref v)) => v,
                _ => "",
            }
        }

        pub fn clear_vrl_program(&mut self) {
            self.override_limit_location = ::std::option::Option::None;
        }

        pub fn has_vrl_program(&self) -> bool {
            match self.override_limit_location {
                ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(..)) => true,
                _ => false,
            }
        }

        // Param is passed by value, moved
        pub fn set_vrl_program(&mut self, v: ::protobuf::Chars) {
            self.override_limit_location = ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(v))
        }

        // Mutable pointer to the field.
        pub fn mut_vrl_program(&mut self) -> &mut ::protobuf::Chars {
            if let ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(_)) = self.override_limit_location {
            } else {
                self.override_limit_location = ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(::protobuf::Chars::new()));
            }
            match self.override_limit_location {
                ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(ref mut v)) => v,
                _ => panic!(),
            }
        }

        // Take field
        pub fn take_vrl_program(&mut self) -> ::protobuf::Chars {
            if self.has_vrl_program() {
                match self.override_limit_location.take() {
                    ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(v)) => v,
                    _ => panic!(),
                }
            } else {
                ::protobuf::Chars::new()
            }
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(1);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "default_size_limit",
                |m: &PerPodLimit| { &m.default_size_limit },
                |m: &mut PerPodLimit| { &mut m.default_size_limit },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
                "vrl_program",
                PerPodLimit::has_vrl_program,
                PerPodLimit::vrl_program,
                PerPodLimit::set_vrl_program,
            ));
            oneofs.push(per_pod_limit::Override_limit_location::generated_oneof_descriptor_data());
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PerPodLimit>(
                "CardinalityLimiterConfig.PerPodLimit",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for PerPodLimit {
        const NAME: &'static str = "PerPodLimit";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.default_size_limit = is.read_uint32()?;
                    },
                    18 => {
                        self.override_limit_location = ::std::option::Option::Some(per_pod_limit::Override_limit_location::VrlProgram(is.read_tokio_chars()?));
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
            if self.default_size_limit != 0 {
                my_size += ::protobuf::rt::uint32_size(1, self.default_size_limit);
            }
            if let ::std::option::Option::Some(ref v) = self.override_limit_location {
                match v {
                    &per_pod_limit::Override_limit_location::VrlProgram(ref v) => {
                        my_size += ::protobuf::rt::string_size(2, &v);
                    },
                };
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.default_size_limit != 0 {
                os.write_uint32(1, self.default_size_limit)?;
            }
            if let ::std::option::Option::Some(ref v) = self.override_limit_location {
                match v {
                    &per_pod_limit::Override_limit_location::VrlProgram(ref v) => {
                        os.write_string(2, v)?;
                    },
                };
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

        fn new() -> PerPodLimit {
            PerPodLimit::new()
        }

        fn clear(&mut self) {
            self.default_size_limit = 0;
            self.override_limit_location = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static PerPodLimit {
            static instance: PerPodLimit = PerPodLimit {
                default_size_limit: 0,
                override_limit_location: ::std::option::Option::None,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for PerPodLimit {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("CardinalityLimiterConfig.PerPodLimit").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for PerPodLimit {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for PerPodLimit {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    /// Nested message and enums of message `PerPodLimit`
    pub mod per_pod_limit {

        #[derive(Clone,PartialEq,Debug)]
        // @@protoc_insertion_point(oneof:pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLimit.override_limit_location)
        pub enum Override_limit_location {
            // @@protoc_insertion_point(oneof_field:pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLimit.vrl_program)
            VrlProgram(::protobuf::Chars),
        }

        impl ::protobuf::Oneof for Override_limit_location {
        }

        impl ::protobuf::OneofFull for Override_limit_location {
            fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
                static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
                descriptor.get(|| <super::PerPodLimit as ::protobuf::MessageFull>::descriptor().oneof_by_name("override_limit_location").unwrap()).clone()
            }
        }

        impl Override_limit_location {
            pub(in super::super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
                ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Override_limit_location>("override_limit_location")
            }
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n3pulse/config/processor/v1/cardinality_limiter.proto\x12\x19pulse.conf\
    ig.processor.v1\x1a\x1egoogle/protobuf/duration.proto\x1a\x17validate/va\
    lidate.proto\"\xaa\x04\n\x18CardinalityLimiterConfig\x12d\n\x0cglobal_li\
    mit\x18\x01\x20\x01(\x0b2?.pulse.config.processor.v1.CardinalityLimiterC\
    onfig.GlobalLimitH\0R\x0bglobalLimit\x12e\n\rper_pod_limit\x18\x02\x20\
    \x01(\x0b2?.pulse.config.processor.v1.CardinalityLimiterConfig.PerPodLim\
    itH\0R\x0bperPodLimit\x12!\n\x07buckets\x18\x03\x20\x01(\rR\x07bucketsB\
    \x07\xfaB\x04*\x02\x20\0\x12F\n\x0crotate_after\x18\x04\x20\x01(\x0b2\
    \x19.google.protobuf.DurationR\x0brotateAfterB\x08\xfaB\x05\xaa\x01\x02*\
    \0\x1a5\n\x0bGlobalLimit\x12&\n\nsize_limit\x18\x01\x20\x01(\rR\tsizeLim\
    itB\x07\xfaB\x04*\x02\x20\0\x1a\x8b\x01\n\x0bPerPodLimit\x125\n\x12defau\
    lt_size_limit\x18\x01\x20\x01(\rR\x10defaultSizeLimitB\x07\xfaB\x04*\x02\
    \x20\0\x12*\n\x0bvrl_program\x18\x02\x20\x01(\tH\0R\nvrlProgramB\x07\xfa\
    B\x04r\x02\x10\x01B\x19\n\x17override_limit_locationB\x11\n\nlimit_type\
    \x12\x03\xf8B\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(::protobuf::well_known_types::duration::file_descriptor().clone());
            deps.push(super::validate::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(CardinalityLimiterConfig::generated_message_descriptor_data());
            messages.push(cardinality_limiter_config::GlobalLimit::generated_message_descriptor_data());
            messages.push(cardinality_limiter_config::PerPodLimit::generated_message_descriptor_data());
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
