SECTIONS
{
  .text 0 : {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(_stack_start - 16);

    KEEP(*(.rodata.reset_handler));
    KEEP(*(.rodata.exceptions));
    __exceptions = .;

    KEEP(*(.rodata.interrupts));
    __interrupts = .;

    ASSERT(. == 0xc0, "ISR vector has the wrong size.");

    *(.text.*);

    KEEP(*(.rust_stack));
    _stack_start = .;

    *(.rodata.*);
  }

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  }

  _sidata = LOADADDR(.data);

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  }

  /* Due to an unfortunate combination of legacy concerns,
     toolchain drawbacks, and insufficient attention to detail,
     rustc has no choice but to mark .debug_gdb_scripts as allocatable.
     We really do not want to upload it to our target, so we
     remove the allocatable bit. Unfortunately, it appears
     that the only way to do this in a linker script is
     the extremely obscure "INFO" output section type specifier. */
  .debug_gdb_scripts 0 (INFO) : {
    KEEP(*(.debug_gdb_scripts))
  }

  /DISCARD/ :
  {
    /* Unused unwinding stuff */
    *(.ARM.exidx.*)
    *(.ARM.extab.*)
  }
}

/* Do not exceed this mark in the error messages below                | */
ASSERT(__interrupts - __exceptions > 0, "
You must specify the interrupt handlers.
Create a non `pub` static variable and place it in the
'.rodata.interrupts' section. (cf. #[link_section]). Apply the
`#[used]` attribute to the variable to help it reach the linker.");

ASSERT(__interrupts - __exceptions <= 0x3c0, "
There can't be more than 240 interrupt handlers.
Fix the '.rodata.interrupts' section. (cf. #[link_section])");
