#![no_implicit_prelude]

extern crate core;
extern crate alloc;
extern crate std;
extern crate image;

fn main()->core::result::Result<(),()> {
  fn m_e<T, E : std::fmt::Debug>(r: Result<T,E>) -> Result<T,()> 
  { if let Result::Err(e) = &r {std::println!("{e:?}",)}
  ; r . map_err(|_|())
  }
  use 
  { image::GenericImageView
  , core::{iter::Iterator, result::Result, option::Option }
  }

// language settings?
; let input          = "./in"
; let output         = "./out"
; let file_header    = "st_"
; let file_extension = "png" // note the file extention is defined as what comes __after__ the '.'
; let threshold      = 25


; let [input_path, output_path] = [input, output].map(std::fs::canonicalize).map(Result::unwrap) 
; std::println!
  ( "\n \
     \n\x1B[0m[\
       \x1B[32mConfiguration \
       \x1B[0m]\
     \n\x1B[38;5;107m   file_header \x1B[0m: {}\
     \n\x1B[38;5;107m     extension \x1B[0m: {}\
     \n\x1B[38;5;107m    input_path \x1B[0m: {}\
     \n\x1B[38;5;107m   output_path \x1B[0m: {}\
     \n\x1B[38;5;107m     threshold \x1B[0m: {}\
    "
  , file_header
  , file_extension
  , input_path.display() 
  , output_path.display()
  , threshold
  )

// ; let args : alloc::vec::Vec<String> = std::env::args() . collect();
// ; std::println!("{args:#?}\n{}", std::env::args().count())

// ; std::process::exit(0)

; let mut img_files = std::fs::read_dir(input) . then(m_e)?
  . filter_map(|e| e . ok())
  . filter(|e|
    { let Result::Ok(x) = std::fs::DirEntry::file_type(e) else {return false}
    ; let Result::Ok(y) = e . file_name() . into_string() else {return false}
    ;    x . is_file()
      && { let l  = y . len()
         ;    &y[l - file_extension . len() - 1 .. l] == std::format!{".{file_extension}"}
           && &y[0 .. file_header . len() ]           == file_header
         }
    }
  )
  . collect::<alloc::vec::Vec<_>>()
; img_files . sort_by_key(std::fs::DirEntry::file_name)

; let mut images_left = img_files.len()
; let images_total = images_left

; let mut images = img_files
  . iter()
  . map(|e|
  { use core::clone::Clone
  ; let name = e . file_name()
  ; let mut path = input_path.clone()
  ; path.push(&name)
  ; (name, path . then(image::open) . unwrap())
  })

; if let Result::Err(_) = std::fs::remove_dir_all(&output_path) {}
; std::fs::create_dir_all(&output_path) . then(m_e)?

; let copy = |name: std::ffi::OsString|
  { let [mut i, mut o]= [&input_path, &output_path] . map(core::clone::Clone::clone)
  ; [&mut i, &mut o] . iter_mut() . map(|path| path . push(&name)) . for_each(core::mem::drop)
  ; std::fs::copy(i, o) . then(m_e)
  }

; let Option::Some((cur_name, mut cur_img)) = images . next() else {return Result::Err(());}
; copy(cur_name)?
; images_left-=1


; let mut progress = 0
; let mut selected = 1

; use std::io::Write
; let ref mut stdout = std::io::stdout()

; for (next_name, next_img) in images
  { 
  // tile checking
    let skip = 'a:
    { let (width, height) = cur_img.dimensions()
    ; const /*TILE*/SIDE : u32 = 8
    // ignoring incomplete tiles
    ; for i in 0 .. width/SIDE { for j in 0 .. height/SIDE
        { let i1 = cur_img  . view(i*SIDE, j*SIDE, SIDE, SIDE)
        ; let i2 = next_img . view(i*SIDE, j*SIDE, SIDE, SIDE)
        ; let nabs_sum = i1 . pixels() . zip(i2 . pixels())
          . map(|((_, _, p1), (_, _, p2))|
            { use core::num::Wrapping as W
            ; let cast = |x| W(x as i16)
            ; let [p1, p2] = [p1.0, p2.0].map(|p|p . map(cast))

            // nabs function "Hackers Delight 2-4"
            ; const SHIFT : usize                = (i16::BITS-1) as usize
            ; const NABS  : fn(W<i16>) -> W<i16> = |x| {let y = x >> SHIFT ; y - x ^ y }
            ; let mut acc : [i16; 4] = [0; 4]
            ; for i in 0 .. p1 . len() { acc[i] += NABS(p1[i] - p2[i]).0 }
            ; acc
            }
          )
          . fold([0_i16; 4], |mut acc, item| {for i in 0 .. 4 { acc[i] += item[i] }; acc })
        ; for i in 0 .. 4 { if nabs_sum[i] < -threshold * ((SIDE * SIDE) as i16) { break 'a false } }
        }
      }
    ; true
    }
  ; if !skip
    { copy(next_name)?
    ; cur_img = next_img
    ; selected += 1
    }

  ; images_left -= 1
  ; let now = (images_total - images_left) / (images_total / 10)
  // progress bar
  ; if now + 1 != progress 
    { progress = now + 1
    ; stdout . write
      ( b"\r[   \x1B[32mProgress\x1B[0m   ] : |\x1B[38;5;240m"
      ) 
      . then(m_e)?
    ; for _ in 0 .. progress - 1    {stdout . write(b"=") . then(m_e)? ;}
    ; if progress <= 10           
      {                              stdout . write(b">") . then(m_e)? 
      ; for _ in 0 .. 10 - progress {stdout . write(b" ") . then(m_e)? ;}
      } 
    ; stdout . write(b"\x1B[0m|") . then(m_e)?
    ;
    }
  ; stdout . write(std::format!("{images_left:#10}\x1B[10D") . as_bytes()) . then(m_e)?
  ; stdout . flush() . then(m_e)?
  }

; core::mem::drop(stdout)
; std::println!("!\n\n\x1B[34;1mProcessing Complete\x1B[0m : {selected} images selected")

; Result::Ok(())
}

trait Do where Self: core::marker::Sized
{ fn then<R>(self, f:impl core::ops::FnOnce(Self)->R)->R { f(self) }
}
impl<T : core::marker::Sized> Do for T {}
