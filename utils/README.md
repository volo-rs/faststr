# Debugger Utilities

## Usage

For GDB, you can download the `faststr_gdb.py` and put it in your `.gdbinit`:

```bash
curl -fsSL https://github.com/volo-rs/faststr/raw/refs/heads/main/utils/faststr_gdb.py -o faststr_gdb.py
echo "source $PWD/faststr_gdb.py" >> $HOME/.gdbinit
```

For LLDB, you can download the `faststr_lldb.py` and put it in your `.lldbinit`:

```bash
curl -fsSL https://github.com/volo-rs/faststr/raw/refs/heads/main/utils/faststr_lldb.py -o faststr_lldb.py
echo "command script import $PWD/faststr_lldb.py" >> $HOME/.lldbinit
```

## Examples

### GDB

```
$ cargo build -p debugger-example
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s

$ gdb ../target/debug/debugger-example
(gdb) break main.rs:14
Breakpoint 1 at 0xd10d: file utils/src/main.rs, line 14.
(gdb) run
Starting program: ../target/debug/debugger-example

Breakpoint 1, debugger_example::main () at utils/src/main.rs:14
14          println!("{s1:?}");
(gdb) source faststr_gdb.py
(gdb) print s1
$1 = FastStr::Empty("")
(gdb) print s2
$2 = FastStr::Bytes("1145141919810114514191981011451419198101145141919810")
(gdb) print s3
$3 = FastStr::ArcStr("1145141919810114514191981011451419198101145141919810")
(gdb) print s4
$4 = FastStr::ArcString("1145141919810114514191981011451419198101145141919810")
(gdb) print s5
$5 = FastStr::StaticStr("1145141919810114514191981011451419198101145141919810")
(gdb) print s6
$6 = FastStr::Inline("Hello, World")
```

### LLDB

```
$ cargo build -p debugger-example
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s

$ lldb ../target/debug/debugger-example
(lldb) target create "../target/debug/debugger-example"
Current executable set to '../target/debug/debugger-example' (x86_64).
(lldb) breakpoint set --file main.rs --line 14
Breakpoint 1: 2 locations.
(lldb) run
Process 114514 launched: '../target/debug/debugger-example' (x86_64)
Process 114514 stopped
* thread #1, name = 'debugger-exampl', stop reason = breakpoint 1.1
    frame #0: 0x000055555556110d debugger-example`debugger_example::main at main.rs:14:5
   11       let s5 = FastStr::from_static_str(LONG_STRING);
   12       let s6 = FastStr::from_string(SHORT_STRING.to_string());
   13
-> 14       println!("{s1:?}");
   15       println!("{s2:?}");
   16       println!("{s3:?}");
   17       println!("{s4:?}");
(lldb) print s1
(faststr::FastStr) FastStr::Empty("")
(lldb) print s2
(faststr::FastStr) FastStr::Bytes("1145141919810114514191981011451419198101145141919810")
(lldb) print s3
(faststr::FastStr) FastStr::ArcStr("1145141919810114514191981011451419198101145141919810")
(lldb) print s4
(faststr::FastStr) FastStr::ArcString("1145141919810114514191981011451419198101145141919810")
(lldb) print s5
(faststr::FastStr) FastStr::StaticStr("1145141919810114514191981011451419198101145141919810")
(lldb) print s6
(faststr::FastStr) FastStr::Inline("Hello, World")
```
