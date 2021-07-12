use crate::consts::{self, OpCode};
use anyhow::Result;
use hidapi::{HidApi, HidDevice};
use packed_struct::prelude::*;
use std::convert::TryInto;

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

    pub fn read_version(&self) -> Result<String> {
        self.write_msg(Request::new(OpCode::VersionRead))?;
        let record: Version = self.read_msg()?;
        Ok(String::from_utf8_lossy(&record.version)
            .trim_end_matches('\u{0}')
            .to_string())
    }

    pub fn read_counter(&self) -> Result<Vec<u32>> {
        self.write_msg(Request::new(OpCode::CounterRead))?;
        let mut counter: Vec<u32> = vec![];
        loop {
            let record: KeyCounter = self.read_msg()?;
            if record._id != 0 {
                break;
            }
            counter.append(&mut record.count[..((record.size / 4) as usize)].to_vec());
        }
        Ok(counter)
    }

    pub fn read_keymap(&self) -> Result<Vec<Vec<u8>>> {
        self.write_msg(Request::new(OpCode::KeymapDataRead))?;
        let mut keymap: Vec<Vec<u8>> = vec![];
        loop {
            let record: KeymapResponse = self.read_msg()?;
            if record._id != 0 {
                break;
            }
            if record.layer as usize > keymap.len() {
                keymap.resize(record.layer as usize, vec![]);
            }
            let layer = &mut keymap[(record.layer - 1) as usize];
            if record.key as usize > layer.len() {
                layer.resize(record.key as usize, 0);
            }
            layer[(record.key - 1) as usize] = record.keycode;
        }
        Ok(keymap)
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

    pub fn write_keymap(&self, keymap: Vec<Vec<u8>>) -> Result<()> {
        self.write_msg(Request::new(OpCode::KeymapDataStart))?;
        for (i, layer) in keymap.iter().enumerate() {
            for (key, &keycode) in layer.iter().enumerate() {
                self.write_msg(Request::new_with_data(
                    OpCode::KeymapData,
                    KeymapData::new((i + 1).try_into()?, (key + 1).try_into()?, keycode).pack()?,
                ))?;
            }
        }
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
