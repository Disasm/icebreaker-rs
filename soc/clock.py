from litex.soc.interconnect.csr import AutoCSR, CSRStorage, CSRStatus, CSRField
from migen import *


class ClockPeripheral(Module, AutoCSR):
    def __init__(self, sys_clk_freq):
        self.coreclk = CSRStatus(32, description="Core clock frequency")
        self.comb += self.coreclk.status.eq(sys_clk_freq)
