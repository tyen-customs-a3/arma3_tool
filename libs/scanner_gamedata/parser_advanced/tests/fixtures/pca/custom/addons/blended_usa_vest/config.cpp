////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_usa_vest
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
	class rhsusf_iotv_ocp_base;
	class pca_iotv_oefcp: rhsusf_iotv_ocp_base
	{
		displayName = "[US] IOTV (OEF-CP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_iotv_ocp_Grenadier;
	class pca_iotv_grenadier_oefcp: rhsusf_iotv_ocp_Grenadier
	{
		displayName = "[US] IOTV (OEF-CP/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_iotv_ocp_Medic;
	class pca_iotv_medic_oefcp: rhsusf_iotv_ocp_Medic
	{
		displayName = "[US] IOTV (OEF-CP/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_iotv_ocp_Repair;
	class pca_iotv_repair_oefcp: rhsusf_iotv_ocp_Repair
	{
		displayName = "[US] IOTV (OEF-CP/Repair)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_iotv_ocp_Rifleman;
	class pca_iotv_rifleman_oefcp: rhsusf_iotv_ocp_Rifleman
	{
		displayName = "[US] IOTV (OEF-CP/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_iotv_ocp_SAW;
	class pca_iotv_saw_oefcp: rhsusf_iotv_ocp_SAW
	{
		displayName = "[US] IOTV (OEF-CP/SAW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_iotv_ocp_Teamleader;
	class pca_iotv_teamleader_oefcp: rhsusf_iotv_ocp_Teamleader
	{
		displayName = "[US] IOTV (OEF-CP/Team Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\iotv_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa"};
	};
	class rhsusf_spcs_ocp;
	class pca_spcs_oefcp: rhsusf_spcs_ocp
	{
		displayName = "[US] SPCS (OEF-CP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear1_oefcp_co.paa"};
	};
	class rhsusf_spcs_ocp_crewman;
	class pca_spcs_crewman_oefcp: rhsusf_spcs_ocp_crewman
	{
		displayName = "[US] SPCS (OEF-CP/Crewman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\mag_proxies\data\pmag_black_co.paa"};
	};
	class rhsusf_spcs_ocp_grenadier;
	class pca_spcs_grenadier_oefcp: rhsusf_spcs_ocp_grenadier
	{
		displayName = "[US] SPCS (OEF-CP/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\m4\data\magazine_co.paa"};
	};
	class rhsusf_spcs_ocp_machinegunner;
	class pca_spcs_mg_oefcp: rhsusf_spcs_ocp_machinegunner
	{
		displayName = "[US] SPCS (OEF-CP/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class rhsusf_spcs_ocp_medic;
	class pca_spcs_medic_oefcp: rhsusf_spcs_ocp_medic
	{
		displayName = "[US] SPCS (OEF-CP/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\m4\data\magazine_co.paa"};
	};
	class rhsusf_spcs_ocp_rifleman;
	class pca_spcs_rifleman_oefcp: rhsusf_spcs_ocp_rifleman
	{
		displayName = "[US] SPCS (OEF-CP/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\mag_proxies\data\pmag_black_co.paa"};
	};
	class rhsusf_spcs_ocp_rifleman_alt;
	class pca_spcs_rifleman_alt_oefcp: rhsusf_spcs_ocp_rifleman_alt
	{
		displayName = "[US] SPCS (OEF-CP/Rifleman Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\grenades\m18\data\m18_green_ca.paa"};
	};
	class rhsusf_spcs_ocp_saw;
	class pca_spcs_saw_oefcp: rhsusf_spcs_ocp_saw
	{
		displayName = "[US] SPCS (OEF-CP/SAW)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class rhsusf_spcs_ocp_squadleader;
	class pca_spcs_squadleader_oefcp: rhsusf_spcs_ocp_squadleader
	{
		displayName = "[US] SPCS (OEF-CP/Squad Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\grenades\m18\data\m18_green_ca.paa"};
	};
	class rhsusf_spcs_ocp_sniper;
	class pca_spcs_sniper_oefcp: rhsusf_spcs_ocp_sniper
	{
		displayName = "[US] SPCS (OEF-CP/Sniper)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\grenades\m18\data\m18_green_ca.paa","rhsusf\addons\rhsusf_weapons2\mk14\data\mk14_co.paa"};
	};
	class rhsusf_spcs_ocp_teamleader;
	class pca_spcs_teamleader_oefcp: rhsusf_spcs_ocp_teamleader
	{
		displayName = "[US] SPCS (OEF-CP/Team Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\grenades\m18\data\m18_green_ca.paa","rhsusf\addons\rhsusf_weapons\mag_proxies\data\pmag_black_co.paa"};
	};
	class rhsusf_spcs_ocp_teamleader_alt;
	class pca_spcs_teamleader_alt_oefcp: rhsusf_spcs_ocp_teamleader_alt
	{
		displayName = "[US] SPCS (OEF-CP/Team Leader Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spcs_base_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear2_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\iotv_gear3_oefcp_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa","rhsusf\addons\rhsusf_weapons\grenades\m18\data\m18_green_ca.paa"};
	};
	class rhsusf_spc;
	class pca_spc_rgr: rhsusf_spc
	{
		displayName = "[US] SPC (Ranger Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_crewman;
	class pca_spc_crewman_rgr: rhsusf_spc_crewman
	{
		displayName = "[US] SPC (Ranger Green/Crewman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_light;
	class pca_spc_light_rgr: rhsusf_spc_light
	{
		displayName = "[US] SPC (Ranger Green/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_corpsman;
	class pca_spc_medic_rgr: rhsusf_spc_corpsman
	{
		displayName = "[US] SPC (Ranger Green/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_mg;
	class pca_spc_mg_rgr: rhsusf_spc_mg
	{
		displayName = "[US] SPC (Ranger Green/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_marksman;
	class pca_spc_marksman_rgr: rhsusf_spc_marksman
	{
		displayName = "[US] SPC (Ranger Green/Marksman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_rifleman;
	class pca_spc_rifleman_rgr: rhsusf_spc_rifleman
	{
		displayName = "[US] SPC (Ranger Green/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_iar;
	class pca_spc_rifleman_alt_rgr: rhsusf_spc_iar
	{
		displayName = "[US] SPC (Ranger Green/Rifleman Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class rhsusf_spc_squadleader;
	class pca_spc_squadleader_rgr: rhsusf_spc_squadleader
	{
		displayName = "[US] SPC (Ranger Green/Squad Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_rgr_co.paa"};
	};
	class pca_spc_blk: pca_spc_rgr
	{
		displayName = "[US] SPC (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_crewman_blk: pca_spc_crewman_rgr
	{
		displayName = "[US] SPC (Black/Crewman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_light_blk: pca_spc_light_rgr
	{
		displayName = "[US] SPC (Black/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_medic_blk: pca_spc_medic_rgr
	{
		displayName = "[US] SPC (Black/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_mg_blk: pca_spc_mg_rgr
	{
		displayName = "[US] SPC (Black/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_marksman_blk: pca_spc_marksman_rgr
	{
		displayName = "[US] SPC (Black/Marksman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_rifleman_blk: pca_spc_rifleman_rgr
	{
		displayName = "[US] SPC (Black/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_rifleman_alt_blk: pca_spc_rifleman_alt_rgr
	{
		displayName = "[US] SPC (Black/Rifleman Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_squadleader_blk: pca_spc_squadleader_rgr
	{
		displayName = "[US] SPC (Black/Squad Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_blk_co.paa"};
	};
	class pca_spc_tan: pca_spc_rgr
	{
		displayName = "[US] SPC (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_crewman_tan: pca_spc_crewman_rgr
	{
		displayName = "[US] SPC (Tan/Crewman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_light_tan: pca_spc_light_rgr
	{
		displayName = "[US] SPC (Tan/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_medic_tan: pca_spc_medic_rgr
	{
		displayName = "[US] SPC (Tan/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_mg_tan: pca_spc_mg_rgr
	{
		displayName = "[US] SPC (Tan/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_marksman_tan: pca_spc_marksman_rgr
	{
		displayName = "[US] SPC (Tan/Marksman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_rifleman_tan: pca_spc_rifleman_rgr
	{
		displayName = "[US] SPC (Tan/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_rifleman_alt_tan: pca_spc_rifleman_alt_rgr
	{
		displayName = "[US] SPC (Tan/Rifleman Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_squadleader_tan: pca_spc_squadleader_rgr
	{
		displayName = "[US] SPC (Tan/Squad Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_tan_co.paa"};
	};
	class pca_spc_wht: pca_spc_rgr
	{
		displayName = "[US] SPC (White)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_crewman_wht: pca_spc_crewman_rgr
	{
		displayName = "[US] SPC (White/Crewman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_light_wht: pca_spc_light_rgr
	{
		displayName = "[US] SPC (White/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_medic_wht: pca_spc_medic_rgr
	{
		displayName = "[US] SPC (White/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_mg_wht: pca_spc_mg_rgr
	{
		displayName = "[US] SPC (White/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_marksman_wht: pca_spc_marksman_rgr
	{
		displayName = "[US] SPC (White/Marksman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_rifleman_wht: pca_spc_rifleman_rgr
	{
		displayName = "[US] SPC (White/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_rifleman_alt_wht: pca_spc_rifleman_alt_rgr
	{
		displayName = "[US] SPC (White/Rifleman Alt)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class pca_spc_squadleader_wht: pca_spc_squadleader_rgr
	{
		displayName = "[US] SPC (White/Squad Leader)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\spc_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear1_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\spc_gear2_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\camelback_wht_co.paa"};
	};
	class rhsusf_mbav;
	class pca_mbav_rgr: rhsusf_mbav
	{
		displayName = "[US] MBAV (Ranger Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_rgr_co.paa"};
	};
	class rhsusf_mbav_grenadier;
	class pca_mbav_grenadier_rgr: rhsusf_mbav_grenadier
	{
		displayName = "[US] MBAV (Ranger Green/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class rhsusf_mbav_light;
	class pca_mbav_light_rgr: rhsusf_mbav_light
	{
		displayName = "[US] MBAV (Ranger Green/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class rhsusf_mbav_medic;
	class pca_mbav_medic_rgr: rhsusf_mbav_medic
	{
		displayName = "[US] MBAV (Ranger Green/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class rhsusf_mbav_mg;
	class pca_mbav_mg_rgr: rhsusf_mbav_mg
	{
		displayName = "[US] MBAV (Ranger Green/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class rhsusf_mbav_rifleman;
	class pca_mbav_rifleman_rgr: rhsusf_mbav_rifleman
	{
		displayName = "[US] MBAV (Ranger Green/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_rgr_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_rgr_co.paa"};
	};
	class pca_mbav_blk: pca_mbav_rgr
	{
		displayName = "[US] MBAV (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_blk_co.paa"};
	};
	class pca_mbav_grenadier_blk: pca_mbav_grenadier_rgr
	{
		displayName = "[US] MBAV (Black/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_blk_co.paa"};
	};
	class pca_mbav_light_blk: pca_mbav_light_rgr
	{
		displayName = "[US] MBAV (Black/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_blk_co.paa"};
	};
	class pca_mbav_medic_blk: pca_mbav_medic_rgr
	{
		displayName = "[US] MBAV (Black/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_blk_co.paa"};
	};
	class pca_mbav_mg_blk: pca_mbav_mg_rgr
	{
		displayName = "[US] MBAV (Black/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_blk_co.paa"};
	};
	class pca_mbav_rifleman_blk: pca_mbav_rifleman_rgr
	{
		displayName = "[US] MBAV (Black/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_blk_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_blk_co.paa"};
	};
	class pca_mbav_tan: pca_mbav_rgr
	{
		displayName = "[US] MBAV (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_tan_co.paa"};
	};
	class pca_mbav_grenadier_tan: pca_mbav_grenadier_rgr
	{
		displayName = "[US] MBAV (Tan/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_tan_co.paa"};
	};
	class pca_mbav_light_tan: pca_mbav_light_rgr
	{
		displayName = "[US] MBAV (Tan/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_tan_co.paa"};
	};
	class pca_mbav_medic_tan: pca_mbav_medic_rgr
	{
		displayName = "[US] MBAV (Tan/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_tan_co.paa"};
	};
	class pca_mbav_mg_tan: pca_mbav_mg_rgr
	{
		displayName = "[US] MBAV (Tan/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_tan_co.paa"};
	};
	class pca_mbav_rifleman_tan: pca_mbav_rifleman_rgr
	{
		displayName = "[US] MBAV (Tan/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_tan_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_tan_co.paa"};
	};
	class pca_mbav_wht: pca_mbav_rgr
	{
		displayName = "[US] MBAV (White)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_wht_co.paa"};
	};
	class pca_mbav_grenadier_wht: pca_mbav_grenadier_rgr
	{
		displayName = "[US] MBAV (White/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_wht_co.paa"};
	};
	class pca_mbav_light_wht: pca_mbav_light_rgr
	{
		displayName = "[US] MBAV (White/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_wht_co.paa"};
	};
	class pca_mbav_medic_wht: pca_mbav_medic_rgr
	{
		displayName = "[US] MBAV (White/Medic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_wht_co.paa"};
	};
	class pca_mbav_mg_wht: pca_mbav_mg_rgr
	{
		displayName = "[US] MBAV (White/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_wht_co.paa"};
	};
	class pca_mbav_rifleman_wht: pca_mbav_rifleman_rgr
	{
		displayName = "[US] MBAV (White/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_vest\data\tex\mbav_base_wht_co.paa","x\pca\custom\addons\blended_usa_vest\data\tex\mbav_gear_wht_co.paa"};
	};
};
