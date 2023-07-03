#[derive(Debug, PartialEq)]
pub enum RespType {
    TypeString,
    TypeError,
    TypeInt,
    TypeBulkBytes,
    TypeArray,
}
//
// impl fmt::Display for RespType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             RespType::TypeString => write!(f, "<string>"),
//             RespType::TypeError => write!(f, "<error>"),
//             RespType::TypeInt => write!(f, "<int>"),
//             RespType::TypeBulkBytes => write!(f, "<bulkbytes>"),
//             RespType::TypeArray => write!(f, "<array>"),
//         }
//     }
// }
//
pub struct Resp {}

// pub(crate) struct Resp<T> {
//     rtype: RespType,
//     value: T,
//     array: Vec<Resp<T>>,
// }
//
// impl Resp<T> {
//     fn is_string(&self) -> bool {
//         self.rtype == RespType::TypeString
//     }
//
//     fn is_error(&self) -> bool {
//         self.rtype == RespType::TypeError
//     }
//
//     fn is_int(&self) -> bool {
//         self.rtype == RespType::TypeInt
//     }
//
//     fn is_bulk_bytes(&self) -> bool {
//         self.rtype == RespType::TypeBulkBytes
//     }
//
//     fn is_array(&self) -> bool {
//         self.rtype == RespType::TypeArray
//     }
//
//     fn new_string(value: String) -> Resp<String> {
//         Resp {
//             rtype: RespType::TypeString,
//             value,
//             array: Vec::new(),
//         }
//     }
//
//     fn new_error(value: String) -> Resp<String> {
//         Resp {
//             rtype: RespType::TypeError,
//             value,
//             array: Vec::new(),
//         }
//     }
//
//     fn new_errorf(format: &str, args: &[&dyn fmt::Display]) -> Resp<String> {
//         Resp::new_error(format!(format, args))
//     }
//
//     fn new_int(value: u8) -> Resp<u8> {
//         Resp {
//             rtype: RespType::TypeInt,
//             value,
//             array: Vec::new(),
//         }
//     }
//
//     fn new_bulk_bytes(value: u8) -> Resp<u8> {
//         Resp {
//             rtype: RespType::TypeBulkBytes,
//             value,
//             array: Vec::new(),
//         }
//     }
//
//     fn new_array(array: Vec<Resp<T>>) -> Resp<T> {
//         Resp {
//             rtype: RespType::TypeArray,
//             value: "",
//             array,
//         }
//     }
// }
