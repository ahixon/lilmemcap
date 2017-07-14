
extern crate rk3399_m0;

const MAX_WAIT_COUNT:u32 = 10000;

pub fn disable_scu_l(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(scu_l), pmu.pmu_pwrdn_st.read().pd_scu_l_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_scu_l_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_scu_l_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(scu_l), pmu.pmu_pwrdn_st.read().pd_scu_l_pwr_stat().bit_is_clear());
}

pub fn disable_isp0(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(isp0), pmu.pmu_pwrdn_st.read().pd_isp0_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_isp0().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_isp0().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_isp0().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(isp0), pmu.pmu_bus_idle_st.read().idle_isp0().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(isp0), pmu.pmu_bus_idle_ack.read().idle_ack_isp0().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_isp0_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_isp0_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(isp0), pmu.pmu_pwrdn_st.read().pd_isp0_pwr_stat().bit_is_clear());
}

pub fn disable_scu_b(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(scu_b), pmu.pmu_pwrdn_st.read().pd_scu_b_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_scu_b_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_scu_b_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(scu_b), pmu.pmu_pwrdn_st.read().pd_scu_b_pwr_stat().bit_is_clear());
}

pub fn disable_vo(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(vo), pmu.pmu_pwrdn_st.read().pd_vo_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_vopl().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_vopl().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_vopl().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(vopl), pmu.pmu_bus_idle_st.read().idle_vopl().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(vopl), pmu.pmu_bus_idle_ack.read().idle_ack_vopl().bit_is_set());
	}
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_vopb().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_vopb().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_vopb().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(vopb), pmu.pmu_bus_idle_st.read().idle_vopb().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(vopb), pmu.pmu_bus_idle_ack.read().idle_ack_vopb().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_vo_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_vo_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(vo), pmu.pmu_pwrdn_st.read().pd_vo_pwr_stat().bit_is_clear());
}

pub fn disable_edp(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(edp), pmu.pmu_pwrdn_st.read().pd_edp_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_edp().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_edp().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_edp().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(edp), pmu.pmu_bus_idle_st.read().idle_edp().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(edp), pmu.pmu_bus_idle_ack.read().idle_ack_edp().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_edp_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_edp_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(edp), pmu.pmu_pwrdn_st.read().pd_edp_pwr_stat().bit_is_clear());
}

pub fn disable_iep(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(iep), pmu.pmu_pwrdn_st.read().pd_iep_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_iep().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_iep().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_iep().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(iep), pmu.pmu_bus_idle_st.read().idle_iep().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(iep), pmu.pmu_bus_idle_ack.read().idle_ack_iep().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_iep_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_iep_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(iep), pmu.pmu_pwrdn_st.read().pd_iep_pwr_stat().bit_is_clear());
}

pub fn disable_rga(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(rga), pmu.pmu_pwrdn_st.read().pd_rga_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_rga().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_rga().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_rga().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(rga), pmu.pmu_bus_idle_st.read().idle_rga().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(rga), pmu.pmu_bus_idle_ack.read().idle_ack_rga().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_rga_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_rga_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(rga), pmu.pmu_pwrdn_st.read().pd_rga_pwr_stat().bit_is_clear());
}

pub fn disable_gic(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(gic), pmu.pmu_pwrdn_st.read().pd_gic_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_gic().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_gic().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_gic().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(gic), pmu.pmu_bus_idle_st.read().idle_gic().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(gic), pmu.pmu_bus_idle_ack.read().idle_ack_gic().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_gic_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_gic_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(gic), pmu.pmu_pwrdn_st.read().pd_gic_pwr_stat().bit_is_clear());
}

pub fn disable_gpu(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(gpu), pmu.pmu_pwrdn_st.read().pd_gpu_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_gpu().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_gpu().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_gpu().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(gpu), pmu.pmu_bus_idle_st.read().idle_gpu().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(gpu), pmu.pmu_bus_idle_ack.read().idle_ack_gpu().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_gpu_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_gpu_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(gpu), pmu.pmu_pwrdn_st.read().pd_gpu_pwr_stat().bit_is_clear());
}

pub fn disable_a72_b1(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(a72_b1), pmu.pmu_pwrdn_st.read().pd_a72_b1_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_a72_b1_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_a72_b1_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(a72_b1), pmu.pmu_pwrdn_st.read().pd_a72_b1_pwr_stat().bit_is_clear());
}

pub fn disable_a72_b0(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(a72_b0), pmu.pmu_pwrdn_st.read().pd_a72_b0_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_a72_b0_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_a72_b0_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(a72_b0), pmu.pmu_pwrdn_st.read().pd_a72_b0_pwr_stat().bit_is_clear());
}

pub fn disable_usb3(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(usb3), pmu.pmu_pwrdn_st.read().pd_usb3_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_usb3().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_usb3().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_usb3().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(usb3), pmu.pmu_bus_idle_st.read().idle_usb3().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(usb3), pmu.pmu_bus_idle_ack.read().idle_ack_usb3().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_usb3_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_usb3_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(usb3), pmu.pmu_pwrdn_st.read().pd_usb3_pwr_stat().bit_is_clear());
}

pub fn disable_isp1(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(isp1), pmu.pmu_pwrdn_st.read().pd_isp1_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_isp1().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_isp1().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_isp1().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(isp1), pmu.pmu_bus_idle_st.read().idle_isp1().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(isp1), pmu.pmu_bus_idle_ack.read().idle_ack_isp1().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_isp1_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_isp1_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(isp1), pmu.pmu_pwrdn_st.read().pd_isp1_pwr_stat().bit_is_clear());
}

pub fn disable_perihp(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(perihp), pmu.pmu_pwrdn_st.read().pd_perihp_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_perihp().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_perihp().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_perihp().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(perihp), pmu.pmu_bus_idle_st.read().idle_perihp().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(perihp), pmu.pmu_bus_idle_ack.read().idle_ack_perihp().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_perihp_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_perihp_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(perihp), pmu.pmu_pwrdn_st.read().pd_perihp_pwr_stat().bit_is_clear());
}

pub fn disable_hdcp(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(hdcp), pmu.pmu_pwrdn_st.read().pd_hdcp_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_hdcp().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_hdcp().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_hdcp().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(hdcp), pmu.pmu_bus_idle_st.read().idle_hdcp().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(hdcp), pmu.pmu_bus_idle_ack.read().idle_ack_hdcp().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_hdcp_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_hdcp_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(hdcp), pmu.pmu_pwrdn_st.read().pd_hdcp_pwr_stat().bit_is_clear());
}

pub fn disable_gmac(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(gmac), pmu.pmu_pwrdn_st.read().pd_gmac_pwr_stat().bit_is_clear());

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
		println!("{:} bus state was idle? {:?}", stringify!(gmac), pmu.pmu_bus_idle_st.read().idle_gmac().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(gmac), pmu.pmu_bus_idle_ack.read().idle_ack_gmac().bit_is_set());
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

	println!("{:} = on? {:?}", stringify!(gmac), pmu.pmu_pwrdn_st.read().pd_gmac_pwr_stat().bit_is_clear());
}

pub fn disable_vdu(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(vdu), pmu.pmu_pwrdn_st.read().pd_vdu_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_vdu().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_vdu().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_vdu().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(vdu), pmu.pmu_bus_idle_st.read().idle_vdu().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(vdu), pmu.pmu_bus_idle_ack.read().idle_ack_vdu().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_vdu_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_vdu_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(vdu), pmu.pmu_pwrdn_st.read().pd_vdu_pwr_stat().bit_is_clear());
}

pub fn disable_emmc(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(emmc), pmu.pmu_pwrdn_st.read().pd_emmc_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_emmc().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_emmc().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_emmc().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(emmc), pmu.pmu_bus_idle_st.read().idle_emmc().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(emmc), pmu.pmu_bus_idle_ack.read().idle_ack_emmc().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_emmc_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_emmc_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(emmc), pmu.pmu_pwrdn_st.read().pd_emmc_pwr_stat().bit_is_clear());
}

pub fn disable_a53_l2(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(a53_l2), pmu.pmu_pwrdn_st.read().pd_a53_l2_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_a53_l2_pwrdwn().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_a53_l2_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(a53_l2), pmu.pmu_pwrdn_st.read().pd_a53_l2_pwr_stat().bit_is_clear());
}

pub fn disable_a53_l3(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(a53_l3), pmu.pmu_pwrdn_st.read().pd_a53_l3_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_a53_l3_pwrdwn().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_a53_l3_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(a53_l3), pmu.pmu_pwrdn_st.read().pd_a53_l3_pwr_stat().bit_is_clear());
}

pub fn disable_a53_l0(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(a53_l0), pmu.pmu_pwrdn_st.read().pd_a53_l0_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_a53_l0_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_a53_l0_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(a53_l0), pmu.pmu_pwrdn_st.read().pd_a53_l0_pwr_stat().bit_is_clear());
}

pub fn disable_a53_l1(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(a53_l1), pmu.pmu_pwrdn_st.read().pd_a53_l1_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_a53_l1_pwrdwn().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_a53_l1_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(a53_l1), pmu.pmu_pwrdn_st.read().pd_a53_l1_pwr_stat().bit_is_clear());
}

pub fn disable_sdioaudio(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(sdioaudio), pmu.pmu_pwrdn_st.read().pd_sdioaudio_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_sdioaudio().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_sdioaudio().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_sdioaudio().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(sdioaudio), pmu.pmu_bus_idle_st.read().idle_sdioaudio().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(sdioaudio), pmu.pmu_bus_idle_ack.read().idle_ack_sdioaudio().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_sdioaudio_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_sdioaudio_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(sdioaudio), pmu.pmu_pwrdn_st.read().pd_sdioaudio_pwr_stat().bit_is_clear());
}

pub fn disable_cci(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(cci), pmu.pmu_pwrdn_st.read().pd_cci_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_ccim0().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_ccim0().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_ccim0().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(ccim0), pmu.pmu_bus_idle_st.read().idle_ccim0().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(ccim0), pmu.pmu_bus_idle_ack.read().idle_ack_ccim0().bit_is_set());
	}
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_ccim1().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_ccim1().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_ccim1().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(ccim1), pmu.pmu_bus_idle_st.read().idle_ccim1().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(ccim1), pmu.pmu_bus_idle_ack.read().idle_ack_ccim1().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_cci_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_cci_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(cci), pmu.pmu_pwrdn_st.read().pd_cci_pwr_stat().bit_is_clear());
}

pub fn disable_tcpd0(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(tcpd0), pmu.pmu_pwrdn_st.read().pd_tcpd0_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_tcpd0_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_tcpd0_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(tcpd0), pmu.pmu_pwrdn_st.read().pd_tcpd0_pwr_stat().bit_is_clear());
}

pub fn disable_tcpd1(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(tcpd1), pmu.pmu_pwrdn_st.read().pd_tcpd1_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_tcpd1_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_tcpd1_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(tcpd1), pmu.pmu_pwrdn_st.read().pd_tcpd1_pwr_stat().bit_is_clear());
}

pub fn disable_sd(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!(sd), pmu.pmu_pwrdn_st.read().pd_sd_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_sd().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_sd().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_sd().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("had timeout while idling bus");
		println!("{:} bus state was idle? {:?}", stringify!(sd), pmu.pmu_bus_idle_st.read().idle_sd().bit_is_set());
		println!("{:} bus state had idle acknoledge? {:?}", stringify!(sd), pmu.pmu_bus_idle_ack.read().idle_ack_sd().bit_is_set());
	}
	

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_sd_pwrdwn_en().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_sd_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("had timeout while disabling power domain");
		println!("pmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
	}

	println!("{:} = on? {:?}", stringify!(sd), pmu.pmu_pwrdn_st.read().pd_sd_pwr_stat().bit_is_clear());
}

