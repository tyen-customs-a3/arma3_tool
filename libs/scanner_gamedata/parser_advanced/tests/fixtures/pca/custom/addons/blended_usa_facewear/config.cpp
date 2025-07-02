////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_usa_facewear
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","A3_Weapons_F_Exp"};
		units[] = {};
		weapons[] = {};
	};
};
class CfgGlasses
{
	class G_Bandanna_blk;
	class G_Balaclava_TI_blk_F;
	class G_Balaclava_TI_G_blk_F;
	class pca_balaclava_ocp: G_Balaclava_TI_blk_F
	{
		scope = 2;
		displayName = "Balaclava (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_ocp_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_facewear\data\rv\balaclava.rvmat"};
	};
	class pca_balaclava_gogg_ocp: G_Balaclava_TI_G_blk_F
	{
		scope = 2;
		displayName = "Balaclava Goggles (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_ocp_co.paa","\a3\characters_f_exp\blufor\data\g_combat_goggles_tna_f_ca.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_facewear\data\rv\balaclava.rvmat"};
	};
	class pca_balaclava_mcalp: pca_balaclava_ocp
	{
		displayName = "Balaclava (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_mca_co.paa"};
	};
	class pca_balaclava_gogg_mcalp: pca_balaclava_gogg_ocp
	{
		displayName = "Balaclava Goggles (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_mca_co.paa","x\pca\custom\addons\blended_usa_facewear\data\tex\combat_goggles_snow_co.paa"};
	};
	class pca_balaclava_mcb: pca_balaclava_ocp
	{
		displayName = "Balaclava (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_mcb_co.paa"};
	};
	class pca_balaclava_gogg_mcb: pca_balaclava_gogg_ocp
	{
		displayName = "Balaclava Goggles (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_mcb_co.paa","x\pca\custom\addons\blended_usa_facewear\data\tex\combat_goggles_blk_co.paa"};
	};
	class pca_balaclava_mct: pca_balaclava_ocp
	{
		displayName = "Balaclava (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_mct_co.paa"};
	};
	class pca_balaclava_gogg_mct: pca_balaclava_gogg_ocp
	{
		displayName = "Balaclava Goggles (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\balaclava_mct_co.paa","\a3\characters_f_exp\blufor\data\g_combat_goggles_tna_f_ca.paa"};
	};
	class pca_bandana_mc: G_Bandanna_blk
	{
		scope = 2;
		displayName = "Bandana (MC)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\bandana_mc_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_facewear\data\rv\bandana.rvmat"};
	};
	class pca_bandana_ocp: pca_bandana_mc
	{
		scope = 2;
		displayName = "Bandana (OCP)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\bandana_ocp_co.paa"};
	};
	class pca_bandana_mcalp: pca_bandana_mc
	{
		displayName = "Bandana (MC-Alpine)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\bandana_mca_co.paa"};
	};
	class pca_bandana_mcb: pca_bandana_mc
	{
		displayName = "Bandana (MC-Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\bandana_mcb_co.paa"};
	};
	class pca_bandana_mct: pca_bandana_mc
	{
		displayName = "Bandana (MC-Tropic)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_usa_facewear\data\tex\bandana_mct_co.paa"};
	};
};
