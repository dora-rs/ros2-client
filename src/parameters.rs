/// Rust-like representation of ROS2 Parameter
#[derive(Debug, Clone)]
pub struct Parameter {
  pub name: String,
  pub value: ParameterValue,
}

/// Rust-like representation of ROS2
/// [ParameterValue](https://github.com/ros2/rcl_interfaces/blob/master/rcl_interfaces/msg/ParameterValue.msg)
#[derive(Debug, Clone)]
pub enum ParameterValue {
  NotSet,
  Boolean(bool),
  Integer(i64),
  Double(f64),
  String(String),
  ByteArray(Vec<u8>),
  BooleanArray(Vec<bool>),
  IntegerArray(Vec<i64>),
  DoubleArray(Vec<f64>),
  StringArray(Vec<String>),
}

impl ParameterValue {
  // https://github.com/ros2/rcl_interfaces/blob/rolling/rcl_interfaces/msg/ParameterType.msg
  pub fn to_parameter_type_enum(p: &ParameterValue) -> u8 {
    match p {
      ParameterValue::NotSet => 0, 
      ParameterValue::Boolean(_) => 1,
      ParameterValue::Integer(_) => 2,
      ParameterValue::Double(_d) => 3,
      ParameterValue::String(_s) => 4,
      ParameterValue::ByteArray(_a) => 5,
      ParameterValue::BooleanArray(_a) => 6,
      ParameterValue::IntegerArray(_a) => 7,
      ParameterValue::DoubleArray(_a) => 8,
      ParameterValue::StringArray(_a) => 9,
    }
  }
}

impl From<raw::Parameter> for Parameter {
  fn from(rp: raw::Parameter) -> Self {
    Parameter {
      name: rp.name,
      value: rp.value.into(),
    }
  }
}

impl From<raw::ParameterValue> for ParameterValue {
  fn from(rpv: raw::ParameterValue) -> ParameterValue {
    match rpv.ptype {
      raw::ParameterType::NOT_SET => ParameterValue::NotSet,
      raw::ParameterType::BOOL => ParameterValue::Boolean(rpv.boolean_value),
      raw::ParameterType::INTEGER => ParameterValue::Integer(rpv.int_value),
      raw::ParameterType::DOUBLE => ParameterValue::Double(rpv.double_value),
      raw::ParameterType::STRING => ParameterValue::String(rpv.string_value),

      raw::ParameterType::BYTE_ARRAY => ParameterValue::ByteArray(rpv.byte_array),
      raw::ParameterType::BOOL_ARRAY => ParameterValue::BooleanArray(rpv.bool_array),
      raw::ParameterType::INTEGER_ARRAY => ParameterValue::IntegerArray(rpv.int_array),
      raw::ParameterType::DOUBLE_ARRAY => ParameterValue::DoubleArray(rpv.double_array),
      raw::ParameterType::STRING_ARRAY => ParameterValue::StringArray(rpv.string_array),

      _ =>
      // This may be an unspecified case.
      // TODO: Do something better, at least log a warning.
      {
        ParameterValue::NotSet
      }
    }
  }
}

impl From<Parameter> for raw::Parameter {
  fn from(p: Parameter) -> raw::Parameter {
    raw::Parameter {
      name: p.name,
      value: p.value.into(),
    }
  }
}

impl From<ParameterValue> for raw::ParameterValue {
  fn from(p: ParameterValue) -> raw::ParameterValue {
    let mut value = raw::ParameterValue {
      ptype: raw::ParameterType::NOT_SET,
      boolean_value: false,
      int_value: 0,
      double_value: 0.0,
      string_value: String::new(),
      byte_array: Vec::new(),
      int_array: Vec::new(),
      bool_array: Vec::new(),
      double_array: Vec::new(),
      string_array: Vec::new(),
    };
    match p {
      ParameterValue::NotSet => (), // already there
      ParameterValue::Boolean(b) => {
        value.ptype = raw::ParameterType::BOOL;
        value.boolean_value = b;
      }
      ParameterValue::Integer(i) => {
        value.ptype = raw::ParameterType::INTEGER;
        value.int_value = i;
      }
      ParameterValue::Double(d) => {
        value.ptype = raw::ParameterType::DOUBLE;
        value.double_value = d;
      }
      ParameterValue::String(s) => {
        value.ptype = raw::ParameterType::STRING;
        value.string_value = s;
      }
      ParameterValue::ByteArray(a) => {
        value.ptype = raw::ParameterType::BYTE_ARRAY;
        value.byte_array = a;
      }
      ParameterValue::BooleanArray(a) => {
        value.ptype = raw::ParameterType::BOOL_ARRAY;
        value.bool_array = a;
      }
      ParameterValue::IntegerArray(a) => {
        value.ptype = raw::ParameterType::INTEGER_ARRAY;
        value.int_array = a;
      }
      ParameterValue::DoubleArray(a) => {
        value.ptype = raw::ParameterType::DOUBLE_ARRAY;
        value.double_array = a;
      }
      ParameterValue::StringArray(a) => {
        value.ptype = raw::ParameterType::STRING_ARRAY;
        value.string_array = a;
      }
    }
    value
  }
} // impl From

// more Rust-like version of SetParamtersResult
pub type SetParametersResult = Result<(),String>;

impl From<SetParametersResult> for raw::SetParametersResult {
  fn from(s: SetParametersResult) -> raw::SetParametersResult {
    match s {
      Ok(_) => 
        raw::SetParametersResult { successful: true, reason: "".to_string() },
      Err(reason) =>
        raw::SetParametersResult { successful: false, reason },
    }
  }
}



// This submodule contains raw, ROS2 -compatible Parameters.
// These are for sending over the wire.
pub mod raw {
  use rustdds::*;
  use serde::{Deserialize, Serialize};

  /// ROS2 [ParameterEvent](https://github.com/ros2/rcl_interfaces/blob/master/rcl_interfaces/msg/ParameterEvent.msg)
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ParameterEvent {
    pub timestamp: Timestamp,
    // fully qualified path
    pub node: String,
    pub new_parameters: Vec<Parameter>,
    pub changed_parameters: Vec<Parameter>,
    pub deleted_parameters: Vec<Parameter>,
  }

  /// [Parameter](https://github.com/ros2/rcl_interfaces/blob/master/rcl_interfaces/msg/Parameter.msg)
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
  }

  /// [ParameterValue](https://github.com/ros2/rcl_interfaces/blob/master/rcl_interfaces/msg/ParameterValue.msg)
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ParameterValue {
    pub ptype: u8,
    pub boolean_value: bool,
    pub int_value: i64,
    pub double_value: f64,
    pub string_value: String,
    pub byte_array: Vec<u8>,
    pub bool_array: Vec<bool>,
    pub int_array: Vec<i64>,
    pub double_array: Vec<f64>,
    pub string_array: Vec<String>,
  }

  /// ROS2 defines this as an empty .msg
  /// [ParameterType](https://github.com/ros2/rcl_interfaces/blob/master/rcl_interfaces/msg/ParameterType.msg)
  pub struct ParameterType {}

  impl ParameterType {
    pub const NOT_SET: u8 = 0;

    pub const BOOL: u8 = 1;
    pub const INTEGER: u8 = 2;
    pub const DOUBLE: u8 = 3;
    pub const STRING: u8 = 4;
    pub const BYTE_ARRAY: u8 = 5;
    pub const BOOL_ARRAY: u8 = 6;
    pub const INTEGER_ARRAY: u8 = 7;
    pub const DOUBLE_ARRAY: u8 = 8;
    pub const STRING_ARRAY: u8 = 9;
  }

  /// https://github.com/ros2/rcl_interfaces/blob/rolling/rcl_interfaces/msg/SetParametersResult.msg
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct SetParametersResult {
    pub successful: bool,
    pub reason: String,
  }
}
