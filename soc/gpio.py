from litex.soc.interconnect.csr import AutoCSR, CSRStorage, CSRStatus, CSRField
from migen import *


class GPIOPeripheral(Module, AutoCSR):
    def __init__(self, pads):
        assert len(pads) <= 32

        fields = [
            CSRField(name="dir%d" % i, description="Pin direction", values=[
                ("0", "input", "Input mode"),
                ("1", "output", "Output mode"),
            ])
            for i in range(32)
        ]
        self.dir = CSRStorage(32, description="GPIO pin direction", fields=fields)

        fields = [
            CSRField(name="otype%d" % i, description="Pin output type", values=[
                ("0", "push_pull", "Push-pull output"),
                ("1", "open_drain", "Open-drain output"),
            ])
            for i in range(32)
        ]
        self.otyper = CSRStorage(32, description="GPIO output type register", fields=fields)

        fields = [
            CSRField(name="id%d" % i, description="Pin input data")
            for i in range(32)
        ]
        self.idr = CSRStatus(32, description="GPIO input data register", fields=fields)

        fields = [
            CSRField(name="od%d" % i, description="Pin output data")
            for i in range(32)
        ]
        self.odr = CSRStorage(32, description="GPIO output data register", write_from_dev=True, fields=fields)

        fields = [
            CSRField(name="bs%d" % i, description="Pin set bit", pulse=True, values=[
                ("1", "set", "Set bit"),
            ])
            for i in range(32)
        ]
        self.bsr = CSRStorage(32, description="GPIO bit set register", fields=fields)

        fields = [
            CSRField(name="bs%d" % i, description="Pin reset bit", pulse=True, values=[
                ("1", "reset", "Reset bit"),
            ])
            for i in range(32)
        ]
        self.brr = CSRStorage(32, description="GPIO bit reset register", fields=fields)

        self.comb += [
            If(self.bsr.re,
                self.odr.dat_w.eq(self.odr.storage | self.bsr.storage),
            ).Elif(self.brr.re,
                self.odr.dat_w.eq(self.odr.storage & ~self.brr.storage),
            ).Else(
                self.odr.dat_w.eq(self.odr.storage)
            ),
            self.odr.we.eq(self.bsr.re | self.brr.re),
        ]

        for i in range(32):
            pad = None
            if i < len(pads):
                pad = pads[i]

            if pad is None:
                continue

            t = TSTriple()
            self.specials += t.get_tristate(pad)

            dir_i = self.dir.storage[i]
            otyper_i = self.otyper.storage[i]
            odr_i = self.odr.storage[i]
            self.comb += [
                t.oe.eq(~(otyper_i & odr_i) & dir_i),
                t.o.eq(odr_i),
                self.idr.status[i].eq(t.i),
            ]
