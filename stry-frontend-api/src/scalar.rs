use {
    juniper::{
        parser::{ParseError, ScalarToken, Token},
        serde::de::{self, Visitor},
        InputValue, ParseScalarResult, ScalarValue, Value,
    },
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[derive(juniper::GraphQLScalarValue)]
pub enum CustomScalarValue {
    SignedInt(i32),
    SignedLong(i64),
    UnsignedInt(u32),
    UnsignedLong(u64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl ScalarValue for CustomScalarValue {
    type Visitor = CustomScalarValueVisitor;

    fn as_int(&self) -> Option<i32> {
        match *self {
            CustomScalarValue::SignedInt(ref i) => Some(*i),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match *self {
            CustomScalarValue::String(ref s) => Some(s.clone()),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match *self {
            CustomScalarValue::String(ref s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match *self {
            CustomScalarValue::SignedInt(ref i) => Some(*i as f64),
            CustomScalarValue::SignedLong(ref i) => Some(*i as f64),
            CustomScalarValue::UnsignedInt(ref i) => Some(*i as f64),
            CustomScalarValue::UnsignedLong(ref i) => Some(*i as f64),
            CustomScalarValue::Float(ref f) => Some(*f),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match *self {
            CustomScalarValue::Boolean(ref b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct CustomScalarValueVisitor;

impl<'de> Visitor<'de> for CustomScalarValueVisitor {
    type Value = CustomScalarValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid input value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<CustomScalarValue, E> {
        Ok(CustomScalarValue::Boolean(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<CustomScalarValue, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::SignedInt(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<CustomScalarValue, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::SignedLong(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<CustomScalarValue, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::UnsignedInt(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<CustomScalarValue, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::UnsignedLong(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<CustomScalarValue, E> {
        Ok(CustomScalarValue::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<CustomScalarValue, E>
    where
        E: de::Error,
    {
        self.visit_string(value.into())
    }

    fn visit_string<E>(self, value: String) -> Result<CustomScalarValue, E> {
        Ok(CustomScalarValue::String(value))
    }
}

#[juniper::graphql_scalar(name = "UnsignedInt")]
impl GraphQLScalar for u32 {
    fn resolve(&self) -> Value {
        Value::scalar(*self)
    }

    fn from_input_value(v: &InputValue) -> Option<u32> {
        match *v {
            InputValue::Scalar(CustomScalarValue::UnsignedInt(i)) => Some(i),
            _ => None,
        }
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, CustomScalarValue> {
        if let ScalarToken::Int(v) = value {
            v.parse()
                .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
                .map(|s: u32| s.into())
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
}

#[juniper::graphql_scalar(name = "UnsignedLong")]
impl GraphQLScalar for u64 {
    fn resolve(&self) -> Value {
        Value::scalar(*self)
    }

    fn from_input_value(v: &InputValue) -> Option<u64> {
        match *v {
            InputValue::Scalar(CustomScalarValue::UnsignedLong(i)) => Some(i),
            _ => None,
        }
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, CustomScalarValue> {
        if let ScalarToken::Int(v) = value {
            v.parse()
                .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
                .map(|s: u64| s.into())
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
}

// TODO: Figure out a custom i32 type
// #[juniper::graphql_scalar(name = "SignedInt")]
// impl GraphQLScalar for i32 {
//     fn resolve(&self) -> Value {
//         Value::scalar(*self)
//     }

//     fn from_input_value(v: &InputValue) -> Option<i32> {
//         match *v {
//             InputValue::Scalar(CustomScalarValue::SignedInt(i)) => Some(i),
//             _ => None,
//         }
//     }

//     fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, CustomScalarValue> {
//         if let ScalarToken::Int(v) = value {
//             v.parse()
//                 .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
//                 .map(|s: i32| s.into())
//         } else {
//             Err(ParseError::UnexpectedToken(Token::Scalar(value)))
//         }
//     }
// }

#[juniper::graphql_scalar(name = "SignedLong")]
impl GraphQLScalar for i64 {
    fn resolve(&self) -> Value {
        Value::scalar(*self)
    }

    fn from_input_value(v: &InputValue) -> Option<i64> {
        match *v {
            InputValue::Scalar(CustomScalarValue::SignedLong(i)) => Some(i),
            _ => None,
        }
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, CustomScalarValue> {
        if let ScalarToken::Int(v) = value {
            v.parse()
                .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
                .map(|s: i64| s.into())
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
}
