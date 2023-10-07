# Bug_in_Rust
<!--
Thank you for filing a bug report! 🐛 Please provide a short summary of the bug,
along with any information you feel relevant to replicating the bug.
-->
----------------------
**Summary :** 
when i am passing 2 argument to a C function from rust, doing changes in the first parameter affects the value of the second parameter
----------------------
I tried this code in which i am just passing some values from Rust to C :
_you can also repeat this bug , code is in this repository :_ https://github.com/Sjain-dir/Bug_in_Rust

**in interface.c code** 
```C
void test1(int* i) {
    printf("in test 1 :::\n");
    printf("i pointer value is : %p\n",i);
    printf("i -> value in that addressr is : %d\n",*i);
}

void test2(char* string1, int *i) {
    printf("in test 2 :::\n");
    printf("i pointer value is : %p\n",i);
    printf("i value in that address is : %d\n",*i);
    printf("string1 value is : %s\n",string1);
}
```
In test1, i am just taking a integer pointer and returning the address and its value
in test2, i am doing the same, but i am also taking character pointer

**in main.rs** 
```rust
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
```
here i am just calling functions from C 
in trying function, i am declare a string and a int and passed its pointer to functions
and after `cargo run`
and the output is as expected :
```
in test 1 :::
i pointer value is : 0x7fff2d8fbbc4
i -> value in that address is : 69420
in test 2 :::
i pointer value is : 0x7fff2d8fbbc4
i value in that address is : 69420
string1 value is : ���FV
```
### **But : the bug is .....**
when i change do a little changes in passing CString like in `main.rs` : 
```rust
extern "C" {
    fn test1(i : *mut c_int);
    //fn test2(string1 : *mut CString, i : *mut c_int);
    fn test2(string1 : CString, i : *mut c_int);
}
```
in `trying()` function
```rust
        test1(&mut i);
        //test2(&mut string1, &mut i);
        test2(string1, &mut i);
```
So the output is : 
```
in test 1 :::
i pointer value is : 0x7fffdfbeff10
i -> value in that address is : 69420
in test 2 :::
i pointer value is : 0x31
Segmentation fault
```
test1 is running perfectly and giving expected output,
**but** in test2 output, somehow, **making changes in CString affects the output of `i` integer pointer**
this shouldn't happen as changes in the first parameter shouldn't affect the second parameter

