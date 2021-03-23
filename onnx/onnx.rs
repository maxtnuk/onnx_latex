/// Attributes
///
/// A named attribute containing either singular float, integer, string, graph,
/// and tensor values, or repeated float, integer, string, graph, and tensor values.
/// An AttributeProto MUST contain the name field, and *only one* of the
/// following content fields, effectively enforcing a C/C++ union equivalent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeProto {
    /// The name field MUST be present for this version of the IR.
    ///
    /// namespace Attribute
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// if ref_attr_name is not empty, ref_attr_name is the attribute name in parent function.
    /// In this case, this AttributeProto does not contain data, and it's a reference of attribute
    /// in parent scope.
    /// NOTE: This should ONLY be used in function (sub-graph). It's invalid to be used in main graph.
    #[prost(string, tag="21")]
    pub ref_attr_name: ::prost::alloc::string::String,
    /// A human-readable documentation for this attribute. Markdown is allowed.
    #[prost(string, tag="13")]
    pub doc_string: ::prost::alloc::string::String,
    /// The type field MUST be present for this version of the IR.
    /// For 0.0.1 versions of the IR, this field was not defined, and
    /// implementations needed to use has_field hueristics to determine
    /// which value field was in use.  For IR_VERSION 0.0.2 or later, this
    /// field MUST be set and match the f|i|s|t|... field in use.  This
    /// change was made to accomodate proto3 implementations.
    ///
    /// discriminator that indicates which field below is in use
    #[prost(enumeration="attribute_proto::AttributeType", tag="20")]
    pub r#type: i32,
    /// Exactly ONE of the following fields must be present for this version of the IR
    ///
    /// float
    #[prost(float, tag="2")]
    pub f: f32,
    /// int
    #[prost(int64, tag="3")]
    pub i: i64,
    /// UTF-8 string
    #[prost(bytes="vec", tag="4")]
    pub s: ::prost::alloc::vec::Vec<u8>,
    /// tensor value
    #[prost(message, optional, tag="5")]
    pub t: ::core::option::Option<TensorProto>,
    /// graph
    #[prost(message, optional, tag="6")]
    pub g: ::core::option::Option<GraphProto>,
    // Do not use field below, it's deprecated.
    // optional ValueProto v = 12;         // value - subsumes everything but graph

    /// list of floats
    #[prost(float, repeated, tag="7")]
    pub floats: ::prost::alloc::vec::Vec<f32>,
    /// list of ints
    #[prost(int64, repeated, tag="8")]
    pub ints: ::prost::alloc::vec::Vec<i64>,
    /// list of UTF-8 strings
    #[prost(bytes="vec", repeated, tag="9")]
    pub strings: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// list of tensors
    #[prost(message, repeated, tag="10")]
    pub tensors: ::prost::alloc::vec::Vec<TensorProto>,
    /// list of graph
    #[prost(message, repeated, tag="11")]
    pub graphs: ::prost::alloc::vec::Vec<GraphProto>,
}
/// Nested message and enum types in `AttributeProto`.
pub mod attribute_proto {
    /// Note: this enum is structurally identical to the OpSchema::AttrType
    /// enum defined in schema.h.  If you rev one, you likely need to rev the other.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttributeType {
        Undefined = 0,
        Float = 1,
        Int = 2,
        String = 3,
        Tensor = 4,
        Graph = 5,
        Floats = 6,
        Ints = 7,
        Strings = 8,
        Tensors = 9,
        Graphs = 10,
    }
}
/// Defines information on value, including the name, the type, and
/// the shape of the value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueInfoProto {
    /// This field MUST be present in this version of the IR.
    ///
    /// namespace Value
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// This field MUST be present in this version of the IR.
    #[prost(message, optional, tag="2")]
    pub r#type: ::core::option::Option<TypeProto>,
    /// A human-readable documentation for this value. Markdown is allowed.
    #[prost(string, tag="3")]
    pub doc_string: ::prost::alloc::string::String,
}
/// Nodes
///
/// Computation graphs are made up of a DAG of nodes, which represent what is
/// commonly called a "layer" or "pipeline stage" in machine learning frameworks.
///
/// For example, it can be a node of type "Conv" that takes in an image, a filter 
/// tensor and a bias tensor, and produces the convolved output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeProto {
    /// namespace Value
    #[prost(string, repeated, tag="1")]
    pub input: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// namespace Value
    #[prost(string, repeated, tag="2")]
    pub output: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An optional identifier for this node in a graph.
    /// This field MAY be absent in ths version of the IR.
    ///
    /// namespace Node
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// The symbolic identifier of the Operator to execute.
    ///
    /// namespace Operator
    #[prost(string, tag="4")]
    pub op_type: ::prost::alloc::string::String,
    /// The domain of the OperatorSet that specifies the operator named by op_type.
    ///
    /// namespace Domain
    #[prost(string, tag="7")]
    pub domain: ::prost::alloc::string::String,
    /// Additional named attributes.
    #[prost(message, repeated, tag="5")]
    pub attribute: ::prost::alloc::vec::Vec<AttributeProto>,
    /// A human-readable documentation for this node. Markdown is allowed.
    #[prost(string, tag="6")]
    pub doc_string: ::prost::alloc::string::String,
}
/// Models
///
/// ModelProto is a top-level file/container format for bundling a ML model and
/// associating its computation graph with metadata.
///
/// The semantics of the model are described by the associated GraphProto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelProto {
    /// The version of the IR this model targets. See Version enum above.
    /// This field MUST be present.
    #[prost(int64, tag="1")]
    pub ir_version: i64,
    /// The OperatorSets this model relies on.
    /// All ModelProtos MUST have at least one entry that
    /// specifies which version of the ONNX OperatorSet is
    /// being imported.
    ///
    /// All nodes in the ModelProto's graph will bind against the operator
    /// with the same-domain/same-op_type operator with the HIGHEST version
    /// in the referenced operator sets.
    #[prost(message, repeated, tag="8")]
    pub opset_import: ::prost::alloc::vec::Vec<OperatorSetIdProto>,
    /// The name of the framework or tool used to generate this model.
    /// This field SHOULD be present to indicate which implementation/tool/framework
    /// emitted the model.
    #[prost(string, tag="2")]
    pub producer_name: ::prost::alloc::string::String,
    /// The version of the framework or tool used to generate this model.
    /// This field SHOULD be present to indicate which implementation/tool/framework
    /// emitted the model.
    #[prost(string, tag="3")]
    pub producer_version: ::prost::alloc::string::String,
    /// Domain name of the model.
    /// We use reverse domain names as name space indicators. For example:
    /// `com.facebook.fair` or `com.microsoft.cognitiveservices`
    ///
    /// Together with `model_version` and GraphProto.name, this forms the unique identity of
    /// the graph.
    #[prost(string, tag="4")]
    pub domain: ::prost::alloc::string::String,
    /// The version of the graph encoded. See Version enum below.
    #[prost(int64, tag="5")]
    pub model_version: i64,
    /// A human-readable documentation for this model. Markdown is allowed.
    #[prost(string, tag="6")]
    pub doc_string: ::prost::alloc::string::String,
    /// The parameterized graph that is evaluated to execute the model.
    #[prost(message, optional, tag="7")]
    pub graph: ::core::option::Option<GraphProto>,
    /// Named metadata values; keys should be distinct.
    #[prost(message, repeated, tag="14")]
    pub metadata_props: ::prost::alloc::vec::Vec<StringStringEntryProto>,
}
/// StringStringEntryProto follows the pattern for cross-proto-version maps.
/// See https://developers.google.com/protocol-buffers/docs/proto3#maps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringStringEntryProto {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// Graphs
///
/// A graph defines the computational logic of a model and is comprised of a parameterized 
/// list of nodes that form a directed acyclic graph based on their inputs and outputs.
/// This is the equivalent of the "network" or "graph" in many deep learning
/// frameworks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphProto {
    /// The nodes in the graph, sorted topologically.
    #[prost(message, repeated, tag="1")]
    pub node: ::prost::alloc::vec::Vec<NodeProto>,
    /// The name of the graph.
    ///
    /// namespace Graph
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// A list of named tensor values, used to specify constant inputs of the graph.
    /// Each TensorProto entry must have a distinct name (within the list) that
    /// also appears in the input list.
    #[prost(message, repeated, tag="5")]
    pub initializer: ::prost::alloc::vec::Vec<TensorProto>,
    /// A human-readable documentation for this graph. Markdown is allowed.
    #[prost(string, tag="10")]
    pub doc_string: ::prost::alloc::string::String,
    /// The inputs and outputs of the graph.
    #[prost(message, repeated, tag="11")]
    pub input: ::prost::alloc::vec::Vec<ValueInfoProto>,
    #[prost(message, repeated, tag="12")]
    pub output: ::prost::alloc::vec::Vec<ValueInfoProto>,
    /// Information for the values in the graph. The ValueInfoProto.name's
    /// must be distinct. It is optional for a value to appear in value_info list.
    #[prost(message, repeated, tag="13")]
    pub value_info: ::prost::alloc::vec::Vec<ValueInfoProto>,
}
/// Tensors
///
/// A serialized tensor value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorProto {
    /// The shape of the tensor.
    #[prost(int64, repeated, tag="1")]
    pub dims: ::prost::alloc::vec::Vec<i64>,
    /// The data type of the tensor.
    #[prost(enumeration="tensor_proto::DataType", tag="2")]
    pub data_type: i32,
    #[prost(message, optional, tag="3")]
    pub segment: ::core::option::Option<tensor_proto::Segment>,
    // Tensor content must be organized in row-major order.
    //
    // Depending on the data_type field, exactly one of the fields below with
    // name ending in _data is used to store the elements of the tensor.

    /// For float and complex64 values
    /// Complex64 tensors are encoded as a single array of floats,
    /// with the real components appearing in odd numbered positions,
    /// and the corresponding imaginary component apparing in the
    /// subsequent even numbered position. (e.g., [1.0 + 2.0i, 3.0 + 4.0i]
    /// is encoded as [1.0, 2.0 ,3.0 ,4.0]
    /// When this field is present, the data_type field MUST be FLOAT or COMPLEX64.
    #[prost(float, repeated, tag="4")]
    pub float_data: ::prost::alloc::vec::Vec<f32>,
    /// For int32, uint8, int8, uint16, int16, bool, and float16 values
    /// float16 values must be bit-wise converted to an uint16_t prior
    /// to writing to the buffer.
    /// When this field is present, the data_type field MUST be
    /// INT32, INT16, INT8, UINT16, INT8, BOOL, or FLOAT16
    #[prost(int32, repeated, tag="5")]
    pub int32_data: ::prost::alloc::vec::Vec<i32>,
    /// For strings.
    /// Each element of string_data is a UTF-8 encoded Unicode
    /// string. No trailing null, no leading BOM. The protobuf "string"
    /// scalar type is not used to match ML community conventions.
    /// When this field is present, the data_type field MUST be STRING
    #[prost(bytes="vec", repeated, tag="6")]
    pub string_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// For int64.
    /// When this field is present, the data_type field MUST be INT64
    #[prost(int64, repeated, tag="7")]
    pub int64_data: ::prost::alloc::vec::Vec<i64>,
    /// Optionally, a name for the tensor.
    ///
    /// namespace Value
    #[prost(string, tag="8")]
    pub name: ::prost::alloc::string::String,
    /// A human-readable documentation for this tensor. Markdown is allowed.
    #[prost(string, tag="12")]
    pub doc_string: ::prost::alloc::string::String,
    /// Serializations can either use one of the fields above, or use this
    /// raw bytes field. The only exception is the string case, where one is
    /// required to store the content in the repeated bytes string_data field.
    ///
    /// When this raw_data field is used to store tensor value, elements MUST
    /// be stored in as fixed-width, little-endian order.
    /// Floating-point data types MUST be stored in IEEE 754 format.
    /// Complex64 elements must be written as two consecutive FLOAT values, real component first.
    /// Complex128 elements must be written as two consecutive DOUBLE values, real component first.
    /// Boolean type MUST be written one byte per tensor element (00000001 for true, 00000000 for false).
    ///
    /// Note: the advantage of specific field rather than the raw_data field is
    /// that in some cases (e.g. int data), protobuf does a better packing via
    /// variable length storage, and may lead to smaller binary footprint.
    /// When this field is present, the data_type field MUST NOT be STRING or UNDEFINED
    #[prost(bytes="vec", tag="9")]
    pub raw_data: ::prost::alloc::vec::Vec<u8>,
    /// For double
    /// Complex64 tensors are encoded as a single array of doubles,
    /// with the real components appearing in odd numbered positions,
    /// and the corresponding imaginary component apparing in the
    /// subsequent even numbered position. (e.g., [1.0 + 2.0i, 3.0 + 4.0i]
    /// is encoded as [1.0, 2.0 ,3.0 ,4.0]
    /// When this field is present, the data_type field MUST be DOUBLE or COMPLEX128
    #[prost(double, repeated, tag="10")]
    pub double_data: ::prost::alloc::vec::Vec<f64>,
    /// For uint64 and uint32 values
    /// When this field is present, the data_type field MUST be
    /// UINT32 or UINT64
    #[prost(uint64, repeated, tag="11")]
    pub uint64_data: ::prost::alloc::vec::Vec<u64>,
}
/// Nested message and enum types in `TensorProto`.
pub mod tensor_proto {
    /// For very large tensors, we may want to store them in chunks, in which
    /// case the following fields will specify the segment that is stored in
    /// the current TensorProto.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Segment {
        #[prost(int64, tag="1")]
        pub begin: i64,
        #[prost(int64, tag="2")]
        pub end: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataType {
        Undefined = 0,
        /// Basic types.
        ///
        /// float
        Float = 1,
        /// uint8_t
        Uint8 = 2,
        /// int8_t
        Int8 = 3,
        /// uint16_t
        Uint16 = 4,
        /// int16_t
        Int16 = 5,
        /// int32_t
        Int32 = 6,
        /// int64_t
        Int64 = 7,
        /// string
        String = 8,
        /// bool
        Bool = 9,
        /// Advanced types
        Float16 = 10,
        Double = 11,
        Uint32 = 12,
        Uint64 = 13,
        /// complex with float32 real and imaginary components
        Complex64 = 14,
        /// complex with float64 real and imaginary components
        Complex128 = 15,
    }
}
/// Defines a tensor shape. A dimension can be either an integer value
/// or a symbolic variable. A symbolic variable represents an unknown
/// dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorShapeProto {
    #[prost(message, repeated, tag="1")]
    pub dim: ::prost::alloc::vec::Vec<tensor_shape_proto::Dimension>,
}
/// Nested message and enum types in `TensorShapeProto`.
pub mod tensor_shape_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dimension {
        /// Standard denotation can optionally be used to denote tensor
        /// dimensions with standard semantic descriptions to ensure
        /// that operations are applied to the correct axis of a tensor.
        /// Refer to https://github.com/onnx/onnx/blob/master/docs/DimensionDenotation.md#denotation-definition
        /// for pre-defined dimension denotations.
        #[prost(string, tag="3")]
        pub denotation: ::prost::alloc::string::String,
        #[prost(oneof="dimension::Value", tags="1, 2")]
        pub value: ::core::option::Option<dimension::Value>,
    }
    /// Nested message and enum types in `Dimension`.
    pub mod dimension {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(int64, tag="1")]
            DimValue(i64),
            /// namespace Shape
            #[prost(string, tag="2")]
            DimParam(::prost::alloc::string::String),
        }
    }
}
/// Types
///
/// The standard ONNX data types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeProto {
    /// An optional denotation can be used to denote the whole 
    /// type with a standard semantic description as to what is 
    /// stored inside. Refer to https://github.com/onnx/onnx/blob/master/docs/TypeDenotation.md#type-denotation-definition
    /// for pre-defined type denotations.
    #[prost(string, tag="6")]
    pub denotation: ::prost::alloc::string::String,
    #[prost(oneof="type_proto::Value", tags="1")]
    pub value: ::core::option::Option<type_proto::Value>,
}
/// Nested message and enum types in `TypeProto`.
pub mod type_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tensor {
        /// This field MUST NOT have the value of UNDEFINED
        /// This field MUST be present for this version of the IR.
        #[prost(enumeration="super::tensor_proto::DataType", tag="1")]
        pub elem_type: i32,
        #[prost(message, optional, tag="2")]
        pub shape: ::core::option::Option<super::TensorShapeProto>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The type of a tensor.
        #[prost(message, tag="1")]
        TensorType(Tensor),
    }
}
/// Operator Sets
///
/// OperatorSets are uniquely identified by a (domain, opset_version) pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorSetIdProto {
    /// The domain of the operator set being identified.
    /// The empty string ("") or absence of this field implies the operator
    /// set that is defined as part of the ONNX specification.
    /// This field MUST be present in this version of the IR when referring to any other operator set.
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    /// The version of the operator set being identified.
    /// This field MUST be present in this version of the IR.
    #[prost(int64, tag="2")]
    pub version: i64,
}
// Overview
//
// ONNX is an open specification that is comprised of the following components:
//
// 1)  A definition of an extensible computation graph model.
// 2)  Definitions of standard data types.
// 3)  Definitions of built-in operators.
//
// This document describes the syntax of models and their computation graphs,
// as well as the standard data types. Together, they are referred to as the ONNX
// Intermediate Representation, or 'IR' for short. 
//
// The normative semantic specification of the ONNX IR is found in docs/IR.md.
// Definitions of the built-in neural network operators may be found in docs/Operators.md.

// Notes
//
// Release
//
// We are still in the very early stage of defining ONNX. The current
// version of ONNX is a starting point. While we are actively working
// towards a complete spec, we would like to get the community involved
// by sharing our working version of ONNX.
//
// Protobuf compatibility
// 
// To simplify framework compatibility, ONNX is defined using the subset of protobuf 
// that is compatible with both protobuf v2 and v3. This means that we do not use any
// protobuf features that are only available in one of the two versions.
//
// Here are the most notable contortions we have to carry out to work around
// these limitations:
//
//   - No 'map' (added protobuf 3.0). We instead represent mappings as lists
//     of key-value pairs, where order does not matter and duplicates
//     are not allowed.

/// Versioning
///
/// ONNX versioning is specified in docs/IR.md and elaborated on in docs/Versioning.md
///
/// To be compatible with both proto2 and proto3, we will use a version number
/// that is not defined by the default value but an explicit enum number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Version {
    /// proto3 requires the first enum value to be zero.
    /// We add this just to appease the compiler.
    StartVersion = 0,
    /// The version field is always serialized and we will use it to store the
    /// version that the  graph is generated from. This helps us set up version
    /// control. 
    /// For the IR, we are using simple numbers starting with with 0x00000001, 
    /// which was the version we published on Oct 10, 2017.
    IrVersion20171010 = 1,
    /// IR_VERSION 2 published on Oct 30, 2017
    /// - Added type discriminator to AttributeProto to support proto3 users
    IrVersion20171030 = 2,
    /// IR VERSION 3 published on Nov 3, 2017
    /// - For operator versioning:
    ///    - Added new message OperatorSetIdProto
    ///    - Added opset_import in ModelProto
    /// - For vendor extensions, added domain in NodeProto
    IrVersion = 3,
}
