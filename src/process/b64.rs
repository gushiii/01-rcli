use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    read::DecoderReader,
};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    if input != "-" {
        let mime = mime_guess::from_path(input).first_or_octet_stream();
        print!("data:{};base64,", mime);
    }

    let mut encoder = match format {
        Base64Format::Standard => base64::write::EncoderWriter::new(io::stdout(), &STANDARD),
        Base64Format::UrlSafe => base64::write::EncoderWriter::new(io::stdout(), &URL_SAFE_NO_PAD),
    };

    io::copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let raw_reader = get_reader(input)?;

    let mut buf_reader = BufReader::new(raw_reader);

    let mut prefix_check = [0u8; 128];
    let n = buf_reader.read(&mut prefix_check)?;
    let mut start_index = 0;

    if let Some(i) = prefix_check.iter().position(|&b| b == b',') {
        start_index = i + 1;
    }

    let remaining_prefix = &prefix_check[start_index..n];
    let final_reader = io::Read::chain(io::Cursor::new(remaining_prefix), buf_reader);
    let filtered_reader = WhitespaceSkipper::new(final_reader);

    let mut decoder = match format {
        Base64Format::Standard => DecoderReader::new(filtered_reader, &STANDARD),
        Base64Format::UrlSafe => DecoderReader::new(filtered_reader, &URL_SAFE_NO_PAD),
    };

    io::copy(&mut decoder, &mut io::stdout())?;

    Ok(())
}

struct WhitespaceSkipper<R: Read> {
    inner: R,
}

impl<R: Read> WhitespaceSkipper<R> {
    fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: Read> Read for WhitespaceSkipper<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut temp = vec![0u8; buf.len()];
        let n = self.inner.read(&mut temp)?;
        let mut j = 0;

        for (_, &val) in temp.iter().enumerate().take(n) {
            if !val.is_ascii_whitespace() {
                buf[j] = val;
                j += 1;
            }
        }

        Ok(j)
    }
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}
