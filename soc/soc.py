#!/usr/bin/env python3

# This file is Copyright (c) 2019 Sean Cross <sean@xobs.io>
# This file is Copyright (c) 2018 David Shah <dave@ds0.me>
# This file is Copyright (c) 2020 Piotr Esden-Tempski <piotr@esden.net>
# This file is Copyright (c) 2020 Vadim Kaushan <admin@disasm.info>
# License: BSD

# This variable defines all the external programs that this module
# relies on.  lxbuildenv reads this variable in order to ensure
# the build will finish without exiting due to missing third-party
# programs.
LX_DEPENDENCIES = ["riscv", "icestorm", "yosys", "nextpnr-ice40"]

# Import lxbuildenv to integrate the deps/ directory
import lxbuildenv

import argparse

from migen import *
from migen.genlib.resetsync import AsyncResetSynchronizer

from litex.build.generic_platform import Pins, IOStandard, Subsignal
from litex.soc.cores.up5kspram import Up5kSPRAM
from litex.soc.cores.spi_flash import SpiFlash
from litex.soc.cores.clock import iCE40PLL
from litex.soc.integration.soc_core import SoCCore
from litex.soc.integration.builder import Builder, builder_argdict, builder_args
from litex.soc.integration.soc_core import soc_core_argdict, soc_core_args
from litex.soc.integration.doc import AutoDoc
from litex.soc.interconnect import wishbone
import litex.soc.doc as lxsocdoc

from litex_boards.platforms.icebreaker import Platform

from gpio import GPIOPeripheral
from clock import ClockPeripheral


# CRG ----------------------------------------------------------------------------------------------

class _CRG(Module, AutoDoc):
    """Icebreaker Clock Resource Generator

    The system is clocked by the external 12MHz clock. But if a sys_clk_freq is set to a value
    that is different from the default 12MHz we will feed it through the PLL block and try to
    generate a clock as close as possible to the selected frequency.
    """
    def __init__(self, platform, sys_clk_freq):
        self.clock_domains.cd_sys = ClockDomain()
        self.clock_domains.cd_por = ClockDomain()

        # # #

        # Clocks
        clk12 = platform.request("clk12")
        if sys_clk_freq == 12e6:
            self.comb += self.cd_sys.clk.eq(clk12)
        else:
            self.submodules.pll = pll = iCE40PLL(primitive="SB_PLL40_PAD")
            pll.register_clkin(clk12, 12e6)
            pll.create_clkout(self.cd_sys, sys_clk_freq, with_reset=False)
        platform.add_period_constraint(self.cd_sys.clk, 1e9 / sys_clk_freq)

        # Power On Reset
        self.reset = Signal()
        por_cycles = 4096
        por_counter = Signal(log2_int(por_cycles), reset=por_cycles - 1)
        self.comb += self.cd_por.clk.eq(self.cd_sys.clk)
        platform.add_period_constraint(self.cd_por.clk, 1e9 / sys_clk_freq)
        self.sync.por += If(por_counter != 0, por_counter.eq(por_counter - 1))
        self.comb += self.cd_sys.rst.eq(por_counter != 0)
        self.specials += AsyncResetSynchronizer(self.cd_por, self.reset)


# BaseSoC ------------------------------------------------------------------------------------------

class BaseSoC(SoCCore):
    """A SoC on iCEBreaker, optionally with a softcore CPU"""

    # Statically-define the memory map, to prevent it from shifting across various litex versions.
    SoCCore.mem_map = {
        "sram":             0x10000000,  # (default shadow @0xa0000000)
        "spiflash":         0x20000000,  # (default shadow @0xa0000000)
        "csr":              0xe0000000,  # (default shadow @0x60000000)
        "hardspi0":         0xf0000000,
        "hardspi1":         0xf0000200,
    }

    def __init__(self, flash_offset, sys_clk_freq, **kwargs):
        """Create a basic SoC for iCEBreaker.

        Returns:
            Newly-constructed SoC
        """
        platform = Platform()

        # Use SERV CPU buy default
        if "cpu_type" not in kwargs:
            kwargs["cpu_type"] = "serv"
            kwargs["cpu_variant"] = "standard"
        else:
            if kwargs["cpu_type"] == "vexriscv" and ("cpu_variant" not in kwargs):
                kwargs["cpu_variant"] = "minimal"

        # Force the SRAM size to 0, because we add our own SRAM with SPRAM
        kwargs["integrated_sram_size"] = 0
        kwargs["integrated_rom_size"] = 0

        # Set CPU reset address
        kwargs["cpu_reset_address"] = self.mem_map["spiflash"] + flash_offset

        # SoCCore
        SoCCore.__init__(self, platform, sys_clk_freq, **kwargs)

        self.submodules.crg = _CRG(platform, sys_clk_freq)

        # UP5K has single port RAM, which is a dedicated 128 kilobyte block.
        # Use this as CPU RAM.
        spram_size = 128 * 1024
        self.submodules.spram = Up5kSPRAM(size=spram_size)
        self.register_mem("sram", self.mem_map["sram"], self.spram.bus, spram_size)

        # The litex SPI module supports memory-mapped reads, as well as a bit-banged mode
        # for doing writes.
        spiflash_size = 16 * 1024 * 1024
        self.submodules.spiflash = SpiFlash(platform.request("spiflash4x"), dummy=6, endianness="little")
        self.add_csr("spiflash")

        # SPI flash cache
        l2_cache_size = 8192
        if l2_cache_size != 0:
            self.submodules.l2_cache = wishbone.Cache(
                cachesize=l2_cache_size // 4,
                master=wishbone.Interface(32),
                slave=self.spiflash.bus,
            )
            self.register_mem("spiflash", self.mem_map["spiflash"], self.l2_cache.master, size=spiflash_size)
        else:
            self.register_mem("spiflash", self.mem_map["spiflash"], self.spiflash.bus, size=spiflash_size)

        # Add ROM linker region
        self.add_memory_region("rom", self.mem_map["spiflash"] + flash_offset, spiflash_size - flash_offset, type="cached+linker")

        # User button as reset
        reset_btn = platform.request("user_btn_n")
        self.comb += self.crg.reset.eq(~reset_btn)

        # Clock peripheral holds the actual sys_clk frequency
        self.submodules.clock = ClockPeripheral(sys_clk_freq)
        self.add_csr("clock")

        # GPIO peripheral
        pin_names = ["PMOD1A:%d" % i for i in range(8)] +\
                    ["PMOD1B:%d" % i for i in range(8)] +\
                    ["PMOD2:%d" % i for i in range(8)]
        gpio_extension = [("gpio", i, Pins(name), IOStandard("LVCMOS33"))
                          for i, name in enumerate(pin_names)]
        platform.add_extension(gpio_extension)
        gpio = []
        for i in range(len(pin_names)):
            gpio.append(platform.request("gpio"))

        self.submodules.gpio = GPIOPeripheral(gpio + [
            platform.request("user_ledr_n"),
            platform.request("user_ledg_n"),
        ])
        self.add_csr("gpio")

        # Suppress synthesis output
        assert hasattr(self.platform.toolchain, "build_template")
        if self.platform.toolchain.build_template[0].startswith("yosys "):
            self.platform.toolchain.build_template[0] = \
                self.platform.toolchain.build_template[0].replace("yosys ", "yosys -q ")


# Build --------------------------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="LiteX SoC on iCEBreaker")
    parser.add_argument("--flash-offset", default=0x40000, help="Boot offset in SPI Flash")
    parser.add_argument("--sys-clk-freq", type=float, default=21e6, help="Select system clock frequency")
    parser.add_argument("--document-only", action="store_true", help="Do not build a soc. Only generate documentation.")
    parser.add_argument("--flash", action="store_true", help="Load bitstream")
    builder_args(parser)
    soc_core_args(parser)
    args = parser.parse_args()

    # Create the SOC
    soc = BaseSoC(flash_offset=args.flash_offset, sys_clk_freq=int(args.sys_clk_freq), **soc_core_argdict(args))

    # Configure command line parameter defaults
    # Don't build software -- we don't include it since we just jump to SPI flash.
    builder_kwargs = builder_argdict(args)
    builder_kwargs["compile_software"] = False

    if args.document_only:
        builder_kwargs["compile_gateware"] = False
    if builder_kwargs["csr_svd"] is None:
        builder_kwargs["csr_svd"] = "../litex-pac/soc.svd"
    if builder_kwargs["memory_x"] is None:
        builder_kwargs["memory_x"] = "../litex-pac/memory.x"

    # Create and run the builder
    builder = Builder(soc, **builder_kwargs)
    builder.build()
    lxsocdoc.generate_docs(soc, "build/documentation/", project_name="iCEBreaker LiteX RISC-V Example SOC", author="Piotr Esden-Tempski")

    # If requested load the resulting bitstream onto the iCEBreaker
    if args.flash:
        prog = soc.platform.create_programmer()
        prog.flash(0x00000000, "build/icebreaker/gateware/icebreaker.bin")


if __name__ == "__main__":
    main()
