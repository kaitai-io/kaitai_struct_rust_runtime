use std;
use kaitai_stream::KaitaiStream;

pub trait KaitaiStruct {
    fn empty() -> Self where Self : Sized;
    fn from_file(path: &str) -> std::result::Result<Self, std::io::Error> where Self : Sized;
    fn new<S: KaitaiStream>(stream: &mut S, parent: &Option<Box<KaitaiStruct>>, root: &Option<Box<KaitaiStruct>>) -> Self where Self : Sized;
    fn read<S: KaitaiStream>(&mut self, stream: &mut S, parent: &Option<Box<KaitaiStruct>>, root: &Option<Box<KaitaiStruct>>) where Self : Sized;
}