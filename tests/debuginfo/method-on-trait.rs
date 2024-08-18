//@ compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// STACK BY REF
// gdb-command:print *self
// gdb-check:$1 = method_on_trait::Struct {x: 100}
// gdb-command:print arg1
// gdb-check:$2 = -1
// gdb-command:print arg2
// gdb-check:$3 = -2
// gdb-command:continue

// STACK BY VAL
// gdb-command:print self
// gdb-check:$4 = method_on_trait::Struct {x: 100}
// gdb-command:print arg1
// gdb-check:$5 = -3
// gdb-command:print arg2
// gdb-check:$6 = -4
// gdb-command:continue

// OWNED BY REF
// gdb-command:print *self
// gdb-check:$7 = method_on_trait::Struct {x: 200}
// gdb-command:print arg1
// gdb-check:$8 = -5
// gdb-command:print arg2
// gdb-check:$9 = -6
// gdb-command:continue

// OWNED BY VAL
// gdb-command:print self
// gdb-check:$10 = method_on_trait::Struct {x: 200}
// gdb-command:print arg1
// gdb-check:$11 = -7
// gdb-command:print arg2
// gdb-check:$12 = -8
// gdb-command:continue

// OWNED MOVED
// gdb-command:print *self
// gdb-check:$13 = method_on_trait::Struct {x: 200}
// gdb-command:print arg1
// gdb-check:$14 = -9
// gdb-command:print arg2
// gdb-check:$15 = -10
// gdb-command:continue


// === LLDB TESTS ==================================================================================

// lldb-command:run

// STACK BY REF
// lldb-command:v *self
// lldbg-check:[...] { x = 100 }
// lldb-command:v arg1
// lldbg-check:[...] -1
// lldb-command:v arg2
// lldbg-check:[...] -2
// lldb-command:continue

// STACK BY VAL
// lldb-command:v self
// lldbg-check:[...] { x = 100 }
// lldb-command:v arg1
// lldbg-check:[...] -3
// lldb-command:v arg2
// lldbg-check:[...] -4
// lldb-command:continue

// OWNED BY REF
// lldb-command:v *self
// lldbg-check:[...] { x = 200 }
// lldb-command:v arg1
// lldbg-check:[...] -5
// lldb-command:v arg2
// lldbg-check:[...] -6
// lldb-command:continue

// OWNED BY VAL
// lldb-command:v self
// lldbg-check:[...] { x = 200 }
// lldb-command:v arg1
// lldbg-check:[...] -7
// lldb-command:v arg2
// lldbg-check:[...] -8
// lldb-command:continue

// OWNED MOVED
// lldb-command:v *self
// lldbg-check:[...] { x = 200 }
// lldb-command:v arg1
// lldbg-check:[...] -9
// lldb-command:v arg2
// lldbg-check:[...] -10
// lldb-command:continue

#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

#[derive(Copy, Clone)]
struct Struct {
    x: isize
}

trait Trait {
    fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize;
    fn self_by_val(self, arg1: isize, arg2: isize) -> isize;
    fn self_owned(self: Box<Self>, arg1: isize, arg2: isize) -> isize;
}

impl Trait for Struct {

    fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize {
        zzz(); // #break
        self.x + arg1 + arg2
    }

    fn self_by_val(self, arg1: isize, arg2: isize) -> isize {
        zzz(); // #break
        self.x + arg1 + arg2
    }

    fn self_owned(self: Box<Struct>, arg1: isize, arg2: isize) -> isize {
        zzz(); // #break
        self.x + arg1 + arg2
    }
}

fn main() {
    let stack = Struct { x: 100 };
    let _ = stack.self_by_ref(-1, -2);
    let _ = stack.self_by_val(-3, -4);

    let owned: Box<_> = Box::new(Struct { x: 200 });
    let _ = owned.self_by_ref(-5, -6);
    let _ = owned.self_by_val(-7, -8);
    let _ = owned.self_owned(-9, -10);
}

fn zzz() {()}
