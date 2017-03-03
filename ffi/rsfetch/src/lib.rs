extern crate libc;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_json;

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

#[no_mangle]
pub extern fn tf(urls: *const *const c_char, len: i32) -> *const *mut c_char {
    let urls = unsafe { 
        slice::from_raw_parts(urls, len as usize)
    };
    for url in urls {
        let url = unsafe {
            CStr::from_ptr(*url)
        };
        println!("{:?}", url);
    }
    let mut s = vec![
        CString::new("Nice").unwrap().into_raw(),
        CString::new("Yo").unwrap().into_raw(),
    ];
    let r = s.as_ptr();
    std::mem::forget(s);
    return r;
}
/*
#[no_mangle]
pub extern fn fetch_array(urls: *const *const c_char, len: i32) -> *const c_char {
    let urls = unsafe { 
        slice::from_raw_parts(urls, len as usize)
    };
    let len = urls.len();

    let (tx, rx) = mpsc::channel();

    for url in urls {
        let url = unsafe {
            CStr::from_ptr(*url)
        }.to_str()?;
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

    let mut result: Vec<String> = vec![];
    for _ in 0..len {
        result.push(rx.recv().unwrap());
    }
    return CString::new(json!(result).to_string()).unwrap().into_raw();
}

fn fetch(urls: Vec<*const c_char>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    let len = urls.len();

    for url in urls {
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

    let mut result: Vec<String> = vec![];
    for _ in 0..len {
        result.push(rx.recv().unwrap());
    }
    return result;
}

#[test]
fn test() {
    let inputs = vec![
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
          "https://gist.githubusercontent.com/huytd/5b80126b3163c5b2a599db5448d40521/raw/4c6a7cc8bbc717287e52edd0d1d0114ae5bb1a1c/download.txt",
    ];
    let result = fetch(inputs);
    println!("{:?}", result);
}

*/
