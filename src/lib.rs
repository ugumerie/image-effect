use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log; //log_1 logs one value, log_2 logs two values
use base64::{encode, decode}; // converts to vector
use image::load_from_memory;
use image::ImageOutputFormat::Png;

//we are borrowing the string bcos we wont be changing the file directly.
// the purpose was to transfer btween javascript and rust
// pub to be accessible to other rust fn. fn are private in nature
// wasm_bindgen macro make function available to javascript

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    //.into() will intelligently detect the datatype we are converting to
    // i.e from str to JSValue 
    log(&"Grayscale called".into());
    
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encode_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encode_img); //format! return a formatted string

    data_url
}
