/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
#![allow(unused_imports)]
use libc::{c_int, c_uint, c_void};
use types::*;
use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;

pub struct PersonQObject {}

#[derive (Clone)]
pub struct PersonEmitter {
    qobject: Arc<Mutex<*const PersonQObject>>,
    user_name_changed: fn(*const PersonQObject),
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
}

pub trait PersonTrait {
    fn create(emit: PersonEmitter) -> Self;
    fn emit(&self) -> &PersonEmitter;
    fn get_user_name(&self) -> String;
    fn set_user_name(&mut self, value: String);
}

#[no_mangle]
pub extern "C" fn person_new(qobject: *const PersonQObject,
        user_name_changed: fn(*const PersonQObject))
        -> *mut Person {
    let emit = PersonEmitter {
        qobject: Arc::new(Mutex::new(qobject)),
        user_name_changed: user_name_changed,
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
