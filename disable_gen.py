from jinja2 import Template

# pmu does:
# clk_gate_con_disable
#	sets PMUCRU_CLKGATE_CON[0..3) -> zero
#   sets CRU_CLKGATE_CON[0..23) -> zero
#
# so, this disables gating (i.e. enables all the clocks) before
# doing the PD switch; presumably so we can actually switch the power
# domain without locking up if something was already disabled
#
# it THEN sets "off" for a bunch of power domains that aren't CPU
# and THEN restores the previous gating setup

# THEN we:
# 	set_hw_idle(BIT(PMU_CLR_CENTER1) |
# 		    BIT(PMU_CLR_ALIVE) |
# 		    BIT(PMU_CLR_MSCH0) |
# 		    BIT(PMU_CLR_MSCH1) |
# 		    BIT(PMU_CLR_CCIM0) |
# 		    BIT(PMU_CLR_CCIM1) |
# 		    BIT(PMU_CLR_CENTER) |
# 		    BIT(PMU_CLR_GIC));
#
# THEN also, maybe need:
# mmio_write_32(GRF_BASE + GRF_SOC_CON4, CCI_FORCE_WAKEUP);
# mmio_write_32(PMU_BASE + PMU_CCI500_CON,
# 	      BIT_WITH_WMSK(PMU_CLR_PREQ_CCI500_HW) |
# 	      BIT_WITH_WMSK(PMU_CLR_QREQ_CCI500_HW) |
# 	      BIT_WITH_WMSK(PMU_QGATING_CCI500_CFG));

# mmio_write_32(PMU_BASE + PMU_ADB400_CON,
# 	      BIT_WITH_WMSK(PMU_CLR_CORE_L_HW) |
# 	      BIT_WITH_WMSK(PMU_CLR_CORE_L_2GIC_HW) |
# 	      BIT_WITH_WMSK(PMU_CLR_GIC2_CORE_L_HW));
#
#
# THEN
# mmio_write_32(PMU_BASE + PMU_ADB400_CON,
# 		      BIT_WITH_WMSK(PMU_PWRDWN_REQ_CORE_B_2GIC_SW) |
# 		      BIT_WITH_WMSK(PMU_PWRDWN_REQ_CORE_B_SW) |
# 		      BIT_WITH_WMSK(PMU_PWRDWN_REQ_GIC2_CORE_B_SW));
#
# THEN we can shutdown SCU_B
# mmio_setbits_32(PMU_BASE + PMU_PWRDN_CON, BIT(PMU_SCU_B_PWRDWN_EN));

POWER_DOMAINS = {
	'a53_l0': {},
	'a53_l1': {'pwrdwn_con': 'a53_l1_pwrdwn'},
	'a53_l2': {'pwrdwn_con': 'a53_l2_pwrdwn'},
	'a53_l3': {'pwrdwn_con': 'a53_l3_pwrdwn'},
	'a72_b0': {},
	'a72_b1': {},
	'scu_l': {}, # see above
	'scu_b': {}, # see above
	'tcpd0' : {},
	'tcpd1' : {},
	'cci' : { 'buses': [ 'ccim0', 'ccim1' ] },
	'perihp' : { 'buses': [ 'perihp' ] },
	'gpu' : { 'buses': [ 'gpu' ] },
	'vdu' : { 'buses': [ 'vdu' ] },
	'rga' : { 'buses': [ 'rga' ] },
	'iep' : { 'buses': [ 'iep' ] },
	'vo' : { 'buses': [ 'vopl', 'vopb' ] },
	# 'vio': { 'buses': [ 'vio' ] }
	#vcodec
	'isp0' : { 'buses': [ 'isp0' ] },
	'isp1' : { 'buses': [ 'isp1' ] },
	'hdcp' : { 'buses': [ 'hdcp' ] },
	'gmac' : { 'buses': [ 'gmac' ] },
	'emmc' : { 'buses': [ 'emmc' ] },
	'usb3' : { 'buses': [ 'usb3' ] },
	'edp' : { 'buses': [ 'edp' ] },
	'gic' : { 'buses': [ 'gic' ] },
	'sd' : { 'buses': [ 'sd' ] },

	# this is on perihp
	'sdioaudio' : { 'buses': [ 'sdioaudio' ] }
}

# SCU_B only off after all A72 are off

# disable pmu_core_pm_conX with CORES_PM_DISABLE
# for CPU, set_cpus_pwr_cdomain_cfg_info
# then pmu_power_domain_ctr(off)

tmpl = Template(open('disable_tmpl.rs', 'r').read())

prelude = """
extern crate rk3399_m0;

const MAX_WAIT_COUNT:u32 = 10000;

"""

with open('src/pd.rs', 'w') as f:
	f.write(prelude)

	for per in POWER_DOMAINS:
		buses = []
		if 'buses' in POWER_DOMAINS[per]:
			buses = POWER_DOMAINS[per]['buses']

		pwrdwn_con = per + '_pwrdwn_en'
		if 'pwrdwn_con' in POWER_DOMAINS[per]:
			pwrdwn_con = POWER_DOMAINS[per]['pwrdwn_con']

		f.write(tmpl.render({
			'e' : per,
			'pwrdwn_con': pwrdwn_con,
			'buses': buses
		}))

		f.write('\n\n')
		print '\tpd::disable_%s(pmu);' % per