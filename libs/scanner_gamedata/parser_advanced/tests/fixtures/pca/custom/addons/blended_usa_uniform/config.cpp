////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_usa_uniform
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","A3_Weapons_F_Exp","rhs_c_troops","rhsusf_c_troops"};
		units[] = {};
		weapons[] = {};
	};
};
class CfgVehicles
{
	class B_Soldier_F;
	class pca_acu_ocp: B_Soldier_F
	{
		scope = 1;
		author = "Red Hammer Studios";
		displayName = "[US] Combat Uniform (OEF-CP)";
		model = "rhsusf\addons\rhsusf_infantry2\acu\rhsusf_uniform_acu.p3d";
		hiddenSelections[] = {"camo1","camo2","camo3","identity","flag","insignia"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\acu_01_ocp_co.paa","x\pca\custom\addons\blended_usa_uniform\data\tex\acu_02_ocp_co.paa","x\pca\custom\addons\blended_usa_uniform\data\tex\acu_03_ocp_co.paa","#(argb,8,8,3)color(0,0,0,0)","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_acc2_co.paa"};
		class Wounds
		{
			tex[] = {};
			mat[] = {"rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_01.rvmat","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_w01.rvmat","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_w01.rvmat","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_02.rvmat","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_w02.rvmat","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_w02.rvmat","A3\Characters_F\Common\Data\basicbody.rvmat","A3\Characters_F\Common\Data\basicbody_injury.rvmat","A3\Characters_F\Common\Data\basicbody_injury.rvmat","a3\characters_f\heads\data\hl_white.rvmat","a3\characters_f\heads\data\hl_white_injury.rvmat","a3\characters_f\heads\data\hl_white_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_bald_muscular.rvmat","A3\Characters_F\Heads\Data\hl_white_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_02_bald_muscular.rvmat","A3\Characters_F\Heads\Data\hl_white_02_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_02_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_black_bald_muscular.rvmat","A3\Characters_F\Heads\Data\hl_black_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_black_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_hairy_muscular.rvmat","A3\Characters_F\Heads\Data\hl_white_hairy_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_hairy_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_old.rvmat","A3\Characters_F\Heads\Data\hl_white_old_injury.rvmat","A3\Characters_F\Heads\Data\hl_white_old_injury.rvmat","A3\Characters_F\Heads\Data\hl_asian_bald_muscular.rvmat","A3\Characters_F\Heads\Data\hl_asian_bald_muscular_injury.rvmat","A3\Characters_F\Heads\Data\hl_asian_bald_muscular_injury.rvmat","A3\Characters_F_Exp\Heads\Data\hl_tanoan_bald_muscular.rvmat","A3\Characters_F_Exp\Heads\Data\hl_tanoan_bald_muscular_injury.rvmat","A3\Characters_F_Exp\Heads\Data\hl_tanoan_bald_muscular_injury.rvmat","A3\Characters_F_Exp\Heads\Data\hl_asian_02_bald_muscular.rvmat","A3\Characters_F_Exp\Heads\Data\hl_asian_02_bald_muscular_injury.rvmat","A3\Characters_F_Exp\Heads\Data\hl_asian_02_bald_muscular_injury.rvmat"};
		};
	};
	class pca_acu_oefcp: pca_acu_ocp
	{
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\acu_01_oefcp_co.paa","x\pca\custom\addons\blended_usa_uniform\data\tex\acu_02_oefcp_co.paa","x\pca\custom\addons\blended_usa_uniform\data\tex\acu_03_ocp_co.paa","#(argb,8,8,3)color(0,0,0,0)","rhsusf\addons\rhsusf_infantry2\acu\data\rhsusf_uniform_acu_acc2_co.paa"};
	};
	class rhsusf_socom_uniform_base;
	class pca_g3_mc: rhsusf_socom_uniform_base
	{
		scope = 1;
		author = "Red Hammer Studios";
		uniformClass = "pca_g3_mc";
		hiddenSelections[] = {"Camo","Camo2","Gloves","FlagLeft","FlagRight","insignia","clan"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_mc_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_mcaus: pca_g3_mc
	{
		uniformClass = "pca_g3_mcaus";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_mcaus_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_mchun: pca_g3_mc
	{
		uniformClass = "pca_g3_mchun";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_mchun_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_mcslo: pca_g3_mc
	{
		uniformClass = "pca_g3_mcslo";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_mcslo_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_aaf: pca_g3_mc
	{
		uniformClass = "pca_g3_aaf";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_aaf_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_bjig: pca_g3_mc
	{
		uniformClass = "pca_g3_bjig";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_bjig_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_cadpat: pca_g3_mc
	{
		uniformClass = "pca_g3_cadpat";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_cadpat_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_ce: pca_g3_mc
	{
		uniformClass = "pca_g3_ce";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_ce_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_dcu: pca_g3_mc
	{
		uniformClass = "pca_g3_dcu";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_dcu_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_estdcuw: pca_g3_mc
	{
		uniformClass = "pca_g3_estdcuw";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_estdcuw_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_geo: pca_g3_mc
	{
		uniformClass = "pca_g3_geo";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_geo_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_gliz: pca_g3_mc
	{
		uniformClass = "pca_g3_gliz";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_gliz_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_kryalt: pca_g3_mc
	{
		uniformClass = "pca_g3_kryalt";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_kryalt_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_kryban: pca_g3_mc
	{
		uniformClass = "pca_g3_kryban";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_kryban_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_kryhl: pca_g3_mc
	{
		uniformClass = "pca_g3_kryhl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_kryhl_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_kryman: pca_g3_mc
	{
		uniformClass = "pca_g3_kryman";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_kryman_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_krynep: pca_g3_mc
	{
		uniformClass = "pca_g3_krynep";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_krynep_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_krynom: pca_g3_mc
	{
		uniformClass = "pca_g3_krynom";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_krynom_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_krypon: pca_g3_mc
	{
		uniformClass = "pca_g3_krypon";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_krypon_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_krytyp: pca_g3_mc
	{
		uniformClass = "pca_g3_krytyp";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_krytyp_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_latpatd: pca_g3_mc
	{
		uniformClass = "pca_g3_latpatd";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_latpatd_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_m2002: pca_g3_mc
	{
		uniformClass = "pca_g3_m2002";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_m2002_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_m05w: pca_g3_mc
	{
		uniformClass = "pca_g3_m05w";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_m05w_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_m84w: pca_g3_mc
	{
		uniformClass = "pca_g3_m84w";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_m84w_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_m90: pca_g3_mc
	{
		uniformClass = "pca_g3_m90";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_m90_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_m98: pca_g3_mc
	{
		uniformClass = "pca_g3_m98";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_m98_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_m98_alt: pca_g3_mc
	{
		uniformClass = "pca_g3_m98_alt";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_m98_alt_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_marpatd: pca_g3_mc
	{
		uniformClass = "pca_g3_marpatd";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_marpatd_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_marpatwd: pca_g3_mc
	{
		uniformClass = "pca_g3_marpatwd";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_marpatwd_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_mexcam: pca_g3_mc
	{
		uniformClass = "pca_g3_mexcam";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_mexcam_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_orshw: pca_g3_mc
	{
		uniformClass = "pca_g3_orshw";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_orshw_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_pf: pca_g3_mc
	{
		uniformClass = "pca_g3_pf";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_pf_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_tsd: pca_g3_mc
	{
		uniformClass = "pca_g3_tsd";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_tsd_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_tan_co.paa","",""};
	};
	class pca_g3_tsw: pca_g3_mc
	{
		uniformClass = "pca_g3_tsw";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_tsw_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_tubitak: pca_g3_mc
	{
		uniformClass = "pca_g3_tubitak";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_tubitak_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_black_co.paa","",""};
	};
	class pca_g3_us4cesf: pca_g3_mc
	{
		uniformClass = "pca_g3_us4cesf";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_us4cesf_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_us4cest: pca_g3_mc
	{
		uniformClass = "pca_g3_us4cest";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_us4cest_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
	class pca_g3_vgw: pca_g3_mc
	{
		uniformClass = "pca_g3_vgw";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_uniform\data\tex\g3_vgw_co.paa","rhsusf\addons\rhsusf_infantry2\data\merrells_blk_co.paa","rhsusf\addons\rhsusf_infantry2\data\mechanix_green_co.paa","",""};
	};
};
class CfgWeapons
{
	class UniformItem;
	class rhs_uniform_acu_ocp;
	class pca_acu_ocp: rhs_uniform_acu_ocp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACU (OCP)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_acu_ocp";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class rhs_uniform_acu_oefcp;
	class pca_acu_oefcp: rhs_uniform_acu_oefcp
	{
		author = "Red Hammer Studios";
		displayName = "[US] ACU (OEF-CP)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_acu_oefcp";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class rhs_uniform_g3_rgr;
	class pca_g3_mc: rhs_uniform_g3_rgr
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (MC)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_mc";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_mcaus: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (MC-Australia)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_mcaus";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_mchun: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (MC-Hungary)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_mchun";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_mcslo: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (MC-Slovania)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_mcslo";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_cadpat: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Cadpat)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_cadpat";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_aaf: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (AAF-Digital)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_aaf";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_bjig: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Belgian Jigsaw)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_bjig";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_ce: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (CE)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_ce";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_dcu: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (DCU)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_dcu";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_estdcuw: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (ESTDCU-Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_estdcuw";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_geo: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Geometric)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_geo";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_gliz: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Greek Lizard)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_gliz";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_kryalt: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Altitude)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_kryalt";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_kryban: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Banshee)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_kryban";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_kryhl: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-HighLander)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_kryhl";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_kryman: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Mandrake)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_kryman";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_krynep: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Neptune)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_krynep";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_krynom: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Nomad)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_krynom";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_krypon: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Pontus)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_krypon";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_krytyp: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Kryptek-Typhon)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_krytyp";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_latpatd: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Latpat-Desert)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_latpatd";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_m2002: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (M2002-Romania)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_m2002";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_m05w: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (M05-Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_m05w";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_m84w: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (M84-Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_m84w";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_m90: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (M90)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_m90";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_m98: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (M98)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_m98";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_m98_alt: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (M98 Alt)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_m98_alt";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_marpatd: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Marpat-Desert)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_marpatd";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_marpatwd: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Marpat-Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_marpatwd";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_mexcam: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Mexcam)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_mexcam";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_orshw: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (ORSH-Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_orshw";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_pf: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Padrao Floresta)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_pf";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_tsd: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Tiger Stripe Desert)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_tsd";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_tsw: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Tiger Stripe Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_tsw";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_tubitak: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Tubitak)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_tubitak";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_us4cesf: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (US4CES-Forest)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_us4cesf";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_us4cest: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (US4CES-Transitional)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_us4cest";
			containerClass = "Supply60";
			mass = 40;
		};
	};
	class pca_g3_vgw: pca_g3_mc
	{
		author = "Red Hammer Studios";
		displayName = "G3 Uniform (Vegetata-Woodland)";
		class ItemInfo: UniformItem
		{
			uniformModel = "-";
			uniformClass = "pca_g3_vgw";
			containerClass = "Supply60";
			mass = 40;
		};
	};
};
