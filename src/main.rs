#![feature(used)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(never_type)]
#![feature(unsize)]
#![feature(compiler_builtins_lib)]
#![no_std]

#[macro_use]
mod serial;

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
	i2c0.read_from(RK808_ADDR, Some(0x23), &mut rk808_buf);
	let current_dcdc = rk808_buf[0];

	println!("DCDC_EN_REG: {:?}", rk808_buf);

	i2c0.read_from(RK808_ADDR, Some(0x24), &mut rk808_buf);
	println!("LDO_EN_REG: {:?}", rk808_buf);

	// disable LDO1, LDO2, LDO4, LDO5, LDO7
	rk808_buf[0] = rk808_buf[0] & 
		!(1 << 0) &
		!(1 << 1) &
		!(1 << 3) &
		!(1 << 4) &
		!(1 << 6);

	i2c0.write_to(RK808_ADDR, Some(0x24), &rk808_buf);

	i2c0.read_from(RK808_ADDR, Some(0x24), &mut rk808_buf);
	println!("LDO_EN_REG now: {:?}", rk808_buf);

	// GPU
	i2c0.read_from(SYR838_ADDR, Some(0x00), &mut rk808_buf);
	println!("GPU VSEL0: {:?}", rk808_buf);

	i2c0.read_from(SYR838_ADDR, Some(0x05), &mut rk808_buf);
	println!("GPU VGOOD: {:?}", rk808_buf);

	let disabled_syr:[u8; 1] = [151 & !(1 << 7); 1];
	println!("Changing VSEL0 and VSEL1 to: {:?}", disabled_syr);

	let res = i2c0.write_to(SYR838_ADDR, Some(0x00), &disabled_syr);
	println!("GPU VSEL0 update result: {:?}", res);
	let res = i2c0.write_to(SYR838_ADDR, Some(0x01), &disabled_syr);
	println!("GPU VSEL1 update result: {:?}", res);

	i2c0.read_from(SYR838_ADDR, Some(0x00), &mut rk808_buf);
	println!("GPU VSEL0 now: {:?}", rk808_buf);

	// disable VSW0 and VSW1
	rk808_buf[0] = current_dcdc & !(1 << 5) & !(1 << 6);

	i2c0.write_to(RK808_ADDR, Some(0x23), &mut rk808_buf);

	i2c0.read_from(RK808_ADDR, Some(0x23), &mut rk808_buf);
	println!("DCDC_EN_REG now: {:?}", rk808_buf);

	read_ina219(&i2c4);

	// VDD_CPU_B
	// i2c0.read_from(SYR837_ADDR, 0x00, &mut rk808_buf);
	// println!("VDD_CPU_B VSEL0: {:?}", rk808_buf);

	// println!("Result: {:?}...", res);

	// setup_clocks();
	// println!("finished clock setup!\n");
	// return;

	// okay, so PWRDN_CON seems to be used to turn on/off power domains
	// (after idling the bus via the PMU as well)
	// and PWRDN_ST is used to check the state

	let pmu = unsafe { &*rk3399_m0::PMU.get() };

	println!("gmac on? {:?}", pmu.pmu_pwrdn_st.read().pd_gmac_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_gmac().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_gmac().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_gmac().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("gmac bus state was idle? {:?}", pmu.pmu_bus_idle_st.read().idle_gmac().bit_is_set());
		println!("gmac bus state had idle acknoledge? {:?}", pmu.pmu_bus_idle_ack.read().idle_ack_gmac().bit_is_set());
	}

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_gmac_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_gmac_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("gmac on? {:?}", pmu.pmu_pwrdn_st.read().pd_gmac_pwr_stat().bit_is_clear());

	// turn off a bunch of shit
	pmu.pmu_pwrdn_con.modify(|_, w| unsafe { 
		// USB PHY
		w.pd_tcpd0_pwrdwn_en().bit(true)
		.pd_tcpd1_pwrdwn_en().bit(true)

		.pd_perihp_pwrdwn_en().bit(true)

		.pd_rga_pwrdwn_en().bit(true)		// for LCD stuff
		.pd_iep_pwrdwn_en().bit(true)		// image enhancement
		.pd_vo_pwrdwn_en().bit(true)		// VOP (video out)
		.pd_isp1_pwrdwn_en().bit(true)	// ISP 1
		.pd_hdcp_pwrdwn_en().bit(true)	// HDCP

		.pd_vdu_pwrdwn_en().bit(true)		// video decode unit
		// vcodec has venc and vdec, which we DO need

		.pd_gpu_pwrdwn_en().bit(true)		// GPU

		// gigabit mac
		// if you powerdown GMAC, then reading
		// some of the power related registers causes
		// aborts for some reason
		.pd_gmac_pwrdwn_en().bit(true)

		.pd_usb3_pwrdwn_en().bit(true)	// USB3
		.pd_edp_pwrdwn_en().bit(true)		// DisplayPort
		// .pd_sdioaudio_pwrdwn_en().bit(true)
		.pd_sd_pwrdwn_en().bit(true)

		// scu is snoop control unit i think
		// for cache coherence

		// cci is cache coherence interface

		// in theory we can turn them all off
		// though we might have to reconfigure the buses

		// turn off the other little core
		// FIXME: core0 has _en name from TRM but others don't.. wtf Rockchip
		.pd_a53_l0_pwrdwn_en().bit(true)
		.pd_a53_l1_pwrdwn().bit(true)
		.pd_a53_l2_pwrdwn().bit(true)
		.pd_a53_l3_pwrdwn().bit(true)

		// turn off the big cores
		.pd_a72_b0_pwrdwn_en().bit(true)
		.pd_a72_b1_pwrdwn_en().bit(true)
	});

	println!("finished pmu change");


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

	// println!("disabliong VDD_CPU_B...");
	// let res = i2c0.write_to(SYR837_ADDR, Some(0x00), &disabled_syr);
	// let res = i2c0.write_to(SYR837_ADDR, Some(0x01), &disabled_syr);
	// println!("VDD_CPU_B update result: {:?}", res);

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