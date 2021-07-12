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
        self.write_msg(Request::new(OpCode::VersionRead))?;
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
        self.write_msg(Request::new(OpCode::CounterRead))?;
        let mut counter: Vec<u32> = vec![];
        loop {
            let count: KeyCounter = self.read_msg()?;
            if count._id != 0 {
                break;
            }
            counter.append(&mut count.count.to_vec());
        }
        println!("{:?}", counter);
        Ok(())
    }

    pub fn print_mapping(&self) -> Result<()> {
        self.write_msg(Request::new(OpCode::KeymapDataRead))?;
        let mut normal: [u8; 256] = [0; 256];
        let mut left: [u8; 256] = [0; 256];
        let mut right: [u8; 256] = [0; 256];
        loop {
            let mapping: KeymapResponse = self.read_msg()?;
            if mapping._id != 0 {
                break;
            }
            match mapping.layer {
                1 => normal[mapping.key as usize] = mapping.keycode,
                2 => left[mapping.key as usize] = mapping.keycode,
                3 => right[mapping.key as usize] = mapping.keycode,
                _ => (),
            }
        }
        for i in 1..67 as usize {
            println!(
                "key {}: 0: {} 1: {} 2: {}",
                i,
                consts::KEY_CODE_NAME[normal[i] as usize],
                consts::KEY_CODE_NAME[left[i] as usize],
                consts::KEY_CODE_NAME[right[i] as usize]
            );
        }
        Ok(())
    }

    pub fn calib(&self) -> Result<()> {
        self.write_msg(Request::new(OpCode::CalibInit))?;
        Ok(())
    }

    pub fn calib_press(&self) -> Result<()> {
        self.write_msg(Request::new(OpCode::CalibPressed))?;
        let _msg: Request = self.read_msg()?;
        Ok(())
    }

    pub fn keylock(&self) -> Result<()> {
        self.write_msg(Request::new(OpCode::KeyLock))?;
        Ok(())
    }
    pub fn keyunlock(&self) -> Result<()> {
        let mut data = [0; 61];
        data[0] = 1;
        self.write_msg(Request::new_with_data(OpCode::KeyLock, data))?;
        Ok(())
    }

    pub fn write_mapping(&self) -> Result<()> {
        // you have to write the complete mapping every time, or you will get a nearly broken
        // keyboard
        self.write_msg(Request::new(OpCode::KeymapDataStart))?;
        self.write_msg(Request::new_with_data(
            OpCode::KeymapData,
            KeymapData::new(1, 30, 15).pack()?,
        ))?;
        self.write_msg(Request::new_with_data(
            OpCode::KeymapDataEnd,
            [OpCode::KeymapDataEnd as u8; 61],
        ))?;
        Ok(())
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct Request {
    _id: u8,
    op: u16,
    data: [u8; 61],
}

impl Request {
    pub fn new(op: consts::OpCode) -> Self {
        Self::new_with_data(op, [0; 61])
    }
    pub fn new_with_data(op: consts::OpCode, data: [u8; 61]) -> Self {
        Self {
            _id: 0,
            op: op as u16,
            data,
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct Version {
    _id: u8,
    _type: u8,
    version: [u8; 62],
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct KeyCounter {
    _id: u8,
    _type: u8,
    size: u8,
    #[packed_field(endian = "lsb")]
    count: [u32; 15],
    _padding: u8,
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct KeymapResponse {
    _id: u8,
    _type: u8,
    layer: u8,
    key: u8,
    _zero: u8,
    active: u8,
    keycode: u8,
    _padding: [u8; 57],
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct KeymapData {
    layer: u8,
    key: u8,
    _zero: u8,
    active: u8,
    keycode: u8,
    _padding: [u8; 56],
}

impl KeymapData {
    pub fn new(layer: u8, key: u8, keycode: u8) -> Self {
        Self {
            layer,
            key,
            _zero: 0,
            active: if keycode == 0 { 0 } else { 1 },
            keycode,
            _padding: [0; 56],
        }
    }
}
