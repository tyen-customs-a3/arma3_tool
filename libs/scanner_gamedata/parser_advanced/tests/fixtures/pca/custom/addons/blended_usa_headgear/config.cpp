////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_usa_headgear
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","A3_Weapons_F_Exp","rhs_c_troops","rhsusf_c_troops"};
		units[] = {};
		weapons[] = {};
	};
};
class CfgWeapons
{
	class rhsusf_ach_helmet_ocp;
	class pca_ach_oefcp: rhsusf_ach_helmet_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACH (OEF-CP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\ach_oefcp_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\ach_acc_co.paa"};
	};
	class rhsusf_ach_helmet_camo_ocp;
	class pca_ach_camo_oefcp: rhsusf_ach_helmet_camo_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACH (OEF-CP/Camo)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\ach_oefcp_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\camo_net_co.paa"};
	};
	class rhsusf_ach_helmet_ESS_ocp;
	class pca_ach_ess_oefcp: rhsusf_ach_helmet_ESS_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACH (OEF-CP/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\ach_oefcp_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\ach_acc_co.paa"};
	};
	class rhsusf_ach_helmet_headset_ocp;
	class pca_ach_headset_oefcp: rhsusf_ach_helmet_headset_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACH (OEF-CP/Headset)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\ach_oefcp_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\ach_acc_co.paa"};
	};
	class rhsusf_ach_helmet_headset_ess_ocp;
	class pca_ach_headset_ess_oefcp: rhsusf_ach_helmet_headset_ess_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACH (OEF-CP/Headset/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\ach_oefcp_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\ach_acc_co.paa"};
	};
	class rhsusf_ach_helmet_ocp_norotos;
	class pca_ach_norotos_oefcp: rhsusf_ach_helmet_ocp_norotos
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACH (OEF-CP/Norotos)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\ach_oefcp_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\ach_acc_co.paa"};
	};
	class rhs_booniehat2_marpatd;
	class pca_booniehat_mc: rhs_booniehat2_marpatd
	{
		displayName = "[US] Boonie Hat (MC)";
		author = "Red Hammer Studios";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\booniehat_mc_co.paa"};
	};
	class rhs_Booniehat_ocp;
	class pca_booniehat_oefcp: rhs_Booniehat_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] Boonie Hat (OEF-CP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\booniehat_oefcp_co.paa"};
	};
	class rhsusf_patrolcap_ucp;
	class pca_patrolcap_oefcp: rhsusf_patrolcap_ucp
	{
		author = "Red Hammer Studios";
		displayName = "[US] Patrol Cap (OEF-CP)";
		hiddenSelections[] = {"camo1"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\patrolcap_oefcp_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_headgear\data\rv\patrolcap.rvmat"};
	};
	class rhsusf_opscore_ut;
	class rhsusf_opscore_ut_pelt;
	class rhsusf_opscore_ut_pelt_cam;
	class rhsusf_opscore_ut_pelt_nsw;
	class pca_opscore_mc: rhsusf_opscore_ut
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc_co.paa"};
	};
	class pca_opscore_ct_mc: rhsusf_opscore_ut_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa",""};
	};
	class pca_opscore_ct_cm_mc: rhsusf_opscore_ut_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","",""};
	};
	class pca_opscore_ct_cw_mc: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_ct_cb_mc: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_mc2: rhsusf_opscore_ut
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc2_co.paa"};
	};
	class pca_opscore_ct_mc2: rhsusf_opscore_ut_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC2/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc2_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa",""};
	};
	class pca_opscore_ct_cm_mc2: rhsusf_opscore_ut_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC2/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc2_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","",""};
	};
	class pca_opscore_ct_cw_mc2: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC2/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc2_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_ct_cb_mc2: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (MC2/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_mc2_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_blk: rhsusf_opscore_ut
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa"};
	};
	class pca_opscore_ct_blk: rhsusf_opscore_ut_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Black/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa",""};
	};
	class pca_opscore_ct_cm_blk: rhsusf_opscore_ut_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Black/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","",""};
	};
	class pca_opscore_ct_cw_blk: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Black/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_ct_cb_blk: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Black/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_rgr: rhsusf_opscore_ut
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Ranger Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa"};
	};
	class pca_opscore_ct_rgr: rhsusf_opscore_ut_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Ranger Green/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa",""};
	};
	class pca_opscore_ct_cm_rgr: rhsusf_opscore_ut_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Ranger Green/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","",""};
	};
	class pca_opscore_ct_cw_rgr: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Ranger Green/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_ct_cb_rgr: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Ranger Green/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_spray: rhsusf_opscore_ut
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Spray)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_spray_co.paa"};
	};
	class pca_opscore_ct_spray: rhsusf_opscore_ut_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Spray/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_spray_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa",""};
	};
	class pca_opscore_ct_cm_spray: rhsusf_opscore_ut_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Spray/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_spray_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","",""};
	};
	class pca_opscore_ct_cw_spray: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Spray/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_spray_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_ct_cb_spray: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Spray/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_spray_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_tan: rhsusf_opscore_ut
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa"};
	};
	class pca_opscore_ct_tan: rhsusf_opscore_ut_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Tan/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa",""};
	};
	class pca_opscore_ct_cm_tan: rhsusf_opscore_ut_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Tan/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","",""};
	};
	class pca_opscore_ct_cw_tan: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Tan/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_ct_cb_tan: rhsusf_opscore_ut_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP (Tan/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class rhsusf_opscore_mc_cover;
	class rhsusf_opscore_mc_cover_pelt;
	class rhsusf_opscore_mc_cover_pelt_cam;
	class rhsusf_opscore_mc_cover_pelt_nsw;
	class pca_opscore_cover_mc: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_mc_co.paa",""};
	};
	class pca_opscore_cover_ct_mc: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_mc_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mc: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mc_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mc: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","usp_gear_head\data\tex\fs_cover_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mc: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mc2: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc_co.paa",""};
	};
	class pca_opscore_cover_ct_mc2: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC2/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mc2: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC2/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mc2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC2/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mc2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC2/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mc3: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc2_co.paa",""};
	};
	class pca_opscore_cover_ct_mc3: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC3/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc2_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mc3: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC3/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc2_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mc3: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC3/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc2_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mc3: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC3/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mc2_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mca: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","usp_gear_head\data\tex\fs_cover_mcd_co.paa",""};
	};
	class pca_opscore_cover_ct_mca: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","usp_gear_head\data\tex\fs_cover_mcd_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mca: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mcd_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mca: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","usp_gear_head\data\tex\fs_cover_mcd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mca: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mcd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mca2: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcd_co.paa",""};
	};
	class pca_opscore_cover_ct_mca2: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid2/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcd_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mca2: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid2/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcd_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mca2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid2/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mca2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Arid2/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mcalp: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","","","","usp_gear_head\data\tex\fs_cover_mca_co.paa",""};
	};
	class pca_opscore_cover_ct_mcalp: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Alpine/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","","","usp_gear_head\data\tex\fs_cover_mca_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mcalp: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Alpine/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mca_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mcalp: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Alpine/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_gry_co.paa","","usp_gear_head\data\tex\fs_cover_mca_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mcalp: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Alpine/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_gry_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mca_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mcb: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcb_co.paa",""};
	};
	class pca_opscore_cover_ct_mcb: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Black/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcb_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mcb: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Black/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcb_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mcb: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Black/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcb_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mcb: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Black/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcb_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mct: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_mct_co.paa",""};
	};
	class pca_opscore_cover_ct_mct: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_mct_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mct: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mct_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mct: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","usp_gear_head\data\tex\fs_cover_mct_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mct: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_mct_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mct2: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mct_co.paa",""};
	};
	class pca_opscore_cover_ct_mct2: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic2/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mct_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mct2: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic2/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mct_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mct2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic2/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mct_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mct2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-Tropic2/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mct_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ocp: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_ocp_co.paa",""};
	};
	class pca_opscore_cover_ct_ocp: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (OCP/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_ocp_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_ocp: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (OCP/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_ocp_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_ocp: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (OCP/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_ocp_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_ocp: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (OCP/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_ocp_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mcaus: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-AUS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcaus_co.paa",""};
	};
	class pca_opscore_cover_ct_mcaus: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-AUS/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcaus_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mcaus: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-AUS/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcaus_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mcaus: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-AUS/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcaus_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mcaus: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (MC-AUS/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_mc_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mcaus_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_aor1: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","usp_gear_head\data\tex\fs_cover_aor1_co.paa",""};
	};
	class pca_opscore_cover_ct_aor1: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","usp_gear_head\data\tex\fs_cover_aor1_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_aor1: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_aor1_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_aor1: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","usp_gear_head\data\tex\fs_cover_aor1_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_aor1: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_aor1_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_aor1_alt: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1 Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor1_co.paa",""};
	};
	class pca_opscore_cover_ct_aor1_alt: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1 Alt/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor1_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_aor1_alt: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1 Alt/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor1_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_aor1_alt: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1 Alt/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor1_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_aor1_alt: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-1 Alt/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor1_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_aor2: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_aor2_co.paa",""};
	};
	class pca_opscore_cover_ct_aor2: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_aor2_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_aor2: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_aor2_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_aor2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","usp_gear_head\data\tex\fs_cover_aor2_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_aor2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_aor2_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_aor2_alt: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2 Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor2_co.paa",""};
	};
	class pca_opscore_cover_ct_aor2_alt: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2 Alt/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor2_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_aor2_alt: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2 Alt/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor2_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_aor2_alt: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2 Alt/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor2_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_aor2_alt: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AOR-2 Alt/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aor2_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_aaf: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AAF-Digital)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aaf_co.paa",""};
	};
	class pca_opscore_cover_ct_aaf: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AAF-Digital/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aaf_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_aaf: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AAF-Digital/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aaf_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_aaf: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AAF-Digital/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aaf_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_aaf: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (AAF-Digital/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_aaf_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_cpw: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Cadpat-WD)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_cpw_co.paa",""};
	};
	class pca_opscore_cover_ct_cpw: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Cadpat-WD/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_cpw_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_cpw: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Cadpat-WD/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_cpw_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_cpw: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Cadpat-WD/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_cpw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_cpw: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Cadpat-WD/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_cpw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_dcu: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (DCU)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","usp_gear_head\data\tex\fs_cover_dcu_co.paa",""};
	};
	class pca_opscore_cover_ct_dcu: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (DCU/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","usp_gear_head\data\tex\fs_cover_dcu_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_dcu: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (DCU/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_dcu_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_dcu: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (DCU/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","usp_gear_head\data\tex\fs_cover_dcu_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_dcu: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (DCU/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_dcu_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_geo: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Geometric)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_geo_co.paa",""};
	};
	class pca_opscore_cover_ct_geo: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Geometric/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_geo_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_geo: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Geometric/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_geo_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_geo: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Geometric/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_geo_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_geo: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Geometric/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_geo_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_flecktarn: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Flecktarn)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_flecktarn_co.paa",""};
	};
	class pca_opscore_cover_ct_flecktarn: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Flecktarn/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_flecktarn_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_flecktarn: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Flecktarn/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_flecktarn_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_flecktarn: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Flecktarn/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","usp_gear_head\data\tex\fs_cover_flecktarn_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_flecktarn: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Flecktarn/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_flecktarn_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ktneptune: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Neptune)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","","","","usp_gear_head\data\tex\fs_cover_ktneptune_co.paa",""};
	};
	class pca_opscore_cover_ct_ktneptune: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Neptune/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","","usp_gear_head\data\tex\fs_cover_ktneptune_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_ktneptune: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Neptune/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_ktneptune_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_ktneptune: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Neptune/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","","usp_gear_head\data\tex\fs_cover_ktneptune_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_ktneptune: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Neptune/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_ktneptune_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_kttyphon: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Typhon)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","","","","usp_gear_head\data\tex\fs_cover_kttyphon_co.paa",""};
	};
	class pca_opscore_cover_ct_kttyphon: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Typhon/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","","usp_gear_head\data\tex\fs_cover_kttyphon_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_kttyphon: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Typhon/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_kttyphon_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_kttyphon: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Typhon/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","","usp_gear_head\data\tex\fs_cover_kttyphon_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_kttyphon: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Typhon/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_kttyphon_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ktyeti: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Yeti)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","","","","usp_gear_head\data\tex\fs_cover_ktyeti_co.paa",""};
	};
	class pca_opscore_cover_ct_ktyeti: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Yeti/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","","","usp_gear_head\data\tex\fs_cover_ktyeti_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_ktyeti: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Yeti/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_ktyeti_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_ktyeti: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Yeti/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_gry_co.paa","","usp_gear_head\data\tex\fs_cover_ktyeti_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_ktyeti: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Kryptek-Yeti/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_wht_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_gry_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_gry_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_ktyeti_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_m81: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M81)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_m81_co.paa",""};
	};
	class pca_opscore_cover_ct_m81: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M81/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_m81_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_m81: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M81/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_m81_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_m81: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M81/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","usp_gear_head\data\tex\fs_cover_m81_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_m81: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M81/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_m81_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_m90: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M90)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m90_co.paa",""};
	};
	class pca_opscore_cover_ct_m90: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M90/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m90_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_m90: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M90/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m90_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_m90: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M90/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m90_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_m90: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M90/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m90_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_m98: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M98)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m98_co.paa",""};
	};
	class pca_opscore_cover_ct_m98: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M98/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m98_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_m98: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M98/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m98_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_m98: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M98/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m98_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_m98: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (M98/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_m98_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mpd: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-D)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpd_co.paa",""};
	};
	class pca_opscore_cover_ct_mpd: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-D/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpd_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mpd: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-D/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpd_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mpd: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-D/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mpd: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-D/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_mpw: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-WD)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpw_co.paa",""};
	};
	class pca_opscore_cover_ct_mpw: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-WD/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpw_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_mpw: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-WD/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpw_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_mpw: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-WD/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_mpw: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Marpat-WD/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_mpw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_tropentarn: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tropentarn)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","usp_gear_head\data\tex\fs_cover_tropentarn_co.paa",""};
	};
	class pca_opscore_cover_ct_tropentarn: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tropentarn/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","usp_gear_head\data\tex\fs_cover_tropentarn_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_tropentarn: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tropentarn/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_tropentarn_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_tropentarn: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tropentarn/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","usp_gear_head\data\tex\fs_cover_tropentarn_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_tropentarn: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tropentarn/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_tropentarn_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_tsd: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSD)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsd_co.paa",""};
	};
	class pca_opscore_cover_ct_tsd: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSD/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsd_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_tsd: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSD/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsd_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_tsd: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSD/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_tsd: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSD/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsd_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_tsw: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_tsw_co.paa",""};
	};
	class pca_opscore_cover_ct_tsw: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_tsw_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_tsw: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_tsw_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_tsw: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","usp_gear_head\data\tex\fs_cover_tsw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_tsw: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_tsw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_tsw2: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsw_co.paa",""};
	};
	class pca_opscore_cover_ct_tsw2: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsw_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_tsw2: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsw_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_tsw2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_tsw2: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (TSW/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\fs_cover_tsw_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_blk: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","","","","usp_gear_head\data\tex\fs_cover_blk_co.paa",""};
	};
	class pca_opscore_cover_ct_blk: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Black/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","","usp_gear_head\data\tex\fs_cover_blk_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_blk: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Black/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_blk_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_blk: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Black/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","","usp_gear_head\data\tex\fs_cover_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_blk: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Black/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_khk: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_khk_co.paa",""};
	};
	class pca_opscore_cover_ct_khk: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Khaki/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_khk_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_khk: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Khaki/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_khk_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_khk: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Khaki/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_khk_co.paa","","usp_gear_head\data\tex\fs_cover_khk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_khk: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Khaki/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_khk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_khk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_nav: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Navy)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","","","","usp_gear_head\data\tex\fs_cover_nav_co.paa",""};
	};
	class pca_opscore_cover_ct_nav: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Navy/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","","usp_gear_head\data\tex\fs_cover_nav_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_nav: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Navy/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_nav_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_nav: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Navy/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","","usp_gear_head\data\tex\fs_cover_nav_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_nav: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Navy/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_blk_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_blk_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_nav_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_rgr: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Ranger Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","","","","usp_gear_head\data\tex\fs_cover_rgr_co.paa",""};
	};
	class pca_opscore_cover_ct_rgr: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Ranger Green/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","","usp_gear_head\data\tex\fs_cover_rgr_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_rgr: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Ranger Green/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_rgr_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_rgr: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Ranger Green/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","","usp_gear_head\data\tex\fs_cover_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_rgr: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Ranger Green/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_rgr_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_rgr_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_tan: rhsusf_opscore_mc_cover
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","","","","usp_gear_head\data\tex\fs_cover_tan_co.paa",""};
	};
	class pca_opscore_cover_ct_tan: rhsusf_opscore_mc_cover_pelt
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tan/CT)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","","usp_gear_head\data\tex\fs_cover_tan_co.paa",""};
	};
	class pca_opscore_cover_ct_cm_tan: rhsusf_opscore_mc_cover_pelt_cam
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tan/CT/CM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_tan_co.paa",""};
	};
	class pca_opscore_cover_ct_cw_tan: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tan/CT/CW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","","usp_gear_head\data\tex\fs_cover_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
	class pca_opscore_cover_ct_cb_tan: rhsusf_opscore_mc_cover_pelt_nsw
	{
		author = "Red Hammer Studios";
		displayName = "[US] FAST XP Cover (Tan/CT/CB)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_headgear\data\tex\opscore_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\comtac_tan_co.paa","x\pca\custom\addons\blended_usa_headgear\data\tex\cw_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa","usp_gear_head\data\tex\fs_cover_tan_co.paa","rhsusf\addons\rhsusf_infantry\gear\head\data\rhs_helmet_ach_acc_co.paa"};
	};
};
