use std::error::Error;

use brotli::{BrotliCompress, enc::BrotliEncoderParams, BrotliDecompress};

#[inline]
fn pick_quality_level(size: usize) -> i32 {
    if size <= 5*1024*1024 /* 5 mb */ {
        return 10;
    }

    9
}

pub fn compress(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>
{
    let mut out = vec![];
    let mut params = BrotliEncoderParams::default();
    params.favor_cpu_efficiency = true;
    params.quality = pick_quality_level(input.len());    
    BrotliCompress(&mut input.as_slice(), &mut out, &params)?;
    Ok(out)
}

pub fn decompress(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut out: Vec<u8> = vec![];

    BrotliDecompress(&mut input.as_slice(), &mut out)?;

    Ok(out)
}