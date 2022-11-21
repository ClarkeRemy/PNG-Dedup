#![no_implicit_prelude]
extern crate core;
extern crate alloc;
extern crate std;
extern crate image;


fn main()->core::result::Result<(),()> {
  fn m_e<T,E>(r:core::result::Result<T,E>)->core::result::Result<T,()> {r . map_err(|_|())}
  use {
    image::GenericImageView
  , core::{iter::Iterator, result::Result, option::Option }
  , alloc::string::String
  }

; let input = "./in"
; let output= "./out"

; let mut images = std::fs::read_dir(input) . then(m_e)?
. filter_map(|e| e . ok())
. filter(|e|{ use Option::Some
  ; let Result::Ok(x) = std::fs::DirEntry::file_type(e) else {return false}
  ; let Result::Ok(y) = e . file_name() . into_string() else {return false}
  ; x . is_file() 
    && y . chars() . nth(0) != Some('.')
    && {
      let _y =  y . as_bytes()
    ; let l  = _y . len()
    ; _y[l - 4 .. l] == b".png"[..]
    }
  })
. collect::<alloc::vec::Vec<_>>()
; images . sort_by_key(std::fs::DirEntry::file_name)

; let mut images = images
. iter()
. map(|e| { use core::{convert::From, clone::Clone }
  ; let name = e . file_name() . into_string() . unwrap()
  ; ( name . clone()
    , (String::from(input) + "/" + &name[..]) . then(image::open) . unwrap() 
    )
  })

; if let Result::Err(_) = std::fs::remove_dir_all(output) {}
; std::fs::create_dir_all(output) . then(m_e)?

; let copy = |name: String| {use core::convert::From
  ; std::fs::copy(
     String::from(input)  + "/" +  &name 
   , String::from(output) + "/" +  &name 
   ) 
  . then(m_e)
  }

; let Option::Some((cur_name, mut cur_img)) = images . next() 
  else {return Result::Err(());}
; copy(cur_name)?

; for (next_name, next_img) in images {
    let skip = 'a:{
      let (width, height) = cur_img.dimensions()
    ; const /*TILE*/SIDE : u32 = 8
    // ignoring incomplete tiles
    ; for i in 0 .. width/SIDE { for j in 0 .. height/SIDE {
        let i1 = cur_img  . view(i*SIDE, j*SIDE, SIDE, SIDE)
      ; let i2 = next_img . view(i*SIDE, j*SIDE, SIDE, SIDE)
      ; let nabs_sum = i1 . pixels() . zip(i2 . pixels())
      . map(|((_, _, p1), (_, _, p2))| { use core::num::Wrapping as W
        ; let cast = |x| W(x as i16)
        ; let [p1, p2] = [p1.0, p2.0].map(|p|p . map(cast))
        // nabs function "Hackers Delight 2-4"
        ; const SHIFT : usize                = (i16::BITS-1) as usize
        ; const NABS  : fn(W<i16>) -> W<i16> = |x| {let y = x >> SHIFT ; y - x ^ y } 
        ; let mut acc : [i16; 4] = [0; 4]
        ; for i in 0 .. p1 . len() {
             acc[i] += NABS(p1[i] - p2[i]).0
          }
        ; acc
        })
      . fold([0_i16; 4], |mut acc, item| {for i in 0 .. 4 { acc[i] += item[i] }; acc })
      ; for i in 0 .. 4 {  if nabs_sum[i] < -25 * ((SIDE * SIDE) as i16) { break 'a false } }
      }}
    ; true
    }
  ; if !skip { 
      copy(next_name)?
    ; cur_img = next_img 
    }
  }

; Result::Ok(())
}

trait Do where Self: core::marker::Sized{
  fn then<R>(self, f:impl core::ops::FnOnce(Self)->R)->R { f(self) }
}
impl<T : core::marker::Sized> Do for T {}
