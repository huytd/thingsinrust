#![feature(core_intrinsics)]
extern crate libc;
extern crate hyper;
extern crate hyper_native_tls;

use libc::c_char;
use std::slice;
use std::ffi::CString;
use std::ffi::CStr;
use std::io::Read;
use std::thread;
use std::sync::mpsc;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

#[no_mangle]
pub extern fn tf(urls: *mut *mut c_char, len: i32) -> *mut c_char {
    let urls = unsafe { 
        slice::from_raw_parts(urls, len as usize)
    };
    for url in urls {
        print_type_of(url);
        let s = unsafe {
            CStr::from_ptr(*url)
        };
        println!("{:?}", s);
    }
    let s = CString::new("Hello").unwrap();
    return s.into_raw();
}

#[no_mangle]
pub extern fn fetch(urls: *mut Vec<&'static str>, len: i32) -> *mut Vec<String> {
    let (tx, rx) = mpsc::channel();

    let urls = unsafe { 
        Vec::from_raw_parts(urls, len as usize, len as usize) 
    };

    for vec_urls in urls {
        for url in vec_urls {
            println!("Now processing: {}", url);
            let tx = tx.clone();

            thread::spawn(move || {
                let ssl = NativeTlsClient::new().unwrap();
                let connector = HttpsConnector::new(ssl);
                let client = Client::with_connector(connector);
                let mut res = client.get(url).send().unwrap();
                let mut result = String::new();
                res.read_to_string(&mut result);

                tx.send(result).unwrap();
            });
        }
    }

    let mut result: Vec<String> = vec![];
    for _ in 0..len {
        result.push(rx.recv().unwrap());
    }
    return Box::into_raw(Box::new(result));
}

//#[test]
fn test() {
    let inputs = vec![
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
    ];
    let result = fetch(Box::into_raw(Box::new(inputs)), 5);
    println!("{:?}", result);
}


