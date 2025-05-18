use std::io::{self, Read, Seek, BufReader};

const MAGIC: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
// const IEND_CHUNK: [u8;4] = [0x49, 0x45, 0x4E, 0x44];


fn read_magic<T: Read>(f: &mut T) -> Result<[u8; 8], &'static str> {
    let mut buffer = [0; 8];
    match f.read_exact(&mut buffer) {
        Ok(_) => Ok(buffer.clone()),
        Err(_) => Err("invalid magic number")
    }
}

fn read_u32<T: Read>(f: &mut T) -> Result<u32, &'static str> {
    let mut buffer = [0; 4];
    match f.read_exact(&mut buffer) {
        Ok(_) => {},
        Err(_) => return Err("failed to read 4 bytes for u32 value"),
    }

    Ok(u32::from_be_bytes(buffer))
}

fn read_chunktype<T: Read>(f: &mut T) -> Result<String, &'static str> {
    let mut buffer = [0; 4];
    match f.read_exact(&mut buffer) {
        Ok(_) => {},
        Err(_) => return Err("failed to read 4 bytes for u32 value"),
    }
    let chunktype = String::from_utf8(buffer.to_vec()).unwrap();
    Ok(chunktype)
}


fn read_ihdr<T: Read>(f: &mut T) -> Result<String, &'static str> {
    fn colour_type_name(n: u8) -> &'static str {
        match n {
            0 => {"Greyscale (1, 2, 4, 8, 16) bit"},
            2 => {"Truecolour (8, 16) bit"},
            3 => {"Indexed-colour (1, 2, 4, 8) bit"},
            4 => {"Greyscale with alpha (8, 16) bit"},
            6 => {"Truecolour with alpha (8, 16) bit"},
            _ => {"error"},
        }
    }
    fn compression_method_name(n: u8) -> &'static str {
        match n {
            0 => {"deflate/inflate compression with a sliding window of at most 32768 bytes"},
            _ => {"error"},
        }
    }
    fn filter_method_name(n: u8) -> &'static str {
        match n {
            0 => {"adaptive filtering with five basic filter types"},
            _ => {"error"},
        }
    }
    fn interlace_method_name(n: u8) -> &'static str {
        match n {
            0 => {"no interlace"},
            1 => {"Adam7 interlace"},
            _ => {"error"},
        }
    }
    let mut buffer4 = [0; 4];
    let mut buffer1 = [0; 1];
    f.read_exact(&mut buffer4).unwrap();
    let width = u32::from_be_bytes(buffer4);
    f.read_exact(&mut buffer4).unwrap();
    let height = u32::from_be_bytes(buffer4);
    f.read_exact(&mut buffer1).unwrap();
    let bit_depth = u8::from_be_bytes(buffer1);
    f.read_exact(&mut buffer1).unwrap();
    let colour_type = u8::from_be_bytes(buffer1);
    f.read_exact(&mut buffer1).unwrap();
    let compression_method = u8::from_be_bytes(buffer1);
    f.read_exact(&mut buffer1).unwrap();
    let filter_method = u8::from_be_bytes(buffer1);
    f.read_exact(&mut buffer1).unwrap();
    let interlace_method = u8::from_be_bytes(buffer1);

    let mut s = String::new();
    s += &format!("    width: {}\n", width);
    s += &format!("    height: {}\n", height);
    s += &format!("    bit_depth: {}\n", bit_depth);
    s += &format!("    colour_type: {} ({})\n", colour_type, colour_type_name(colour_type));
    s += &format!("    compression_method: {} ({})\n", compression_method, compression_method_name(compression_method));
    s += &format!("    filter_method: {} ({})\n", filter_method, filter_method_name(filter_method));
    s += &format!("    interlace_method: {} ({})\n", interlace_method, interlace_method_name(interlace_method));
    return Ok(s);
}

pub fn show_chunks_info<T: Read + Seek>(mut f: BufReader<T>) {
    let m = read_magic(&mut f).unwrap();
    if m != MAGIC {
        panic!("incorrect magic number");
    }
    let mut ihdr: String = String::new();
    let mut idat_chunks = 0;
    let mut idat_length = 0;
    loop {
        let len = read_u32(&mut f).unwrap();
        let chunktype = read_chunktype(&mut f).unwrap();
        if chunktype == "IHDR" {
            ihdr = read_ihdr(&mut f).unwrap();
        } else if chunktype == "IDAT" {
            idat_chunks += 1;
            idat_length += len;
        }

        if chunktype == "IHDR" || chunktype == "IEND" {

        } else {
            f.seek(io::SeekFrom::Current(len as i64)).unwrap();
        }
        let crc = read_u32(&mut f).unwrap();
        println!("{}: len:{}, crc: 0x{:X}", chunktype, len, crc);
        if chunktype == "IEND" {
            break;
        }
    }
    println!("----------------\nIHDR");
    print!("{}", ihdr);
    println!("----------------\nIDAT\n    chunks: {}\n    length: {}", idat_chunks, idat_length);
}