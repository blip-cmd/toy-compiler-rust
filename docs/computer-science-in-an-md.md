# Computer Science in an MD: How It All Fits Together

From physical electricity to a running line of Rust, every layer in computing is an abstraction boundary built on top of the layer below it.

```text
+-------------------------------------------------------+
|  High-Level Language (Rust, Python, C)                |
+---------------------------+---------------------------+
                            | (Compiler / Assembler)
+---------------------------v---------------------------+
|  Assembly & Machine Code (OpCodes: 0s and 1s)         |
+---------------------------+---------------------------+
                            | (Executed in User Space)
+---------------------------v---------------------------+
|  Applications & Background Daemons                    |
+---------------------------+---------------------------+
                            | (System Calls: syscall)
+---------------------------v---------------------------+
|  Operating System Kernel (Memory, CPU scheduling)     |
+---------------------------+---------------------------+
                            | (Hardware Handover at Boot)
+---------------------------v---------------------------+
|  Firmware / BIOS / UEFI                                |
+---------------------------+---------------------------+
                            | (Electrical Signaling)
+---------------------------v---------------------------+
|  Physical Hardware: CPU Registers, RAM, Cache, Busses  |
+-------------------------------------------------------+
```

---

## 1. The Physical Foundation: 0s, 1s, and Memory

### Electricity to Logic
At the silicon level, digital circuits do not know what "numbers" or "letters" are. They only know **voltage levels**:
- High voltage (e.g., $1.8\text{V}$ or $3.3\text{V}$) represents logical `1`.
- Low voltage (e.g., $0\text{V}$) represents logical `0`.

Using microscopic switches called **MOSFET transistors**, engineers build logic gates (`AND`, `OR`, `NOT`, `XOR`). Combining millions of these produces arithmetic units (`ALU`) that can add, subtract, and compare binary patterns.

### ASCII and Character Encoding
How does the letter `'A'` become bits?
- By international agreement, the **ASCII table** maps standard human characters to 7-bit/8-bit integers.
- `'0'` is decimal $48$ (`00110000`).
- `'9'` is decimal $57$ (`00111001`).
- `'A'` is decimal $65$ (`01000001`).
When your lexer checks `ch.is_ascii_digit()`, it is comparing byte values between $48$ and $57$.

### RAM and the Memory Hierarchy
Memory is organized by speed and physical proximity to the CPU cores:

| Layer | Typical Access Latency | Capacity | Description |
|---|---|---|---|
| **CPU Registers** | $< 1\text{ ns}$ (Single clock cycle) | A few hundred bytes | Immediate scratchpad for calculations (`RAX`, `RBX`, `RSP`). |
| **L1/L2/L3 Cache** | $1 - 15\text{ ns}$ | A few megabytes | High-speed static RAM (SRAM) baked onto the CPU die. |
| **Main RAM (DRAM)** | $50 - 100\text{ ns}$ | Gigabytes | Dynamic RAM; capacitors that must be constantly refreshed. Fast, but volatile (clears when powered off). |
| **NVMe / SSD / Disk** | Microseconds to milliseconds | Terabytes | Non-volatile persistent flash storage. |

In RAM, every single byte has a numerical **memory address** ($0\text{x}00000000$ to $0\text{x}FFFFFFFF...$). A pointer in Rust or C is simply an integer storing one of these RAM addresses.

---

## 2. Booting Up: BIOS, Firmware, and OS Initialization

When you press the power button on a computer:

1. **Power-On Self-Test (POST)**: Motherboard components verify electrical integrity.
2. **UEFI / BIOS (Basic Input/Output System)**:
   - Non-volatile firmware stored on a ROM chip on the motherboard.
   - Initializes essential hardware: clock timings, memory controllers, keyboard, and storage controllers.
   - Reads the drive partition table (GPT/MBR) to locate the **Bootloader** (e.g., Windows Boot Manager or GRUB).
3. **The Handoff**: The bootloader loads the **OS Kernel** into RAM and jumps CPU execution to the kernel's entry point. Once the kernel takes control, UEFI steps back into a passive role.

---

## 3. The Kernel and Operating System

The **Kernel** is the core program that has total, unrestricted access to the computer's physical hardware.

### Key Responsibilities of the Kernel
- **Virtual Memory & Paging**: The kernel ensures that Program A cannot read or overwrite Program B's memory. Every program thinks it owns the entire address space; the CPU's Memory Management Unit (MMU) translates virtual addresses to physical RAM addresses.
- **CPU Scheduling**: The kernel rapidly swaps running programs on and off CPU cores thousands of times per second (preemptive multitasking), creating the illusion that everything runs concurrently.
- **Device Drivers**: Translates abstract requests ("write 512 bytes to disk") into electrical commands for specific hardware.

### User Space vs. Kernel Space
Modern CPUs support hardware protection rings (Ring 0 and Ring 3 on x86-64):
- **Ring 0 (Kernel Space)**: Unrestricted hardware access. Only the kernel runs here.
- **Ring 3 (User Space)**: Sandbox where normal applications run. Direct hardware access is forbidden.

### System Calls (`syscall`)
If a normal program in user space wants to print to the screen, open a file, or allocate heap memory, it cannot touch the screen or disk directly. It executes a CPU interrupt instruction called a **system call (`syscall`)**:
1. User application prepares arguments in CPU registers.
2. CPU triggers `syscall`, switching execution into Ring 0 (Kernel).
3. The kernel validates the request, talks to the hardware, and copies results back to user memory.
4. The CPU switches back to Ring 3 (User Space).

---

## 4. Programs, Applications, and Daemons

- **Program**: A passive file on your disk (ELF on Linux, PE `.exe` on Windows) containing compiled machine code instructions and initial data.
- **Process**: An active, running instance of a program loaded into RAM, equipped with its own virtual address space, stack, heap, and file descriptors.
- **Application**: A user-facing process usually attached to an interactive terminal or graphical desktop interface (e.g., browser, code editor, game).
- **Daemon (or Windows Service)**: A non-interactive background process that runs continuously without a terminal or window attached (e.g., `sshd` handling incoming SSH connections, system clock synchronizers, database engines).

---

## 5. From High-Level Code to Raw Instructions

Now we connect the whole picture to the compiler you are building.

### The Pipeline at a Glance

```text
High-Level Source (Rust):
    let area = 3 * radius;
          │
          ▼  [Lexer + Parser]
Abstract Syntax Tree (AST):
    BinaryOp(*) [Literal(3), Variable("radius")]
          │
          ▼  [Semantic Analyzer + Type Checker]
Validated Typed AST
          │
          ▼  [Intermediate Representation (IR)]
IR (Three-Address Code):
    %t0 = load i64, %radius
    %t1 = mul i64 3, %t0
          │
          ▼  [Backend / Code Generator]
Assembly Code (x86_64 text instructions):
    mov rax, QWORD PTR [rbp-8]
    imul rax, 3
          │
          ▼  [Assembler]
Machine Code (Raw OpCodes in binary / hex):
    48 8b 45 f8     (mov rax, [rbp-8])
    48 6b c0 03     (imul rax, 3)
```

### High-Level Language (Rust, Python, C)
Human-readable syntax designed for reasoning, safety, and abstractions (types, ownership, functions, loops).

### Assembly Code
A direct, human-readable 1-to-1 text mnemonic for CPU instructions:
- `mov`: Copy data between registers or memory.
- `add` / `imul`: Execute math in the ALU.
- `cmp` / `jmp`: Compare values and jump execution to another instruction address (how `if` and `while` work).

### Machine Code (0s and 1s)
The actual numerical bytes read by the CPU instruction decoder:
- `0x48 0x89` might tell the CPU: "interpret the next byte as source and destination register IDs, then copy data between them."
- The CPU's Instruction Pointer (`RIP`/`PC`) steps through these bytes in RAM at gigahertz speeds, decoding bit patterns into electrical control signals.

---

## Summary Takeaway

When your toy compiler finishes evaluating `12 + 3 * (4 - 1)`:
1. You wrote text characters encoded via **ASCII** in **RAM**.
2. Your **lexer** turned raw character bytes into typed **tokens**.
3. Your **parser** organized those tokens into an **AST**.
4. Your **evaluator** computed the result using CPU registers.
5. In later levels, we will lower that AST into **bytecode and machine instructions**, directly commanding the CPU's registers and memory addresses.
