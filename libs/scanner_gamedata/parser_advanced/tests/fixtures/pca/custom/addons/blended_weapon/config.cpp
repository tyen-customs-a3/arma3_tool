////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:41 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_mods_blended_weapon
	{
		author = "PCA";
		requiredVersion = 1.6;
		requiredAddons[] = {"A3_Characters_F","A3_Weapons_F_Exp","cba_jr","rhs_main","rhs_c_weapons"};
		units[] = {};
		weapons[] = {"pca_weap_qbz95","pca_weap_qjb95","pca_weap_qbz95_qlg91b","pca_weap_svd_wood","pca_weap_svd_wood_npz"};
	};
};
class Mode_SemiAuto;
class Mode_FullAuto;
class MuzzleSlot;
class SlotInfo;
class CowsSlot;
class PointerSlot;
class UnderBarrelSlot;
class asdg_SlotInfo;
class asdg_OpticRail1913_short;
class asdg_FrontSideRail;
class CfgWeapons
{
	class Rifle;
	class UGL_F;
	class Rifle_Base_F: Rifle
	{
		class WeaponSlotsInfo;
		class GunParticles;
	};
	class pca_weap_qbz95_base: Rifle_Base_F
	{
		scope = 0;
		displayName = "QBZ-95 Base";
		descriptionShort = "QBZ95 Assault Rifle";
		overviewPicture = "\a3\data_f_exp\images\weaponcar_ca.paa";
		model = "x\pca\custom\addons\blended_weapon\data\qbz95.p3d";
		handAnim[] = {"OFP2_ManSkeleton","\A3\Weapons_F_Exp\Rifles\CTAR\Anim\CTAR_F.rtm"};
		reloadAction = "GestureReloadCTAR";
		magazines[] = {"30Rnd_580x42_Mag_F","30Rnd_580x42_Mag_Tracer_F"};
		magazineWell[] = {"CTAR_580x42","CTAR_580x42_Large"};
		magazineReloadSwitchPhase = 0.48;
		inertia = 0.35;
		aimTransitionSpeed = 1.2;
		reloadMagazineSound[] = {"a3\sounds_f\arsenal\weapons\rifles\katiba\reload_katiba",1.1,1.1,10};
		magazineReloadTime = 0;
		initSpeed = 930;
		recoil = "recoil_car";
		maxZeroing = 500;
		bullet1[] = {"a3\sounds_f\weapons\shells\7_62\metal_762_01",0.5012,1,15};
		bullet2[] = {"a3\sounds_f\weapons\shells\7_62\metal_762_02",0.5012,1,15};
		bullet3[] = {"a3\sounds_f\weapons\shells\7_62\metal_762_03",0.5012,1,15};
		bullet4[] = {"a3\sounds_f\weapons\shells\7_62\metal_762_04",0.5012,1,15};
		bullet5[] = {"a3\sounds_f\weapons\shells\7_62\dirt_762_01",0.39811,1,15};
		bullet6[] = {"a3\sounds_f\weapons\shells\7_62\dirt_762_02",0.39811,1,15};
		bullet7[] = {"a3\sounds_f\weapons\shells\7_62\dirt_762_03",0.39811,1,15};
		bullet8[] = {"a3\sounds_f\weapons\shells\7_62\dirt_762_04",0.39811,1,15};
		bullet9[] = {"a3\sounds_f\weapons\shells\7_62\grass_762_01",0.2512,1,15};
		bullet10[] = {"a3\sounds_f\weapons\shells\7_62\grass_762_02",0.2512,1,15};
		bullet11[] = {"a3\sounds_f\weapons\shells\7_62\grass_762_03",0.2512,1,15};
		bullet12[] = {"a3\sounds_f\weapons\shells\7_62\grass_762_04",0.2512,1,15};
		soundBullet[] = {"bullet1",0.083,"bullet2",0.083,"bullet3",0.083,"bullet4",0.083,"bullet5",0.083,"bullet6",0.083,"bullet7",0.083,"bullet8",0.083,"bullet9",0.083,"bullet10",0.083,"bullet11",0.083,"bullet12",0.083};
		class Library
		{
			libTextDesc = "$STR_A3_CfgWeapons_arifle_CTAR_base_F_Library0";
		};
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			class CowsSlot: asdg_OpticRail1913_short
			{
				iconPosition[] = {0.45,0.28};
				iconScale = 0.2;
			};
			class PointerSlot: asdg_FrontSideRail
			{
				iconPosition[] = {0.35,0.45};
				iconScale = 0.2;
			};
			class MuzzleSlot: MuzzleSlot
			{
				iconPosition[] = {0,0.4};
				iconScale = 0.2;
			};
			mass = 74;
		};
		aiDispersionCoefX = 25;
		aiDispersionCoefY = 10;
		distanceZoomMin = 300;
		distanceZoomMax = 300;
		modes[] = {"Single","FullAuto","AI_Single","AI_Burst","AI_Far"};
		class Single: Mode_SemiAuto
		{
			reloadTime = 0.092;
			dispersion = 0.00116355;
			minRange = 0;
			minRangeProbab = 0;
			midRange = 0;
			midRangeProbab = 0;
			maxRange = 0;
			maxRangeProbab = 0;
			class BaseSoundModeType;
			class StandardSound: BaseSoundModeType
			{
				soundSetShot[] = {"CAR_95_Shot_SoundSet","CAR_95_Tail_SoundSet","CAR_95_interiorTail_SoundSet"};
			};
			class SilencedSound: BaseSoundModeType
			{
				soundSetShot[] = {"CAR_95_silencerShot_SoundSet","CAR_95_silencerTail_SoundSet","CAR_95_silencerInteriorTail_SoundSet"};
			};
		};
		class FullAuto: Mode_FullAuto
		{
			reloadTime = 0.092;
			dispersion = 0.00116355;
			minRange = 0;
			minRangeProbab = 0;
			midRange = 0;
			midRangeProbab = 0;
			maxRange = 0;
			maxRangeProbab = 0;
			class BaseSoundModeType;
			class StandardSound: BaseSoundModeType
			{
				soundSetShot[] = {"CAR_95_Shot_SoundSet","CAR_95_Tail_SoundSet","CAR_95_interiorTail_SoundSet"};
			};
			class SilencedSound: BaseSoundModeType
			{
				soundSetShot[] = {"CAR_95_silencerShot_SoundSet","CAR_95_silencerTail_SoundSet","CAR_95_silencerInteriorTail_SoundSet"};
			};
		};
		class AI_Single: Single
		{
			showToPlayer = 0;
			dispersion = 0.00116355;
			minRange = 2;
			minRangeProbab = 0.5;
			midRange = 300;
			midRangeProbab = 0.7;
			maxRange = 600;
			maxRangeProbab = 0.3;
			aiRateOfFire = 0.2;
			aiRateOfFireDispersion = 2.8;
		};
		class AI_Burst: AI_Single
		{
			burst = 3;
			burstRangeMax = 8;
			dispersion = 0.00116355;
			minRange = 2;
			minRangeProbab = 0.3;
			midRange = 300;
			midRangeProbab = 0.5;
			maxRange = 600;
			maxRangeProbab = 0.2;
			aiRateOfFire = 0.5;
			aiRateOfFireDispersion = 2.5;
		};
		class AI_Far: AI_Single
		{
			dispersion = 0.00116355;
			minRange = 500;
			minRangeProbab = 0.5;
			midRange = 700;
			midRangeProbab = 0.7;
			maxRange = 900;
			maxRangeProbab = 0.5;
			aiRateOfFire = 0.5;
			aiRateOfFireDispersion = 2.5;
		};
	};
	class pca_weap_qbz95: pca_weap_qbz95_base
	{
		scope = 2;
		displayName = "QBZ-95 (Assault Rifle)";
		picture = "\a3\weapons_f_exp\rifles\ctar\data\ui\icon_arifle_ctar_blk_f_x_ca.paa";
	};
	class pca_weap_qjb95: pca_weap_qbz95_base
	{
		scope = 2;
		displayName = "QJB-95 (LSW)";
		model = "x\pca\custom\addons\blended_weapon\data\qjb95.p3d";
		picture = "\a3\weapons_f_exp\rifles\ctar\data\ui\icon_arifle_ctar_blk_f_x_ca.paa";
		recoil = "recoil_car_lsw";
		maxZeroing = 1000;
		initSpeed = 970;
		magazines[] = {"100Rnd_580x42_Mag_F","100Rnd_580x42_Mag_Tracer_F"};
		magazineWell[] = {"CTAR_580x42_Large","CTAR_580x42"};
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			class CowsSlot: asdg_OpticRail1913_short
			{
				iconPosition[] = {0.45,0.28};
				iconScale = 0.2;
			};
			class PointerSlot: asdg_FrontSideRail
			{
				iconPosition[] = {0.35,0.45};
				iconScale = 0.2;
			};
			class MuzzleSlot: MuzzleSlot
			{
				iconPosition[] = {0,0.4};
				iconScale = 0.2;
			};
			class UnderBarrelSlot: UnderBarrelSlot
			{
				iconPosition[] = {0.35,0.45};
				iconScale = 0.2;
			};
			mass = 88;
		};
		modes[] = {"Single","FullAuto","AI_Burst_Close","AI_Burst_Medium","AI_Burst_Far"};
		class Single: Single
		{
			reloadTime = 0.092;
			dispersion = 0.00116355;
			minRange = 0;
			minRangeProbab = 0;
			midRange = 0;
			midRangeProbab = 0;
			maxRange = 0;
			maxRangeProbab = 0;
		};
		class FullAuto: FullAuto
		{
			reloadTime = 0.092;
			dispersion = 0.00116355;
			minRange = 0;
			minRangeProbab = 0;
			midRange = 0;
			midRangeProbab = 0;
			maxRange = 0;
			maxRangeProbab = 0;
		};
		class AI_Burst_Close: Single
		{
			showToPlayer = 0;
			burst = 3;
			burstRangeMax = 12;
			dispersion = 0.00116355;
			minRange = 2;
			minRangeProbab = 0.5;
			midRange = 100;
			midRangeProbab = 0.7;
			maxRange = 200;
			maxRangeProbab = 0.3;
			aiRateOfFire = 0.5;
			aiRateOfFireDispersion = 2;
		};
		class AI_Burst_Medium: AI_Burst_Close
		{
			burst = 3;
			burstRangeMax = 10;
			dispersion = 0.00116355;
			minRange = 200;
			minRangeProbab = 0.3;
			midRange = 300;
			midRangeProbab = 0.5;
			maxRange = 600;
			maxRangeProbab = 0.2;
			aiRateOfFire = 0.5;
			aiRateOfFireDispersion = 3;
		};
		class AI_Burst_Far: AI_Burst_Close
		{
			burst = 3;
			burstRangeMax = 8;
			dispersion = 0.00116355;
			minRange = 500;
			minRangeProbab = 0.5;
			midRange = 700;
			midRangeProbab = 0.7;
			maxRange = 900;
			maxRangeProbab = 0.5;
			aiRateOfFire = 0.5;
			aiRateOfFireDispersion = 4;
		};
	};
	class pca_weap_qbz95_qlg91b: pca_weap_qbz95
	{
		scope = 2;
		displayName = "QBZ-95 (Assault Rifle/QLG-91B)";
		model = "x\pca\custom\addons\blended_weapon\data\qbz95_qgl.p3d";
		picture = "\a3\weapons_f_exp\rifles\ctar\data\ui\icon_arifle_ctar_gl_blk_f_x_ca.paa";
		uiPicture = "\a3\weapons_f\data\ui\icon_gl_ca.paa";
		handAnim[] = {"OFP2_ManSkeleton","\A3\Weapons_F_Exp\Rifles\CTAR\Anim\CTARGL.rtm"};
		inertia = 0.42;
		aimTransitionSpeed = 1;
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			mass = 106;
		};
		class EGLM: UGL_F
		{
			displayName = "QLG-91B";
			useModelOptics = 0;
			useExternalOptic = "false";
			cameraDir = "op_look";
			magazines[] = {"rhs_VOG25","rhs_VOG25P","rhs_VG40TB","rhs_VG40SZ","rhs_VG40OP_white","rhs_VG40OP_green","rhs_VG40OP_red","rhs_GRD40_White","rhs_GRD40_green","rhs_GRD40_red","rhs_GDM40","rhs_VG40MD"};
			discreteDistance[] = {50,100,150,200,250,300,350,400};
			discreteDistanceCameraPoint[] = {"OP_eye_50","OP_eye_100","OP_eye_150","OP_eye_200","OP_eye_250","OP_eye_300","OP_eye_350","OP_eye_400"};
			discreteDistanceInitIndex = 1;
			reloadAction = "GestureReloadMXUGL";
		};
		muzzles[] = {"this","EGLM"};
	};
	class rhs_weap_svdp;
	class pca_weap_svd_wood: rhs_weap_svdp
	{
		scope = 2;
		author = "Red Hammer Studios";
		displayName = "SVD (Wood)";
		hiddenSelections[] = {"Camo1","Camo2"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_weapon\data\tex\svd_wood_co.paa","rhsafrf\addons\rhs_weapons\svd\data\svds_co.paa"};
		rhs_npz = "pca_weap_svd_wood_npz";
		baseWeapon = "pca_weap_svd_wood";
	};
	class rhs_weap_svdp_npz;
	class pca_weap_svd_wood_npz: rhs_weap_svdp_npz
	{
		scope = 2;
		author = "Red Hammer Studios";
		displayName = "SVD (Wood/NPZ)";
		hiddenSelections[] = {"Camo1","Camo2"};
		hiddenSelectionsTextures[] = {"x\pca\custom\addons\blended_weapon\data\tex\svd_wood_co.paa","rhsafrf\addons\rhs_weapons\svd\data\svds_co.paa"};
		rhs_npz = "pca_weap_svd_wood";
		baseWeapon = "pca_weap_svd_wood_npz";
	};
};
