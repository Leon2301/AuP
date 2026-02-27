/// AuP Hilfsfunktionen zur Eingabe von Integerparametern
///
/// #Beispiele
/// ```rust
/// let x = arg_i32(1);
/// let y = argr_u16(2)?;
/// if let Some(z) = argo_u8(3) { ...
/// ```
///
/// #Panic
/// Alle Funktionen arg_xNN() paniken, wenn das Argument nicht angegeben wurde
/// oder nicht korrekt formatiert ist.

    use std::str::FromStr;
    use std::fmt::Debug;
    use std::io::{Error, ErrorKind,Result};

    /* Für den interessierten Leser:
       
       1. Natürlich ist der Code völlig unrostig. Eigentlich sollten keine einzelnen Funktionen definiert werden, 
          sondern ein Trait mit assoziierten Funktionen.
          Jedoch sind im AuP-Kurs bei der ersten Nutzung von 'aup.rs' noch keine Traits und keine vollqualifizierten 
          Namen eingeführt.
       2. Generische Funktionen sind ebenfalls noch nicht eingeführt, so dass 'argX<T>' nicht direkt genutzt
          werden kann.
       3. Ein einfaches Macro 'generate_all_args!(type)' scheitert an der Hygiene: concat_idents! ist noch
          experimentell.
       4. Eine Alternative wäre, statt der Funktionen entsprechende Macros zur Verfügung zu stellen => TODO
    */
    pub fn arg_len() -> usize {
        std::env::args().len()
    }
    
    fn arg<T: FromStr>(nd: usize) -> T  where <T as FromStr>::Err: Debug {
	std::env::args().nth(nd).unwrap().parse::<T>().unwrap()
    }

    fn argr<T: FromStr>(nd: usize) -> Result<T> {
	match std::env::args().nth(nd).ok_or(Error::from(ErrorKind::NotFound))?.parse() {
	    Ok(val) => Ok(val),
	    Err(_) => Err(Error::from(ErrorKind::InvalidInput))
	}
    }

    fn argo<T: FromStr>(nd: usize) -> Option<T> {
	match std::env::args().nth(nd)?.parse() {
	    Ok(val) => Some(val),
	    Err(_)  => None
	}
    }

    macro_rules!gen_arg{
        ($i:ident,$t:ty) => {
           #[allow(dead_code)]
            pub fn $i(nd: usize) -> $t {
              arg::<$t>(nd)
            }
        }
    }
    
    macro_rules!gen_argr{
        ($i:ident,$t:ty) => {
           #[allow(dead_code)]
            pub fn $i(nd: usize) -> Result<$t> {
              argr::<$t>(nd)
            }
        }
    }
    
    macro_rules!gen_argo{
        ($i:ident,$t:ty) => {
           #[allow(dead_code)]
            pub fn $i(nd: usize) -> Option<$t> {
              argo::<$t>(nd)
            }
        }
    }

/* Ab hier werden die einzelnen Funktionen instanziiert */    
    gen_arg!{arg_u8,u8}
    gen_argr!{argr_u8,u8}
    gen_argo!{argo_u8,u8}

    gen_arg!{arg_u16,u16}
    gen_argr!{argr_u16,u16}
    gen_argo!{argo_u16,u16}
    
    gen_arg!{arg_u32,u32}
    gen_argr!{argr_u32,u32}
    gen_argo!{argo_u32,u32}

    gen_arg!{arg_i8,i8}
    gen_argr!{argr_i8,i8}
    gen_argo!{argo_i8,i8}

    gen_arg!{arg_i16,i16}
    gen_argr!{argr_i16,i16}
    gen_argo!{argo_i16,i16}
    
    gen_arg!{arg_i32,i32}
    gen_argr!{argr_i32,i32}
    gen_argo!{argo_i32,i32}

    gen_arg!{arg_f32,f32}
    gen_argr!{argr_f32,f32}
    gen_argo!{argo_f32,f32}

    gen_arg!{arg_f64,f64}
    gen_argr!{argr_f64,f64}
    gen_argo!{argo_f64,f64}
