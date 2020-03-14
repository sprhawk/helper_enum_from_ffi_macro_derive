pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    
    type idevice_error_t = i32;
    pub const idevice_error_t_IDEVICE_E_SUCCESS: idevice_error_t = 0;
    pub const idevice_error_t_IDEVICE_E_INVALID_ARG: idevice_error_t = -1;
    pub const idevice_error_t_IDEVICE_E_UNKNOWN_ERROR: idevice_error_t = -2;
    pub const idevice_error_t_IDEVICE_E_NO_DEVICE: idevice_error_t = -3;
    pub const idevice_error_t_IDEVICE_E_NOT_ENOUGH_DATA: idevice_error_t = -4;
    pub const idevice_error_t_IDEVICE_E_SSL_ERROR: idevice_error_t = -6;
    pub const idevice_error_t_IDEVICE_E_TIMEOUT: idevice_error_t = -7;
}

pub type DeviceResult<T> = Result<T, DeviceError>;

use helper_enum_from_ffi_macro_derive::match_enum_from_ffi;

#[match_enum_from_ffi(prefix="ffi::idevice_error_t_IDEVICE_E_")]
#[derive(Debug, PartialEq)]
pub enum DeviceError {
    #[ffi_enum(undefined)]
    Undefined(i32), // this is used to handle when enum value returned from C has been changed somehow
    #[ffi_enum(suffix="SUCCESS", success)]
    Success,
    #[ffi_enum(suffix="INVALID_ARG")]
    InvalidArg,
    #[ffi_enum(suffix="UNKNOWN_ERROR")]
    UnknownError,
    #[ffi_enum(suffix="NO_DEVICE")]
    NoDevice,
    #[ffi_enum(suffix="NOT_ENOUGH_DATA")]
    NotEnoughData,
    #[ffi_enum(suffix="SSL_ERROR")]
    SslError,
    #[ffi_enum(suffix="TIMEOUT")]
    Timeout,
}


#[test]
fn undefined() {
    let n = 199;
    let e = match_device_error!(n);
    assert_eq!(e, DeviceError::Undefined(n));
}

#[test]
fn predefined() {
    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_SUCCESS);
    assert_eq!(e, DeviceError::Success);
    
    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_INVALID_ARG);
    assert_eq!(e, DeviceError::InvalidArg);

    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_UNKNOWN_ERROR);
    assert_eq!(e, DeviceError::UnknownError);

    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_NO_DEVICE);
    assert_eq!(e, DeviceError::NoDevice);

    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_NOT_ENOUGH_DATA);
    assert_eq!(e, DeviceError::NotEnoughData);

    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_SSL_ERROR);
    assert_eq!(e, DeviceError::SslError);

    let e = match_device_error!(ffi::idevice_error_t_IDEVICE_E_TIMEOUT);
    assert_eq!(e, DeviceError::Timeout);
}
