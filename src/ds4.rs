/// refer to https://www.psdevwiki.com/ps4/DS4-BT#0x11

#[derive(Debug, Default, Clone)]
pub struct Ds4Buttons<T: Clone> {
  pub north: T,
  pub east: T,
  pub south: T,
  pub west: T,
  pub square: T,
  pub cross: T,
  pub circle: T,
  pub triangle: T,
  pub l1: T,
  pub r1: T,
  pub l2: T,
  pub r2: T,
  pub pause: T,
  pub option: T,
  pub l3: T,
  pub r3: T,
  pub ps: T,
  pub tpad: T,
}

fn gen_buttons(raw: [u8; 3]) -> Ds4Buttons<bool> {
  let mut n = false;
  let mut e = false;
  let mut s = false;
  let mut w = false;
  match raw[0] & 0xF {
    0 => { n = true; },
    1 => { n = true; e = true; },
    2 => { e = true; },
    3 => { e = true; s = true; },
    4 => { s = true; },
    5 => { s = true; w = true; },
    6 => { w = true; },
    7 => { w = true; n = true; },
    _ => (),
  }
  Ds4Buttons {
    north: n,
    east: e,
    south: s,
    west: w,
    square: (raw[0] & 0x10) != 0,
    cross: (raw[0] & 0x20) != 0,
    circle: (raw[0] & 0x40) != 0,
    triangle: (raw[0] & 0x80) != 0,
    l1: (raw[1] & 0x01) != 0,
    r1: (raw[1] & 0x02) != 0,
    l2: (raw[1] & 0x04) != 0,
    r2: (raw[1] & 0x08) != 0,
    pause: (raw[1] & 0x10) != 0,
    option: (raw[1] & 0x20) != 0,
    l3: (raw[1] & 0x40) != 0,
    r3: (raw[1] & 0x80) != 0,
    ps: (raw[2] & 0x01) != 0,
    tpad: (raw[2] & 0x02) != 0,
  }
}

#[derive(Debug, Default, Clone)]
pub struct Ds4Finger {
  pub down: bool,
  pub track_num: u8,
  pub coord: (u16, u16), // (x, y) in 12bit
}

fn gen_finger(raw: [u8;4]) -> Ds4Finger {
  Ds4Finger {
    down: (raw[0] & 0x80) != 0,
    track_num: (raw[0] & 0x7f),
    coord: (
      (raw[1] as u16) | (((raw[2] & 0xF) as u16) << 8),
      ((raw[2] >> 4) as u16) | ((raw[3] as u16) << 4)
    ),
  }
}

#[derive(Debug, Default, Clone)]
pub struct Ds4Input {
  pub report_id: u8,
  pub stick_left: (u8, u8), // 0(left, top)..+256(right, bottom)
  pub stick_right: (u8, u8), // 0(left, top)..+256(right, bottom)
  pub buttons: Ds4Buttons<bool>,
  pub counter: u8,
  pub trigger_left: u8, // 0..+256
  pub trigger_right: u8, // 0..+256
  pub gyro: (i16, i16, i16), // (x, y, z) 
  pub accel: (i16, i16, i16), // (x, y, z) positive: right, up, backward
  pub finger1: Ds4Finger,
  pub finger2: Ds4Finger,
}

pub fn input(rep: &Vec<u8>, ofs: usize) -> Ds4Input {
  Ds4Input {
    report_id: rep[ofs + 0],
    stick_left: (rep[ofs + 2], rep[ofs + 3]),
    stick_right: (rep[ofs + 4], rep[ofs + 5]),
    buttons: gen_buttons([rep[ofs + 6], rep[ofs + 7], rep[ofs + 8]]),
    counter: rep[ofs + 8] >> 2,
    trigger_left: rep[ofs + 9],
    trigger_right: rep[ofs + 10],
    gyro: (
      (rep[ofs + 14] as i16)|((rep[ofs + 15] as i16) << 8),
      (rep[ofs + 16] as i16)|((rep[ofs + 17] as i16) << 8),
      (rep[ofs + 18] as i16)|((rep[ofs + 19] as i16) << 8)),
    accel: (
      (rep[ofs + 20] as i16)|((rep[ofs + 21] as i16) << 8),
      (rep[ofs + 22] as i16)|((rep[ofs + 23] as i16) << 8),
      (rep[ofs + 24] as i16)|((rep[ofs + 25] as i16) << 8)),
    finger1: gen_finger([rep[ofs + 36], rep[ofs + 37], rep[ofs + 39], rep[ofs + 40]]),
    finger2: gen_finger([rep[ofs + 41], rep[ofs + 42], rep[ofs + 43], rep[ofs + 44]]),
  }
}
