use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_invoke_method(port_: i64, channel: *mut wire_MethodChannel) {
    wire_invoke_method_impl(port_, channel)
}

#[no_mangle]
pub extern "C" fn wire_find_my_local_ip(port_: i64) {
    wire_find_my_local_ip_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_init_(port_: i64) {
    wire_init__impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_bluetooth_adapter_state(port_: i64) {
    wire_get_bluetooth_adapter_state_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_connect_to_bluetooth_device(
    port_: i64,
    service_uuid: *mut wire_uint_8_list,
) {
    wire_connect_to_bluetooth_device_impl(port_, service_uuid)
}

#[no_mangle]
pub extern "C" fn wire_disconnect_bluetooth_device(
    port_: i64,
    service_uuid: *mut wire_uint_8_list,
) {
    wire_disconnect_bluetooth_device_impl(port_, service_uuid)
}

#[no_mangle]
pub extern "C" fn wire_bluetooth_write_bytes(
    port_: i64,
    service_uuid: *mut wire_uint_8_list,
    address: *mut wire_uint_8_list,
    data: *mut wire_uint_8_list,
) {
    wire_bluetooth_write_bytes_impl(port_, service_uuid, address, data)
}

#[no_mangle]
pub extern "C" fn wire_bluetooth_start_scan(port_: i64, timeout_sec: i64) {
    wire_bluetooth_start_scan_impl(port_, timeout_sec)
}

#[no_mangle]
pub extern "C" fn wire_bluetooth_stop_scan(port_: i64) {
    wire_bluetooth_stop_scan_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_scanner_process_image(port_: i64, image_path: *mut wire_uint_8_list) {
    wire_scanner_process_image_impl(port_, image_path)
}

#[no_mangle]
pub extern "C" fn wire_get_windows_info(port_: i64) {
    wire_get_windows_info_impl(port_)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_method_channel_0() -> *mut wire_MethodChannel {
    support::new_leak_box_ptr(wire_MethodChannel::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::microseconds(self)
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<BluetoothDevice> for wire_BluetoothDevice {
    fn wire2api(self) -> BluetoothDevice {
        BluetoothDevice {
            name: self.name.wire2api(),
            address: self.address.wire2api(),
            status: self.status.wire2api(),
            service_uuid: self.service_uuid.wire2api(),
        }
    }
}

impl Wire2Api<MethodChannel> for *mut wire_MethodChannel {
    fn wire2api(self) -> MethodChannel {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MethodChannel>::wire2api(*wrap).into()
    }
}

impl Wire2Api<MethodChannel> for wire_MethodChannel {
    fn wire2api(self) -> MethodChannel {
        MethodChannel {
            command: self.command.wire2api(),
            device: self.device.wire2api(),
            bytes: self.bytes.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BluetoothDevice {
    name: *mut wire_uint_8_list,
    address: *mut wire_uint_8_list,
    status: bool,
    service_uuid: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MethodChannel {
    command: *mut wire_uint_8_list,
    device: wire_BluetoothDevice,
    bytes: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_BluetoothDevice {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            address: core::ptr::null_mut(),
            status: Default::default(),
            service_uuid: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_BluetoothDevice {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_MethodChannel {
    fn new_with_null_ptr() -> Self {
        Self {
            command: core::ptr::null_mut(),
            device: Default::default(),
            bytes: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_MethodChannel {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
