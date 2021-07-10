use crate::consts::{self, COMMAND};
use anyhow::Result;
use hidapi::{HidApi, HidDevice};
use packed_struct::prelude::*;

pub struct Keyboard {
    device: HidDevice,
}

impl Keyboard {
    pub fn open() -> Result<Keyboard> {
        let hidapi = HidApi::new()?;
        for device in hidapi.device_list() {
            if device.vendor_id() == consts::VENDOR_ID
                && device.product_id() == consts::PRODUCT_ID
                && device.interface_number() == consts::INTERFACE_ID
            {
                let device = device.open_device(&hidapi)?;
                return Ok(Keyboard { device });
            }
        }
        Err(anyhow::anyhow!("no mactching device found"))
    }

    pub fn write_msg<T: PackedStruct<ByteArray = [u8; 64]>>(&self, msg: T) -> Result<usize> {
        Ok(self.device.write(&msg.pack()?)?)
    }

    pub fn read_msg<T: PackedStruct<ByteArray = [u8; 64]>>(&self) -> Result<T> {
        let mut buf: [u8; 64] = [0; 64];
        self.device.read_timeout(&mut buf, 100)?;
        Ok(T::unpack(&buf)?)
    }

    pub fn print_version(&self) -> Result<()> {
        self.write_msg(Message::command(COMMAND::VERSION))?;
        let msg: Version = self.read_msg()?;
        println!("{}", String::from_utf8_lossy(&msg.version)
            .trim_end_matches('\u{0}')
            .to_string());
        Ok(())
    }

    pub fn print_counter(&self) -> Result<()> {
        self.write_msg(Message::command(COMMAND::READ_COUNTER))?;
        let mut counter: Vec<u32> = vec![];
        loop {
            let count: KeyCount = self.read_msg()?;
            if count.flag != 0 {
                break;
            }
            counter.append(
                &mut count
                    .data
                    .chunks_exact(4)
                    .into_iter()
                    .map(|x| {
                        let mut y = [0; 4];
                        y.copy_from_slice(x);
                        y
                    })
                    .map(|x| u32::from_le_bytes(x))
                    .collect::<Vec<u32>>(),
            );
        }
        println!("{:?}", counter);
        Ok(())
    }

    pub fn print_mapping(&self) -> Result<()> {
        self.write_msg(Message::command(COMMAND::READ_ALL))?;
        let mut normal: [u8; 256] = [0; 256];
        let mut left: [u8; 256] = [0; 256];
        let mut right: [u8; 256] = [0; 256];
        loop {
            let mapping: KeyMapping = self.read_msg()?;
            if mapping.flag != 0 {
                break;
            }
            match mapping.level {
                1 => normal[mapping.key as usize] = mapping.keycode,
                2 => left[mapping.key as usize] = mapping.keycode,
                3 => right[mapping.key as usize] = mapping.keycode,
                _ => (),
            }
        }
        for i in 1..66 as usize {
            println!(
                "key {}: normal {} left {} right {}",
                i,
                consts::HWCODE[normal[i] as usize],
                consts::HWCODE[left[i] as usize],
                consts::HWCODE[right[i] as usize]
            );
        }
        Ok(())
    }
}

#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "msb")]
pub struct KeyCount {
    flag: u8,
    fixed: u8,
    size: u8,
    data: [u8; 61],
}

#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "msb")]
pub struct KeyMapping {
    flag: u8,
    command: u8,
    level: u8,
    key: u8,
    zero: u8,
    mapped: u8,
    keycode: u8,
    padding: [u8; 57],
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct Version {
    report: u8,
    command: u8,
    version: [u8; 62],
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct Message {
    report: u8,
    command: u16,
    data: [u8; 61],
}

impl Message {
    pub fn command(cmd: consts::COMMAND) -> Self {
        Self {
            command: cmd as u16,
            ..Message::default()
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            report: 0,
            command: 0,
            data: [0; 61],
        }
    }
}
