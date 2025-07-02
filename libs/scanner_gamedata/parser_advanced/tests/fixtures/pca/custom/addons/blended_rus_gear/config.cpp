////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:42 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_rus_gear
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","rhs_main","rhs_c_troops","rhsgref_c_troops","rhssaf_c_troops"};
		units[] = {};
		weapons[] = {"pca_uniform_afghanka","pca_uniform_afghanka_alpenflage","pca_uniform_afghanka_brn","pca_uniform_afghanka_butan_autumn","pca_uniform_afghanka_cdf_ttsko_desert","pca_uniform_afghanka_cdf_ttsko_mountain","pca_uniform_afghanka_cdf_wdl","pca_uniform_afghanka_gray","pca_uniform_afghanka_kamysh_leska_1","pca_uniform_afghanka_kamysh_leska_2","pca_uniform_afghanka_kamysh_mixed_1","pca_uniform_afghanka_kamysh_mixed_2","pca_uniform_afghanka_kamysh_urb","pca_uniform_afghanka_kamysh_wdl","pca_uniform_afghanka_klmk_mountain","pca_uniform_afghanka_leska","pca_uniform_afghanka_leska_kamysh_1","pca_uniform_afghanka_leska_kamysh_2","pca_uniform_afghanka_m89_oakleaf","pca_uniform_afghanka_oli","pca_uniform_afghanka_orel","pca_uniform_afghanka_plum","pca_uniform_afghanka_puma","pca_uniform_afghanka_smk_urb_1","pca_uniform_afghanka_smk_urb_2","pca_uniform_afghanka_smk_wdl_1","pca_uniform_afghanka_smk_wdl_2","pca_uniform_afghanka_smk_wdl_3","pca_uniform_afghanka_smk_wdl_4","pca_uniform_afghanka_spetsodezhda","pca_uniform_afghanka_spetsodezhda_od","pca_uniform_afghanka_strichtarn","pca_uniform_afghanka_swirl","pca_uniform_afghanka_taki_lizard","pca_uniform_afghanka_taki_lizard_od_1","pca_uniform_afghanka_taki_lizard_od_2","pca_uniform_afghanka_tan","pca_uniform_afghanka_tigr_desert","pca_uniform_afghanka_tigr_urb_1","pca_uniform_afghanka_tigr_urb_2","pca_uniform_afghanka_tigr_wdl_1","pca_uniform_afghanka_tigr_wdl_2","pca_uniform_afghanka_tigr_wdl_3","pca_uniform_afghanka_ttsko","pca_uniform_afghanka_ttsko_forest","pca_uniform_afghanka_vsr_1","pca_uniform_afghanka_vsr_2","pca_uniform_afghanka_vsr_3","pca_uniform_afghanka_vsr_4","pca_uniform_afghanka_vsr_5","pca_uniform_afghanka_yugo_m68","pca_uniform_afghanka_winter_ttsko_forest","pca_uniform_klmk_oversuit_berezka_desert","pca_uniform_klmk_oversuit_berezka_winter","pca_uniform_klmk_oversuit_frogskin","pca_uniform_klmk_oversuit_white","pca_nvg_gloves_wool","pca_nvg_gloves_wool_blu","pca_nvg_gloves_wool_brn","pca_nvg_gloves_wool_gry","pca_nvg_gloves_wool_grn"};
	};
};
class CfgVehicles
{
	class rhs_afghanka_base;
	class pca_afghanka: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Olive Drab)";
		uniformClass = "pca_uniform_afghanka";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_co.paa"};
	};
	class pca_afghanka_alpenflage: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Alpenflage)";
		uniformClass = "pca_uniform_afghanka_alpenflage";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_alpenflage_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_alpenflage_co.paa"};
	};
	class pca_afghanka_brn: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Brown)";
		uniformClass = "pca_uniform_afghanka_brn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_brown_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_brown_co.paa"};
	};
	class pca_afghanka_butan_autumn: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Butan Autumn)";
		uniformClass = "pca_uniform_afghanka_butan_a";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_butan_a_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_butan_a_co.paa"};
	};
	class pca_afghanka_cdf_ttsko_desert: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (CDF TTsKO Desert)";
		uniformClass = "pca_uniform_afghanka_cdf_ttsko_desert";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_cdf_ttsko_desert_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_cdf_ttsko_desert_co.paa"};
	};
	class pca_afghanka_cdf_ttsko_mountain: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (CDF TTsKO Mountain)";
		uniformClass = "pca_uniform_afghanka_cdf_ttsko_mountain";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_cdf_ttsko_mountain_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_cdf_ttsko_mountain_co.paa"};
	};
	class pca_afghanka_cdf_wdl: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (CDF Woodland)";
		uniformClass = "pca_uniform_afghanka_cdf_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_cdf_wdl_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_cdf_wdl_co.paa"};
	};
	class pca_afghanka_gray: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Gray)";
		uniformClass = "pca_uniform_afghanka_gray";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_gray_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_gray_co.paa"};
	};
	class pca_afghanka_kamysh_leska_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Kamysh/Leska 1)";
		uniformClass = "pca_uniform_afghanka_kamysh_leska_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_kamysh_urb_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_leska_co.paa"};
	};
	class pca_afghanka_kamysh_leska_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Kamysh/Leska 2)";
		uniformClass = "pca_uniform_afghanka_kamysh_leska_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_kamysh_wdl_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_leska_co.paa"};
	};
	class pca_afghanka_kamysh_mixed_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Kamysh Mixed 1)";
		uniformClass = "pca_uniform_afghanka_kamysh_mixed_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_kamysh_urb_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_kamysh_wdl_co.paa"};
	};
	class pca_afghanka_kamysh_mixed_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Kamysh Mixed 2)";
		uniformClass = "pca_uniform_afghanka_kamysh_mixed_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_kamysh_wdl_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_kamysh_urb_co.paa"};
	};
	class pca_afghanka_kamysh_urb: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Kamysh Urban)";
		uniformClass = "pca_uniform_afghanka_kamysh_urb";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_kamysh_urb_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_kamysh_urb_co.paa"};
	};
	class pca_afghanka_kamysh_wdl: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Kamysh Woodland)";
		uniformClass = "pca_uniform_afghanka_kamysh_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_kamysh_wdl_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_kamysh_wdl_co.paa"};
	};
	class pca_afghanka_klmk_mountain: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (KLMK Mountain)";
		uniformClass = "pca_uniform_afghanka_klmk_mountain";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_klmk_mountain_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_klmk_mountain_co.paa"};
	};
	class pca_afghanka_leska: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Leska)";
		uniformClass = "pca_uniform_afghanka_leska";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_leska_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_leska_co.paa"};
	};
	class pca_afghanka_leska_kamysh_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Leska/Kamysh 1)";
		uniformClass = "pca_uniform_afghanka_leska";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_leska_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_kamysh_urb_co.paa"};
	};
	class pca_afghanka_leska_kamysh_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Leska/Kamysh 2)";
		uniformClass = "pca_uniform_afghanka_leska";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_leska_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_kamysh_wdl_co.paa"};
	};
	class pca_afghanka_m89_oakleaf: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (M89 Oakleaf)";
		uniformClass = "pca_uniform_afghanka_m89_oakleaf";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_m89oakleaf_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_m89oakleaf_co.paa"};
	};
	class pca_afghanka_oli: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Olive Green)";
		uniformClass = "pca_uniform_afghanka_oli";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_olive_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_olive_co.paa"};
	};
	class pca_afghanka_orel: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Orel)";
		uniformClass = "pca_uniform_afghanka_orel";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_orel_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_orel_co.paa"};
	};
	class pca_afghanka_plum: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Plum)";
		uniformClass = "pca_uniform_afghanka_plum";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_plum_co.paa"};
	};
	class pca_afghanka_puma: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Puma)";
		uniformClass = "pca_uniform_afghanka_puma";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_puma_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_puma_co.paa"};
	};
	class pca_afghanka_smk_urb_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (SMK Urban 1)";
		uniformClass = "pca_uniform_afghanka_smk_urb_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_smk_urb_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_urb_1_co.paa"};
	};
	class pca_afghanka_smk_urb_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (SMK Urban 2)";
		uniformClass = "pca_uniform_afghanka_smk_urb_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_smk_urb_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_urb_2_co.paa"};
	};
	class pca_afghanka_smk_wdl_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 1)";
		uniformClass = "pca_uniform_afghanka_smk_wdl_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_smk_wdl_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_1_co.paa"};
	};
	class pca_afghanka_smk_wdl_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 2)";
		uniformClass = "pca_uniform_afghanka_smk_wdl_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_smk_wdl_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_2_co.paa"};
	};
	class pca_afghanka_smk_wdl_3: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 3)";
		uniformClass = "pca_uniform_afghanka_smk_wdl_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_smk_wdl_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_3_co.paa"};
	};
	class pca_afghanka_smk_wdl_4: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 4)";
		uniformClass = "pca_uniform_afghanka_smk_wdl_4";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_smk_wdl_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_4_co.paa"};
	};
	class pca_afghanka_spetsodezhda: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Spetsodezhda)";
		uniformClass = "pca_uniform_afghanka_spetsodezhda";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_spetsodezhda_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_spetsodezhda_co.paa"};
	};
	class pca_afghanka_spetsodezhda_od: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Spetsodezhda Olive Drab)";
		uniformClass = "pca_uniform_afghanka_spetsodezhda_od";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_spetsodezhda_od_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_spetsodezhda_od_co.paa"};
	};
	class pca_afghanka_strichtarn: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Strichtarn)";
		uniformClass = "pca_uniform_afghanka_strichtarn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_strichtarn_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_strichtarn_co.paa"};
	};
	class pca_afghanka_swirl: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Swirl)";
		uniformClass = "pca_uniform_afghanka_swirl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_swirl_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_swirl_co.paa"};
	};
	class pca_afghanka_taki_lizard: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Takistan Lizard)";
		uniformClass = "pca_uniform_afghanka_taki_lizard";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_taki_lizard_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_taki_lizard_co.paa"};
	};
	class pca_afghanka_taki_lizard_od_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Takistan Lizard/Olive Drab 1)";
		uniformClass = "pca_uniform_afghanka_taki_lizard_od_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_taki_lizard_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_co.paa"};
	};
	class pca_afghanka_taki_lizard_od_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Takistan Lizard/Olive Drab 2)";
		uniformClass = "pca_uniform_afghanka_taki_lizard_od_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_taki_lizard_co.paa"};
	};
	class pca_afghanka_tan: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Tan)";
		uniformClass = "pca_uniform_afghanka_tan";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tan_co.paa"};
	};
	class pca_afghanka_tigr_desert: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Desert)";
		uniformClass = "pca_uniform_afghanka_tigr_desert";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tigr_desert_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_desert_co.paa"};
	};
	class pca_afghanka_tigr_urb_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Tigr Urban 1)";
		uniformClass = "pca_uniform_afghanka_tigr_urb_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tigr_urb_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_urb_1_co.paa"};
	};
	class pca_afghanka_tigr_urb_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Tigr Urban 2)";
		uniformClass = "pca_uniform_afghanka_tigr_urb_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tigr_urb_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_urb_2_co.paa"};
	};
	class pca_afghanka_tigr_wdl_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Tigr Woodland 1)";
		uniformClass = "pca_uniform_afghanka_tigr_wdl_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tigr_wdl_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_wdl_1_co.paa"};
	};
	class pca_afghanka_tigr_wdl_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Tigr Woodland 2)";
		uniformClass = "pca_uniform_afghanka_tigr_wdl_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tigr_wdl_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_wdl_2_co.paa"};
	};
	class pca_afghanka_tigr_wdl_3: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (Tigr Woodland 3)";
		uniformClass = "pca_uniform_afghanka_tigr_wdl_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_tigr_wdl_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_wdl_3_co.paa"};
	};
	class pca_afghanka_ttsko: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (TTsKO)";
		uniformClass = "pca_uniform_afghanka_ttsko";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_ttsko_co.paa"};
	};
	class pca_afghanka_ttsko_forest: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (TTsKO Forest)";
		uniformClass = "pca_uniform_afghanka_forest";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_ttsko_forest_co.paa"};
	};
	class pca_afghanka_vsr_1: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (VSR 1)";
		uniformClass = "pca_uniform_afghanka_vsr_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_vsr_1_co.paa"};
	};
	class pca_afghanka_vsr_2: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (VSR 2)";
		uniformClass = "pca_uniform_afghanka_vsr_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_vsr_2_co.paa"};
	};
	class pca_afghanka_vsr_3: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (VSR 3)";
		uniformClass = "pca_uniform_afghanka_vsr_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_vsr_3_co.paa"};
	};
	class pca_afghanka_vsr_4: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (VSR 4)";
		uniformClass = "pca_uniform_afghanka_vsr_4";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_vsr_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_vsr_4_co.paa"};
	};
	class pca_afghanka_vsr_5: rhs_afghanka_base
	{
		displayName = "[RU] M88 Uniform (VSR 5)";
		uniformClass = "pca_uniform_afghanka_vsr_5";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_vsr_5_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_vsr_5_co.paa"};
	};
	class pca_afghanka_yugo_m68: rhs_afghanka_base
	{
		displayName = "[RU] M68 Uniform (Yugoslavia M68)";
		uniformClass = "pca_uniform_afghanka_yugo_m68";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vest_yugo_m68_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_yugo_m68_co.paa"};
	};
	class rhs_afghanka_winter_moldovan_ttsko_base;
	class pca_afghanka_winter_smk_urb_1: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (SMK Urban 1)";
		uniformClass = "pca_uniform_afghanka_winter_smk_urb_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_smk_urb_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_urb_1_co.paa"};
	};
	class pca_afghanka_winter_smk_urb_2: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (SMK Urban 2)";
		uniformClass = "pca_uniform_afghanka_winter_smk_urb_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_smk_urb_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_urb_2_co.paa"};
	};
	class pca_afghanka_winter_smk_wdl_1: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 1)";
		uniformClass = "pca_uniform_afghanka_winter_smk_wdl_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_smk_wdl_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_1_co.paa"};
	};
	class pca_afghanka_winter_smk_wdl_2: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 2)";
		uniformClass = "pca_uniform_afghanka_winter_smk_wdl_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_smk_wdl_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_2_co.paa"};
	};
	class pca_afghanka_winter_smk_wdl_3: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 3)";
		uniformClass = "pca_uniform_afghanka_winter_smk_wdl_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_smk_wdl_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_3_co.paa"};
	};
	class pca_afghanka_winter_smk_wdl_4: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 4)";
		uniformClass = "pca_uniform_afghanka_winter_smk_wdl_4";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_smk_wdl_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_smk_wdl_4_co.paa"};
	};
	class pca_afghanka_winter_spetsodezhda_od: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (Spetsodezhda Olive Drab)";
		uniformClass = "pca_uniform_afghanka_winter_spetsodezhda_od";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_spetsodezhda_od_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_spetsodezhda_od_co.paa"};
	};
	class pca_afghanka_winter_tigr_urb_1: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Urban 1)";
		uniformClass = "pca_uniform_afghanka_winter_tigr_urb_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_tigr_urb_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_urb_1_co.paa"};
	};
	class pca_afghanka_winter_tigr_urb_2: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Urban 2)";
		uniformClass = "pca_uniform_afghanka_winter_tigr_urb_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_tigr_urb_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_urb_2_co.paa"};
	};
	class pca_afghanka_winter_tigr_wdl_1: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Woodland 1)";
		uniformClass = "pca_uniform_afghanka_winter_tigr_wdl_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_tigr_wdl_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_wdl_1_co.paa"};
	};
	class pca_afghanka_winter_tigr_wdl_2: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Woodland 2)";
		uniformClass = "pca_uniform_afghanka_winter_tigr_wdl_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_tigr_wdl_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_wdl_2_co.paa"};
	};
	class pca_afghanka_winter_tigr_wdl_3: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Woodland 3)";
		uniformClass = "pca_uniform_afghanka_winter_tigr_wdl_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_tigr_wdl_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_tigr_wdl_3_co.paa"};
	};
	class pca_afghanka_winter_ttsko: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (TTsKO)";
		uniformClass = "pca_uniform_afghanka_winter_ttsko";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_ttsko_co.paa"};
	};
	class pca_afghanka_winter_ttsko_forest: rhs_afghanka_winter_moldovan_ttsko_base
	{
		displayName = "[RU] M88 Uniform Winter (TTsKO Forest)";
		uniformClass = "pca_uniform_afghanka_winter_forest";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_winter_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_pants_ttsko_forest_co.paa"};
	};
	class rhs_afghanka_para_vsr_base;
	class pca_afghanka_vdv_granite_tan: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (Granite Tan)";
		uniformClass = "pca_uniform_afghanka_vdv_granite_tan";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_granite_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_granite_tan_co.paa"};
	};
	class pca_afghanka_vdv_cdf_ttsko_autumn: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (CDF TTsKO Autumn)";
		uniformClass = "pca_uniform_afghanka_vdv_cdf_ttsko_autumn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_cdf_ttsko_autumn_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_cdf_ttsko_autumn_co.paa"};
	};
	class pca_afghanka_vdv_cdf_ttsko_forest: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (CDF TTsKO Forest)";
		uniformClass = "pca_uniform_afghanka_vdv_cdf_ttsko_forest";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_cdf_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_cdf_ttsko_forest_co.paa"};
	};
	class pca_afghanka_vdv_cdf_plum: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (CDF Plum)";
		uniformClass = "pca_uniform_afghanka_vdv_cdf_plum";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_cdf_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_cdf_plum_co.paa"};
	};
	class pca_afghanka_vdv_ttsko: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (TTsKO)";
		uniformClass = "pca_uniform_afghanka_vdv_ttsko";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_ttsko_co.paa"};
	};
	class pca_afghanka_vdv_ttsko_oxblood: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (TTsKO Oxblood)";
		uniformClass = "pca_uniform_afghanka_vdv_ttsko_oxblood";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_ttsko_oxblood_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_ttsko_oxblood_co.paa"};
	};
	class pca_afghanka_vdv_vsr_1: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (VSR 1)";
		uniformClass = "pca_uniform_afghanka_vdv_vsr_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_vsr_1_co.paa"};
	};
	class pca_afghanka_vdv_vsr_2: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (VSR 2)";
		uniformClass = "pca_uniform_afghanka_vdv_vsr_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_vsr_2_co.paa"};
	};
	class pca_afghanka_vdv_vsr_3: rhs_afghanka_para_vsr_base
	{
		displayName = "[RU] M88 Uniform VDV (VSR 3)";
		uniformClass = "pca_uniform_afghanka_vdv_vsr_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\afghanka\afghanka_vdv_pants_vsr_3_co.paa"};
	};
	class rhs_vdv_gorka_r_y_rifleman;
	class rhs_vdv_gorka_r_y_gloves_rifleman;
	class pca_gorka_cdf_ttsko_winter: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (CDF TTsKO Winter)";
		uniformClass = "pca_uniform_gorka_cdf_ttsko_winter";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_cdf_ttsko_winter_co.paa"};
	};
	class pca_gorka_cdf_ttsko_winter_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (CDF TTsKO Winter/Gloves)";
		uniformClass = "pca_uniform_gorka_cdf_ttsko_winter_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_cdf_ttsko_winter_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_emr: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (EMR)";
		uniformClass = "pca_uniform_gorka_emr";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_emr_co.paa"};
	};
	class pca_gorka_emr_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (EMR/Gloves)";
		uniformClass = "pca_uniform_gorka_emr_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_emr_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_flecktarn: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Flecktarn)";
		uniformClass = "pca_uniform_gorka_flecktarn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_flecktarn_co.paa"};
	};
	class pca_gorka_flecktarn_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Flecktarn/Gloves)";
		uniformClass = "pca_uniform_gorka_flecktarn_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_flecktarn_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_kamysh_urb: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Kamysh Urban)";
		uniformClass = "pca_uniform_gorka_kamysh_urb";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_kamysh_urb_co.paa"};
	};
	class pca_gorka_kamysh_urb_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Kamysh Urban/Gloves)";
		uniformClass = "pca_uniform_gorka_kamysh_urb_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_kamysh_urb_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_kamysh_wdl: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Kamysh Woodland)";
		uniformClass = "pca_uniform_gorka_kamysh_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_kamysh_wdl_co.paa"};
	};
	class pca_gorka_kamysh_wdl_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Kamysh Woodland/Gloves)";
		uniformClass = "pca_uniform_gorka_kamysh_wdl_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_kamysh_wdl_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_leto: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Leto)";
		uniformClass = "pca_uniform_gorka_leto";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_leto_co.paa"};
	};
	class pca_gorka_leto_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Leto/Gloves)";
		uniformClass = "pca_uniform_gorka_leto_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_leto_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_smk_urb: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (SMK Urban)";
		uniformClass = "pca_uniform_gorka_smk_urb";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_smk_urb_co.paa"};
	};
	class pca_gorka_smk_urb_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (SMK Urban/Gloves)";
		uniformClass = "pca_uniform_gorka_smk_urb_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_smk_urb_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_smk_wdl: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (SMK Woodland)";
		uniformClass = "pca_uniform_gorka_smk_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_smk_wdl_co.paa"};
	};
	class pca_gorka_smk_wdl_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (SMK Woodland/Gloves)";
		uniformClass = "pca_uniform_gorka_smk_wdl_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_smk_wdl_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_tigr_urb: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Tigr Urban)";
		uniformClass = "pca_uniform_gorka_tigr_urb";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_tigr_urb_co.paa"};
	};
	class pca_gorka_tigr_urb_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Tigr Urban/Gloves)";
		uniformClass = "pca_uniform_gorka_tigr_urb_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_tigr_urb_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_tigr_wdl: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Tigr Woodland)";
		uniformClass = "pca_uniform_gorka_tigr_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_tigr_wdl_co.paa"};
	};
	class pca_gorka_tigr_wdl_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Tigr Woodland/Gloves)";
		uniformClass = "pca_uniform_gorka_tigr_wdl_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_tigr_wdl_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class pca_gorka_wdl: rhs_vdv_gorka_r_y_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Woodland)";
		uniformClass = "pca_uniform_gorka_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_wdl_co.paa"};
	};
	class pca_gorka_wdl_gloves: rhs_vdv_gorka_r_y_gloves_rifleman
	{
		scope = 1;
		displayName = "[RU] Gorka Suit (Woodland/Gloves)";
		uniformClass = "pca_uniform_gorka_wdl_gloves";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\gorka\gorka_wdl_co.paa","rhsafrf\addons\rhs_infantry3\ratnik\data\gloves_co.paa"};
	};
	class rhs_msv_rifleman_patchless;
	class pca_m88_field_brn: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Brown)";
		uniformClass = "pca_uniform_m88_field_brn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_brn_co.paa"};
	};
	class pca_m88_field_cdf_ttsko_autumn: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Autumn)";
		uniformClass = "pca_uniform_m88_field_cdf_ttsko_autumn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_cdf_ttsko_autumn_co.paa"};
	};
	class pca_m88_field_cdf_ttsko_desert: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Desert)";
		uniformClass = "pca_uniform_m88_field_cdf_ttsko_desert";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_cdf_ttsko_desert_co.paa"};
	};
	class pca_m88_field_cdf_ttsko_forest: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Forest)";
		uniformClass = "pca_uniform_m88_field_cdf_ttsko_forest";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_cdf_ttsko_forest_co.paa"};
	};
	class pca_m88_field_cdf_plum: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (CDF Plum)";
		uniformClass = "pca_uniform_m88_field_cdf_plum";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_cdf_plum_co.paa"};
	};
	class pca_m88_field_cdf_ttsko_mountain: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Mountain)";
		uniformClass = "pca_uniform_m88_field_cdf_ttsko_mountain";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_cdf_ttsko_mountain_co.paa"};
	};
	class pca_m88_field_cdf_wdl: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (CDF Woodland)";
		uniformClass = "pca_uniform_m88_field_cdf_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_cdf_wdl_co.paa"};
	};
	class pca_m88_field_flora: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Flora)";
		uniformClass = "pca_uniform_m88_field_flora";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_flora_co.paa"};
	};
	class pca_m88_field_khk: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Khaki)";
		uniformClass = "pca_uniform_m88_field_khk";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_khk_co.paa"};
	};
	class pca_m88_field_kamysh_mixed_1: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Kamysh Mixed 1)";
		uniformClass = "pca_uniform_m88_field_kamysh_mixed_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_kamysh_mixed_1_co.paa"};
	};
	class pca_m88_field_kamysh_mixed_2: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Kamysh Mixed 2)";
		uniformClass = "pca_uniform_m88_field_kamysh_mixed_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_kamysh_mixed_2_co.paa"};
	};
	class pca_m88_field_kamysh_urb: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Kamysh Urban)";
		uniformClass = "pca_uniform_m88_field_kamysh_urb";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_kamysh_urb_co.paa"};
	};
	class pca_m88_field_kamysh_wdl: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Kamysh Woodland)";
		uniformClass = "pca_uniform_m88_field_kamysh_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_kamysh_wdl_co.paa"};
	};
	class pca_m88_field_mgrn: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Military Green)";
		uniformClass = "pca_uniform_m88_field_mgrn";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_mgrn_co.paa"};
	};
	class pca_m88_field_od: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Olive Drab)";
		uniformClass = "pca_uniform_m88_field_od";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_od_co.paa"};
	};
	class pca_m88_field_oli: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Olive Green)";
		uniformClass = "pca_uniform_m88_field_oli";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_olive_co.paa"};
	};
	class pca_m88_field_spetsodezhda: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Spetsodezhda)";
		uniformClass = "pca_uniform_m88_field_spetsodezhda";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_spetsodezhda_co.paa"};
	};
	class pca_m88_field_tan: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Tan)";
		uniformClass = "pca_uniform_m88_field_tan";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_tan_co.paa"};
	};
	class pca_m88_field_tigr_wdl: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Tigr Woodland)";
		uniformClass = "pca_uniform_m88_field_tigr_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_tigr_wdl_co.paa"};
	};
	class pca_m88_field_ttsko_desat: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (TTsKO Desaturated)";
		uniformClass = "pca_uniform_m88_field_ttsko_desat";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_ttsko_desat_co.paa"};
	};
	class pca_m88_field_ttsko_mountain: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (TTsKO Mountain)";
		uniformClass = "pca_uniform_m88_field_ttsko_mountain";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_ttsko_mountain_co.paa"};
	};
	class pca_m88_field_vsr_1: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (VSR 1)";
		uniformClass = "pca_uniform_m88_field_vsr_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_vsr_1_co.paa"};
	};
	class pca_m88_field_vsr_2: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (VSR 2)";
		uniformClass = "pca_uniform_m88_field_vsr_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_vsr_2_co.paa"};
	};
	class pca_m88_field_wdl: rhs_msv_rifleman_patchless
	{
		scope = 1;
		displayName = "[RU] M88 Field Uniform (Woodland)";
		uniformClass = "pca_uniform_m88_field_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\m88\m88_wdl_co.paa"};
	};
	class rhs_klmk_oversuit_base;
	class pca_klmk_oversuit_berezka_desert: rhs_klmk_oversuit_base
	{
		displayName = "[RU] KLMK Oversuit (Berezka Desert)";
		uniformClass = "pca_uniform_klmk_oversuit_berezka_desert";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_vest_berezka_desert_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_pants_berezka_desert_co.paa"};
	};
	class pca_klmk_oversuit_berezka_winter: rhs_klmk_oversuit_base
	{
		displayName = "[RU] KLMK Oversuit (Berezka Winter)";
		uniformClass = "pca_uniform_klmk_oversuit_berezka_winter";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_vest_berezka_winter_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_pants_berezka_winter_co.paa"};
	};
	class pca_klmk_oversuit_frogskin: rhs_klmk_oversuit_base
	{
		displayName = "[RU] KLMK Oversuit (Winter)";
		uniformClass = "pca_uniform_klmk_oversuit_frogskin";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_vest_frogskin_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_pants_frogskin_co.paa"};
	};
	class pca_klmk_oversuit_white: rhs_klmk_oversuit_base
	{
		displayName = "[RU] KLMK Oversuit (Winter)";
		uniformClass = "pca_uniform_klmk_oversuit_white";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_vest_snow_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\oversuit_pants_snow_co.paa"};
	};
	class rhs_rd54;
	class pca_rd54_blk: rhs_rd54
	{
		displayName = "[RU] RD54 (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_black_co.paa"};
	};
	class pca_rd54_brn: rhs_rd54
	{
		displayName = "[RU] RD54 (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_brown_co.paa"};
	};
	class pca_rd54_flora: rhs_rd54
	{
		displayName = "[RU] RD54 (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_flora_co.paa"};
	};
	class pca_rd54_od: rhs_rd54
	{
		displayName = "[RU] RD54 (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_od_co.paa"};
	};
	class pca_rd54_mgrn: rhs_rd54
	{
		displayName = "[RU] RD54 (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_green_co.paa"};
	};
	class pca_rd54_oli: rhs_rd54
	{
		displayName = "[RU] RD54 (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_oli_co.paa"};
	};
	class pca_rd54_tan: rhs_rd54
	{
		displayName = "[RU] RD54 (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\rd54_tan_co.paa"};
	};
	class rhs_assault_umbts;
	class pca_umbts_flora: rhs_assault_umbts
	{
		displayName = "[RU] UMBTS (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\umbts_flora_co.paa"};
	};
};
class ItemInfo;
class CfgWeapons
{
	class ItemCore;
	class Vest_Camo_Base: ItemCore
	{
		class ItemInfo;
	};
	class rhs_uniform_afghanka;
	class pca_uniform_afghanka: rhs_uniform_afghanka
	{
		scope = 2;
		displayName = "[RU] M88 Uniform (Olive Drab)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_alpenflage: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Alpenflage)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_alpenflage";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_brn: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Brown)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_brn";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_butan_autumn: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Butan Autumn)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_butan_autumn";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_cdf_ttsko_desert: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (CDF TTsKO Desert)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_cdf_ttsko_desert";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_cdf_ttsko_mountain: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (CDF TTsKO Mountain)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_cdf_ttsko_mountain";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_cdf_wdl: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (CDF Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_cdf_wdl";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_gray: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Gray)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_gray";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_kamysh_leska_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Kamysh/Leska 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_kamysh_leska_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_kamysh_leska_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Kamysh/Leska 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_kamysh_leska_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_kamysh_mixed_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Kamysh Mixed 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_kamysh_mixed_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_kamysh_mixed_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Kamysh Mixed 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_kamysh_mixed_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_kamysh_urb: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Kamysh Urban)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_kamysh_urb";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_kamysh_wdl: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Kamysh Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_kamysh_wdl";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_klmk_mountain: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (KLMK Mountain)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_klmk_mountain";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_leska: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Leska)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_leska";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_leska_kamysh_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Leska/Kamysh 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_leska_kamysh_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_leska_kamysh_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Leska/Kamysh 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_leska_kamysh_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_m89_oakleaf: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (M89 Oakleaf)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_m89_oakleaf";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_oli: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Olive Green)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_oli";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_orel: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Orel)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_orel";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_plum: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Plum)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_plum";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_puma: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Puma)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_puma";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_smk_urb_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (SMK Urban 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_smk_urb_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_smk_urb_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (SMK Urban 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_smk_urb_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_smk_wdl_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_smk_wdl_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_smk_wdl_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_smk_wdl_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_smk_wdl_3: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 3)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_smk_wdl_3";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_smk_wdl_4: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (SMK Woodland 4)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_smk_wdl_4";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_spetsodezhda: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Spetsodezhda)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_spetsodezhda";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_spetsodezhda_od: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Spetsodezhda Olive Drab)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_spetsodezhda_od";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_strichtarn: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Strichtarn)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_strichtarn";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_swirl: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Swirl)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_swirl";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_taki_lizard: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Takistan Lizard)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_taki_lizard";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_taki_lizard_od_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Takistan Lizard/Olive Drab 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_taki_lizard_od_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_taki_lizard_od_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Takistan Lizard/Olive Drab 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_taki_lizard_od_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tan: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tan)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tan";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tigr_desert: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tigr Desert)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tigr_desert";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tigr_urb_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tigr Urban 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tigr_urb_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tigr_urb_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tigr Urban 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tigr_urb_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tigr_wdl_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tigr Woodland 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tigr_wdl_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tigr_wdl_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tigr Woodland 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tigr_wdl_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_tigr_wdl_3: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Tigr Woodland 3)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_tigr_wdl_3";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_ttsko: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (TTsKO)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_ttsko";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_ttsko_forest: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (TTsKO Forest)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_ttsko_forest";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vsr_1: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (VSR 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vsr_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vsr_2: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (VSR 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vsr_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vsr_3: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (VSR 3)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vsr_3";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vsr_4: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (VSR 4)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vsr_4";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vsr_5: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (VSR 5)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vsr_5";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_yugo_m68: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform (Yugoslavia M68)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_yugo_m68";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhs_uniform_afghanka_winter_moldovan_ttsko;
	class pca_uniform_afghanka_winter_spetsodezhda_od: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (Spetsodezhda Olive Drab)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_spetsodezhda_od";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_smk_urb_1: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (SMK Urban 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_smk_urb_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_smk_urb_2: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (SMK Urban 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_smk_urb_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_smk_wdl_1: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_smk_wdl_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_smk_wdl_2: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_smk_wdl_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_smk_wdl_3: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 3)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_smk_wdl_3";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_smk_wdl_4: rhs_uniform_afghanka_winter_moldovan_ttsko
	{
		displayName = "[RU] M88 Uniform Winter (SMK Woodland 4)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_smk_wdl_4";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_tigr_urb_1: pca_uniform_afghanka_winter_spetsodezhda_od
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Urban 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_tigr_urb_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_tigr_urb_2: pca_uniform_afghanka_winter_spetsodezhda_od
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Urban 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_tigr_urb_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_tigr_wdl_1: pca_uniform_afghanka_winter_spetsodezhda_od
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Woodland 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_tigr_wdl_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_tigr_wdl_2: pca_uniform_afghanka_winter_spetsodezhda_od
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Woodland 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_tigr_wdl_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_tigr_wdl_3: pca_uniform_afghanka_winter_spetsodezhda_od
	{
		displayName = "[RU] M88 Uniform Winter (Tigr Woodland 3)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_tigr_wdl_3";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_ttsko: pca_uniform_afghanka_winter_spetsodezhda_od
	{
		displayName = "[RU] M88 Uniform Winter (TTsKO)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_ttsko";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_winter_ttsko_forest: pca_uniform_afghanka
	{
		displayName = "[RU] M88 Uniform Winter (TTsKO Forest)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_winter_ttsko_forest";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhs_uniform_afghanka_para_vsr;
	class pca_uniform_afghanka_vdv_granite_tan: rhs_uniform_afghanka_para_vsr
	{
		displayName = "[RU] M88 Uniform VDV (Granite Tan)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_granite_tan";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_cdf_ttsko_autumn: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (CDF TTsKO Autumn)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_cdf_ttsko_autumn";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_cdf_ttsko_forest: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (CDF TTsKO Forest)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_cdf_ttsko_forest";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_cdf_plum: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (CDF Plum)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_cdf_plum";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_ttsko: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (TTsKO)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_ttsko";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_ttsko_oxblood: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (TTsKO Oxblood)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_ttsko_oxblood";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_vsr_1: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (VSR 1)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_vsr_1";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_vsr_2: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (VSR 2)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_vsr_2";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_afghanka_vdv_vsr_3: pca_uniform_afghanka_vdv_granite_tan
	{
		displayName = "[RU] M88 Uniform VDV (VSR 3)";
		class ItemInfo: ItemInfo
		{
			uniformClass = "pca_afghanka_vdv_vsr_3";
			uniformModel = "-";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhs_uniform_flora;
	class pca_uniform_m88_field_brn: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Brown)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_brn";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_cdf_ttsko_autumn: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Autumn)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_cdf_ttsko_autumn";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_cdf_ttsko_desert: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Desert)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_cdf_ttsko_desert";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_cdf_ttsko_forest: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Forest)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_cdf_ttsko_forest";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_cdf_plum: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (CDF Plum)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_cdf_plum";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_cdf_ttsko_mountain: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (CDF TTsKO Mountain)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_cdf_ttsko_mountain";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_cdf_wdl: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (CDF Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_cdf_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_flora: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Flora)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_flora";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_khk: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Khaki)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_khk";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_kamysh_mixed_1: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Kamysh Mixed 1)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_kamysh_mixed_1";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_kamysh_mixed_2: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Kamysh Mixed 2)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_kamysh_mixed_2";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_kamysh_urb: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Kamysh Urban)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_kamysh_urb";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_kamysh_wdl: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Kamysh Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_kamysh_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_mgrn: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Military Green)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_mgrn";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_od: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Olive Drab)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_od";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_oli: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Olive Green)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_oli";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_spetsodezhda: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Spetsodezhda)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_spetsodezhda";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_tan: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Tan)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_tan";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_tigr_wdl: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Tigr Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_tigr_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_ttsko_desat: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (TTsKO Desaturated)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_ttsko_desat";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_ttsko_mountain: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (TTsKO Mountain)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_ttsko_mountain";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_vsr_1: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (VSR 1)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_vsr_1";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_vsr_2: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (VSR 2)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_vsr_2";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m88_field_wdl: rhs_uniform_flora
	{
		displayName = "[RU] M88 Field Uniform (Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m88_field_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhs_uniform_gorka_r_y;
	class pca_uniform_gorka_cdf_ttsko_winter: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (CDF TTsKO Winter)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_cdf_ttsko_winter";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_cdf_ttsko_winter_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (CDF TTsKO Winter/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_cdf_ttsko_winter_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_emr: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (EMR)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_emr";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_emr_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (EMR/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_emr_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_flecktarn: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Flecktarn)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_flecktarn";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_flecktarn_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Flecktarn/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_flecktarn_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_kamysh_urb: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Kamysh Urban)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_kamysh_urb";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_kamysh_urb_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Kamysh Urban/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_kamysh_urb_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_kamysh_wdl: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Kamysh Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_kamysh_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_kamysh_wdl_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Kamysh Woodland/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_kamysh_wdl_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_leto: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Leto)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_leto";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_leto_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Leto/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_leto_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_smk_urb: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (SMK Urban)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_smk_urb";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_smk_urb_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (SMK Urban/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_smk_urb_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_smk_wdl: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (SMK Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_smk_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_smk_wdl_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (SMK Woodland/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_smk_wdl_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_tigr_urb: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Tigr Urban)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_tigr_urb";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_tigr_urb_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Tigr Urban/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_tigr_urb_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_tigr_wdl: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Tigr Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_tigr_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_tigr_wdl_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Tigr Woodland/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_tigr_wdl_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_wdl: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_gorka_wdl_gloves: rhs_uniform_gorka_r_y
	{
		displayName = "[RU] Gorka Suit (Woodland/Gloves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_gorka_wdl_gloves";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhs_uniform_klmk_oversuit;
	class pca_uniform_klmk_oversuit_berezka_desert: rhs_uniform_klmk_oversuit
	{
		scope = 2;
		author = "Red Hammer Studios";
		displayName = "[RU] KLMK Oversuit (Berezka Desert)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_klmk_oversuit_berezka_desert";
			containerClass = "Supply50";
			mass = 40;
		};
	};
	class pca_uniform_klmk_oversuit_berezka_winter: rhs_uniform_klmk_oversuit
	{
		scope = 2;
		author = "Red Hammer Studios";
		displayName = "[RU] KLMK Oversuit (Berezka Winter)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_klmk_oversuit_berezka_winter";
			containerClass = "Supply50";
			mass = 40;
		};
	};
	class pca_uniform_klmk_oversuit_frogskin: rhs_uniform_klmk_oversuit
	{
		scope = 2;
		author = "Red Hammer Studios";
		displayName = "[RU] KLMK Oversuit (Frogskin)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_klmk_oversuit_frogskin";
			containerClass = "Supply50";
			mass = 40;
		};
	};
	class pca_uniform_klmk_oversuit_white: rhs_uniform_klmk_oversuit
	{
		scope = 2;
		author = "Red Hammer Studios";
		displayName = "[RU] KLMK Oversuit (Winter)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_klmk_oversuit_white";
			containerClass = "Supply50";
			mass = 40;
		};
	};
	class NVGoggles;
	class pca_nvg_gloves_wool: NVGoggles
	{
		scope = 2;
		displayName = "Woolen Gloves (Black)";
		picture = "x\pca\custom\addons\blended_rus_gear\data\tex\woolen_gloves_ca.paa";
		model = "x\pca\custom\addons\blended_rus_gear\wool_gloves.p3d";
		visionMode[] = {"Normal","Normal"};
		hiddenSelections[] = {"camo"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\woolen_gloves_co.paa"};
		identityTypes[] = {};
		class ItemInfo: ItemInfo
		{
			type = 616;
			uniformModel = "x\pca\custom\addons\blended_rus_gear\wool_gloves.p3d";
			modelOff = "x\pca\custom\addons\blended_rus_gear\wool_gloves.p3d";
			hiddenSelections[] = {"camo"};
		};
	};
	class pca_nvg_gloves_wool_blu: pca_nvg_gloves_wool
	{
		displayName = "Woolen Gloves (Blue)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\woolen_gloves_blue_co.paa"};
	};
	class pca_nvg_gloves_wool_brn: pca_nvg_gloves_wool
	{
		displayName = "Woolen Gloves (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\woolen_gloves_brown_co.paa"};
	};
	class pca_nvg_gloves_wool_gry: pca_nvg_gloves_wool
	{
		displayName = "Woolen Gloves (Gray)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\woolen_gloves_gray_co.paa"};
	};
	class pca_nvg_gloves_wool_grn: pca_nvg_gloves_wool
	{
		displayName = "Woolen Gloves (Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\woolen_gloves_green_co.paa"};
	};
	class rhs_6b3;
	class rhs_6b3_AK;
	class rhs_6b3_AK_2;
	class rhs_6b3_AK_3;
	class rhs_6b3_holster;
	class rhs_6b3_off;
	class rhs_6b3_R148;
	class rhs_6b3_RPK;
	class rhs_6b3_VOG;
	class rhs_6b3_VOG_2;
	class rhs_6b3_brn: rhs_6b3
	{
		displayName = "[RU] 6B3 Vest (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_brn_ak_1: rhs_6b3_AK
	{
		displayName = "[RU] 6B3 Vest (Brown/AK 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_brn_ak_2: rhs_6b3_AK_2
	{
		displayName = "[RU] 6B3 Vest (Brown/AK 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_brn_ak_3: rhs_6b3_AK_3
	{
		displayName = "[RU] 6B3 Vest (Brown/AK 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_brn_holster: rhs_6b3_holster
	{
		displayName = "[RU] 6B3 Vest (Brown/Holster)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\off_light_co.paa"};
	};
	class rhs_6b3_brn_officer: rhs_6b3_off
	{
		displayName = "[RU] 6B3 Vest (Brown/Officer)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\off_light_co.paa"};
	};
	class rhs_6b3_brn_r148: rhs_6b3_R148
	{
		displayName = "[RU] 6B3 Vest (Brown/R-148)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\r148_co.paa"};
	};
	class rhs_6b3_brn_rpk: rhs_6b3_RPK
	{
		displayName = "[RU] 6B3 Vest (Brown/RPK)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_brn_vog_1: rhs_6b3_VOG
	{
		displayName = "[RU] 6B3 Vest (Brown/VOG 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa","rhsafrf\addons\rhs_infantry3\data\vog_pouchs_co.paa"};
	};
	class rhs_6b3_brn_vog_2: rhs_6b3_VOG_2
	{
		displayName = "[RU] 6B3 Vest (Brown/VOG 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_brown_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa","rhsafrf\addons\rhs_infantry3\data\vog_pouchs_co.paa"};
	};
	class rhs_6b3_od: rhs_6b3
	{
		displayName = "[RU] 6B3 Vest (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_od_ak_1: rhs_6b3_AK
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/AK 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_od_ak_2: rhs_6b3_AK_2
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/AK 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_od_ak_3: rhs_6b3_AK_3
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/AK 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_od_holster: rhs_6b3_holster
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/Holster)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\off_light_co.paa"};
	};
	class rhs_6b3_od_officer: rhs_6b3_off
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/Officer)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\off_light_co.paa"};
	};
	class rhs_6b3_od_r148: rhs_6b3_R148
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/R-148)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\r148_co.paa"};
	};
	class rhs_6b3_od_rpk: rhs_6b3_RPK
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/RPK)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa"};
	};
	class rhs_6b3_od_vog_1: rhs_6b3_VOG
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/VOG 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa","rhsafrf\addons\rhs_infantry3\data\vog_pouchs_co.paa"};
	};
	class rhs_6b3_od_vog_2: rhs_6b3_VOG_2
	{
		displayName = "[RU] 6B3 Vest (Olive Drab/VOG 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b3_vest_od_co.paa","rhsafrf\addons\rhs_infantry3\data\gear_ak_co.paa","rhsafrf\addons\rhs_infantry3\data\vog_pouchs_co.paa"};
	};
	class rhs_6b5_vsr;
	class rhs_6b5_rifleman_vsr;
	class rhs_6b5_medic_vsr;
	class rhs_6b5_officer_vsr;
	class rhs_6b5_sniper_vsr;
	class pca_6b5_brn: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa"};
	};
	class pca_6b5_rifleman_brn: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa"};
	};
	class pca_6b5_medic_brn: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa"};
	};
	class pca_6b5_officer_brn: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa"};
	};
	class pca_6b5_sniper_brn: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_brown_co.paa"};
	};
	class pca_6b5_khk: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa"};
	};
	class pca_6b5_rifleman_khk: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa"};
	};
	class pca_6b5_medic_khk: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa"};
	};
	class pca_6b5_officer_khk: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa"};
	};
	class pca_6b5_sniper_khk: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Khaki)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_khaki_co.paa"};
	};
	class pca_6b5_mgrn: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa"};
	};
	class pca_6b5_rifleman_mgrn: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa"};
	};
	class pca_6b5_medic_mgrn: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa"};
	};
	class pca_6b5_officer_mgrn: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa"};
	};
	class pca_6b5_sniper_mgrn: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_green_co.paa"};
	};
	class pca_6b5_plum: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Plum)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa"};
	};
	class pca_6b5_rifleman_plum: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Plum)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa"};
	};
	class pca_6b5_medic_plum: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Plum)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa"};
	};
	class pca_6b5_officer_plum: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Plum)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa"};
	};
	class pca_6b5_sniper_plum: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Plum)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_plum_co.paa"};
	};
	class pca_6b5_oli: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa"};
	};
	class pca_6b5_rifleman_oli: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa"};
	};
	class pca_6b5_medic_oli: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa"};
	};
	class pca_6b5_officer_oli: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa"};
	};
	class pca_6b5_sniper_oli: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_olive_co.paa"};
	};
	class pca_6b5_spetsodezhda: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa"};
	};
	class pca_6b5_rifleman_spetsodezhda: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa"};
	};
	class pca_6b5_medic_spetsodezhda: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa"};
	};
	class pca_6b5_officer_spetsodezhda: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa"};
	};
	class pca_6b5_sniper_spetsodezhda: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_spetsodezhda_co.paa"};
	};
	class pca_6b5_tan: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa"};
	};
	class pca_6b5_rifleman_tan: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa"};
	};
	class pca_6b5_medic_tan: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa"};
	};
	class pca_6b5_officer_tan: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa"};
	};
	class pca_6b5_sniper_tan: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_tan_co.paa"};
	};
	class pca_6b5_ttsko: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (TTsKO)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa"};
	};
	class pca_6b5_rifleman_ttsko: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (TTsKO)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa"};
	};
	class pca_6b5_medic_ttsko: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (TTsKO)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa"};
	};
	class pca_6b5_officer_ttsko: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (TTsKO)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa"};
	};
	class pca_6b5_sniper_ttsko: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (TTsKO)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_co.paa"};
	};
	class pca_6b5_ttsko_forest: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa"};
	};
	class pca_6b5_rifleman_ttsko_forest: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa"};
	};
	class pca_6b5_medic_ttsko_forest: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa"};
	};
	class pca_6b5_officer_ttsko_forest: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa"};
	};
	class pca_6b5_sniper_ttsko_forest: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_ttsko_forest_co.paa"};
	};
	class pca_6b5_vsr_1: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa"};
	};
	class pca_6b5_rifleman_vsr_1: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa"};
	};
	class pca_6b5_medic_vsr_1: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa"};
	};
	class pca_6b5_officer_vsr_1: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa"};
	};
	class pca_6b5_sniper_vsr_1: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_1_co.paa"};
	};
	class pca_6b5_vsr_2: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa"};
	};
	class pca_6b5_rifleman_vsr_2: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa"};
	};
	class pca_6b5_medic_vsr_2: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa"};
	};
	class pca_6b5_officer_vsr_2: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa"};
	};
	class pca_6b5_sniper_vsr_2: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_2_co.paa"};
	};
	class pca_6b5_vsr_3: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa"};
	};
	class pca_6b5_rifleman_vsr_3: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa"};
	};
	class pca_6b5_medic_vsr_3: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa"};
	};
	class pca_6b5_officer_vsr_3: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa"};
	};
	class pca_6b5_sniper_vsr_3: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_3_co.paa"};
	};
	class pca_6b5_vsr_4: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa"};
	};
	class pca_6b5_rifleman_vsr_4: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa"};
	};
	class pca_6b5_medic_vsr_4: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa"};
	};
	class pca_6b5_officer_vsr_4: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa"};
	};
	class pca_6b5_sniper_vsr_4: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_4_co.paa"};
	};
	class pca_6b5_vsr_5: rhs_6b5_vsr
	{
		displayName = "[RU] 6B5 Vest (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa"};
	};
	class pca_6b5_rifleman_vsr_5: rhs_6b5_rifleman_vsr
	{
		displayName = "[RU] 6B5 Vest Rifleman (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa"};
	};
	class pca_6b5_medic_vsr_5: rhs_6b5_medic_vsr
	{
		displayName = "[RU] 6B5 Vest Medic (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa"};
	};
	class pca_6b5_officer_vsr_5: rhs_6b5_officer_vsr
	{
		displayName = "[RU] 6B5 Vest Officer (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa"};
	};
	class pca_6b5_sniper_vsr_5: rhs_6b5_sniper_vsr
	{
		displayName = "[RU] 6B5 Vest Sniper (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b5_vest_vsr_5_co.paa"};
	};
	class rhs_6b13_6sh92;
	class rhs_6b13_6sh92_radio;
	class rhs_6b13_6sh92_vog;
	class pca_6b13_6sh92_flora: rhs_6b13_6sh92
	{
		displayName = "[RU] 6B13 6Sh92 (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b13_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b13_6sh92_flora_radio: rhs_6b13_6sh92_radio
	{
		displayName = "[RU] 6B13 6Sh92 (Flora/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b13_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b13_6sh92_flora_vog: rhs_6b13_6sh92_vog
	{
		displayName = "[RU] 6B13 6Sh92 (Flora/VOG)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b13_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class rhs_6b23_digi_6sh92;
	class rhs_6b23_digi_6sh92_headset;
	class rhs_6b23_digi_6sh92_headset_spetsnaz;
	class rhs_6b23_digi_6sh92_radio;
	class rhs_6b23_digi_6sh92_spetsnaz;
	class rhs_6b23_digi_6sh92_spetsnaz2;
	class rhs_6b23_digi_6sh92_vog;
	class rhs_6b23_digi_6sh92_vog_headset;
	class rhs_6b23_digi_6sh92_vog_radio_spetsnaz;
	class rhs_6b23_digi_6sh92_vog_spetsnaz;
	class pca_6b23_6sh92_flora: rhs_6b23_digi_6sh92
	{
		displayName = "[RU] 6B23 6Sh92 (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_headset: rhs_6b23_digi_6sh92_headset
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/Headset)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_headset_spetsnaz: rhs_6b23_digi_6sh92_headset_spetsnaz
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/Headset/Spetsnaz)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_radio: rhs_6b23_digi_6sh92_radio
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_radio_spetsnaz: rhs_6b23_digi_6sh92_spetsnaz
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/Radio/Spetsnaz)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_spetsnaz: rhs_6b23_digi_6sh92_spetsnaz2
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/Spetsnaz)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_vog: rhs_6b23_digi_6sh92_vog
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/VOG)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_vog_headset: rhs_6b23_digi_6sh92_vog_headset
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/VOG/Headset)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_vog_radio: rhs_6b23_digi_6sh92_vog_radio_spetsnaz
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/VOG/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class pca_6b23_6sh92_flora_vog_spetsnaz: rhs_6b23_digi_6sh92_vog_spetsnaz
	{
		displayName = "[RU] 6B23 6Sh92 (Flora/VOG/Spetsnaz)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh92_flora_co.paa"};
	};
	class rhs_6b23_6sh116;
	class rhs_6b23_6sh116_vog;
	class pca_6b23_6sh116_flora: rhs_6b23_6sh116
	{
		displayName = "[RU] 6B23 6Sh116 (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh116_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_gear_flora_co.paa"};
	};
	class pca_6b23_6sh116_flora_vog: rhs_6b23_6sh116_vog
	{
		displayName = "[RU] 6B23 6Sh116 (Flora/VOG)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\6b23_vest_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_6sh116_flora_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\6b23_gear_flora_co.paa"};
	};
	class rhs_6b47;
	class rhs_6b47_emr: rhs_6b47
	{
		class ItemInfo;
	};
	class pca_6b47_emr_1: rhs_6b47_emr
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B47 (EMR Cover 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b47_co.paa"};
	};
	class rhs_6b47_emr_1: rhs_6b47
	{
		class ItemInfo;
	};
	class pca_6b47_emr_2: rhs_6b47_emr_1
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B47 (EMR Cover 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b47_1_co.paa"};
	};
	class rhs_6b47_emr_2: rhs_6b47
	{
		class ItemInfo;
	};
	class pca_6b47_emr_3: rhs_6b47_emr_2
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B47 (EMR Cover 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b47_2_co.paa"};
	};
	class rhs_6b47_6B50: rhs_6b47
	{
		class ItemInfo;
	};
	class pca_6b47_emr_ess: rhs_6b47_6B50
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B47 (EMR Cover/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b47_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b47_ess_cover_co.paa"};
	};
	class rhs_6b43;
	class rhs_6b45: rhs_6b43
	{
		class ItemInfo;
	};
	class pca_6b45: rhs_6b45
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa"};
	};
	class rhs_6b45_holster: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_holster: rhs_6b45_holster
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Holster)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6b45_light: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_light: rhs_6b45_light
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Light)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa"};
	};
	class rhs_6b45_grn: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_grn: rhs_6b45_grn
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6b45_mg: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_mg: rhs_6b45_mg
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6b45_off: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_off: rhs_6b45_off
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Officer)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_radio_co.paa"};
		class ItemInfo: ItemInfo
		{
			hiddenSelections[] = {"Camo","Camo1","Camo2","Camo3"};
		};
	};
	class rhs_6b45_rifleman: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_rifleman: rhs_6b45_rifleman
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Rifleman 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6b45_rifleman_2: rhs_6b45
	{
		class ItemInfo;
	};
	class pca_6b45_rifleman_2: rhs_6b45_rifleman_2
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6B45 (EMR/Rifleman 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6sh117_rifleman: Vest_Camo_Base
	{
		class ItemInfo;
	};
	class pca_6sh117_rifleman: rhs_6sh117_rifleman
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6sh117_ar: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_ar: rhs_6sh117_ar
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/Automatic Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6sh117_grn: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_grn: rhs_6sh117_grn
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/Grenadier)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6sh117_mg: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_mg: rhs_6sh117_mg
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/Machine Gunner)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa"};
	};
	class rhs_6sh117_nco: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_officer: rhs_6sh117_nco
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/Officer)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_radio_co.paa"};
	};
	class rhs_6sh117_nco_azart: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_officer_azart: rhs_6sh117_nco_azart
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/Officer/Azart)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_radio_co.paa"};
	};
	class rhs_6sh117_svd: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_svd: rhs_6sh117_svd
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/SVD)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_radio_co.paa"};
	};
	class rhs_6sh117_val: rhs_6sh117_rifleman
	{
		class ItemInfo;
	};
	class pca_6sh117_val: rhs_6sh117_val
	{
		author = "Red Hammer Studios";
		displayName = "[RUAF] 6Sh117 (EMR/VAL)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_117_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6b45_118_co.paa","x\pca\custom\addons\blended_rus_gear\data\tex\modern\rhs_6sh117_radio_co.paa"};
	};
};
