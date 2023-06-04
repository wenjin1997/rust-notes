// use std::io::{BufWriter, Write};
// use std::net::TcpStream;
// #[derive(Debug)]
// struct MyWriter<W> {
//     writer: W,
// }
//
// impl<W: Write> MyWriter<W> {
//     pub fn new(addr: &str) -> Self {
//         let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//         Self {
//             writer: BufWriter::new(stream),
//         }
//     }
//
//     pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
//         self.writer.write_all(buf.as_bytes())
//     }
// }
//
// #[test]
// fn it_works() {
//     let mut writer = MyWriter::new("127.0.0.1:8080");
//     writer.write("hello world!");
// }