#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiTrackResponse<'a> {
    pub athleteId: i32,
    pub trackPoints: Vec<track_response::ApiTrackPoint<'a>>,
}

impl<'a> MessageRead<'a> for ApiTrackResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.athleteId = r.read_int32(bytes)?,
                Ok(18) => msg.trackPoints.push(r.read_message::<track_response::ApiTrackPoint>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApiTrackResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.athleteId) as u64)
        + self.trackPoints.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.athleteId))?;
        for s in &self.trackPoints { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiTrackPoint<'a> {
    pub timestamp: f64,
    pub lat: f32,
    pub lng: f32,
    pub altitude: f32,
    pub altitudeAgl: f32,
    pub status: Option<Cow<'a, str>>,
    pub speed: f32,
    pub verticalSpeed: f32,
}

impl<'a> MessageRead<'a> for ApiTrackPoint<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.timestamp = r.read_double(bytes)?,
                Ok(21) => msg.lat = r.read_float(bytes)?,
                Ok(29) => msg.lng = r.read_float(bytes)?,
                Ok(37) => msg.altitude = r.read_float(bytes)?,
                Ok(45) => msg.altitudeAgl = r.read_float(bytes)?,
                Ok(50) => msg.status = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(61) => msg.speed = r.read_float(bytes)?,
                Ok(69) => msg.verticalSpeed = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApiTrackPoint<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + 8
        + 1 + 4
        + 1 + 4
        + 1 + 4
        + 1 + 4
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + 4
        + 1 + 4
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(9, |w| w.write_double(*&self.timestamp))?;
        w.write_with_tag(21, |w| w.write_float(*&self.lat))?;
        w.write_with_tag(29, |w| w.write_float(*&self.lng))?;
        w.write_with_tag(37, |w| w.write_float(*&self.altitude))?;
        w.write_with_tag(45, |w| w.write_float(*&self.altitudeAgl))?;
        if let Some(ref s) = self.status { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        w.write_with_tag(61, |w| w.write_float(*&self.speed))?;
        w.write_with_tag(69, |w| w.write_float(*&self.verticalSpeed))?;
        Ok(())
    }
}
