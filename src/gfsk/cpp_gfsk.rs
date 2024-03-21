use std::collections::VecDeque;
use num_complex::Complex;
use crate::gfsk::packet::Packet;

type RawOutFunc = unsafe extern "C" fn(*mut (), Packet, usize);

type CF<T> = [T; 2];

struct CppFullDecoder {
    _private: ()
}

extern "C" {
    fn create_decoder(center: f64, hz: f64, baud: f64, id: usize, out_func: RawOutFunc, arg: *mut ()) -> *mut CppFullDecoder;
    fn feed_into_decoder(ffi: *mut CppFullDecoder, buf: *const CF<f32>, len: usize);
    fn destroy_decoder(ffi: *mut CppFullDecoder);
}


unsafe extern "C" fn push_queue(q_ptr: *mut (), packet: Packet, id: usize) {
    let queue = &mut *(q_ptr as *mut VecDeque<Packet>);
    queue.push_back(packet);
}

pub struct FullDecoder {
    ffi: *mut CppFullDecoder,
    queue: *mut VecDeque<Packet>,

    center: f64,
    hz: f64,
    baud: f64
}

impl FullDecoder {
    pub fn new(center: f64, hz: f64, baud: f64) -> FullDecoder {
        let queue_ptr: *mut VecDeque<Packet> = Box::into_raw(Box::new(VecDeque::new()));
        FullDecoder {
            ffi: unsafe { create_decoder(center, hz, baud, 0, push_queue, queue_ptr as *mut ()) },
            queue: queue_ptr,
            center, hz, baud
        }
    }

    pub fn feed(&mut self, items: &[Complex<f32>]) {
        unsafe { feed_into_decoder(self.ffi, items.as_ptr() as *const CF<f32>, items.len()) }
    }

    pub fn get_queued(&mut self) -> Option<Packet> {
        unsafe { &mut *self.queue }.pop_front()
    }

    pub fn center(&self) -> f64 { self.center }
    pub fn hz(&self) -> f64 { self.hz }
    pub fn baud(&self) -> f64 { self.baud }
}

impl Drop for FullDecoder {
    fn drop(&mut self) {
        unsafe {
            destroy_decoder(self.ffi);
            drop(Box::from_raw(self.queue));
        }
    }
}