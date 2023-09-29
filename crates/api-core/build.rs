use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;
use protobuf_codegen::CustomizeCallback;

fn main() {
    struct GenSerde;

    impl CustomizeCallback for GenSerde {
        fn message(&self, message: &MessageDescriptor) -> Customize {
            #[cfg(not(feature = "async-graphql"))]
            return Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = \"camelCase\")]");

            #[cfg(feature = "async-graphql")]
            {
                let derives = format!("#[derive(::serde::Serialize, ::serde::Deserialize, ::async_graphql::{}Object)] #[serde(rename_all = \"camelCase\")]", match message.name() {
                    "CategoryInput" => {
                      "Input"
                    },
                    "Category" => {
                      "Simple"
                    },
                    _ => {
                      unreachable!("check your proto message names")
                    }
                });

                Customize::default().before(&derives)
            }
        }

        #[cfg(not(feature = "async-graphql"))]
        fn field(&self, field: &FieldDescriptor) -> Customize {
            if field.proto().type_() == Type::TYPE_ENUM {
                // `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
                Customize::default().before(
                    "#[serde(serialize_with = \"crate::serialize_enum_or_unknown\", deserialize_with = \"crate::deserialize_enum_or_unknown\")]")
            } else {
                Customize::default()
            }
        }

        #[cfg(feature = "async-graphql")]
        fn field(&self, field: &FieldDescriptor) -> Customize {
            if field.proto().type_() == Type::TYPE_ENUM {
                // `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
                Customize::default().before(
                    "#[serde(serialize_with = \"crate::serialize_enum_or_unknown\", deserialize_with = \"crate::deserialize_enum_or_unknown\")] #[graphql(skip_input)]")
            } else {
                Customize::default().before("#[graphql(skip_input)]")
            }
        }

        fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
            #[cfg(not(feature = "async-graphql"))]
            return Customize::default().before("#[serde(skip)]");

            #[cfg(feature = "async-graphql")]
            Customize::default().before("#[serde(skip)] #[graphql(skip)]")
        }
    }

    Codegen::new()
        .cargo_out_dir("protos")
        .include("src")
        .inputs(["src/proto/category.proto"])
        .customize_callback(GenSerde)
        .run_from_script();
}
