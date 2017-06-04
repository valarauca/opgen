
extern crate xmlgeneral;
use xmlgeneral::{XmlItem,read_xml};

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_brief<'a>(x: &'a XmlItem) -> Option<&'a String> {
  x.children.iter().filter(|z| z.name == "brief").map(|z| &z.data).next()
}
fn get_desc<'a>(x: &'a XmlItem) -> Option<&'a String> {
  x.children.iter().filter(|z| z.name == "dscrp").map(|z| &z.data).next()
}
fn get_mnem<'a>(x: &'a XmlItem) -> Option<&'a String> {
  x.children.iter().filter( |z| z.name == "mnem").map(|z| &z.data).next()
}
fn get_args<'a>(x: &'a XmlItem) -> Option<&'a String> {
  x.children.iter().filter( |z| z.name == "args").map(|z| &z.data).next()
}

fn build_str(mnem: &str, args: &str, desc: &str) -> String {
  let mnem = mnem.to_string().to_lowercase();
  if args == "void" || args == "" {
    format!("\t///\n\t/// '{};' {}\n", mnem, desc)
  } else {
    format!("\t///\n\t/// '{} {};' {}\n", mnem, args, desc)
  }
}
fn append_str(base: &mut String, mnem: &str, args: &str, desc: &str) {
  base.push_str(&build_str(mnem, args, desc));
}

fn collect_mnem64<'a>(x: &'a XmlItem) -> HashMap<&'a String, String> {
  let mut map = HashMap::new();
  for data in x.children
      .iter()
      .filter(|z| if z.name == "ins" { 
          match z.attributes.get("x64m") {
            Option::Some(string) => string != "I",
            Option::None => false
          }
        } else { false }) {

    let mnem = match get_mnem(data) {
      Option::Some(x) => x,
      Option::None => continue
    };
    let args = match get_args(data) {
      Option::Some(x) => x,
      Option::None => continue
    };
    let desc = match get_desc(data) {
      Option::Some(x) => x,
      Option::None => continue
    };

    let flag = map.contains_key(mnem);
    if flag {
      match map.get_mut(mnem) {
        Option::Some(val) => {
          append_str(val, mnem, args, desc);
          continue;
        },
        Option::None => { },
      };
      map.remove(mnem);
    }
    map.insert(mnem, build_str(mnem, args, desc));
  }
  map
}

fn collect_mnem32<'a>(x: &'a XmlItem) -> HashMap<&'a String, String> {
  let mut map = HashMap::new();
  for data in x.children
      .iter()
      .filter(|z| if z.name == "ins" { 
          match z.attributes.get("x32m") {
            Option::Some(string) => string != "I",
            Option::None => false
          }
        } else { false }) {

    let mnem = match get_mnem(data) {
      Option::Some(x) => x,
      Option::None => continue
    };
    let args = match get_args(data) {
      Option::Some(x) => x,
      Option::None => continue
    };
    let desc = match get_desc(data) {
      Option::Some(x) => x,
      Option::None => continue
    };

    let flag = map.contains_key(mnem);
    if flag {
      match map.get_mut(mnem) {
        Option::Some(val) => {
          append_str(val, mnem, args, desc);
          continue;
        },
        Option::None => { },
      };
      map.remove(mnem);
    }
    map.insert(mnem, build_str(mnem, args, desc));
  }
  map
}

fn main() {

  let mut file = match File::open("parseable_instructions.xml") {
    Ok(x) => x,
    Err(e) => panic!("Could not open file {:?}",e)
  };
  let mut buffer = BufReader::with_capacity(32768,file);
  let items = match read_xml(buffer) {
    Ok(x) => x,
    Err(e) => panic!("Failed to read xml {:?}",e)
  };

  let mut out64 = String::with_capacity(4096);
  let mut out32 = String::with_capacity(4096);
  out64.push_str("\n#[derive(Copy,Clone,Debug)]\npub enum IntelOp64 {\n");
  out32.push_str("\n#[derive(Copy,Clone,Debug)]\npub enum IntelOp32 {\n");
  ///loop over common
  for common in items.iter().flat_map(|z| z.children.iter()) {
    let brief = match get_brief(common) {
      Option::Some(x) => x,
      Option::None => continue,
    };

    let mnem_info64 = collect_mnem64(common);
    let mnem_info32 = collect_mnem32(common);
    out64.push_str(&format!("// {}\n", brief));
    for (key,value) in mnem_info64.iter() {
      out64.push_str(value);
      out64.push_str(&format!("\t{},\n", key));
    }
    out32.push_str(&format!("// {}\n", brief));
    for (key,value) in mnem_info32.iter() {
      out32.push_str(value);
      out32.push_str(&format!("\t{},\n", key));
    }
  }
  out64.push_str("}\n\n");
  out32.push_str("}\n\n");

  let mut file32 = File::create("intel_op32.rs").unwrap();
  let mut file64 = File::create("intel_op64.rs").unwrap();

  file32.write_all(out32.as_bytes()).unwrap();
  file64.write_all(out64.as_bytes()).unwrap();
  
  file32.flush().unwrap();
  file64.flush().unwrap();
}
