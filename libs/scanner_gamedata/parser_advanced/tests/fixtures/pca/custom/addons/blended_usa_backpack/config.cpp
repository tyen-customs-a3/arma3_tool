////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_usa_backpack
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","A3_Weapons_F_Exp","rhs_c_troops","rhsusf_c_troops"};
		units[] = {"pca_assaultpack_mc","pca_assaultpack_mcalp","pca_assaultpack_mcb","pca_assaultpack_mct","pca_assaultpack_mct2","pca_assaultpack_ocp","pca_assaultpack_ocp2","pca_assaultpack_ocp3","pca_assaultpack_m81","pca_assaultpack_aor1","pca_assaultpack_cbr","pca_assaultpack_od","pca_carryall_mc","pca_carryall_mct","pca_carryall_ocp","pca_carryall_m81","pca_carryall_aor1","pca_carryall_cbr","pca_carryall_od","pca_kitbag_mc","pca_kitbag_mcalp","pca_kitbag_mcb","pca_kitbag_mct","pca_kitbag_mct2","pca_kitbag_ocp","pca_kitbag_ocp2","pca_kitbag_ocp3","pca_kitbag_m81","pca_kitbag_aor1","pca_kitbag_cbr","pca_kitbag_od","pca_tacticalpack_mc","pca_tacticalpack_mcalp","pca_tacticalpack_mcb","pca_tacticalpack_mct","pca_tacticalpack_ocp"};
		weapons[] = {};
	};
};
class CfgVehicles
{
	class B_AssaultPack_mcamo;
	class B_Carryall_mcamo;
	class B_Kitbag_mcamo;
	class B_TacticalPack_mcamo;
	class pca_assaultpack_mc: B_AssaultPack_mcamo
	{
		scope = 2;
		displayName = "[US] Assault Pack (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_mc_co.paa"};
	};
	class pca_assaultpack_mcalp: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_mca_co.paa"};
	};
	class pca_assaultpack_mcb: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_mcb_co.paa"};
	};
	class pca_assaultpack_mct: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_mct_co.paa"};
	};
	class pca_assaultpack_mct2: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (MC-Tropic2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_mct2_co.paa"};
	};
	class pca_assaultpack_ocp: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_ocp_co.paa"};
	};
	class pca_assaultpack_ocp2: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (OCP2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_ocp2_co.paa"};
	};
	class pca_assaultpack_ocp3: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (OCP3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_ocp3_co.paa"};
	};
	class pca_assaultpack_m81: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (M81)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_m81_co.paa"};
	};
	class pca_assaultpack_aor1: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (AOR-1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_aor1_co.paa"};
	};
	class pca_assaultpack_cbr: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (Coyote Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_cbr_co.paa"};
	};
	class pca_assaultpack_od: pca_assaultpack_mc
	{
		displayName = "[US] Assault Pack (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\assault_od_co.paa"};
	};
	class pca_carryall_mc: B_Carryall_mcamo
	{
		scope = 2;
		displayName = "[US] Carryall Backpack (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_mc_co.paa"};
	};
	class pca_carryall_mct: pca_carryall_mc
	{
		displayName = "[US] Carryall Backpack (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_mct_co.paa"};
	};
	class pca_carryall_ocp: pca_carryall_mc
	{
		displayName = "[US] Carryall Backpack (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_ocp_co.paa"};
	};
	class pca_carryall_m81: pca_carryall_mc
	{
		displayName = "[US] Carryall Backpack (M81)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_m81_co.paa"};
	};
	class pca_carryall_aor1: pca_carryall_mc
	{
		displayName = "[US] Carryall Backpack (AOR-1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_aor1_co.paa"};
	};
	class pca_carryall_cbr: pca_carryall_mc
	{
		displayName = "[US] Carryall Backpack (Coyote Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_cbr_co.paa"};
	};
	class pca_carryall_od: pca_carryall_mc
	{
		displayName = "[US] Carryall Backpack (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\carryall_od_co.paa"};
	};
	class pca_kitbag_mc: B_Kitbag_mcamo
	{
		scope = 2;
		displayName = "[US] Kitbag (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_mc_co.paa"};
	};
	class pca_kitbag_mcalp: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_mca_co.paa"};
	};
	class pca_kitbag_mcb: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_mcb_co.paa"};
	};
	class pca_kitbag_mct: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_mct_co.paa"};
	};
	class pca_kitbag_mct2: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (MC-Tropic2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_mct2_co.paa"};
	};
	class pca_kitbag_ocp: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_ocp_co.paa"};
	};
	class pca_kitbag_ocp2: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (OCP2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_ocp2_co.paa"};
	};
	class pca_kitbag_ocp3: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (OCP3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_ocp3_co.paa"};
	};
	class pca_kitbag_m81: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (M81)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_m81_co.paa"};
	};
	class pca_kitbag_aor1: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (AOR-1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_aor1_co.paa"};
	};
	class pca_kitbag_cbr: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (Coyote Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_cbr_co.paa"};
	};
	class pca_kitbag_od: pca_kitbag_mc
	{
		displayName = "[US] Kitbag (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\kitbag_od_co.paa"};
	};
	class pca_tacticalpack_mc: B_TacticalPack_mcamo
	{
		scope = 2;
		displayName = "[US] Tactical Backpack (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\tactical_mc_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_backpack\data\rv\tactical.rvmat"};
	};
	class pca_tacticalpack_mcalp: pca_tacticalpack_mc
	{
		displayName = "[US] Tactical Backpack (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\tactical_mca_co.paa"};
	};
	class pca_tacticalpack_mcb: pca_tacticalpack_mc
	{
		displayName = "[US] Tactical Backpack (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\tactical_mcb_co.paa"};
	};
	class pca_tacticalpack_mct: pca_tacticalpack_mc
	{
		displayName = "[US] Tactical Backpack (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\tactical_mct_co.paa"};
	};
	class pca_tacticalpack_ocp: pca_tacticalpack_mc
	{
		displayName = "[US] Tactical Backpack (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\tactical_ocp_co.paa"};
	};
	class rhsusf_assault_eagleaiii_coy;
	class pca_eagle_a3_mc: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_mc_co.paa"};
	};
	class pca_eagle_a3_ocp: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_ocp_co.paa"};
	};
	class pca_eagle_a3_oefcp: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (OEF-CP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_oefcp_co.paa"};
	};
	class pca_eagle_a3_m81: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (M81)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_m81_co.paa"};
	};
	class pca_eagle_a3_mpw: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Marpat-Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_mpw_co.paa"};
	};
	class pca_eagle_a3_atacsfg: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (ATACS-FG)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_atacsfg_co.paa"};
	};
	class pca_eagle_a3_blk: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_blk_co.paa"};
	};
	class pca_eagle_a3_cbr: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Coyote Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_cbr_co.paa"};
	};
	class pca_eagle_a3_khk: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_khk_co.paa"};
	};
	class pca_eagle_a3_od: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_od_co.paa"};
	};
	class pca_eagle_a3_oli: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_oli_co.paa"};
	};
	class pca_eagle_a3_rgr: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Ranger Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_rgr_co.paa"};
	};
	class pca_eagle_a3_tan: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_tan_co.paa"};
	};
	class pca_eagle_a3_wht: rhsusf_assault_eagleaiii_coy
	{
		displayName = "[US] Eagle A-III (White)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_backpack\data\tex\eagle_a3_wht_co.paa"};
	};
};
