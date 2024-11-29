

fn read_keys_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<HashMap<String, DbEntry>> {
    let file = match File::open(path).await {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Default::default()),
        Err(e) => anyhow::bail!(e),
    };
    let mut reader = BufReader::new(file);
    {
        let mut header = [0; 9]; // MAGIC[5] + VERSION[4]
        reader.read_exact(&mut header).await?;
        anyhow::ensure!(&header[..5] == b"REDIS");
    }
    let mut map = HashMap::new();
    loop {
        match reader.read_u8().await? {
            // auxilary fields
            0xFA => {
                let key = decode_string(&mut reader).await?;
                let val = decode_string(&mut reader).await?;
                println!("aux fields: {key} => {val}");
            }
            0xFB => {
                let hm_len: usize = read_length_encoding(&mut reader).await?.try_into()?;
                let ehm_len: usize = read_length_encoding(&mut reader).await?.try_into()?;
                println!("hashmap size: {hm_len:?}");
                println!("expire hashmap size: {ehm_len:?}");
            }
            0xFC => {
                todo!("Expire time in milliseconds")
            }
            0xFD => {
                todo!("Expire time in seconds")
            }
            // database descriptor
            0xFE => {
                let db_num = reader.read_u8().await?;
                println!("database number: {}", db_num);
            }
            // end of the database
            0xFF => {
                anyhow::bail!("unexpected end of database");
                return Ok(map);
            }
            value_type => {
                println!("got value type: {}", value_type);
                anyhow::ensure!(value_type == 0, "only string values supported");
                let (key, val) = read_key_value(&mut reader).await?;
                map.insert(key, DbEntry::from(val));
                // just read a single entry for now
                break Ok(map);
            }
        }
    }
}
async fn read_key_value(reader: &mut BufReader<File>) -> anyhow::Result<(String, String)> {
    let key = decode_string(reader).await?;
    let val = decode_string(reader).await?;
    Ok((key, val))
}
async fn decode_string(reader: &mut BufReader<File>) -> anyhow::Result<String> {
    match read_length_encoding(reader).await? {
        Length::Normal(len) => {
            let mut buf = vec![0; len];
            reader.read_exact(&mut buf).await?;
            Ok(String::from_utf8(buf)?)
        }
        Length::Special(discriminant) => match discriminant {
            0 => Ok(reader.read_u8().await?.to_string()),
            1 => Ok(reader.read_u16_le().await?.to_string()),
            2 => Ok(reader.read_u32_le().await?.to_string()),
            3 => anyhow::bail!("compressed string encoding not supported yet"),
            d => anyhow::bail!("unknown special encoding with discriminant {d}"),
        },
    }
}

async fn read_length_encoding(reader: &mut BufReader<File>) -> anyhow::Result<Length> {
    let first_byte = reader.read_u8().await?;
    let mask = 0b00111111;
    match first_byte >> 6 {
        0b00 => {
            let n = first_byte & mask;
            Ok(Length::Normal(n as usize))
        }
        0b01 => {
            let hi = (first_byte & mask) as usize;
            let lo = reader.read_u8().await? as usize;
            Ok(Length::Normal((hi << 8) | lo))
        }
        0b10 => {
            let n = reader.read_u32_le().await?;
            Ok(Length::Normal(n as usize))
        }
        0b11 => Ok(Length::Special(first_byte & mask)),
        _ => unreachable!(),
    }
}
enum Length {
    Normal(usize),
    Special(u8),
}