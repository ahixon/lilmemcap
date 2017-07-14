#![feature(used)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(never_type)]
#![feature(unsize)]
#![feature(compiler_builtins_lib)]
#![no_std]

#[macro_use]
mod serial;
mod pd;

extern crate compiler_builtins;
extern crate embedded_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate rk3399_m0;
extern crate rockchip;
use rockchip::i2c::{I2C, I2CTrait};

// use cortex_m::asm;
// use cortex_m::interrupt;

const RK808_ADDR:u8 = 0x1b;			// connected to I2C0
const SYR837_ADDR:u8 = 0x40;		// U11 (VDD_CPU_B) on IC20
const SYR838_ADDR:u8 = 0x41;		// U8 (GPU) on I2C0

const INA219_ADDR:u8 = 0x40;
// something at 0x1c

const FUSB302B_ADDR:u8 = 0b0100010;

const INA219_REG_CONFIG:u8 = 0x00;
const INA219_REG_SHUNTVOLTAGE:u8 = 0x01;
const INA219_REG_BUSVOLTAGE:u8 = 0x02;
const INA219_REG_CURRENT:u8 = 0x04;
const INA219_REG_CALIBRATION:u8 = 0x05;

const INA219_CONFIG_BVOLTAGERANGE_32V:u16 = 0x2000;
const INA219_CONFIG_GAIN_8_320MV:u16 = 0x1800;
const INA219_CONFIG_BADCRES_12BIT:u16 = 0x0400; // 12-bit bus res = 0..4097
const INA219_CONFIG_SADCRES_12BIT_1S_532US:u16 = 0x0018; // 1 x 12-bit shunt sample
const INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS:u16 = 0x0007;

const ina219_currentDivider_mA:u16 = 10;

const PLL_MODE_SLOW:u8 = 0;
const PLL_MODE_NORM:u8 = 1;

const MHZ:u32 = 1000*1000;

const MAX_WAIT_COUNT:u32 = 10000;

fn rkclk_pll_get_rate_ppll() -> u32 {
	let pmucru = unsafe { &*rk3399_m0::PMUCRU.get() };

	let mode = pmucru.pmucru_ppll_con3.read().pll_work_mode().bits();

	match mode {
	    PLL_MODE_SLOW => 24 * MHZ,
	    PLL_MODE_NORM => {
			// integer mode
			let mut rate64:u64 = (24 * MHZ) as u64 * pmucru.pmucru_ppll_con0.read().fbdiv().bits() as u64;
			rate64 = rate64 / pmucru.pmucru_ppll_con1.read().refdiv().bits() as u64;

			// fractional mode
			if pmucru.pmucru_ppll_con3.read().dsmpd().bit_is_clear() {
				let mut frac_rate64:u64 = (24 * MHZ) as u64 * pmucru.pmucru_ppll_con2.read().fracdiv().bits() as u64;
				frac_rate64 = pmucru.pmucru_ppll_con1.read().refdiv().bits() as u64;

				rate64 += frac_rate64 >> 24;
			}

			rate64 = rate64 / pmucru.pmucru_ppll_con1.read().postdiv1().bits() as u64;
			rate64 = rate64 / pmucru.pmucru_ppll_con1.read().postdiv2().bits() as u64;
			rate64 as u32
	    },
	    _ 			  => 32768 // deep slow mode
	}
}

fn rkclk_pll_get_rate_gpll() -> u32 {
	let cru = unsafe { &*rk3399_m0::CRU.get() };

	let mode = cru.cru_gpll_con3.read().pll_work_mode().bits();

	match mode {
	    PLL_MODE_SLOW => 24 * MHZ,
	    PLL_MODE_NORM => {
			// integer mode
			let mut rate64:u64 = (24 * MHZ) as u64 * cru.cru_gpll_con0.read().fbdiv().bits() as u64;
			rate64 = rate64 / cru.cru_gpll_con1.read().refdiv().bits() as u64;

			// fractional mode
			if cru.cru_gpll_con3.read().dsmpd().bit_is_clear() {
				let mut frac_rate64:u64 = (24 * MHZ) as u64 * cru.cru_gpll_con2.read().fracdiv().bits() as u64;
				frac_rate64 = cru.cru_gpll_con1.read().refdiv().bits() as u64;

				rate64 += frac_rate64 >> 24;
			}

			rate64 = rate64 / cru.cru_gpll_con1.read().postdiv1().bits() as u64;
			rate64 = rate64 / cru.cru_gpll_con1.read().postdiv2().bits() as u64;
			rate64 as u32
	    },
	    _ 			  => 32768 // deep slow mode
	}
}

fn rkclk_pll_get_rate_cpll() -> u32 {
	let cru = unsafe { &*rk3399_m0::CRU.get() };

	let mode = cru.cru_cpll_con3.read().pll_work_mode().bits();

	match mode {
	    PLL_MODE_SLOW => 24 * MHZ,
	    PLL_MODE_NORM => {
			// integer mode
			let mut rate64:u64 = (24 * MHZ) as u64 * cru.cru_cpll_con0.read().fbdiv().bits() as u64;
			rate64 = rate64 / cru.cru_cpll_con1.read().refdiv().bits() as u64;

			// fractional mode
			if cru.cru_cpll_con3.read().dsmpd().bit_is_clear() {
				let mut frac_rate64:u64 = (24 * MHZ) as u64 * cru.cru_cpll_con2.read().fracdiv().bits() as u64;
				frac_rate64 = cru.cru_cpll_con1.read().refdiv().bits() as u64;

				rate64 += frac_rate64 >> 24;
			}

			rate64 = rate64 / cru.cru_cpll_con1.read().postdiv1().bits() as u64;
			rate64 = rate64 / cru.cru_cpll_con1.read().postdiv2().bits() as u64;
			rate64 as u32
	    },
	    _ 			  => 32768 // deep slow mode
	}
}


fn rkclk_get_i2c4_clk() -> u32 {
	let pmu_pll = rkclk_pll_get_rate_ppll();
	let pmucru = unsafe { &*rk3399_m0::PMUCRU.get() };

	let div = pmucru.pmucru_clksel_con3.read().i2c4_div_con().bits() as u32 + 1;
	return pmu_pll / div;
}

// fn rkclk_get_i2c0_clk() -> u32 {
// 	let pmu_pll = rkclk_pll_get_rate_ppll();
// 	let pmucru = unsafe { &*rk3399_tools::PMUCRU.get() };

// 	let div = pmucru.pmucru_clksel_con2.read().i2c0_div_con().bits() as u32 + 1;
// 	return pmu_pll / div;
// }

// fn rkclk_get_i2c1_clk() -> u32 {
// 	let pmu_pll = rkclk_pll_get_rate_ppll();
// 	let cru = unsafe { &*rk3399_tools::CRU.get() };

// 	let div = cru.cru_clksel_con61.read().clk_i2c1_div_con().bits() as u32 + 1;
// 	let sel = cru.cru_clksel_con61.read().clk_i2c1_pll_sel().bit();

// 	if sel {
// 		// general pll
// 		rkclk_pll_get_rate_gpll() / div
// 	} else {
// 		// codec pll
// 		rkclk_pll_get_rate_cpll() / div
// 	}
// }


fn rk_ceil(a:u32, b:u32) -> u32  {
	let _a = a as u64;
	let _b = b as u64;

	((_a + _b  - 1) / _b) as u32
}

fn i2c_get_div(div:u32) -> (u32, u32) {
	if div % 2 == 0 {
		(div / 2, div / 2)
	} else {
		(rk_ceil(div, 2), div / 2)
	}
}

fn i2c4_set_clk(i2c4_regs:&rk3399_m0::I2C4, scl_rate:u32) -> () {
	let i2c_rate = rkclk_get_i2c4_clk();

	let div = rk_ceil(i2c_rate, scl_rate * 8) - 2;
	let (divh, divl) = if div < 0 {
		(0, 0)
	} else {
		i2c_get_div(div)
	};

	i2c4_regs.rki2c_clkdiv.write(|w| unsafe { w.
		clkdivh().bits(divh as u16).
		clkdivl().bits(divl as u16)
	});
}


// fn i2c1_set_clk(i2c1_regs:&rk3399_tools::I2C1, scl_rate:u32) -> () {
// 	let i2c_rate = rkclk_get_i2c1_clk();

// 	let div = rk_ceil(i2c_rate, scl_rate * 8) - 2;
// 	let (divh, divl) = if div < 0 {
// 		(0, 0)
// 	} else {
// 		i2c_get_div(div)
// 	};

// 	i2c1_regs.rki2c_clkdiv.write(|w| unsafe { w.
// 		clkdivh().bits(divh as u16).
// 		clkdivl().bits(divl as u16)
// 	});
// }

fn read_ina219(i2c4:&I2C<rk3399_m0::I2C4>) {
	// read bus voltage
	let mut ina219buf:[u8; 2] = [0; 2];
	i2c4.read_from(INA219_ADDR, Some(INA219_REG_BUSVOLTAGE), &mut ina219buf).expect(
		"reading INA219 bus voltage");

	let busvolt_resp:u16 = (ina219buf[0] as u16) << 8 | ina219buf[1] as u16;
	let busvolt_raw:u16 = (busvolt_resp >> 3) * 4;

	// let busvolt = (busvolt_raw as f32) * 0.001;
	let busvolt = busvolt_raw / 1000;
	println!("Bus voltage: {:?}V", busvolt);

	// read shunt voltage
	i2c4.read_from(INA219_ADDR, Some(INA219_REG_SHUNTVOLTAGE), &mut ina219buf).expect(
		"reading INA219 shunt voltage");

	let shuntvolt_raw:u16 = (ina219buf[0] as u16) << 8 | ina219buf[1] as u16;
	// let shuntvolt_mv = (busvolt_raw as f32) * 0.01;
	let shuntvolt_mv = shuntvolt_raw / 100;
	println!("Shunt voltage: {:?}mV", shuntvolt_mv);

	let loadvoltage = busvolt + (shuntvolt_mv / 1000);
	println!("Load voltage: {:?}V", loadvoltage);

	// read current
	i2c4.read_from(INA219_ADDR, Some(INA219_REG_CURRENT), &mut ina219buf).expect(
		"reading INA219 current");

	let current_raw:u16 = (ina219buf[0] as u16) << 8 | ina219buf[1] as u16;
	let current = current_raw / 10;
	// let current_float:f32 = current_raw as f32;
	// let current = current_float / ina219_currentDivider_mA as f32;

	println!("Current: {:?}mA", current);
}

fn main() {
	println!("Hello from lilmemcap!");

	let pmugrf = unsafe { &*rk3399_m0::PMUGRF.get() };

	// setup iomux to select I2C4 line
	pmugrf.pmugrf_gpio1b_iomux.modify(|_, w| unsafe {
		w.
		write_enable().bits(
			3 << 8 |
			3 << 6
		).
		gpio1b3_sel().bits(1).	// i2c4 sda
		gpio1b4_sel().bits(1)	// i2c4 scl
	});

	// try to read i2c
	let i2c0_regs = unsafe { &*rk3399_m0::I2C0.get() };
	let i2c0 = I2C(i2c0_regs);

	let i2c4_regs = unsafe { &*rk3399_m0::I2C4.get() };
	let i2c4 = I2C(i2c4_regs);
	i2c4_set_clk(i2c4_regs, 100 * 1000); // 100KHz

	// setup iomux to select I2C1 lines
	// let grf = unsafe { &*rk3399_tools::GRF.get() };
	// grf.grf_gpio4a_iomux.modify(|_, w| unsafe {
	// 	w.
	// 	write_enable().bits(
	// 		3 << 2 |
	// 		3 << 4).
	// 	gpio4a1_sel().bits(1). 	// i2c1 sda
	// 	gpio4a2_sel().bits(1)	// i2c1 scl
	// });

	// let i2c1_regs = unsafe { &*rk3399_tools::I2C1.get() };
	// let i2c1 = I2C(i2c1_regs);
	// i2c1_set_clk(i2c1_regs, 100 * 1000); // 100KHz

	// bus probe
	// for i in 0..0x99 {
	// 	let mut buf:[u8; 1] = [0; 1];
	// 	let res = i2c4.read_from(i, Some(0x01), &mut buf);
	// 	println!("read from 0x{:x}: {:?} {:?}", i, res, buf);	
	// }
	
	// from Adafruit_INA219::setCalibration_32V_2A(void):
	let ina219_calValue:u16 = 4096;
	let ina219_powerDivider_mW = 2;

	// calibrate INA219
	let calbuf:[u8; 2] = [((ina219_calValue >> 8) & 0xff) as u8, (ina219_calValue & 0xff) as u8];
	i2c4.write_to(INA219_ADDR, Some(INA219_REG_CALIBRATION), &calbuf).expect(
		"writing INA219 calibration register");

	let config:u16 = INA219_CONFIG_BVOLTAGERANGE_32V |
		INA219_CONFIG_GAIN_8_320MV | 
		INA219_CONFIG_BADCRES_12BIT |
        INA219_CONFIG_SADCRES_12BIT_1S_532US |
        INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS;

    let setupbuf:[u8; 2] = [((config >> 8) & 0xff) as u8, (config & 0xff) as u8];
	i2c4.write_to(INA219_ADDR, Some(INA219_REG_CONFIG), &setupbuf).expect(
		"writing INA219 config register");

	read_ina219(&i2c4);

	// register 0x28 on rk808 should read back 0b00011111 = 31
	let mut rk808_buf:[u8; 1] = [0; 1];
	i2c0.read_from(RK808_ADDR, Some(0x23), &mut rk808_buf).expect("read DCDC_EN_REG");
	let current_dcdc = rk808_buf[0];

	println!("DCDC_EN_REG: {:?}", rk808_buf);

	i2c0.read_from(RK808_ADDR, Some(0x24), &mut rk808_buf).expect("read LDO_EN_REG");
	println!("LDO_EN_REG: {:?}", rk808_buf);

	// disable LDO1, LDO2, LDO4, LDO5, LDO7
	rk808_buf[0] = rk808_buf[0] & 
		!(1 << 0) &
		!(1 << 1) &
		!(1 << 3) &
		!(1 << 4) &
		!(1 << 6);

	i2c0.write_to(RK808_ADDR, Some(0x24), &rk808_buf).expect("update LDO_EN_REG");

	i2c0.read_from(RK808_ADDR, Some(0x24), &mut rk808_buf).expect("re-read LDO_EN_REG");
	println!("LDO_EN_REG now: {:?}", rk808_buf);

	// GPU
	i2c0.read_from(SYR838_ADDR, Some(0x00), &mut rk808_buf).expect("read GPU VSEL0");
	println!("GPU VSEL0: {:?}", rk808_buf);

	let disabled_syr:[u8; 1] = [rk808_buf[0] & !(1 << 7); 1];

	i2c0.read_from(SYR838_ADDR, Some(0x05), &mut rk808_buf).expect("read GPU VGOOD");
	println!("GPU VGOOD: {:?}", rk808_buf);

	println!("Changing VSEL0 and VSEL1 to: {:?}", disabled_syr);
	i2c0.write_to(SYR838_ADDR, Some(0x00), &disabled_syr).expect("update GPU VSEL0");
	i2c0.write_to(SYR838_ADDR, Some(0x01), &disabled_syr).expect("update GPU VSEL1");

	i2c0.read_from(SYR838_ADDR, Some(0x00), &mut rk808_buf).expect("re-read GPU VSEL0");
	println!("GPU VSEL0 now: {:?}", rk808_buf);

	i2c0.read_from(SYR838_ADDR, Some(0x05), &mut rk808_buf).expect("re-read GPU VGOOD");
	println!("GPU VGOOD now: {:?}", rk808_buf);

	// disable VSW0 and VSW1
	rk808_buf[0] = current_dcdc & !(1 << 5) & !(1 << 6);

	i2c0.write_to(RK808_ADDR, Some(0x23), &mut rk808_buf).expect("disable VSW0, VSW1");

	i2c0.read_from(RK808_ADDR, Some(0x23), &mut rk808_buf).expect("read VSW result");
	println!("DCDC_EN_REG now: {:?}", rk808_buf);

	read_ina219(&i2c4);

	// okay, so PWRDN_CON seems to be used to turn on/off power domains
	// (after idling the bus via the PMU as well)
	// and PWRDN_ST is used to check the state

	let pmu = unsafe { &*rk3399_m0::PMU.get() };

    pd::disable_isp0(pmu);
    pd::disable_vo(pmu);
    pd::disable_edp(pmu);
    pd::disable_iep(pmu);
    pd::disable_rga(pmu);
    pd::disable_gic(pmu);
    pd::disable_gpu(pmu);

    pd::disable_usb3(pmu);
    pd::disable_isp1(pmu);
    pd::disable_perihp(pmu);
    pd::disable_hdcp(pmu);
    pd::disable_gmac(pmu);
    pd::disable_vdu(pmu);
    pd::disable_emmc(pmu);
    pd::disable_sdioaudio(pmu);
    
    pd::disable_tcpd0(pmu);
    pd::disable_tcpd1(pmu);
    pd::disable_sd(pmu);

    // SCU turns off now?
    unsafe {
	    pmu.pmu_cci500_con.write(|w| 
	    	w.write_enable().bits(
	    		1 << 1  |
	    		1 << 6  |
	    		1 << 7
			).
	    	clr_preq_cci500().set_bit().
	    	clr_qreq_cci500().set_bit().
	    	qgating_cci500_cfg().set_bit()
		);
	}

	// TODO: probably actually just need to set PWRMODE_CON

    pmu.pmu_cpu0apm_con.write(|w| w.cpu_l0_wfi_pwrdn_en().set_bit());
    pmu.pmu_cpu0bpm_con.write(|w| w.cpu_b0_wfi_pwrdn_en().set_bit());

    pmu.pmu_cpu1apm_con.write(|w| w.cpu_l1_wfi_pwrdn_en().set_bit());
    pmu.pmu_cpu1bpm_con.write(|w| w.cpu_b0_wfi_pwrdn_en().set_bit());	// FIXME: doc typo

    pmu.pmu_cpu2apm_con.write(|w| w.cpu_l2_wfi_pwrdn_en().set_bit());
    pmu.pmu_cpu3apm_con.write(|w| w.cpu_l3_wfi_pwrdn_en().set_bit());

    unsafe { asm!("dsb sy"); }

    // need to idle request via ADB400
    unsafe {
	    pmu.pmu_adb400_con.write(|w| w.
	    	write_enable().bits(
	    		1 << 9  |
	    	  	1 << 10 |
	    	 	1 << 11
	    	).
	    	clr_core_l().set_bit().
	    	clr_core_l_2gic().set_bit().
	    	clr_gic2_core_l().set_bit()
		);

		pmu.pmu_adb400_con.write(|w| w.
	    	write_enable().bits(
	    		1 << 1
	    	).
	    	pwrdwn_req_core_l().set_bit()
		);
	}

	unsafe { asm!("dsb sy"); }

	// # mmio_write_32(PMU_BASE + PMU_ADB400_CON,
	// # 		      BIT_WITH_WMSK(PMU_PWRDWN_REQ_CORE_B_2GIC_SW) |
	// # 		      BIT_WITH_WMSK(PMU_PWRDWN_REQ_CORE_B_SW) |
	// # 		      BIT_WITH_WMSK(PMU_PWRDWN_REQ_GIC2_CORE_B_SW));
	// #

    // pd::disable_cci(pmu);

    pd::disable_a72_b1(pmu);
    pd::disable_a72_b0(pmu);

    pd::disable_a53_l2(pmu);
    pd::disable_a53_l3(pmu);
    pd::disable_a53_l0(pmu);
    pd::disable_a53_l1(pmu);


    pd::disable_scu_l(pmu);
    pd::disable_scu_b(pmu);

	println!("finished pmu change");

	// clock::setup_clocks();
	// println!("finished clock setup\n");

	// print_clocks();

	// let grf = unsafe { &*rk3399_m0::GRF.get() };
	// let gpio0 = unsafe { &*rk3399_m0::GPIO0.get() };
	// let gpio4 = unsafe { &*rk3399_m0::GPIO4.get() };

	// println!("switching SPDIF IOMUX to GPIO...");
	// grf.grf_gpio4c_iomux.modify(|_, w| unsafe { w.gpio4c5_sel().bits(0) });
	// pmugrf.pmugrf_gpio0b_iomux.modify(|_, w| unsafe { w.gpio0b5_sel().bits(0) });

	// println!("setting as GPIO output");
	// gpio4.gpio_swporta_ddr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << 21)) });
	// // gpio4.gpio_swporta_dr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << 21)) });

	// // turn on green led
	// gpio0.gpio_swporta_ddr.modify(|r, w| unsafe { w.bits(r.bits() | 1 << 13) });
	// gpio0.gpio_swporta_dr.modify(|r, w| unsafe { w.bits(r.bits() | 1 << 13) });

	// FIXME: i don't think this works right, we should see more of a mA drop...
	i2c0.read_from(SYR837_ADDR, Some(0x00), &mut rk808_buf).expect("read VDD_GPU_B VSEL0");
	println!("VDD_GPU_B VSEL0: {:?}", rk808_buf);

	let disabled_syr:[u8; 1] = [rk808_buf[0] & !(1 << 7); 1];

	i2c0.read_from(SYR837_ADDR, Some(0x05), &mut rk808_buf).expect("read VDD_GPU_B VGOOD");
	println!("VDD_GPU_B VGOOD: {:?}", rk808_buf);

	println!("Disabling VDD_CPU_B...");
	i2c0.write_to(SYR837_ADDR, Some(0x00), &disabled_syr).expect("VDD_GPU_B VSEL0");
	i2c0.write_to(SYR837_ADDR, Some(0x01), &disabled_syr).expect("VDD_GPU_B VSEL1");

	i2c0.read_from(SYR837_ADDR, Some(0x00), &mut rk808_buf).expect("re-read VDD_CPU_B VSEL0");
	println!("VDD_CPU_B VSEL0 now: {:?}", rk808_buf);

	i2c0.read_from(SYR837_ADDR, Some(0x05), &mut rk808_buf).expect("re-read VDD_CPU_B VGOOD");
	println!("VDD_CPU_B VGOOD now: {:?}", rk808_buf);

	println!("Disabling BUCK2 (VDD_CPU_L)...");
	// tell RK808 to disable little core
	let mut dcdc_buf:[u8; 1] = [0; 1];
	i2c0.read_from(RK808_ADDR, Some(0x23), &mut dcdc_buf).expect("read DCDC after PMU");

	println!("DCDC_EN_REG: {:?}", dcdc_buf);

	// little core is BUCK2
	dcdc_buf[0] = dcdc_buf[0] & !(1 << 1);
	i2c0.write_to(RK808_ADDR, Some(0x23), &dcdc_buf).expect("clear BUCK2");

	i2c0.read_from(RK808_ADDR, Some(0x23), &mut rk808_buf).expect("read BUCK2 result");
	println!("DCDC_EN_REG now: {:?}", rk808_buf);

	for _ in 0..100 {
		print!(".");
	}

	read_ina219(&i2c4);
}
 
// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 32] = [default_handler; 32];
 
extern "C" fn default_handler() {
	// serial.write(b"Help! Interrupt!\n");
}