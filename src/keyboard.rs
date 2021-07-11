use crate::consts::{self, OpCode};
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
        self.write_msg(Message::command(OpCode::VersionRead))?;
        let msg: Version = self.read_msg()?;
        println!(
            "{}",
            String::from_utf8_lossy(&msg.version)
                .trim_end_matches('\u{0}')
                .to_string()
        );
        Ok(())
    }

    pub fn print_counter(&self) -> Result<()> {
        self.write_msg(Message::command(OpCode::CounterRead))?;
        let mut counter: Vec<u32> = vec![];
        loop {
            let count: KeyCount = self.read_msg()?;
            if count.flag != 0 {
                break;
            }
            counter.append(&mut count.data.to_vec());
        }
        println!("{:?}", counter);
        Ok(())
    }

    pub fn print_mapping(&self) -> Result<()> {
        self.write_msg(Message::command(OpCode::KeymapDataRead))?;
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
        for i in 1..67 as usize {
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

    pub fn calib(&self) -> Result<()> {
        self.write_msg(Message::command(OpCode::CalibInit))?;
        Ok(())
    }
    
    pub fn calib_press(&self) -> Result<()> {
        self.write_msg(Message::command(OpCode::CalibPressed))?;
        let msg: Message = self.read_msg()?;
        println!("{}", msg);
        Ok(())
    }

    pub fn keylock(&self) -> Result<()> {
        self.write_msg(Message::command(OpCode::KeyLock))?;
        Ok(())
    }
    pub fn keyunlock(&self) -> Result<()> {
        let mut data = [0; 61];
        data[0] = 1;
        self.write_msg(Message::command_with_data(OpCode::KeyLock, data))?;
        Ok(())
    }

    pub fn write_mapping(&self) -> Result<()> {
        // you have to write the complete mapping every time, or you will get a nearly broken
        // keyboard
        self.write_msg(Message::command(OpCode::KeymapDataStart))?;
        let mut data = [0; 61];
        data[0] = 1; // layer normal
        data[1] = 30; // key caps
        data[2] = 0x00;
        data[3] = 0x01;
        data[4] = 15; // code backspace
        println!("{:?}", data);
        self.write_msg(Message::command_with_data(OpCode::KeymapData, data))?;
        self.write_msg(Message::command_with_data(
            OpCode::KeymapDataEnd,
            [OpCode::KeymapDataEnd as u8; 61],
        ))?;
        Ok(())
    }
}

#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "msb")]
pub struct KeyCount {
    flag: u8,
    fixed: u8,
    size: u8,
    #[packed_field(endian = "lsb")]
    data: [u32; 15],
    padding: u8,
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
    pub fn command(cmd: consts::OpCode) -> Self {
        Self {
            command: cmd as u16,
            ..Message::default()
        }
    }
    pub fn command_with_data(cmd: consts::OpCode, data: [u8; 61]) -> Self {
        Self {
            command: cmd as u16,
            data,
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
