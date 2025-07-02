////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_rus_headgear
	{
		name = "PCA Blended Russian Headgear";
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","A3_Weapons_F_Exp","rhs_main","rhs_c_troops","pca_gear_blended_dataholder"};
		units[] = {};
		weapons[] = {};
	};
};
class CfgWeapons
{
	class ItemCore;
	class H_HelmetB: ItemCore
	{
		class ItemInfo;
	};
	class pca_ssh68_helmet: H_HelmetB
	{
		displayName = "[RU] SSh-68 Helmet (Olive Drab)";
		model = "x\pca\custom\addons\blended_rus_headgear\ssh68_helmet.p3d";
		picture = "x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_ca.paa";
		hiddenSelections[] = {"camo"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_co.paa"};
		class ItemInfo: ItemInfo
		{
			uniformModel = "x\pca\custom\addons\blended_rus_headgear\ssh68_helmet.p3d";
			class HitpointsProtectionInfo
			{
				class Head
				{
					hitPointName = "HitHead";
					armor = 2;
					passThrough = 0.5;
				};
			};
		};
	};
	class pca_ssh68_helmet_blk: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_black_co.paa"};
	};
	class pca_ssh68_helmet_emblem_napa: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Emblem - NAPA)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_emblem_napa_co.paa"};
	};
	class pca_ssh68_helmet_emblem_redstar: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Emblem - Red Star)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_emblem_redstar_co.paa"};
	};
	class pca_ssh68_helmet_mgrn: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_green_co.paa"};
	};
	class pca_ssh68_helmet_ogrn: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_olive_co.paa"};
	};
	class pca_ssh68_helmet_tan: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_tan_co.paa"};
	};
	class pca_ssh68_helmet_taki_clouds: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (Takistan Clouds)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_taki_clouds_co.paa"};
	};
	class pca_ssh68_helmet_un_1: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (UN 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_un_1_co.paa"};
	};
	class pca_ssh68_helmet_un_2: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet (UN 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_un_2_co.paa"};
	};
	class pca_ssh68_helmet_cover: pca_ssh68_helmet
	{
		displayName = "[RU] SSh-68 Helmet Cover (Olive Drab)";
		model = "x\pca\custom\addons\blended_rus_headgear\ssh68_helmet_cover.p3d";
		picture = "x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_ca.paa";
		hiddenSelections[] = {"camo"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_co.paa"};
		class ItemInfo: ItemInfo
		{
			uniformModel = "x\pca\custom\addons\blended_rus_headgear\ssh68_helmet_cover.p3d";
			class HitpointsProtectionInfo
			{
				class Head
				{
					hitPointName = "HitHead";
					armor = 2;
					passThrough = 0.5;
				};
			};
		};
	};
	class pca_ssh68_helmet_cover_blk: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_black_co.paa"};
	};
	class pca_ssh68_helmet_cover_berezka: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Berezka)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_berezka_co.paa"};
	};
	class pca_ssh68_helmet_cover_berezka_desert: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Berezka Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_berezka_desert_co.paa"};
	};
	class pca_ssh68_helmet_cover_berezka_winter: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Berezka Winter)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_berezka_winter_co.paa"};
	};
	class pca_ssh68_helmet_cover_granite_tan: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Granite Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_granite_tan_co.paa"};
	};
	class pca_ssh68_helmet_cover_mgrn: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_green_co.paa"};
	};
	class pca_ssh68_helmet_cover_ogrn: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_olive_co.paa"};
	};
	class pca_ssh68_helmet_cover_spetsodezhda: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_spetsodezhda_co.paa"};
	};
	class pca_ssh68_helmet_cover_spetsodezhda_od: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Spetsodezhda Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_spetsodezhda_od_co.paa"};
	};
	class pca_ssh68_helmet_cover_tan: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_tan_co.paa"};
	};
	class pca_ssh68_helmet_cover_taki_lizard: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (Takistan Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_taki_lizard_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_1: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_1_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_2: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_2_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_3: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_3_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_4: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_4_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_forest_1: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Forest 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_forest_1_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_forest_2: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Forest 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_forest_2_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_forest_3: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Forest 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_forest_3_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_forest_4: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Forest 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_forest_4_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_mountain_1: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Mountain 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_mountain_1_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_mountain_2: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Mountain 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_mountain_2_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_mountain_3: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Mountain 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_mountain_3_co.paa"};
	};
	class pca_ssh68_helmet_cover_ttsko_mountain_4: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (TTsKO Mountain 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_ttsko_mountain_4_co.paa"};
	};
	class pca_ssh68_helmet_cover_un: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (UN)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_un_co.paa"};
	};
	class pca_ssh68_helmet_cover_vdv_vsr_1: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VDV VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vdv_vsr_1_co.paa"};
	};
	class pca_ssh68_helmet_cover_vdv_vsr_2: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VDV VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vdv_vsr_2_co.paa"};
	};
	class pca_ssh68_helmet_cover_vdv_vsr_3: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VDV VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vdv_vsr_3_co.paa"};
	};
	class pca_ssh68_helmet_cover_vsr_1: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vsr_1_co.paa"};
	};
	class pca_ssh68_helmet_cover_vsr_2: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vsr_2_co.paa"};
	};
	class pca_ssh68_helmet_cover_vsr_3: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vsr_3_co.paa"};
	};
	class pca_ssh68_helmet_cover_vsr_4: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vsr_4_co.paa"};
	};
	class pca_ssh68_helmet_cover_vsr_5: pca_ssh68_helmet_cover
	{
		displayName = "[RU] SSh-68 Helmet Cover (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_new\ssh68_helmet_cover_vsr_5_co.paa"};
	};
	class rhs_beanie_green;
	class pca_beanie: rhs_beanie_green
	{
		displayName = "[RU] Beanie (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\beanie_co.paa"};
	};
	class pca_beanie_blk: rhs_beanie_green
	{
		displayName = "[RU] Beanie (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\beanie_black_co.paa"};
	};
	class pca_beanie_gry: rhs_beanie_green
	{
		displayName = "[RU] Beanie (Gray)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\beanie_gray_co.paa"};
	};
	class pca_beanie_mgrn: rhs_beanie_green
	{
		displayName = "[RU] Beanie (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\beanie_green_co.paa"};
	};
	class pca_beanie_tan: rhs_beanie_green
	{
		displayName = "[RU] Beanie (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\beanie_tan_co.paa"};
	};
	class pca_beanie_white: rhs_beanie_green
	{
		displayName = "[RU] Beanie (White)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\beanie_white_co.paa"};
	};
	class rhs_Booniehat_flora;
	class pca_booniehat_flora: rhs_Booniehat_flora
	{
		displayName = "[RU] Boonie Hat (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\boonie_flora_co.paa"};
	};
	class rhs_booniehat2_marpatd;
	class pca_booniehat_flecktarn: rhs_booniehat2_marpatd
	{
		displayName = "[RU] Boonie Hat (Flecktarn)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\boonie_flecktarn_co.paa"};
	};
	class rhs_fieldcap_m88_vsr;
	class pca_m88_fieldcap: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_co.paa"};
	};
	class pca_m88_fieldcap_alpenflage: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Alpenflage)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_alpenflage_co.paa"};
	};
	class pca_m88_fieldcap_granite_tan: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Granite Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_granite_tan_co.paa"};
	};
	class pca_m88_fieldcap_gray: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Gray)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_gray_co.paa"};
	};
	class pca_m88_fieldcap_mgrn: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_green_co.paa"};
	};
	class pca_m88_fieldcap_ogrn: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Olive)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_olive_co.paa"};
	};
	class pca_m88_fieldcap_orel: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Orel)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_orel_co.paa"};
	};
	class pca_m88_fieldcap_puma: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Puma)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_puma_co.paa"};
	};
	class pca_m88_fieldcap_smk_urb_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (SMK Urban 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_smk_urb_1_co.paa"};
	};
	class pca_m88_fieldcap_smk_urb_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (SMK Urban 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_smk_urb_2_co.paa"};
	};
	class pca_m88_fieldcap_smk_wdl_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (SMK Woodland 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_smk_wdl_1_co.paa"};
	};
	class pca_m88_fieldcap_smk_wdl_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (SMK Woodland 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_smk_wdl_2_co.paa"};
	};
	class pca_m88_fieldcap_smk_wdl_3: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (SMK Woodland 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_smk_wdl_3_co.paa"};
	};
	class pca_m88_fieldcap_smk_wdl_4: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (SMK Woodland 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_smk_wdl_4_co.paa"};
	};
	class pca_m88_fieldcap_tan: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tan_co.paa"};
	};
	class pca_m88_fieldcap_taki_lizard: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Takistan Lizard)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_taki_lizard_co.paa"};
	};
	class pca_m88_fieldcap_spetsodezhda: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_spetsodezhda_co.paa"};
	};
	class pca_m88_fieldcap_spetsodezhda_od: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Spetsodezhda Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_spetsodezhda_od_co.paa"};
	};
	class pca_m88_fieldcap_tigr_desert: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tigr Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tigr_desert_co.paa"};
	};
	class pca_m88_fieldcap_tigr_urb_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tigr Urban 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tigr_urb_1_co.paa"};
	};
	class pca_m88_fieldcap_tigr_urb_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tigr Urban 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tigr_urb_2_co.paa"};
	};
	class pca_m88_fieldcap_tigr_wdl_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tigr Woodland 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tigr_wdl_1_co.paa"};
	};
	class pca_m88_fieldcap_tigr_wdl_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tigr Woodland 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tigr_wdl_2_co.paa"};
	};
	class pca_m88_fieldcap_tigr_wdl_3: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (Tigr Woodland 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_tigr_wdl_3_co.paa"};
	};
	class pca_m88_fieldcap_ttsko: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (TTsKO)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_ttsko_co.paa"};
	};
	class pca_m88_fieldcap_ttsko_forest: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_ttsko_forest_co.paa"};
	};
	class pca_m88_fieldcap_ttsko_mountain: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (TTsKO Mountain)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_ttsko_mountain_co.paa"};
	};
	class pca_m88_fieldcap_ttsko_vdv_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (TTsKO VDV 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_ttsko_vdv_1_co.paa"};
	};
	class pca_m88_fieldcap_ttsko_vdv_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (TTsKO VDV 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_ttsko_vdv_2_co.paa"};
	};
	class pca_m88_fieldcap_vdv_vsr_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VDV VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vdv_vsr_1_co.paa"};
	};
	class pca_m88_fieldcap_vdv_vsr_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VDV VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vdv_vsr_2_co.paa"};
	};
	class pca_m88_fieldcap_vdv_vsr_3: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VDV VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vdv_vsr_3_co.paa"};
	};
	class pca_m88_fieldcap_vsr_1: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vsr_1_co.paa"};
	};
	class pca_m88_fieldcap_vsr_2: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vsr_2_co.paa"};
	};
	class pca_m88_fieldcap_vsr_3: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vsr_3_co.paa"};
	};
	class pca_m88_fieldcap_vsr_4: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vsr_4_co.paa"};
	};
	class pca_m88_fieldcap_vsr_5: rhs_fieldcap_m88_vsr
	{
		displayName = "[RU] M88 Field Cap (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m88_cap_vsr_5_co.paa"};
	};
	class rhs_6b27m;
	class rhs_6b27m_ess;
	class pca_6b27m_cdf_ttsko_autumn: rhs_6b27m
	{
		displayName = "[RU] 6B27M (CDF TTsKO Autumn)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_autumn_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_autumn_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (CDF TTsKO Autumn/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_autumn_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_desert: rhs_6b27m
	{
		displayName = "[RU] 6B27M (CDF TTsKO Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_desert_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_desert_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (CDF TTsKO Desert/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_desert_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_forest: rhs_6b27m
	{
		displayName = "[RU] 6B27M (CDF TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_forest_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_forest_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (CDF TTsKO Forest/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_forest_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_mountain: rhs_6b27m
	{
		displayName = "[RU] 6B27M (CDF TTsKO Mountain)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_mountain_co.paa"};
	};
	class pca_6b27m_cdf_ttsko_mountain_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (CDF TTsKO Mountain/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_mountain_co.paa"};
	};
	class pca_6b27m_cdf_wdl: rhs_6b27m
	{
		displayName = "[RU] 6B27M (CDF Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_wdl_co.paa"};
	};
	class pca_6b27m_cdf_wdl_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (CDF Woodland/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_wdl_co.paa"};
	};
	class pca_6b27m_cdf_spetsodezhda: rhs_6b27m
	{
		displayName = "[RU] 6B27M (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_spetsodezhda_co.paa"};
	};
	class pca_6b27m_cdf_spetsodezhda_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (Spetsodezhda/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_spetsodezhda_co.paa"};
	};
	class pca_6b27m_flora: rhs_6b27m
	{
		displayName = "[RU] 6B27M (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_flora_co.paa"};
	};
	class pca_6b27m_flora_ess: rhs_6b27m_ess
	{
		displayName = "[RU] 6B27M (Flora/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_flora_co.paa"};
	};
	class rhs_6b7_1m_emr;
	class rhs_6b7_1m_emr_ess;
	class pca_6b7_1m_flora: rhs_6b7_1m_emr
	{
		displayName = "[RU] 6B7-1M (Flora)";
		hiddenSelectionsTextures[] = {"rhsafrf\addons\rhs_infantry2\gear\head\data\rhs_6b7-1m_co.paa","x\pca\custom\addons\blended_rus_headgear\data\tex\6b7_1m_helmet_flora_co.paa"};
	};
	class pca_6b7_1m_flora_ess: rhs_6b7_1m_emr_ess
	{
		displayName = "[RU] 6B7-1M (Flora/ESS)";
		hiddenSelectionsTextures[] = {"rhsafrf\addons\rhs_infantry2\gear\head\data\rhs_6b7-1m_co.paa","x\pca\custom\addons\blended_rus_headgear\data\tex\6b7_1m_helmet_flora_co.paa"};
	};
	class rhs_fieldcap;
	class pca_fieldcap_cdf_ttsko_autumn: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (CDF TTsKO Autumn)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_autumn_co.paa"};
	};
	class pca_fieldcap_cdf_ttsko_desert: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (CDF TTsKO Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_desert_co.paa"};
	};
	class pca_fieldcap_cdf_ttsko_forest: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (CDF TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_forest_co.paa"};
	};
	class pca_fieldcap_cdf_ttsko_mountain: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (CDF TTsKO Mountain)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_ttsko_mountain_co.paa"};
	};
	class pca_fieldcap_cdf_wdl: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (CDF Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_cdf_wdl_co.paa"};
	};
	class pca_fieldcap_flora: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (Flora)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_flora_co.paa"};
	};
	class pca_fieldcap_spetsodezhda: rhs_fieldcap
	{
		displayName = "[RU] Field Cap (Spetsodezhda)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\6b27_helmet_spetsodezhda_co.paa"};
	};
	class rhssaf_helmet_m97_woodland;
	class rhssaf_helmet_m97_woodland_black_ess;
	class rhssaf_helmet_m97_woodland_black_ess_bare;
	class pca_m97_pasgt_cdf_ttsko_autumn: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Autumn)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_autumn_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_autumn_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Autumn/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_autumn_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_autumn_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Autumn/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_autumn_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_desert: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_desert_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_desert_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Desert/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_desert_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_desert_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Desert/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_desert_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_forest: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Forest)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_forest_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_forest_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Forest/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_forest_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_forest_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Forest/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_forest_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_plum: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (CDF Plum)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_plum_co.paa"};
	};
	class pca_m97_pasgt_cdf_plum_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (CDF Plum/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_plum_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_plum_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (CDF Plum/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_plum_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_mountain: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Mountain)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_mountain_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_mountain_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Mountain/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_mountain_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_ttsko_mountain_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (CDF TTsKO Mountain/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_ttsko_mountain_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_wdl: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (CDF Woodland)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_wdl_co.paa"};
	};
	class pca_m97_pasgt_cdf_wdl_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (CDF Woodland/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_wdl_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_cdf_wdl_ess_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (CDF Woodland/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_cdf_wdl_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_white: rhssaf_helmet_m97_woodland
	{
		displayName = "[RU] M97 Helmet (White)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_white_co.paa"};
	};
	class pca_m97_pasgt_white_ess: rhssaf_helmet_m97_woodland_black_ess
	{
		displayName = "[RU] M97 Helmet (White/ESS)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_white_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class pca_m97_pasgt_white_bare: rhssaf_helmet_m97_woodland_black_ess_bare
	{
		displayName = "[RU] M97 Helmet (White/ESS Bare)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\pasgt_helmet_white_co.paa","\rhssaf\addons\rhssaf_t_headgear_m97\data\rhssaf_m97_ess_black_co.paa"};
	};
	class rhs_headband;
	class pca_headband: rhs_headband
	{
		displayName = "[RU] Headband (Olive Drab)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\headband_co.paa"};
	};
	class pca_headband_blk: rhs_headband
	{
		displayName = "[RU] Headband (Black)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\headband_black_co.paa"};
	};
	class pca_headband_red: rhs_headband
	{
		displayName = "[RU] Headband (Red)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\headband_red_co.paa"};
	};
	class pca_headband_tan: rhs_headband
	{
		displayName = "[RU] Headband (Tan)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\headband_tan_co.paa"};
	};
	class rhs_ssh68;
	class pca_ssh68_camo: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Camo)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_camo_co.paa"};
	};
	class pca_ssh68_camo_net: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Camo/Net)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_camo_net_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_rus_headgear\data\rv\ssh68_net.rvmat"};
	};
	class pca_ssh68_camo_des: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Camo/Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_camo_des_co.paa"};
	};
	class pca_ssh68_camo_grn: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Camo/Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_camo_grn_co.paa"};
	};
	class pca_ssh68_camo_mix: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Camo/Mixed)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_camo_mix_co.paa"};
	};
	class pca_ssh68_grn: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_grn_co.paa"};
	};
	class pca_ssh68_grn_net: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Green/Net)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_grn_net_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_rus_headgear\data\rv\ssh68_net.rvmat"};
	};
	class pca_ssh68_dgrn: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Dark Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_dgrn_co.paa"};
	};
	class pca_ssh68_mgrn: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Military Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_mgrn_co.paa"};
	};
	class pca_ssh68_ogrn: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Olive Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_ogrn_co.paa"};
	};
	class pca_ssh68_sgrn: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Sea Green)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_sgrn_co.paa"};
	};
	class pca_ssh68_mp: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Military Police)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_mp_co.paa"};
	};
	class pca_ssh68_winter: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Winter)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_winter_co.paa"};
	};
	class pca_ssh68_winter_net: rhs_ssh68
	{
		displayName = "[RU] SSh-68 (Winter/Net)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ssh68_winter_net_co.paa"};
		hiddenSelectionsMaterials[] = {"x\pca\custom\addons\blended_rus_headgear\data\rv\ssh68_net.rvmat"};
	};
	class rhs_stsh81_butan;
	class pca_sfera_smk_urb_1: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (SMK Urban 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_smk_urb_1_co.paa"};
	};
	class pca_sfera_smk_urb_2: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (SMK Urban 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_smk_urb_2_co.paa"};
	};
	class pca_sfera_smk_wdl_1: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (SMK Woodland 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_smk_wdl_1_co.paa"};
	};
	class pca_sfera_smk_wdl_2: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (SMK Woodland 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_smk_wdl_2_co.paa"};
	};
	class pca_sfera_smk_wdl_3: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (SMK Woodland 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_smk_wdl_3_co.paa"};
	};
	class pca_sfera_smk_wdl_4: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (SMK Woodland 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_smk_wdl_4_co.paa"};
	};
	class pca_sfera_tigr_desert: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (Tigr Desert)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_tigr_desert_co.paa"};
	};
	class pca_sfera_tigr_urb_1: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (Tigr Urban 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_tigr_urb_1_co.paa"};
	};
	class pca_sfera_tigr_urb_2: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (Tigr Urban 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_tigr_urb_2_co.paa"};
	};
	class pca_sfera_tigr_wdl_1: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (Tigr Woodland 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_tigr_wdl_1_co.paa"};
	};
	class pca_sfera_tigr_wdl_2: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (Tigr Woodland 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_tigr_wdl_2_co.paa"};
	};
	class pca_sfera_tigr_wdl_3: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (Tigr Woodland 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_tigr_wdl_3_co.paa"};
	};
	class pca_sfera_un: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (UN)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_un_blue_co.paa"};
	};
	class pca_sfera_vsr_1: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (VSR 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_vsr_1_co.paa"};
	};
	class pca_sfera_vsr_2: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (VSR 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_vsr_2_co.paa"};
	};
	class pca_sfera_vsr_3: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (VSR 3)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_vsr_3_co.paa"};
	};
	class pca_sfera_vsr_4: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (VSR 4)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_vsr_4_co.paa"};
	};
	class pca_sfera_vsr_5: rhs_stsh81_butan
	{
		displayName = "[RU] STSh-81 Helmet (VSR 5)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\sfera_helmet_vsr_5_co.paa"};
	};
	class rhs_ushanka;
	class pca_ushanka: rhs_ushanka
	{
		displayName = "[RU] Ushanka (Dark Blue)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\ushanka_co.paa"};
	};
	class rhssaf_helmet_m59_85_oakleaf;
	class pca_m59_cover_m89_oakleaf: rhssaf_helmet_m59_85_oakleaf
	{
		displayName = "[RU] M59 Helmet Cover (M89 Oakleaf)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\m59_helmet_cover_m89oakleaf_co.paa"};
	};
};
class CfgGlasses
{
	class rhs_balaclava;
	class rhs_balaclava1_olive;
	class pca_balaclava_blk_1: rhs_balaclava1_olive
	{
		displayName = "[RU] Balaclava (Black 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\balaclava_blk_1_co.paa"};
	};
	class pca_balaclava_blk_2: rhs_balaclava
	{
		displayName = "[RU] Balaclava (Black 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\balaclava_blk_2_co.paa"};
	};
	class pca_balaclava_oli_1: rhs_balaclava1_olive
	{
		displayName = "[RU] Balaclava (Olive Green 1)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\balaclava_oli_1_co.paa"};
	};
	class pca_balaclava_oli_2: rhs_balaclava
	{
		displayName = "[RU] Balaclava (Olive Green 2)";
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_rus_headgear\data\tex\balaclava_oli_2_co.paa"};
	};
};
