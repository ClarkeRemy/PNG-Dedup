#![no_implicit_prelude]
extern crate core;
extern crate alloc;
extern crate std;
extern crate image;


fn main()->core::result::Result<(),()> {
  fn m_e<T,E>(r:core::result::Result<T,E>)->core::result::Result<T,()> {r . map_err(|_|())}
  use {
    image::GenericImageView
  , core::iter::Iterator
  }

; let input = "./in"
; let output= "./out"
; let mut images = std::fs::read_dir(input) . then(m_e)?
. filter_map(|e| e . ok())
. filter(|e|{ use core::{ result::Result::Ok, option::Option::Some }
  ; let Ok(x) = std::fs::DirEntry::file_type(e) else {return false}
  ; let Ok(y) = e . file_name() . into_string()     else {return false}
  ; x.is_file() 
    && y.chars().nth(0) != Some('.')
    && {
      let _y =  y . as_bytes()
    ; let l  = _y . len()
    ; _y[l-4..l] == b".png"[..]
    }
  })
. map(|e| { use { alloc::string::String, core::{convert::From, clone::Clone } }
  ; let name = e . file_name() . into_string() . unwrap()
  ; (name.clone()
    , (String::from(input) + "/" + &name[..]) . then(image::open) . unwrap() 
    )
  })
. collect::<alloc::vec::Vec<_>>()

; images.dedup_by(| (_,img_2), (_,img_1)| 
    img_1.pixels()
. zip(img_2.pixels())
  . map(|((_,_,p1),(_,_,p2))| { use core::num::Wrapping as W // Simd?
    ; let cast = |x| W(x as i8)
    ; let [p1, p2] = [p1.0, p2.0].map(|p|p . map(cast))
    // nabs function "Hackers Delight 2-4"
    ; const SHIFT : usize            = (i8::BITS-1) as usize
    ; const NABS  : fn(W<i8>)->W<i8> = |x| {let y = x >> SHIFT ; y-x^y } 
    ; for i in 0 .. p1 . len() {
        if NABS(p1[i]-p2[i]) < W(-71) /* tolerance parameter later? */ { return false; };
      }
    ; true
    })
  . fold(true, |acc, i| acc&&i)
 )

; if let core::result::Result::Err(_) = std::fs::remove_dir_all(output) {}
; std::fs::create_dir_all(output) . then(m_e)?

; for (name,img) in images{ use {alloc::string::String, core::convert::From}
  ; std::println!("{name}")
  ;  img . save(String::from(output) + "/" + &name) . then(m_e)?;
  }

; core::result::Result::Ok(())
}

trait Do where Self: core::marker::Sized{
  fn then<R>(self, f:impl core::ops::FnOnce(Self)->R)->R { f(self) }
}
impl<T : core::marker::Sized> Do for T {}
