////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_gear
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","rhs_c_troops","rhsgref_c_troops","rhsusf_c_troops","rhssaf_c_troops"};
		units[] = {"pca_fieldpack_greek_lizard","pca_fieldpack_wdl","pca_kitbag_wdl","pca_tacticalpack_3cd","pca_tacticalpack_dpm","pca_tacticalpack_dpm_desert","pca_tacticalpack_greek_lizard","pca_umbts_wdl","pca_alicepack_greek_lizard","pca_alicepack_wdl"};
		weapons[] = {"pca_combat_fatigue_irn_dpm","pca_combat_fatigue_rs_irn_dpm","pca_combat_fatigue_p87_wdl","pca_combat_fatigue_rs_p87_wdl","pca_combat_fatigue_type07_hex_des","pca_combat_fatigue_rs_type07_hex_des","pca_combat_fatigue_type07_hex_uni","pca_combat_fatigue_rs_type07_hex_uni","pca_combat_fatigue_type07_hex_wdl","pca_combat_fatigue_rs_type07_hex_wdl","pca_uniform_m93_field_brushstroke","pca_uniform_m93_field_dpm","pca_uniform_m93_field_greek_dgtl","pca_uniform_m93_field_greek_lizard_1","pca_uniform_m93_field_greek_lizard_2","pca_uniform_m93_field_greek_lizard_3","pca_uniform_m93_field_tubitak","pca_uniform_m93_field_type99_dl","pca_uniform_m93_field_wdl","pca_uniform_m10_atacs_au","pca_uniform_m10_atacs_fg","pca_uniform_m10_marpatwd","pca_uniform_m10_wdl"};
	};
};
class CfgVehicles
{
	class I_soldier_F;
	class I_Soldier_SL_F;
	class pca_combat_fatigue_irn_dpm: I_soldier_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Iranian DPM)";
		uniformClass = "pca_combat_fatigue_irn_dpm";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_irn_dpm_co.paa"};
	};
	class pca_combat_fatigue_rs_irn_dpm: I_Soldier_SL_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Iranian DPM/Rolled Sleeves)";
		uniformClass = "pca_combat_fatigue_rs_irn_dpm";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_irn_dpm_co.paa"};
	};
	class pca_combat_fatigue_p87_wdl: I_soldier_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Pattern 87 Woodland)";
		uniformClass = "pca_combat_fatigue_p87_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_p87_wdl_co.paa"};
	};
	class pca_combat_fatigue_rs_p87_wdl: I_Soldier_SL_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Pattern 87 Woodland/Rolled Sleeves)";
		uniformClass = "pca_combat_fatigue_rs_p87_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_p87_wdl_co.paa"};
	};
	class pca_combat_fatigue_type07_hex_des: I_soldier_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Desert)";
		uniformClass = "pca_combat_fatigue_type07_hex_des";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_t7_hex_des_co.paa"};
	};
	class pca_combat_fatigue_rs_type07_hex_des: I_Soldier_SL_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Desert/Rolled Sleeves)";
		uniformClass = "pca_combat_fatigue_rs_type07_hex_des";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_t7_hex_des_co.paa"};
	};
	class pca_combat_fatigue_type07_hex_uni: I_soldier_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Universal)";
		uniformClass = "pca_combat_fatigue_type07_hex_uni";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_t7_hex_uni_co.paa"};
	};
	class pca_combat_fatigue_rs_type07_hex_uni: I_Soldier_SL_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Universal/Rolled Sleeves)";
		uniformClass = "pca_combat_fatigue_rs_type07_hex_uni";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_t7_hex_uni_co.paa"};
	};
	class pca_combat_fatigue_type07_hex_wdl: I_soldier_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Woodland)";
		uniformClass = "pca_combat_fatigue_type07_hex_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_t7_hex_wdl_co.paa"};
	};
	class pca_combat_fatigue_rs_type07_hex_wdl: I_Soldier_SL_F
	{
		scope = 1;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Woodland/Rolled Sleeves)";
		uniformClass = "pca_combat_fatigue_rs_type07_hex_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\combat_fatigues_t7_hex_wdl_co.paa"};
	};
	class rhssaf_army_m10_para_rifleman_m21;
	class pca_m10_atacs_au: rhssaf_army_m10_para_rifleman_m21
	{
		scope = 1;
		displayName = "M10 Combat Uniform (ATACS AU)";
		uniformClass = "pca_uniform_m10_atacs_au";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m10_atacs_au_co.paa"};
	};
	class pca_m10_atacs_fg: rhssaf_army_m10_para_rifleman_m21
	{
		scope = 1;
		displayName = "M10 Combat Uniform (ATACS FG)";
		uniformClass = "pca_uniform_m10_atacs_fg";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m10_atacs_fg_co.paa"};
	};
	class pca_m10_marpatwd: rhssaf_army_m10_para_rifleman_m21
	{
		scope = 1;
		displayName = "M10 Combat Uniform (Marpat Woodland)";
		uniformClass = "pca_uniform_m10_marpatwd";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m10_marpatwd_co.paa"};
	};
	class pca_m10_wdl: rhssaf_army_m10_para_rifleman_m21
	{
		scope = 1;
		displayName = "M10 Combat Uniform (Woodland)";
		uniformClass = "pca_uniform_m10_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m10_wdl_co.paa"};
	};
	class rhsgref_hidf_ERDL;
	class pca_m93_field_brushstroke: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Brushstroke)";
		uniformClass = "pca_uniform_m93_field_brushstroke";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_brushstroke_co.paa"};
	};
	class pca_m93_field_dpm: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (DPM)";
		uniformClass = "pca_uniform_m93_field_dpm";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_dpm_co.paa"};
	};
	class pca_m93_field_greek_dgtl: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Greek Digital)";
		uniformClass = "pca_m93_field_greek_dgtl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_greek_dgtl_co.paa"};
	};
	class pca_m93_field_greek_lizard_1: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Greek Lizard 1)";
		uniformClass = "pca_uniform_m93_field_greek_lizard_1";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_greek_lizard_1_co.paa"};
	};
	class pca_m93_field_greek_lizard_2: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Greek Lizard 2)";
		uniformClass = "pca_uniform_m93_field_greek_lizard_2";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_greek_lizard_2_co.paa"};
	};
	class pca_m93_field_greek_lizard_3: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Greek Lizard 3)";
		uniformClass = "pca_uniform_m93_field_greek_lizard_3";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_greek_lizard_3_co.paa"};
	};
	class pca_m93_field_tubitak: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Tubitak)";
		uniformClass = "pca_uniform_m93_field_tubitak";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_tubitak_co.paa"};
	};
	class pca_m93_field_type99_wdl: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Type 99 Woodland)";
		uniformClass = "pca_uniform_m93_field_type99_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_type99_wdl_co.paa"};
	};
	class pca_m93_field_wdl: rhsgref_hidf_ERDL
	{
		scope = 1;
		displayName = "M93 Field Uniform (Woodland)";
		uniformClass = "pca_uniform_m93_field_wdl";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\m93_wdl_co.paa"};
	};
	class B_FieldPack_oli;
	class pca_fieldpack_greek_lizard: B_FieldPack_oli
	{
		scope = 2;
		displayName = "Field Pack (Greek Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\fieldpack_greek_lizard_co.paa"};
	};
	class pca_fieldpack_wdl: B_FieldPack_oli
	{
		scope = 2;
		displayName = "Field Pack (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\fieldpack_wdl_co.paa"};
	};
	class B_Kitbag_mcamo;
	class pca_kitbag_wdl: B_Kitbag_mcamo
	{
		scope = 2;
		displayName = "Kitbag (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\kitbag_wdl_co.paa"};
	};
	class B_TacticalPack_mcamo;
	class pca_tacticalpack_3cd: B_TacticalPack_mcamo
	{
		scope = 2;
		displayName = "Tactical Backpack (3 Color Desert)";
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_backpack\data\rv\tactical.rvmat"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\tactical_3cd_co.paa"};
	};
	class pca_tacticalpack_dpm: B_TacticalPack_mcamo
	{
		scope = 2;
		displayName = "Tactical Backpack (DPM)";
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_backpack\data\rv\tactical.rvmat"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\tactical_dpm_co.paa"};
	};
	class pca_tacticalpack_dpm_desert: B_TacticalPack_mcamo
	{
		scope = 2;
		displayName = "Tactical Backpack (DPM Desert)";
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_backpack\data\rv\tactical.rvmat"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\tactical_dpm_desert_co.paa"};
	};
	class pca_tacticalpack_greek_lizard: B_TacticalPack_mcamo
	{
		scope = 2;
		displayName = "Tactical Backpack (Greek Lizard)";
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_backpack\data\rv\tactical.rvmat"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\tactical_greek_lizard_co.paa"};
	};
	class rhsusf_assault_eagleaiii_coy;
	class pca_umbts_wdl: rhsusf_assault_eagleaiii_coy
	{
		scope = 2;
		displayName = "UMBTS (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\umbts_wdl_co.paa"};
	};
	class rhsgref_wdl_alicepack;
	class pca_alicepack_greek_lizard: rhsgref_wdl_alicepack
	{
		scope = 2;
		displayName = "Alice Backpack (Greek Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\alicepack_greek_lizard_co.paa"};
	};
	class pca_alicepack_wdl: rhsgref_wdl_alicepack
	{
		scope = 2;
		displayName = "Alice Backpack (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\alicepack_wdl_co.paa"};
	};
};
class ItemInfo;
class CfgWeapons
{
	class U_I_CombatUniform;
	class U_I_CombatUniform_shortsleeve;
	class pca_combat_fatigue_irn_dpm: U_I_CombatUniform
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Iranian DPM)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_irn_dpm";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_rs_irn_dpm: U_I_CombatUniform_shortsleeve
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Iranian DPM/Rolled Sleeves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_rs_irn_dpm";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_p87_wdl: U_I_CombatUniform
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Pattern 87 Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_p87_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_rs_p87_wdl: U_I_CombatUniform_shortsleeve
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Pattern 87 Woodland/Rolled Sleeves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_rs_p87_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_type07_hex_des: U_I_CombatUniform
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Desert)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_type07_hex_des";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_rs_type07_hex_des: U_I_CombatUniform_shortsleeve
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Desert/Rolled Sleeves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_rs_type07_hex_des";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_type07_hex_uni: U_I_CombatUniform
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Universal)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_type07_hex_uni";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_rs_type07_hex_uni: U_I_CombatUniform_shortsleeve
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Universal/Rolled Sleeves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_rs_type07_hex_uni";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_type07_hex_wdl: U_I_CombatUniform
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_type07_hex_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_combat_fatigue_rs_type07_hex_wdl: U_I_CombatUniform_shortsleeve
	{
		scope = 2;
		displayName = "[CSAT] Combat Fatigue (Type 07 Hex Woodland/Rolled Sleeves)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_combat_fatigue_rs_type07_hex_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhsgref_uniform_ERDL;
	class pca_uniform_m93_field_brushstroke: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Brushstroke)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_brushstroke";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_dpm: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (DPM)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_dpm";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_greek_dgtl: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Greek Digital)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_greek_dgtl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_greek_lizard_1: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Greek Lizard 1)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_greek_lizard_1";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_greek_lizard_2: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Greek Lizard 2)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_greek_lizard_2";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_greek_lizard_3: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Greek Lizard 3)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_greek_lizard_3";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_tubitak: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Tubitak)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_tubitak";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_type99_dl: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Type 99 Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_type99_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m93_field_wdl: rhsgref_uniform_ERDL
	{
		scope = 2;
		displayName = "M93 Field Uniform (Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m93_field_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhssaf_uniform_m10_digital;
	class pca_uniform_m10_atacs_au: rhssaf_uniform_m10_digital
	{
		scope = 2;
		displayName = "M10 Combat Uniform (ATACS AU)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m10_atacs_au";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m10_atacs_fg: rhssaf_uniform_m10_digital
	{
		scope = 2;
		displayName = "M10 Combat Uniform (ATACS FG)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m10_atacs_fg";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m10_marpatwd: rhssaf_uniform_m10_digital
	{
		scope = 2;
		displayName = "M10 Combat Uniform (Marpat Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m10_marpatwd";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class pca_uniform_m10_wdl: rhssaf_uniform_m10_digital
	{
		scope = 2;
		displayName = "M10 Combat Uniform (Woodland)";
		class ItemInfo: ItemInfo
		{
			uniformModel = "-";
			uniformClass = "pca_m10_wdl";
			containerClass = "Supply40";
			mass = 40;
		};
	};
	class rhssaf_vest_md98_woodland;
	class rhssaf_vest_md98_rifleman;
	class pca_md98_3cd: rhssaf_vest_md98_woodland
	{
		displayName = "MD98 Vest (3CD)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_3cd_co.paa"};
	};
	class pca_md98_3cd_rifleman: rhssaf_vest_md98_rifleman
	{
		displayName = "MD98 Vest (3CD/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_3cd_co.paa"};
	};
	class pca_md98_blk: rhssaf_vest_md98_woodland
	{
		displayName = "MD98 Vest (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_black_co.paa"};
	};
	class pca_md98_blk_rifleman: rhssaf_vest_md98_rifleman
	{
		displayName = "MD98 Vest (Black/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_black_co.paa"};
	};
	class pca_md98_od: rhssaf_vest_md98_woodland
	{
		displayName = "MD98 Vest (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_od_co.paa"};
	};
	class pca_md98_od_rifleman: rhssaf_vest_md98_rifleman
	{
		displayName = "MD98 Vest (Olive Drab/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_od_co.paa"};
	};
	class pca_md98_wdl: rhssaf_vest_md98_woodland
	{
		displayName = "MD98 Vest (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_wdl_co.paa"};
	};
	class pca_md98_wdl_rifleman: rhssaf_vest_md98_rifleman
	{
		displayName = "MD98 Vest (Woodland/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md98_vest_wdl_co.paa"};
	};
	class rhssaf_vest_md12_digital;
	class rhssaf_vest_md12_m70_rifleman;
	class pca_md12_coy: rhssaf_vest_md12_digital
	{
		displayName = "MD12 Combat Vest (Coyote)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md12_vest_coy_co.paa"};
	};
	class pca_md12_coy_pouch_ak: rhssaf_vest_md12_m70_rifleman
	{
		displayName = "MD12 Combat Vest (Coyote/AK Pouch)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md12_vest_coy_co.paa","x\pca\custom\addons\blended_gear\data\tex\md12_pouch_ak_coy_co.paa"};
	};
	class pca_md12_wdl: rhssaf_vest_md12_digital
	{
		displayName = "MD12 Combat Vest (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md12_vest_wdl_co.paa"};
	};
	class pca_md12_wdl_pouch_ak: rhssaf_vest_md12_m70_rifleman
	{
		displayName = "MD12 Combat Vest (Woodland/AK Pouch)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md12_vest_wdl_co.paa","x\pca\custom\addons\blended_gear\data\tex\md12_pouch_ak_wdl_co.paa"};
	};
	class rhssaf_vest_md99_woodland;
	class rhssaf_vest_md99_woodland_radio;
	class rhssaf_vest_md99_woodland_rifleman;
	class rhssaf_vest_md99_woodland_rifleman_radio;
	class pca_md99_3cd: rhssaf_vest_md99_woodland
	{
		displayName = "MD99 Tactical Vest (3CD)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_3cd_co.paa"};
	};
	class pca_md99_3cd_radio: rhssaf_vest_md99_woodland_radio
	{
		displayName = "MD99 Tactical Vest (3CD/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_3cd_co.paa"};
	};
	class pca_md99_3cd_rifleman: rhssaf_vest_md99_woodland_rifleman
	{
		displayName = "MD99 Tactical Vest (3CD/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_3cd_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_3cd_co.paa"};
	};
	class pca_md99_3cd_rifleman_radio: rhssaf_vest_md99_woodland_rifleman_radio
	{
		displayName = "MD99 Tactical Vest (3CD/Rifleman/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_3cd_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_3cd_co.paa"};
	};
	class pca_md99_blk: rhssaf_vest_md99_woodland
	{
		displayName = "MD99 Tactical Vest (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_black_co.paa"};
	};
	class pca_md99_blk_radio: rhssaf_vest_md99_woodland_radio
	{
		displayName = "MD99 Tactical Vest (Black/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_black_co.paa"};
	};
	class pca_md99_blk_rifleman: rhssaf_vest_md99_woodland_rifleman
	{
		displayName = "MD99 Tactical Vest (Black/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_black_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_black_co.paa"};
	};
	class pca_md99_blk_rifleman_radio: rhssaf_vest_md99_woodland_rifleman_radio
	{
		displayName = "MD99 Tactical Vest (Black/Rifleman/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_black_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_black_co.paa"};
	};
	class pca_md99_od: rhssaf_vest_md99_woodland
	{
		displayName = "MD99 Tactical Vest (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_od_co.paa"};
	};
	class pca_md99_od_radio: rhssaf_vest_md99_woodland_radio
	{
		displayName = "MD99 Tactical Vest (Olive Drab/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_od_co.paa"};
	};
	class pca_md99_od_rifleman: rhssaf_vest_md99_woodland_rifleman
	{
		displayName = "MD99 Tactical Vest (Olive Drab/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_od_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_od_co.paa"};
	};
	class pca_md99_od_rifleman_radio: rhssaf_vest_md99_woodland_rifleman_radio
	{
		displayName = "MD99 Tactical Vest (Olive Drab/Rifleman/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_od_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_od_co.paa"};
	};
	class pca_md99_wdl: rhssaf_vest_md99_woodland
	{
		displayName = "MD99 Tactical Vest (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_wdl_co.paa"};
	};
	class pca_md99_wdl_radio: rhssaf_vest_md99_woodland_radio
	{
		displayName = "MD99 Tactical Vest (Woodland/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_wdl_co.paa"};
	};
	class pca_md99_wdl_rifleman: rhssaf_vest_md99_woodland_rifleman
	{
		displayName = "MD99 Tactical Vest (Woodland/Rifleman)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_wdl_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_wdl_co.paa"};
	};
	class pca_md99_wdl_rifleman_radio: rhssaf_vest_md99_woodland_rifleman_radio
	{
		displayName = "MD99 Tactical Vest (Woodland/Rifleman/Radio)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\md99_vest_wdl_co.paa","x\pca\custom\addons\blended_gear\data\tex\md98_vest_wdl_co.paa"};
	};
	class V_PlateCarrierIA1_dgtl;
	class pca_otv_greek_dgtl: V_PlateCarrierIA1_dgtl
	{
		displayName = "OTV Rig (Greek Digital)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\otv_greek_dgtl_co.paa"};
	};
	class pca_otv_greek_lizard: V_PlateCarrierIA1_dgtl
	{
		displayName = "OTV Rig (Greek Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\otv_greek_lizard_co.paa"};
	};
	class rhs_6b27m;
	class rhs_6b27m_ess;
	class pca_6b27m_irn_dpm: rhs_6b27m
	{
		displayName = "[CSAT] 6B27M (Iranian DPM)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_irn_dpm_co.paa"};
	};
	class pca_6b27m_irn_dpm_ess: rhs_6b27m_ess
	{
		displayName = "[CSAT] 6B27M (Iranian DPM/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_irn_dpm_co.paa"};
	};
	class pca_6b27m_p87_wdl: rhs_6b27m
	{
		displayName = "[CSAT] 6B27M (Pattern 87 Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_p87_wdl_co.paa"};
	};
	class pca_6b27m_p87_wdl_ess: rhs_6b27m_ess
	{
		displayName = "[CSAT] 6B27M (Pattern 87 Woodland/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_p87_wdl_co.paa"};
	};
	class pca_6b27m_type07_hex_des: rhs_6b27m
	{
		displayName = "[CSAT] 6B27M (Type 07 Hex Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_t7_hex_des_co.paa"};
	};
	class pca_6b27m_type07_hex_des_ess: rhs_6b27m_ess
	{
		displayName = "[CSAT] 6B27M (Type 07 Hex Desert/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_t7_hex_des_co.paa"};
	};
	class pca_6b27m_type07_hex_uni: rhs_6b27m
	{
		displayName = "[CSAT] 6B27M (Type 07 Hex Universal)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_t7_hex_uni_co.paa"};
	};
	class pca_6b27m_type07_hex_uni_ess: rhs_6b27m_ess
	{
		displayName = "[CSAT] 6B27M (Type 07 Hex Universal/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_t7_hex_uni_co.paa"};
	};
	class pca_6b27m_type07_hex_wdl: rhs_6b27m
	{
		displayName = "[CSAT] 6B27M (Type 07 Hex Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_t7_hex_wdl_co.paa"};
	};
	class pca_6b27m_type07_hex_wdl_ess: rhs_6b27m_ess
	{
		displayName = "[CSAT] 6B27M (Type 07 Hex Woodland/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\csat\6b27_t7_hex_wdl_co.paa"};
	};
	class rhssaf_helmet_m97_woodland;
	class rhssaf_helmet_m97_woodland_black_ess;
	class rhssaf_helmet_m97_woodland_black_ess_bare;
	class pca_m97_pasgt_greek_dgtl: rhssaf_helmet_m97_woodland
	{
		displayName = "M97 PASGT (Greek Digital)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_dgtl_co.paa"};
	};
	class pca_m97_pasgt_greek_dgtl_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "M97 PASGT (Greek Digital/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_dgtl_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_greek_dgtl_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "M97 PASGT (Greek Digital/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_dgtl_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_greek_lizard_1: rhssaf_helmet_m97_woodland
	{
		displayName = "M97 PASGT (Greek Lizard 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_lizard_1_co.paa"};
	};
	class pca_m97_pasgt_greek_lizard_1_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "M97 PASGT (Greek Lizard 1/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_lizard_1_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_greek_lizard_1_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "M97 PASGT (Greek Lizard 1/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_lizard_1_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_greek_lizard_2: rhssaf_helmet_m97_woodland
	{
		displayName = "M97 PASGT (Greek Lizard 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_lizard_2_co.paa"};
	};
	class pca_m97_pasgt_greek_lizard_2_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "M97 PASGT (Greek Lizard 2/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_lizard_2_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_greek_lizard_2_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "M97 PASGT (Greek Lizard 2/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_greek_lizard_2_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_wdl_1: rhssaf_helmet_m97_woodland
	{
		displayName = "M97 PASGT (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_wdl_co.paa"};
	};
	class pca_m97_pasgt_wdl_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "M97 PASGT (Woodland/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_wdl_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_wdl_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "M97 PASGT (Woodland/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\pasgt_wdl_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class rhs_beanie_green;
	class pca_beanie_brn: rhs_beanie_green
	{
		displayName = "Beanie (Brown)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\beanie_brn_co.paa"};
	};
	class pca_beanie_oli: rhs_beanie_green
	{
		displayName = "Beanie (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\beanie_oli_co.paa"};
	};
	class pca_beanie_wdl: rhs_beanie_green
	{
		displayName = "Beanie (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\beanie_wdl_co.paa"};
	};
	class rhs_booniehat2_marpatd;
	class pca_booniehat_atacs_au: rhs_booniehat2_marpatd
	{
		displayName = "Boonie Hat (ATACS AU)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\boonie_atacs_au_co.paa"};
	};
	class pca_booniehat_atacs_fg: rhs_booniehat2_marpatd
	{
		displayName = "Boonie Hat (ATACS FG)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\boonie_atacs_fg_co.paa"};
	};
	class pca_booniehat_greek_lizard_1: rhs_booniehat2_marpatd
	{
		displayName = "Boonie Hat (Greek Lizard 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\boonie_greek_lizard_1_co.paa"};
	};
	class pca_booniehat_greek_lizard_2: rhs_booniehat2_marpatd
	{
		displayName = "Boonie Hat (Greek Lizard 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\boonie_greek_lizard_2_co.paa"};
	};
	class pca_booniehat_marpatwd: rhs_booniehat2_marpatd
	{
		displayName = "Boonie Hat (Marpat Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\boonie_marpatwd_co.paa"};
	};
	class pca_booniehat_wdl: rhs_booniehat2_marpatd
	{
		displayName = "Boonie Hat (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\boonie_wdl_co.paa"};
	};
	class H_Cap_oli;
	class pca_baseball_cap_greek_lizard: H_Cap_oli
	{
		displayName = "Baseball Cap (Greek Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\baseball_cap_greek_lizard_co.paa"};
	};
	class pca_baseball_cap_wdl: H_Cap_oli
	{
		displayName = "Baseball Cap (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\baseball_cap_wdl_co.paa"};
	};
	class rhsusf_patrolcap_ucp;
	class pca_patrolcap_greek_lizard: rhsusf_patrolcap_ucp
	{
		displayName = "Patrol Cap (Greek Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\patrolcap_greek_lizard_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_headgear\data\rv\patrolcap.rvmat"};
	};
	class pca_patrolcap_marpatwd: rhsusf_patrolcap_ucp
	{
		displayName = "Patrol Cap (Marpat Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\patrolcap_marpatwd_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_headgear\data\rv\patrolcap.rvmat"};
	};
	class pca_patrolcap_police: rhsusf_patrolcap_ucp
	{
		displayName = "Patrol Cap (Police)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\patrolcap_police_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_headgear\data\rv\patrolcap.rvmat"};
	};
	class pca_patrolcap_wdl: rhsusf_patrolcap_ucp
	{
		displayName = "Patrol Cap (Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_gear\data\tex\patrolcap_wdl_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_usa_headgear\data\rv\patrolcap.rvmat"};
	};
};
