/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
#![allow(unused_imports)]
use libc::{c_int, c_uint, c_void};
use types::*;
use std::sync::{Arc, Mutex};
use std::ptr::null;

use testimplementation::*;

pub struct PersonQObject {}

#[derive (Clone)]
pub struct PersonEmitter {
    qobject: Arc<Mutex<*const PersonQObject>>,
    user_name_changed: fn(*const PersonQObject),
    age_changed: fn(*const PersonQObject),
    active_changed: fn(*const PersonQObject),
    icon_changed: fn(*const PersonQObject),
}

unsafe impl Send for PersonEmitter {}

impl PersonEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn user_name_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.user_name_changed)(ptr);
        }
    }
    pub fn age_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.age_changed)(ptr);
        }
    }
    pub fn active_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.active_changed)(ptr);
        }
    }
    pub fn icon_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.icon_changed)(ptr);
        }
    }
}

pub trait PersonTrait {
    fn create(emit: PersonEmitter) -> Self;
    fn emit(&self) -> &PersonEmitter;
    fn get_user_name(&self) -> String;
    fn set_user_name(&mut self, value: String);
    fn get_age(&self) -> c_int;
    fn get_active(&self) -> bool;
    fn set_active(&mut self, value: bool);
    fn get_icon(&self) -> Vec<u8>;
    fn set_icon(&mut self, value: Vec<u8>);
}

#[no_mangle]
pub extern "C" fn person_new(qobject: *const PersonQObject,
        user_name_changed: fn(*const PersonQObject),
        age_changed: fn(*const PersonQObject),
        active_changed: fn(*const PersonQObject),
        icon_changed: fn(*const PersonQObject))
        -> *mut Person {
    let emit = PersonEmitter {
        qobject: Arc::new(Mutex::new(qobject)),
        user_name_changed: user_name_changed,
        age_changed: age_changed,
        active_changed: active_changed,
        icon_changed: icon_changed,
    };
    let d = Person::create(emit);
    Box::into_raw(Box::new(d))
}

#[no_mangle]
pub unsafe extern "C" fn person_free(ptr: *mut Person) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn person_user_name_get(ptr: *const Person,
        p: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).get_user_name();
    set(p, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn person_user_name_set(ptr: *mut Person, v: QStringIn) {
    (&mut *ptr).set_user_name(v.convert());
}

#[no_mangle]
pub unsafe extern "C" fn person_age_get(ptr: *const Person) -> c_int {
    (&*ptr).get_age()
}

#[no_mangle]
pub unsafe extern "C" fn person_active_get(ptr: *const Person) -> bool {
    (&*ptr).get_active()
}

#[no_mangle]
pub unsafe extern "C" fn person_active_set(ptr: *mut Person, v: bool) {
    (&mut *ptr).set_active(v);
}

#[no_mangle]
pub unsafe extern "C" fn person_icon_get(ptr: *const Person,
        p: *mut c_void,
        set: fn(*mut c_void, QByteArray)) {
    let data = (&*ptr).get_icon();
    set(p, QByteArray::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn person_icon_set(ptr: *mut Person, v: QByteArray) {
    (&mut *ptr).set_icon(v.convert());
}

pub struct DirectoryQObject {}

#[derive (Clone)]
pub struct DirectoryEmitter {
    qobject: Arc<Mutex<*const DirectoryQObject>>,
    path_changed: fn(*const DirectoryQObject),
}

unsafe impl Send for DirectoryEmitter {}

impl DirectoryEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn path_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.path_changed)(ptr);
        }
    }
}

pub struct DirectoryList {
    qobject: *const DirectoryQObject,
    directory_begin_insert_rows: fn(*const DirectoryQObject, c_int, c_int),
    directory_end_insert_rows: fn(*const DirectoryQObject),
    directory_begin_remove_rows: fn(*const DirectoryQObject, c_int, c_int),
    directory_end_remove_rows: fn(*const DirectoryQObject),
}

impl DirectoryList {
    pub fn directory_begin_insert_rows(&self, first: c_int, last: c_int) {
        (self.directory_begin_insert_rows)(self.qobject, first, last);
    }
    pub fn directory_end_insert_rows(&self) {
        (self.directory_end_insert_rows)(self.qobject);
    }
    pub fn directory_begin_remove_rows(&self, first: c_int, last: c_int) {
        (self.directory_begin_remove_rows)(self.qobject, first, last);
    }
    pub fn directory_end_remove_rows(&self) {
        (self.directory_end_remove_rows)(self.qobject);
    }
}

pub trait DirectoryTrait {
    fn create(emit: DirectoryEmitter, model: DirectoryList) -> Self;
    fn emit(&self) -> &DirectoryEmitter;
    fn get_path(&self) -> String;
    fn set_path(&mut self, value: String);
    fn row_count(&self) -> c_int;
    fn can_fetch_more(&self) -> bool { false }
    fn fetch_more(&self) {}
    fn file_icon(&self, row: c_int) -> Vec<u8>;
    fn file_path(&self, row: c_int) -> String;
    fn file_name(&self, row: c_int) -> String;
    fn file_permissions(&self, row: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn directory_new(qobject: *const DirectoryQObject,
        path_changed: fn(*const DirectoryQObject),
        directory_begin_insert_rows: fn(*const DirectoryQObject,
            c_int,
            c_int),
        directory_end_insert_rows: fn(*const DirectoryQObject),
        directory_begin_remove_rows: fn(*const DirectoryQObject,
            c_int,
            c_int),
        directory_end_remove_rows: fn(*const DirectoryQObject))
        -> *mut Directory {
    let emit = DirectoryEmitter {
        qobject: Arc::new(Mutex::new(qobject)),
        path_changed: path_changed,
    };
    let model = DirectoryList {
        qobject: qobject,
        directory_begin_insert_rows: directory_begin_insert_rows,
        directory_end_insert_rows: directory_end_insert_rows,
        directory_begin_remove_rows: directory_begin_remove_rows,
        directory_end_remove_rows: directory_end_remove_rows,
    };
    let d = Directory::create(emit, model);
    Box::into_raw(Box::new(d))
}

#[no_mangle]
pub unsafe extern "C" fn directory_free(ptr: *mut Directory) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn directory_path_get(ptr: *const Directory,
        p: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).get_path();
    set(p, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn directory_path_set(ptr: *mut Directory, v: QStringIn) {
    (&mut *ptr).set_path(v.convert());
}

#[no_mangle]
pub unsafe extern "C" fn directory_row_count(ptr: *const Directory) -> c_int {
    (&*ptr).row_count()
}
#[no_mangle]
pub unsafe extern "C" fn directory_can_fetch_more(ptr: *const Directory) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn directory_fetch_more(ptr: *mut Directory) {
    (&mut *ptr).fetch_more()
}

#[no_mangle]
pub unsafe extern "C" fn directory_data_file_icon(ptr: *const Directory,
                                    row: c_int,
        d: *mut c_void,
        set: fn(*mut c_void, QByteArray)) {
    let data = (&*ptr).file_icon(row);
    set(d, QByteArray::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn directory_data_file_path(ptr: *const Directory,
                                    row: c_int,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).file_path(row);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn directory_data_file_name(ptr: *const Directory,
                                    row: c_int,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).file_name(row);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn directory_data_file_permissions(ptr: *const Directory, row: c_int) -> c_int {
    (&*ptr).file_permissions(row)
}
