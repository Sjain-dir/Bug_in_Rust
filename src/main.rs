use std::ffi::{
    c_int,
    CString
};

#[link (name = "interface", kind = "static")]
extern "C" {
    fn test1(i : *mut c_int);
    fn test2(string1 : *mut CString, i : *mut c_int);
    //fn test2(string1 : CString, i : *mut c_int);
}

fn main() {
    trying();
}

fn trying() {
    unsafe {
        let mut string1 = CString::new("Likhe jo khat tujhe, jo teri yaad mee...........").expect("lol error in payload");
        let mut i : c_int = 69420;

        test1(&mut i);
        test2(&mut string1, &mut i);
        //test2(string1, &mut i);
    }
}


