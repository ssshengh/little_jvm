use thiserror::Error;

/// 解析文件 Result
pub type ClassFileParserResult<T> = std::result::Result<T, ClassFileParserError>;

/// 解析文件 error
#[derive(Error, Debug, PartialEq)]
pub enum ClassFileParserError {
    #[error("The major_version={0}, minor_version={1} not match any SdkVersion! Maybe not supported yet.")]
    ClassFileVersionError(u16, u16),
    #[error("The index={0} of constant pool is error!")]
    WrongConstantPoolIndexError(u16),
    #[error("The index={0} points to a PhantomEntry, please make sure the second slots of Long and Double are not selected.")]
    ConstantPoolIndexToPhantomEntryError(u16),
    #[error("The filed descriptor={0} is invalidate!")]
    InvalidFiledTypeDescriptor(String),
    #[error("The method descriptor={0} is invalidate!")]
    InvalidMethodTypeDescriptor(String),
    #[error("expected a zero byte after {0} and the index at address {1}")]
    InvalidInstructionData(String, usize),
    #[error("cannot read instruction as address={0}")]
    CanNotReadInstructionError(usize),
    #[error("cannot find arguments for instruction at address={0}")]
    CanNotFindArgumentsError(usize),
    #[error("invalid jump offset at address={0}")]
    InvalidOffsetError(usize),
    #[error("unexpected end of data")]
    UnexpectedEndOfData,
    #[error("invalid cesu8 string")]
    InvalidCesu8String,
    #[error("Invalidate class data={name:?}, is invalidate constant pool idx={is_invalidate_constant_pool_idx:?}")]
    InvalidClassData{
        name: String,
        is_invalidate_constant_pool_idx: bool,
    },
    #[error("Unsupported version majorVersion={0}, minorVersion={1}")]
    UnsupportedVersion(u16, u16),
    #[error("Error while parsing a given type descriptor in the file={0}")]
    InvalidTypeDescriptor(String),
}