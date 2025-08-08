HW ?= pc
# HW ?= f429disco

include   hw/$(HW)/$(HW).mk
include  cpu/$(CPU)/$(CPU).mk
include arch/$(ARCH)/$(ARCH).mk
include   os/$(OS)/$(OS).mk

ELF = bin/$(BINFILE).elf
DFU = bin/$(BINFILE).dfu

.PHONY: elf
elf: $(ELF)

.PHONY: dfu
dfu: $(DFU)
$(DFU): $(ELF)
	~/elf2dfuse/bin/elf2dfuse $< $@

.PHONY: qemu
qemu: $(ELF)
	$(QEMU) $(QEMU_CFG) -gdb tcp::12345 -S -kernel $<
