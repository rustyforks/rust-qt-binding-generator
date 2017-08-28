/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_int, c_void, uint8_t, uint16_t};
use std::slice;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use time_series_implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> From<Option<T>> for COption<T> where T: Default {
    fn from(t: Option<T>) -> COption <T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true
            }
        } else {
            COption {
                data: T::default(),
                some: false
            }
        }
    }
}


#[repr(C)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}

pub struct TimeSeriesQObject {}

#[derive (Clone)]
pub struct TimeSeriesEmitter {
    qobject: Arc<Mutex<*const TimeSeriesQObject>>,
    new_data_ready: fn(*const TimeSeriesQObject),
}

unsafe impl Send for TimeSeriesEmitter {}

impl TimeSeriesEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn new_data_ready(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

pub struct TimeSeriesList {
    qobject: *const TimeSeriesQObject,
    data_changed: fn(*const TimeSeriesQObject, usize, usize),
    begin_reset_model: fn(*const TimeSeriesQObject),
    end_reset_model: fn(*const TimeSeriesQObject),
    begin_insert_rows: fn(*const TimeSeriesQObject, usize, usize),
    end_insert_rows: fn(*const TimeSeriesQObject),
    begin_remove_rows: fn(*const TimeSeriesQObject, usize, usize),
    end_remove_rows: fn(*const TimeSeriesQObject),
}

impl TimeSeriesList {
    pub fn data_changed(&self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait TimeSeriesTrait {
    fn create(emit: TimeSeriesEmitter, model: TimeSeriesList) -> Self;
    fn emit(&self) -> &TimeSeriesEmitter;
    fn row_count(&self) -> usize;
    fn can_fetch_more(&self) -> bool { false }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn input(&self, item: usize) -> u32;
    fn set_input(&mut self, item: usize, u32) -> bool;
    fn result(&self, item: usize) -> u32;
    fn set_result(&mut self, item: usize, u32) -> bool;
}

#[no_mangle]
pub extern "C" fn time_series_new(time_series: *mut TimeSeriesQObject,
        new_data_ready: fn(*const TimeSeriesQObject),
        data_changed: fn(*const TimeSeriesQObject, usize, usize),
        begin_reset_model: fn(*const TimeSeriesQObject),
        end_reset_model: fn(*const TimeSeriesQObject),
        begin_insert_rows: fn(*const TimeSeriesQObject,
            usize,
            usize),
        end_insert_rows: fn(*const TimeSeriesQObject),
        begin_remove_rows: fn(*const TimeSeriesQObject,
            usize,
            usize),
        end_remove_rows: fn(*const TimeSeriesQObject))
        -> *mut TimeSeries {
    let time_series_emit = TimeSeriesEmitter {
        qobject: Arc::new(Mutex::new(time_series)),
        new_data_ready: new_data_ready,
    };
    let model = TimeSeriesList {
        qobject: time_series,
        data_changed: data_changed,
        begin_reset_model: begin_reset_model,
        end_reset_model: end_reset_model,
        begin_insert_rows: begin_insert_rows,
        end_insert_rows: end_insert_rows,
        begin_remove_rows: begin_remove_rows,
        end_remove_rows: end_remove_rows,
    };
    let d_time_series = TimeSeries::create(time_series_emit, model);
    Box::into_raw(Box::new(d_time_series))
}

#[no_mangle]
pub unsafe extern "C" fn time_series_free(ptr: *mut TimeSeries) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn time_series_row_count(ptr: *const TimeSeries) -> c_int {
    (&*ptr).row_count() as c_int
}
#[no_mangle]
pub unsafe extern "C" fn time_series_can_fetch_more(ptr: *const TimeSeries) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn time_series_fetch_more(ptr: *mut TimeSeries) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn time_series_sort(ptr: *mut TimeSeries, column: u8, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn time_series_data_input(ptr: *const TimeSeries, row: c_int) -> u32 {
    (&*ptr).input(row as usize).into()
}

#[no_mangle]
pub unsafe extern "C" fn time_series_set_data_input(ptr: *mut TimeSeries, row: c_int, v: u32) -> bool {
    (&mut *ptr).set_input(row as usize, v)
}

#[no_mangle]
pub unsafe extern "C" fn time_series_data_result(ptr: *const TimeSeries, row: c_int) -> u32 {
    (&*ptr).result(row as usize).into()
}

#[no_mangle]
pub unsafe extern "C" fn time_series_set_data_result(ptr: *mut TimeSeries, row: c_int, v: u32) -> bool {
    (&mut *ptr).set_result(row as usize, v)
}
