use actix_web::HttpRequest;
use std::sync::atomic::{AtomicU32, Ordering};

pub fn change_counters(incr: &AtomicU32, to_swap: &AtomicU32, to_add: u32) -> u32 {
    to_swap.swap(0, Ordering::SeqCst);
    incr.fetch_add(to_add, Ordering::SeqCst) + to_add
}

pub fn get_accept_header(req: &HttpRequest) -> Option<&str> {
    match req.headers().get("accept") {
        Some(accept) => accept.to_str().ok(),
        None => None,
    }
}
