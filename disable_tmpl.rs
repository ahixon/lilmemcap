pub fn disable_{{ e }}(pmu: &rk3399_m0::PMU) {
	println!("{:} = on? {:?}", stringify!({{ e }}), pmu.pmu_pwrdn_st.read().pd_{{ e }}_pwr_stat().bit_is_clear());

	// if already in state we want to transition to, we're done
	// otherwise...

	// if we want to turn it on, we call `pmu_power_domain_ctr`
	// which enables the power domain

	// now we handle the bus via `pmu_bus_idle_req`
	// if we're turning on, we request the bus go active
	// if we're turning off, we request the bus go idle:
	{% for bus in buses %}
	pmu.pmu_bus_idle_req.modify(|_, w| w.idle_req_{{ bus }}().bit(true));

	let mut bus_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let bus_state = pmu.pmu_bus_idle_st.read().idle_{{ bus }}().bit_is_set();
		let bus_ack = pmu.pmu_bus_idle_ack.read().idle_ack_{{ bus }}().bit_is_set();

		// while ((bus_state != bus_req || bus_ack != bus_req)
		// and bus_req = state ? bus_id : 0  (ie target for bit is 1 if turn off, or bit unset if turning on)
		if bus_ack || bus_state {
			bus_timeout = false;
			break;
		}
	}

	if bus_timeout {
		println!("\terror: had timeout while idling bus");
		println!("\t{:} bus state was idle? {:?}", stringify!({{ bus }}), pmu.pmu_bus_idle_st.read().idle_{{ bus }}().bit_is_set());
		println!("\t{:} bus state had idle acknoledge? {:?}", stringify!({{ bus }}), pmu.pmu_bus_idle_ack.read().idle_ack_{{ bus }}().bit_is_set());
		return;
	}
	{% endfor %}

	// if we're powering on, we're done! it has power and the bus is back
	// if we're powering off, we finally need to disable the power domain:
	pmu.pmu_pwrdn_con.modify(|_, w| w.pd_{{ pwrdwn_con }}().bit(true));

	unsafe { asm!("dsb sy"); }

	// now, keep checking to see if it actually turned off
	let mut pd_timeout = true;
	for _ in 1..MAX_WAIT_COUNT {
		let powered_off = pmu.pmu_pwrdn_st.read().pd_{{ e }}_pwr_stat().bit_is_set();
		if powered_off {
			pd_timeout = false;
			break;
		}
	}

	if pd_timeout {
		println!("\terror: had timeout while disabling power domain");
		println!("\tpmu_pwrdn_st: {:?}", pmu.pmu_pwrdn_st.read().bits());
		return;
	}

	println!("{:} = on? {:?}", stringify!({{ e }}), pmu.pmu_pwrdn_st.read().pd_{{ e }}_pwr_stat().bit_is_clear());
}