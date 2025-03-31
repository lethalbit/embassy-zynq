# Embassy Zynq™ 7000 and Zynq UltraScale+™ HALs

> [!CAUTION]
> These are a *huge* work in progress, and only tested minimally with hardware I have *on hand*
> they may not be fully stable, or functional.

This project provides 2 HAL crates to provide [embassy-rs] with support for the AMD [Zynq™ 7000] SoC and the AMD [Zynq UltraScale+™ MPSoC]/[Zynq UltraScale+™ RFSoC].

## About The Zynq™ SoCs

The AMD (formally Xilinx) Zynq™ SoCs are devices that combine an SoC core/cores (referred to as the PS or Programmable System) with FPGA fabric (referred to as the PL or Programmable Logic) as such they are "hybrid" devices.

The flow for getting a Zynq™ device up and running is fairly complicated, and involves core configuration along with custom hardware via an HDL design done in [Vivado], along with either a [PetaLinux] image or some other bare metal application done in C or C++ from [Vitis], this is then packaged together into an image the Zynq™ boots from.

The HDL design is **required** for the Zynq™ to start up at all, as it is responsible for setting up the core, along with it's peripheral configuration and placement.

> [!IMPORTANT]
> **A Note about the PL**
>
> On the Zynq™ devices, the PL is connected to the PS via some [AXI] interfaces, and as the hardware
> on the PL is entirely custom, we can't provide fully wrapped devices or register definitions
> configuration for it.

## `embassy-zynq-7000`

The [`embassy-zynq-7000`] crate provides the HAL for the [Zynq™ 7000].

The Zynq™ 7000 itself comes in 2 variants SoC wise that we care about, the 7000 and the 7000S:

* 7000S - Single-core ARM Cortex-A9 MPCore™
* 7000 - Dual-core ARM Cortex-A9 MPCore™

## `embassy-zynq-usplus`

> [!WARNING]
> This is just a stub, it won't really be worked on until the [`embassy-zynq-7000`] HAL is
> somewhat functional.

The [`embassy-zynq-usplus`] crate provides the HAL for the [Zynq UltraScale+™ MPSoC] and [Zynq UltraScale+™ RFSoC] family of devices.

The MPSoC has 3 variants, the CG, EG, and EV, core-wise we can group the EG and EV, so the following 2 configurations are what we care about:

* CG
  * Dual-core Arm Cortex-A53 MPCore™
  * Dual-core Arm Cortex-R5F MPCore™
* EG/EV
  * Quad-core Arm Cortex-A53 MPCore™
  * Dual-core Arm Cortex-R5F MPCore™
  * Arm Mali-400 MP2 GPU

In addition, the EV variants of the MPSoC has hardware H.264/H.265.

The RFSoC has a core configuration of:

* Quad-core Arm Cortex-A53 MPCore™
* Dual-core Arm Cortex-R5F MPCore™

## License

The Zynq™ 7000 and Zynq UltraScale+™ HALs for embassy are licensed under the [BSD-3-Clause] license. The full text of which can be found in the [`LICENSE`] file.

[embassy-rs]: https://embassy.dev/
[Zynq™ 7000]: https://www.amd.com/en/products/adaptive-socs-and-fpgas/soc/zynq-7000.html
[Zynq UltraScale+™ MPSoC]: https://www.amd.com/en/products/adaptive-socs-and-fpgas/soc/zynq-ultrascale-plus-mpsoc.html
[Zynq UltraScale+™ RFSoC]: https://www.amd.com/en/products/adaptive-socs-and-fpgas/soc/zynq-ultrascale-plus-rfsoc.html
[Vivado]: https://www.amd.com/en/products/software/adaptive-socs-and-fpgas/vivado.html
[PetaLinux]: https://www.amd.com/en/products/software/adaptive-socs-and-fpgas/embedded-software/petalinux-sdk.html
[Vitis]: https://www.amd.com/en/products/software/adaptive-socs-and-fpgas/vitis.html
[`embassy-zynq-7000`]: ./embassy-zynq-7000/
[`embassy-zynq-usplus`]: ./embassy-zynq-usplus/
[BSD-3-Clause]: https://spdx.org/licenses/BSD-3-Clause.htm
[`LICENSE`]: ./LICENSE
