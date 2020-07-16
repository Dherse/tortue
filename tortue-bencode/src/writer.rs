use crate::BencodedValue;
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub fn write<'a, T: Write>(
    value: &BencodedValue<'a>,
    writer: &mut T,
) -> io::Result<()> {
    match value {
        BencodedValue::Binary(ref bin) => write_bin(bin, writer),
        BencodedValue::String(ref str) => write_str(str, writer),
        BencodedValue::Integer(int) => write_int(*int, writer),
        BencodedValue::List(lst) => write_list(lst, writer),
        BencodedValue::Dictionary(dict) => write_dict(dict, writer),
        BencodedValue::None => Ok(()),
        BencodedValue::BinaryOwned(bin) => write_bin(&bin[..], writer),
        BencodedValue::StringOwned(str) => write_str(&str, writer),
        BencodedValue::DictionaryOwned(dict) => write_owned_dict(dict, writer),
    }
}

pub fn write_bin<'a, T: Write>(
    value: &'a [u8],
    writer: &mut T,
) -> io::Result<()> {
    writer.write_all(format!("{}", value.len()).as_bytes())?;
    writer.write_all(b":")?;
    writer.write_all(value)
}

pub fn write_str<'a, T: Write>(str: &'a str, writer: &mut T) -> io::Result<()> {
    writer.write_all(format!("{}:{}", str.len(), str).as_bytes())
}

pub fn write_int<T: Write>(int: i64, writer: &mut T) -> io::Result<()> {
    writer.write_all(format!("i{}e", int).as_bytes())
}

pub fn write_list<'a, T: Write>(
    list: &[BencodedValue<'a>],
    writer: &mut T,
) -> io::Result<()> {
    writer.write_all(b"l")?;

    for element in list.iter() {
        write(element, writer)?;
    }

    writer.write_all(b"e")
}

pub fn write_dict<'a, T: Write>(
    list: &HashMap<&'a str, BencodedValue<'a>>,
    writer: &mut T,
) -> io::Result<()> {
    writer.write_all(b"d")?;

    for (key, value) in list.iter() {
        write_str(key, writer)?;
        write(value, writer)?;
    }

    writer.write_all(b"e")
}

pub fn write_owned_dict<'a, T: Write>(
    list: &HashMap<String, BencodedValue<'a>>,
    writer: &mut T,
) -> io::Result<()> {
    writer.write_all(b"d")?;

    for (key, value) in list.iter() {
        write_str(key, writer)?;
        write(value, writer)?;
    }

    writer.write_all(b"e")
}
