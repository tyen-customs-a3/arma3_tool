////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 11:18:15 2025 : 'file' last modified on Sun Feb 20 06:59:36 2022
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class sp_fwa_weapon_base
	{
		requiredaddons[] = {"A3_Data_F_Tank_Loadorder","A3_Data_F","A3_UI_F","A3_Anims_F","A3_Anims_F_Config_Sdr","A3_Weapons_F"};
		requiredversion = 0.1;
		units[] = {};
		weapons[] = {};
		magazines[] = {};
	};
};
class CfgFunctions
{
	class FreeWorldArmoury
	{
		class Misc
		{
			class weaponrested
			{
				tag = "Spearpoint";
				description = "Weapon deployment animation switch";
				file = "sp_fwa_weapon_base\fncs\fnc_weaponrested.sqf";
			};
		};
	};
};
class RscInGameUI
{
	class RscWeaponZeroing;
	class sp_fwa_weapondeploy: RscWeaponZeroing
	{
		onLoad = "_this call FreeWorldArmoury_fnc_weaponrested;";
	};
};
class CfgMovesBasic
{
	class Default;
	class DefaultDie;
	class ManActions
	{
		sp_fwa_GestureDeployedWeapon = "sp_fwa_GestureDeployedWeapon";
	};
	class Actions
	{
		class RifleBaseStandActions;
		class RifleAdjustProneBaseActions;
		class NoActions: ManActions
		{
			sp_fwa_GestureDeployedWeapon[] = {"sp_fwa_GestureDeployedWeapon","Gesture"};
		};
		class RifleProneActions: RifleBaseStandActions
		{
			sp_fwa_GestureDeployedWeapon[] = {"sp_fwa_GestureDeployedWeapon_prone","Gesture"};
		};
		class RifleAdjustRProneActions: RifleAdjustProneBaseActions
		{
			sp_fwa_GestureDeployedWeapon[] = {"sp_fwa_GestureDeployedWeapon_context","Gesture"};
		};
		class RifleAdjustLProneActions: RifleAdjustProneBaseActions
		{
			sp_fwa_GestureDeployedWeapon[] = {"sp_fwa_GestureDeployedWeapon_context","Gesture"};
		};
		class RifleAdjustFProneActions: RifleAdjustProneBaseActions
		{
			sp_fwa_GestureDeployedWeapon[] = {"sp_fwa_GestureDeployedWeapon_context","Gesture"};
		};
	};
};
class CfgGesturesMale
{
	class Default;
	class States
	{
		class sp_fwa_GestureDeployedWeapon: Default
		{
			file = "sp_fwa_weapon_base\anims\hands_gpmg_deployed.rtm";
			looped = 1;
			speed = 1;
			mask = "leftHand";
			leftHandIKBeg = 1;
			leftHandIKEnd = 1;
			leftHandIKCurve[] = {0};
			enableOptics = 1;
			canPullTrigger = 1;
			disableWeapons = 0;
		};
		class sp_fwa_GestureDeployedWeapon_prone: sp_fwa_GestureDeployedWeapon
		{
			file = "sp_fwa_weapon_base\anims\hands_lmg_deployed_prone.rtm";
		};
		class sp_fwa_GestureDeployedWeapon_Context: sp_fwa_GestureDeployedWeapon
		{
			mask = "handsWeapon_context";
		};
	};
};
class CfgSoundSets
{
	class SPAR01_silencerShot_SoundSet;
	class sp_fwa_556_semiauto_silencerShot_SoundSet: SPAR01_silencerShot_SoundSet
	{
		soundShaders[] = {"SPAR01_Closure_SoundShader","sp_fwa_556_semiauto_silencerShot_SoundShader"};
	};
	class DMR06_silencerShot_SoundSet;
	class sp_fwa_762_semiauto_silencerShot_SoundSet: DMR06_silencerShot_SoundSet
	{
		soundShaders[] = {"DMR06_Closure_SoundShader","sp_fwa_762_semiauto_silencerShot_SoundShader"};
	};
};
class CfgSoundShaders
{
	class SPAR01_silencerShot_SoundShader;
	class sp_fwa_556_semiauto_silencerShot_SoundShader: SPAR01_silencerShot_SoundShader
	{
		samples[] = {{"sp_fwa_weapon_base\snd\556_silenced_close_01.wav",0.2},{"sp_fwa_weapon_base\snd\556_silenced_close_02.wav",0.2},{"sp_fwa_weapon_base\snd\556_silenced_close_03.wav",0.2},{"sp_fwa_weapon_base\snd\556_silenced_close_04.wav",0.2},{"sp_fwa_weapon_base\snd\556_silenced_close_05.wav",0.2}};
		volume = 1;
	};
	class DMR06_silencerShot_SoundShader;
	class sp_fwa_762_semiauto_silencerShot_SoundShader: DMR06_silencerShot_SoundShader
	{
		samples[] = {{"sp_fwa_weapon_base\snd\762_silenced_close_01.wav",0.2},{"sp_fwa_weapon_base\snd\762_silenced_close_02.wav",0.2},{"sp_fwa_weapon_base\snd\762_silenced_close_03.wav",0.2},{"sp_fwa_weapon_base\snd\762_silenced_close_04.wav",0.2},{"sp_fwa_weapon_base\snd\762_silenced_close_05.wav",0.2},{"sp_fwa_weapon_base\snd\762_silenced_close_06.wav",0.2},{"sp_fwa_weapon_base\snd\762_silenced_close_07.wav",0.2}};
		volume = 1;
	};
};
class Mode_FullAuto;
class Mode_SemiAuto;
class CfgWeapons
{
	class Rifle_Base_F;
	class SlotInfo;
	class sp_fwa_rifle_base: Rifle_Base_F
	{
		scope = 1;
		afMax = 0;
		author = "Free World Armoury";
		hiddenSelections[] = {"texWeapon_01","texWeapon_02","texWeapon_03","texWeapon_04"};
		htMax = 480;
		htMin = 1;
		irLaserEnd = "laser_dir";
		irLaserPos = "laser_pos";
		memoryPointCamera = "eye";
		mFact = 1;
		mfMax = 0;
		muzzleEnd = "konec hlavne";
		muzzlePos = "usti hlavne";
		selectionFireAnim = "muzzleflash";
		shotEnd = "konec hlavne";
		shotPos = "usti hlavne";
		tBody = 100;
		UiPicture = "\A3\weapons_f\data\UI\icon_regular_CA.paa";
		cursor = "arifle";
		changeFiremodeSound[] = {"A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR_01_firemode",0.316228,1,5};
		zeroingSound[] = {"A3\Sounds_F\arsenal\sfx\shared\zeroing_knob_tick_metal",0.316228,1,5};
	};
	class sp_fwa_rifle_762_base: sp_fwa_rifle_base
	{
		scope = 1;
		aiDispersionCoefX = 2;
		aiDispersionCoefY = 3;
		bullet1[] = {"A3\sounds_f\weapons\shells\7_62\metal_762_01",0.501187,1,15};
		bullet10[] = {"A3\sounds_f\weapons\shells\7_62\grass_762_02",0.251189,1,15};
		bullet11[] = {"A3\sounds_f\weapons\shells\7_62\grass_762_03",0.251189,1,15};
		bullet12[] = {"A3\sounds_f\weapons\shells\7_62\grass_762_04",0.251189,1,15};
		bullet2[] = {"A3\sounds_f\weapons\shells\7_62\metal_762_02",0.501187,1,15};
		bullet3[] = {"A3\sounds_f\weapons\shells\7_62\metal_762_03",0.501187,1,15};
		bullet4[] = {"A3\sounds_f\weapons\shells\7_62\metal_762_04",0.501187,1,15};
		bullet5[] = {"A3\sounds_f\weapons\shells\7_62\dirt_762_01",0.398107,1,15};
		bullet6[] = {"A3\sounds_f\weapons\shells\7_62\dirt_762_02",0.398107,1,15};
		bullet7[] = {"A3\sounds_f\weapons\shells\7_62\dirt_762_03",0.398107,1,15};
		bullet8[] = {"A3\sounds_f\weapons\shells\7_62\dirt_762_04",0.398107,1,15};
		bullet9[] = {"A3\sounds_f\weapons\shells\7_62\grass_762_01",0.251189,1,15};
		descriptionShort = "Battle Rifle<br />Caliber: 7.62x51 mm";
		dexterity = 1.3;
		discreteDistanceInitIndex = 0;
		drySound[] = {"A3\Sounds_F\arsenal\weapons\LongRangeRifles\Mk18\Mk18_dry",0.630957,1,30};
		handAnim[] = {"OFP2_ManSkeleton","\A3\Weapons_F_Mark\LongRangeRifles\DMR_06\data\Anim\DMR_06.rtm"};
		inertia = 0.7;
		initSpeed = -1;
		magazineReloadSwitchPhase = 0.41;
		maxZeroing = 1000;
		modes[] = {"Single","FullAuto","single_close_optics1","single_medium_optics1","single_far_optics1","fullauto_medium"};
		optics = 1;
		opticsDisablePeripherialVision = 1;
		recoil = "sp_fwa_recoil_battlerifle_762_medium";
		reloadAction = "GestureReloadDMR06";
		reloadMagazineSound[] = {"A3\Sounds_F_Mark\arsenal\weapons\LongRangeRifles\DMR_06_Mk14\DMR_06_reload",1,1,10};
		soundBullet[] = {"bullet1",0.083,"bullet2",0.083,"bullet3",0.083,"bullet4",0.083,"bullet5",0.083,"bullet6",0.083,"bullet7",0.083,"bullet8",0.083,"bullet9",0.083,"bullet10",0.083,"bullet11",0.083,"bullet12",0.083};
		weaponInfoType = "RscWeaponZeroing";
		zeroingSound[] = {"A3\Sounds_F\arsenal\sfx\shared\zeroing_knob_tick_metal",0.316228,1,5};
		changeFiremodeSound[] = {"A3\Sounds_F_Exp\arsenal\weapons\Rifles\SPAR01\SPAR01_firemode",0.177828,1,5};
		class Library
		{
			libTextDesc = "This is an FWA firearm in need of a description.";
		};
		class GunParticles
		{
			class Particle1
			{
				effectName = "sp_fwa_ComplexEffect_fire_gas_smallarms";
				positionName = "usti hlavne";
				directionName = "konec hlavne";
			};
			class Particle2
			{
				effectName = "sp_fwa_ComplexEffect_fire_haze_smallarms";
				positionName = "usti hlavne";
				directionName = "konec hlavne";
			};
			class Particle3
			{
				effectName = "sp_fwa_ComplexEffect_fire_gas_smallarms_subtle";
				positionName = "Nabojniceend";
				directionName = "Nabojnicestart";
			};
			class Particle4
			{
				effectName = "sp_fwa_ComplexEffect_fire_gas_smallarms_subtle";
				positionName = "gasBlockEffect_left";
				directionName = "gasBlockEffect_start";
			};
			class Particle5
			{
				effectName = "sp_fwa_ComplexEffect_fire_gas_smallarms_subtle";
				positionName = "gasBlockEffect_right";
				directionName = "gasBlockEffect_start";
			};
		};
		class FullAuto: Mode_FullAuto
		{
			aiRateOfFire = 1e-06;
			dispersion = 0.0007994;
			maxRange = 30;
			maxRangeProbab = 0.05;
			midRange = 15;
			midRangeProbab = 0.7;
			minRange = 0;
			minRangeProbab = 0.9;
			recoil = "recoil_auto_primary_3outof10";
			recoilProne = "recoil_auto_primary_prone_3outof10";
			reloadTime = 0.085;
			requiredOpticType = 0;
			sounds[] = {"StandardSound","SilencedSound"};
			class SilencedSound
			{
				SoundSetShot[] = {"sp_fwa_762_semiauto_silencerShot_SoundSet","DMR06_silencerTail_SoundSet","DMR06_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				soundSetShot[] = {"DMR06_Shot_SoundSet","DMR06_tail_SoundSet","DMR06_InteriorTail_SoundSet"};
			};
		};
		class fullauto_medium: FullAuto
		{
			aiRateOfFire = 2;
			aiRateOfFireDispersion = 2;
			burst = 2;
			burstRangeMax = 4;
			maxRange = 200;
			maxRangeProbab = 0.05;
			midRange = 100;
			midRangeProbab = 0.7;
			minRange = 2;
			minRangeProbab = 0.5;
			showToPlayer = 0;
		};
		class Single: Mode_SemiAuto
		{
			dispersion = 0.0007994;
			aiRateOfFireDispersion = 2;
			maxRange = 400;
			maxRangeProbab = 0.05;
			midRange = 300;
			midRangeProbab = 0.7;
			minRange = 2;
			minRangeProbab = 0.3;
			requiredOpticType = 0;
			recoil = "recoil_single_primary_3outof10";
			recoilProne = "recoil_single_primary_prone_3outof10";
			reloadTime = 0.085;
			sounds[] = {"StandardSound","SilencedSound"};
			class SilencedSound
			{
				SoundSetShot[] = {"sp_fwa_762_semiauto_silencerShot_SoundSet","DMR06_silencerTail_SoundSet","DMR06_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				soundSetShot[] = {"DMR06_Shot_SoundSet","DMR06_tail_SoundSet","DMR06_InteriorTail_SoundSet"};
			};
		};
		class single_close_optics1: Single
		{
			aiRateOfFire = 2;
			aiRateOfFireDistance = 300;
			maxRange = 400;
			maxRangeProbab = 0.01;
			midRange = 300;
			midRangeProbab = 0.8;
			minRange = 2;
			minRangeProbab = 0.05;
			multiplier = 1;
			requiredOpticType = 1;
			showToPlayer = 0;
		};
		class single_medium_optics1: single_close_optics1
		{
			aiRateOfFire = 2;
			aiRateOfFireDistance = 400;
			maxRange = 450;
			maxRangeProbab = 0.05;
			midRange = 400;
			midRangeProbab = 0.7;
			minRange = 300;
			minRangeProbab = 0.05;
		};
		class single_far_optics1: single_medium_optics1
		{
			aiRateOfFire = 4;
			aiRateOfFireDistance = 600;
			maxRange = 600;
			maxRangeProbab = 0.05;
			midRange = 500;
			midRangeProbab = 0.5;
			minRange = 300;
			minRangeProbab = 0.05;
			requiredOpticType = 2;
		};
		class WeaponSlotsInfo
		{
			allowedSlots[] = {901};
			Mass = 97;
			class MuzzleSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\MUZZLE";
				compatibleItems[] = {};
			};
			class CowsSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\TOP";
				compatibleItems[] = {};
			};
			class PointerSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\SIDE";
				compatibleItems[] = {};
			};
			class UnderBarrelSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\UNDERBARREL";
				compatibleItems[] = {};
			};
		};
	};
	class sp_fwa_rifle_556_base: sp_fwa_rifle_base
	{
		scope = 1;
		aiDispersionCoefX = 2;
		aiDispersionCoefY = 3;
		bullet1[] = {"A3\sounds_f\weapons\shells\5_56\metal_556_01",0.501187,1,15};
		bullet10[] = {"A3\sounds_f\weapons\shells\5_56\grass_556_02",0.251189,1,15};
		bullet11[] = {"A3\sounds_f\weapons\shells\5_56\grass_556_03",0.251189,1,15};
		bullet12[] = {"A3\sounds_f\weapons\shells\5_56\grass_556_04",0.251189,1,15};
		bullet2[] = {"A3\sounds_f\weapons\shells\5_56\metal_556_02",0.501187,1,15};
		bullet3[] = {"A3\sounds_f\weapons\shells\5_56\metal_556_03",0.501187,1,15};
		bullet4[] = {"A3\sounds_f\weapons\shells\5_56\metal_556_04",0.501187,1,15};
		bullet5[] = {"A3\sounds_f\weapons\shells\5_56\dirt_556_01",0.398107,1,15};
		bullet6[] = {"A3\sounds_f\weapons\shells\5_56\dirt_556_02",0.398107,1,15};
		bullet7[] = {"A3\sounds_f\weapons\shells\5_56\dirt_556_03",0.398107,1,15};
		bullet8[] = {"A3\sounds_f\weapons\shells\5_56\dirt_556_04",0.398107,1,15};
		bullet9[] = {"A3\sounds_f\weapons\shells\5_56\grass_556_01",0.251189,1,15};
		changeFiremodeSound[] = {"A3\Sounds_F\arsenal\weapons\LongRangeRifles\Mk18\Mk18_firemode",0.251189,1,5};
		descriptionShort = "Assault Rifle<br />Caliber: 5.56x45 mm";
		dexterity = 1.5;
		discreteDistanceInitIndex = 0;
		drySound[] = {"A3\Sounds_F\arsenal\weapons\LongRangeRifles\Mk18\Mk18_dry",0.630957,1,30};
		handAnim[] = {"OFP2_ManSkeleton","\A3\Weapons_F_Exp\Rifles\SPAR_01\Data\Anim\SPAR_01.rtm"};
		inertia = 0.5;
		initSpeed = -1;
		magazineReloadSwitchPhase = 0.48;
		magazines[] = {"30Rnd_556x45_Stanag","30Rnd_556x45_Stanag_green","30Rnd_556x45_Stanag_red","30Rnd_556x45_Stanag_Tracer_Red","30Rnd_556x45_Stanag_Tracer_Green","30Rnd_556x45_Stanag_Tracer_Yellow"};
		magazineWell[] = {"STANAG_556x45","STANAG_556x45_Large"};
		maxZeroing = 800;
		modes[] = {"Single","FullAuto","single_medium_optics1","single_medium_optics2","fullauto_medium"};
		optics = 1;
		opticsDisablePeripherialVision = 1;
		recoil = "sp_fwa_recoil_assaultrifle_556_medium";
		reloadAction = "GestureReloadSPAR_01";
		reloadMagazineSound[] = {"A3\Sounds_F_Exp\arsenal\weapons\Rifles\SPAR01\SPAR01_reload",1,1,10};
		soundBullet[] = {"bullet1",0.083,"bullet2",0.083,"bullet3",0.083,"bullet4",0.083,"bullet5",0.083,"bullet6",0.083,"bullet7",0.083,"bullet8",0.083,"bullet9",0.083,"bullet10",0.083,"bullet11",0.083,"bullet12",0.083};
		weaponInfoType = "RscWeaponZeroing";
		class Library
		{
			libTextDesc = "This is an FWA firearm in need of a description.";
		};
		class FullAuto: Mode_FullAuto
		{
			aiRateOfFire = 1e-06;
			dispersion = 0.00073;
			maxRange = 30;
			maxRangeProbab = 0.05;
			midRange = 15;
			midRangeProbab = 0.7;
			minRange = 2;
			minRangeProbab = 0.9;
			reloadTime = 0.07;
			class SilencedSound
			{
				SoundSetShot[] = {"sp_fwa_556_semiauto_silencerShot_SoundSet","SPAR01_silencerTail_SoundSet","SPAR01_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				soundSetShot[] = {"SPAR01_Shot_SoundSet","SPAR01_Tail_SoundSet","SPAR01_InteriorTail_SoundSet"};
			};
		};
		class fullauto_medium: FullAuto
		{
			aiRateOfFire = 2;
			aiRateOfFireDispersion = 2;
			burst = 2;
			burstRangeMax = 5;
			maxRange = 200;
			maxRangeProbab = 0.05;
			midRange = 100;
			midRangeProbab = 0.7;
			minRange = 2;
			minRangeProbab = 0.5;
			showToPlayer = 0;
		};
		class Single: Mode_SemiAuto
		{
			dispersion = 0.00073;
			aiRateOfFireDispersion = 2;
			maxRange = 250;
			maxRangeProbab = 0.2;
			midRange = 150;
			midRangeProbab = 0.7;
			minRange = 2;
			minRangeProbab = 0.5;
			reloadTime = 0.07;
			class SilencedSound
			{
				SoundSetShot[] = {"sp_fwa_556_semiauto_silencerShot_SoundSet","SPAR01_silencerTail_SoundSet","SPAR01_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				soundSetShot[] = {"SPAR01_Shot_SoundSet","SPAR01_Tail_SoundSet","SPAR01_InteriorTail_SoundSet"};
			};
		};
		class single_medium_optics1: Single
		{
			aiRateOfFire = 5;
			aiRateOfFireDistance = 500;
			maxRange = 450;
			maxRangeProbab = 0.3;
			midRange = 300;
			midRangeProbab = 0.7;
			minRange = 5;
			minRangeProbab = 0.2;
			requiredOpticType = 1;
			showToPlayer = 0;
		};
		class single_medium_optics2: single_medium_optics1
		{
			aiRateOfFire = 6;
			aiRateOfFireDistance = 600;
			maxRange = 600;
			maxRangeProbab = 0.05;
			midRange = 400;
			midRangeProbab = 0.7;
			minRange = 100;
			minRangeProbab = 0.1;
			requiredOpticType = 2;
		};
		class WeaponSlotsInfo
		{
			allowedSlots[] = {901};
			Mass = 68;
			class MuzzleSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\MUZZLE";
				compatibleItems[] = {};
			};
			class CowsSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\TOP";
				compatibleItems[] = {};
			};
			class PointerSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\SIDE";
				compatibleItems[] = {};
			};
			class UnderBarrelSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\UNDERBARREL";
				compatibleItems[] = {};
			};
		};
	};
	class sp_fwa_smg_9mm_base: sp_fwa_rifle_base
	{
		aiDispersionCoefX = 4;
		aiDispersionCoefY = 5;
		bullet1[] = {"A3\sounds_f\weapons\shells\9mm\metal_9mm_01",0.501187,1,15};
		bullet10[] = {"A3\sounds_f\weapons\shells\9mm\grass_9mm_02",0.398107,1,15};
		bullet11[] = {"A3\sounds_f\weapons\shells\9mm\grass_9mm_03",0.398107,1,15};
		bullet12[] = {"A3\sounds_f\weapons\shells\9mm\grass_9mm_04",0.398107,1,15};
		bullet2[] = {"A3\sounds_f\weapons\shells\9mm\metal_9mm_02",0.501187,1,15};
		bullet3[] = {"A3\sounds_f\weapons\shells\9mm\metal_9mm_03",0.501187,1,15};
		bullet4[] = {"A3\sounds_f\weapons\shells\9mm\metal_9mm_04",0.501187,1,15};
		bullet5[] = {"A3\sounds_f\weapons\shells\9mm\dirt_9mm_01",0.501187,1,15};
		bullet6[] = {"A3\sounds_f\weapons\shells\9mm\dirt_9mm_02",0.501187,1,15};
		bullet7[] = {"A3\sounds_f\weapons\shells\9mm\dirt_9mm_03",0.501187,1,15};
		bullet8[] = {"A3\sounds_f\weapons\shells\9mm\dirt_9mm_04",0.501187,1,15};
		bullet9[] = {"A3\sounds_f\weapons\shells\9mm\grass_9mm_01",0.398107,1,15};
		changeFiremodeSound[] = {"A3\Sounds_F\arsenal\weapons\SMG\Sting\firemode_Sting",0.251189,1,5};
		descriptionShort = "Submachinegun<br />Caliber: 9x19 mm";
		dexterity = 1.8;
		discreteDistance[] = {100,200,300,400,500,600};
		discreteDistanceInitIndex = 0;
		drySound[] = {"A3\Sounds_F\arsenal\weapons\SMG\Sting\Dry_Sting",0.251189,1,10};
		handAnim[] = {"OFP2_ManSkeleton","\A3\Weapons_F_beta\Smgs\SMG_01\data\Anim\SMG_01.rtm"};
		inertia = 0.2;
		initSpeed = -1;
		magazines[] = {"30Rnd_45ACP_Mag_SMG_01","30Rnd_45ACP_Mag_SMG_01_tracer_green","30Rnd_45ACP_Mag_SMG_01_Tracer_Red","30Rnd_45ACP_Mag_SMG_01_Tracer_Yellow"};
		magazineWell[] = {"CBA_45ACP_Glock_Full"};
		maxZeroing = 200;
		maxRange = 300;
		modes[] = {"SemiAuto","Fullauto","Burst","BurstMid"};
		optics = 1;
		opticsDisablePeripherialVision = 1;
		recoil = "sp_fwa_recoil_smg_9mm_medium";
		reloadAction = "GestureReloadSMG_01";
		reloadMagazineSound[] = {"A3\Sounds_F\arsenal\weapons\SMG\Sting\reload_sting",1,1,10};
		selectionFireAnim = "muzzleFlash";
		soundBullet[] = {"bullet1",0.083,"bullet2",0.083,"bullet3",0.083,"bullet4",0.083,"bullet5",0.083,"bullet6",0.083,"bullet7",0.083,"bullet8",0.083,"bullet9",0.083,"bullet10",0.083,"bullet11",0.083,"bullet12",0.083};
		weaponInfoType = "RscWeaponZeroing";
		class GunParticles
		{
			class Particle1
			{
				effectName = "sp_fwa_ComplexEffect_fire_gas_smallarms";
				positionName = "usti hlavne";
				directionName = "konec hlavne";
			};
			class Particle2
			{
				effectName = "sp_fwa_ComplexEffect_fire_haze_smallarms";
				positionName = "usti hlavne";
				directionName = "konec hlavne";
			};
			class Particle3
			{
				effectName = "sp_fwa_ComplexEffect_fire_gas_smallarms_subtle";
				positionName = "Nabojniceend";
				directionName = "Nabojnicestart";
			};
		};
		class Library
		{
			libTextDesc = "This is an FWA firearm in need of a description.";
		};
		class FullAuto: Mode_FullAuto
		{
			aiRateOfFire = 1e-06;
			aiRateOfFireDistance = 50;
			dispersion = 0.00316;
			maxRange = 50;
			maxRangeProbab = 0.1;
			midRange = 15;
			midRangeProbab = 0.7;
			minRange = 0;
			minRangeProbab = 0.9;
			reloadTime = 0.08;
			class SilencedSound
			{
				SoundSetShot[] = {"sp_fwa_556_semiauto_silencerShot_SoundSet","SMGSting_silencerTail_SoundSet","SMGSting_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				SoundSetShot[] = {"SMGSting_Shot_SoundSet","SMGSting_Tail_SoundSet","SMGSting_InteriorTail_SoundSet"};
			};
		};
		class SemiAuto: Mode_SemiAuto
		{
			aiRateOfFire = 2;
			aiRateOfFireDispersion = 2;
			aiRateOfFireDistance = 300;
			dispersion = 0.00116;
			maxRange = 300;
			maxRangeProbab = 0.05;
			midRange = 250;
			midRangeProbab = 0.7;
			minRange = 220;
			minRangeProbab = 0.3;
			reloadTime = 0.08;
			class SilencedSound
			{
				SoundSetShot[] = {"sp_fwa_556_semiauto_silencerShot_SoundSet","SMGSting_silencerTail_SoundSet","SMGSting_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				SoundSetShot[] = {"SMGSting_Shot_SoundSet","SMGSting_Tail_SoundSet","SMGSting_InteriorTail_SoundSet"};
			};
		};
		class Burst: FullAuto
		{
			aiRateOfFire = 2;
			aiRateOfFireDispersion = 2;
			reloadTime = 0.1;
			showToPlayer = 0;
			maxRange = 200;
			midRange = 100;
			burst = 3;
			burstRangeMax = 5;
		};
		class BurstMid: Burst
		{
			aiRateOfFire = 4;
			aiRateOfFireDispersion = 2;
			reloadTime = 0.1;
			showToPlayer = 0;
			maxRange = 250;
			midRange = 100;
			burst = 2;
			burstRangeMax = 4;
		};
		class WeaponSlotsInfo
		{
			allowedSlots[] = {901};
			Mass = 68;
			class MuzzleSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\MUZZLE";
				compatibleItems[] = {};
			};
			class CowsSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\TOP";
				compatibleItems[] = {};
			};
			class PointerSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\SIDE";
				compatibleItems[] = {};
			};
			class UnderBarrelSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\UNDERBARREL";
				compatibleItems[] = {};
			};
		};
	};
	class Pistol;
	class Pistol_Base_F: Pistol
	{
		class Library;
		class WeaponSlotsInfo;
	};
	class sp_fwa_pistol_base: Pistol_Base_F
	{
		afMax = 0;
		author = "Free World Armoury";
		cartridgePos = "cartridge_pos";
		cartridgeVel = "cartridge_dir";
		descriptionShort = "Pistol<br />Caliber: 9x19 mm";
		dexterity = 1.8;
		displayName = "P1 9x19 mm";
		drySound[] = {"A3\Sounds_F\arsenal\weapons\Pistols\P07\dry_P07",0.398107,1,20};
		hiddenSelections[] = {"texWeapon_01","texWeapon_02","texWeapon_03","texWeapon_04"};
		htMax = 480;
		htMin = 1;
		inertia = 0.2;
		initSpeed = -1;
		irLaserEnd = "laser_dir";
		irLaserPos = "laser_pos";
		magazines[] = {"16Rnd_9x21_Mag","16Rnd_9x21_red_Mag","16Rnd_9x21_green_Mag","16Rnd_9x21_yellow_Mag","30Rnd_9x21_Mag","30Rnd_9x21_Red_Mag","30Rnd_9x21_Yellow_Mag","30Rnd_9x21_Green_Mag"};
		magazineWell[] = {"Pistol_9x21"};
		maxZeroing = 100;
		memoryPointCamera = "eye";
		mFact = 1;
		mfMax = 0;
		model = "\A3\weapons_F\Pistols\Rook40\Rook40_F.p3d";
		modes[] = {"manual"};
		muzzleEnd = "konec hlavne";
		muzzlePos = "usti hlavne";
		recoil = "sp_fwa_recoil_pistol_9mm_medium";
		reloadAction = "GestureReloadPistol";
		reloadMagazineSound[] = {"A3\Sounds_F\arsenal\weapons\Pistols\P07\reload_P07",1,1,10};
		selectionFireAnim = "muzzleflash";
		shotEnd = "konec hlavne";
		shotPos = "usti hlavne";
		tBody = 100;
		UiPicture = "\A3\weapons_f\data\UI\icon_regular_CA.paa";
		class manual: Mode_SemiAuto
		{
			aiRateOfFire = 2;
			aiRateOfFireDispersion = 2;
			aiRateOfFireDistance = 25;
			dispersion = 0.0066323;
			maxRange = 50;
			maxRangeProbab = 0.1;
			midRange = 25;
			midRangeProbab = 0.6;
			minRange = 5;
			minRangeProbab = 0.3;
			reloadTime = 0.1;
			sounds[] = {"StandardSound","SilencedSound"};
			class SilencedSound
			{
				SoundSetShot[] = {"Rook40_silencerShot_SoundSet","Rook40_silencerTail_SoundSet","Rook40_silencerInteriorTail_SoundSet"};
			};
			class StandardSound
			{
				soundSetShot[] = {"Rook40_Shot_SoundSet","Rook40_Tail_SoundSet","Rook40_InteriorTail_SoundSet"};
			};
		};
		class Library: Library
		{
			libTextDesc = "This is an FWA firearm in need of a description.";
		};
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			Mass = 17.3333;
			class MuzzleSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\MUZZLE";
				compatibleItems[] = {};
			};
			class CowsSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\TOP";
				compatibleItems[] = {};
			};
			class PointerSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\SIDE";
				compatibleItems[] = {};
			};
			class UnderBarrelSlot: SlotInfo
			{
				linkProxy = "\A3\data_f\proxies\weapon_slots\UNDERBARREL";
				compatibleItems[] = {};
			};
		};
	};
};
class CfgMagazineWells
{
	class CBA_3006_Belt
	{
		sp_Magazines[] = {"sp_fwa_50Rnd_3006_mag","sp_fwa_50Rnd_3006_mag_turret","sp_fwa_50Rnd_3006_mag_ball","sp_fwa_100Rnd_3006_mag","sp_fwa_100Rnd_3006_mag_turret","sp_fwa_100Rnd_3006_mag_ball","sp_fwa_200Rnd_3006_mag","sp_fwa_200Rnd_3006_mag_turret","sp_fwa_200Rnd_3006_mag_ball"};
	};
};
class CfgMagazines
{
	class sp_fwa_50Rnd_762_mag;
	class sp_fwa_50Rnd_762_mag_turret;
	class sp_fwa_50Rnd_762_mag_ball;
	class sp_fwa_100Rnd_762_mag;
	class sp_fwa_100Rnd_762_mag_turret;
	class sp_fwa_100Rnd_762_mag_ball;
	class sp_fwa_200Rnd_762_mag;
	class sp_fwa_200Rnd_762_mag_turret;
	class sp_fwa_200Rnd_762_mag_ball;
	class sp_fwa_50Rnd_3006_mag: sp_fwa_50Rnd_762_mag
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 50<br />Used in: AA52";
		displayname = ".30-06 50rnd Belt (4B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_50Rnd_3006_mag_turret: sp_fwa_50Rnd_762_mag_turret
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 50<br />Used in: AA52";
		displayname = ".30-06 50rnd Belt (1B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_50Rnd_3006_mag_ball: sp_fwa_50Rnd_762_mag_ball
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 50<br />Used in: AA52";
		displayname = ".30-06 50rnd Belt (Ball)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_100Rnd_3006_mag: sp_fwa_100Rnd_762_mag
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 100<br />Used in: AA52";
		displayname = ".30-06 100rnd Belt (4B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_100Rnd_3006_mag_turret: sp_fwa_100Rnd_762_mag_turret
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 100<br />Used in: AA52";
		displayname = ".30-06 100rnd Belt (1B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_100Rnd_3006_mag_ball: sp_fwa_100Rnd_762_mag_ball
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 100<br />Used in: AA52";
		displayname = ".30-06 100rnd Belt (Ball)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_200Rnd_3006_mag: sp_fwa_200Rnd_762_mag
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 200<br />Used in: AA52";
		displayname = ".30-06 200rnd Belt (4B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_200Rnd_3006_mag_turret: sp_fwa_200Rnd_762_mag_turret
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 200<br />Used in: AA52";
		displayname = ".30-06 200rnd Belt (1B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_200Rnd_3006_mag_ball: sp_fwa_200Rnd_762_mag_ball
	{
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 200<br />Used in: AA52";
		displayname = ".30-06 200rnd Belt (Ball)";
		modelSpecial = "";
		ammo = "sp_fwa_B_3006_Tracer_Red";
	};
	class sp_fwa_50Rnd_765_french_mag: sp_fwa_50Rnd_762_mag
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 50<br />Used in: AA52";
		displayname = "7.5mm 50rnd Belt (4B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_50Rnd_765_french_mag_turret: sp_fwa_50Rnd_762_mag_turret
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 50<br />Used in: AA52";
		displayname = "7.5mm 50rnd Belt (1B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_50Rnd_765_french_mag_ball: sp_fwa_50Rnd_762_mag_ball
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 50<br />Used in: AA52";
		displayname = "7.5mm 50rnd Belt (Ball)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_100Rnd_75_french_mag: sp_fwa_100Rnd_762_mag
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 100<br />Used in: AA52";
		displayname = "7.5mm 100rnd Belt (4B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_100Rnd_75_french_mag_turret: sp_fwa_100Rnd_762_mag_turret
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 100<br />Used in: AA52";
		displayname = "7.5mm 100rnd Belt (1B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_100Rnd_75_french_mag_ball: sp_fwa_100Rnd_762_mag_ball
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 100<br />Used in: AA52";
		displayname = "7.5mm 100rnd Belt (Ball)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_200Rnd_75_french_mag: sp_fwa_200Rnd_762_mag
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 200<br />Used in: AA52";
		displayname = "7.5mm 200rnd Belt (4B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_200Rnd_75_french_mag_turret: sp_fwa_200Rnd_762_mag_turret
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 200<br />Used in: AA52";
		displayname = "7.5mm 200rnd Belt (1B/1T)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class sp_fwa_200Rnd_75_french_mag_ball: sp_fwa_200Rnd_762_mag_ball
	{
		descriptionshort = "Caliber: 7.5x54 mm French<br />Rounds: 200<br />Used in: AA52";
		displayname = "7.5mm 200rnd Belt (Ball)";
		modelSpecial = "";
		ammo = "sp_fwa_B_75x54_Tracer_Red";
	};
	class 20Rnd_762x51_Mag;
	class sp_fwa_stripper_5rnd_75: 20Rnd_762x51_Mag
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 7.5x54mm French<br />Rounds: 5";
		displayname = "7.5x54mm 5rnd Stripper Clip";
		mass = 3;
		count = 5;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		ammo = "sp_fwa_B_75x54_Tracer_Red";
		displaynameshort = "Ball";
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class sp_fwa_stripper_5rnd_3006: 20Rnd_762x51_Mag
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: .30-06 Springfield<br />Rounds: 5";
		displayname = ".30-06 5rnd Stripper Clip";
		mass = 3;
		count = 5;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		ammo = "sp_fwa_B_3006_Tracer_Red";
		displaynameshort = "Ball";
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class sp_fwa_stripper_5rnd_303: 20Rnd_762x51_Mag
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: .303 British<br />Rounds: 5";
		displayname = ".303 5rnd Stripper Clip";
		mass = 3;
		count = 5;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		ammo = "sp_fwa_B_303_Tracer_Red";
		displaynameshort = "Ball";
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class sp_fwa_stripper_5rnd_762CETME: 20Rnd_762x51_Mag
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 7.62Ã—51mm CETME<br />Rounds: 5";
		displayname = "7.62mm CETME 5rnd Stripper Clip";
		mass = 3;
		count = 5;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		ammo = "sp_fwa_B_762x51_CETME_Tracer_Red";
		displaynameshort = "Ball";
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class sp_fwa_stripper_5rnd_762: 20Rnd_762x51_Mag
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 7.62x51mm NATO<br />Rounds: 5";
		displayname = "7.62x51mm 5rnd Stripper Clip";
		mass = 3;
		count = 5;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		displaynameshort = "Ball";
		ammo = "B_762x51_Tracer_Red";
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class 30Rnd_556x45_Stanag;
	class sp_fwa_stripper_5rnd_556: 30Rnd_556x45_Stanag
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 5.56x45mm NATO<br />Rounds: 5";
		displayname = "5.56x45mm 5rnd Stripper Clip";
		mass = 2;
		count = 5;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		displaynameshort = "Ball";
		ammo = "B_556x45_Ball_Tracer_Red";
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class sp_fwa_stripper_10rnd_556: sp_fwa_stripper_5rnd_556
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 5.56x45mm NATO<br />Rounds: 10";
		displayname = "5.56x45mm 10rnd Stripper Clip";
		mass = 4;
		count = 10;
		picture = "\sp_fwa_weapon_base\icons\stripperclip_icon_ca.paa";
	};
	class 30Rnd_9x21_Mag_SMG_02;
	class sp_fwa_box_50Rnd_9mm: 30Rnd_9x21_Mag_SMG_02
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 9mm<br />Rounds: 50";
		displayname = "9mm 50rnd Box Ammo";
		mass = 8;
		displaynameshort = "Ball";
		count = 50;
		tracersEvery = 0;
		lastRoundsTracer = 0;
		ammo = "B_9x21_Ball_Tracer_Red";
		picture = "\sp_fwa_weapon_base\icons\9mmbox_icon_ca.paa";
	};
	class sp_fwa_box_20Rnd_45acp: 30Rnd_9x21_Mag_SMG_02
	{
		author = "Free World Armoury";
		descriptionshort = "Caliber: 45acp<br />Rounds: 20";
		displayname = "45ACP 50rnd Box Ammo";
		modelSpecialIsProxy = 1;
		mass = 4;
		displaynameshort = "Ball";
		count = 20;
		ammo = "B_45ACP_Ball";
		picture = "\sp_fwa_weapon_base\icons\9mmbox_icon_ca.paa";
	};
};
class CfgAmmo
{
	class B_45ACP_Ball_Yellow;
	class sp_fwa_B_45ACP_Ball_Red: B_45ACP_Ball_Yellow
	{
		model = "\A3\Weapons_f\Data\bullettracer\tracer_red";
	};
	class B_762x51_Tracer_Red;
	class sp_fwa_B_75x54_Tracer_Red: B_762x51_Tracer_Red
	{
		icon = "";
	};
	class sp_fwa_B_75x54_Blank: sp_fwa_B_75x54_Tracer_Red
	{
		timeToLive = 0.01;
	};
	class sp_fwa_B_75x55_Tracer_Red: B_762x51_Tracer_Red
	{
		icon = "";
	};
	class sp_fwa_B_75x55_Projector: B_762x51_Tracer_Red
	{
		timeToLive = 0.01;
	};
	class sp_fwa_B_3006_Tracer_Red: B_762x51_Tracer_Red
	{
		icon = "";
	};
	class sp_fwa_B_303_Tracer_Red: B_762x51_Tracer_Red
	{
		icon = "";
	};
	class sp_fwa_B_735x51_Tracer_Red: B_762x51_Tracer_Red
	{
		icon = "";
	};
	class sp_fwa_B_792x57_Tracer_Yellow: B_762x51_Tracer_Red
	{
		icon = "";
		model = "\A3\Weapons_f\Data\bullettracer\tracer_yellow";
	};
	class sp_fwa_B_792x57_Tracer_Red: B_762x51_Tracer_Red
	{
		icon = "";
	};
	class B_556x45_Ball_Tracer_Red;
	class sp_fwa_B_30Carbine_Ball_Red: B_556x45_Ball_Tracer_Red
	{
		hit = 7;
		cartridge = "sp_fwa_FxCartridge_30Carbine";
		typicalSpeed = 610;
	};
	class B_762x39_Ball_Green_F;
	class sp_fwa_B_762x39_Ball_Yellow_F: B_762x39_Ball_Green_F
	{
		model = "\A3\Weapons_f\Data\bullettracer\tracer_yellow";
	};
	class sp_fwa_B_762x39_Ball_Red_F: B_762x39_Ball_Green_F
	{
		model = "\A3\Weapons_f\Data\bullettracer\tracer_red";
	};
};
class CfgRecoils
{
	class recoil_default;
	class sp_fwa_recoil_base: recoil_default
	{
		kickBack[] = {0.03,0.06};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.3,1,0.3,0.2};
		permanent = 0.1;
		temporary = 0.03;
	};
	class sp_fwa_recoil_battlerifle_762_medium: sp_fwa_recoil_base
	{
		kickBack[] = {0.04,0.07};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.4,1.5,0.6,0.4};
		permanent = 0.1;
		temporary = 0.01;
	};
	class sp_fwa_recoil_battlerifle_762_light: sp_fwa_recoil_battlerifle_762_medium
	{
		kickBack[] = {0.036,0.063};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.36,1.35,0.54,0.46};
		permanent = 0.1;
		temporary = 0.01;
	};
	class sp_fwa_recoil_battlerifle_762_heavy: sp_fwa_recoil_battlerifle_762_medium
	{
		kickBack[] = {0.044,0.077};
		muzzleInner[] = {0,0,0.11,0.11};
		muzzleOuter[] = {0.44,1.65,0.66,0.44};
		permanent = 0.11;
		temporary = 0.011;
	};
	class sp_fwa_recoil_battlerifle_762_super: sp_fwa_recoil_battlerifle_762_medium
	{
		kickBack[] = {0.048,0.084};
		muzzleInner[] = {0,0,0.12,0.12};
		muzzleOuter[] = {0.48,1.8,0.72,0.48};
		permanent = 0.12;
		temporary = 0.012;
	};
	class sp_fwa_recoil_assaultrifle_556_medium: sp_fwa_recoil_base
	{
		kickBack[] = {0.01,0.03};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.1,0.6,0.2,0.2};
		permanent = 0.1;
		temporary = 0.01;
	};
	class sp_fwa_recoil_assaultrifle_556_light: sp_fwa_recoil_assaultrifle_556_medium
	{
		kickBack[] = {0.009,0.027};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.1,0.54,0.18,0.18};
		permanent = 0.1;
		temporary = 0.01;
	};
	class sp_fwa_recoil_assaultrifle_556_heavy: sp_fwa_recoil_assaultrifle_556_medium
	{
		kickBack[] = {0.011,0.033};
		muzzleInner[] = {0,0,0.11,0.11};
		muzzleOuter[] = {0.11,0.66,0.22,0.22};
		permanent = 0.11;
		temporary = 0.011;
	};
	class sp_fwa_recoil_assaultrifle_556_super: sp_fwa_recoil_assaultrifle_556_medium
	{
		kickBack[] = {0.012,0.036};
		muzzleInner[] = {0,0,0.12,0.12};
		muzzleOuter[] = {0.12,0.72,0.24,0.24};
		permanent = 0.12;
		temporary = 0.012;
	};
	class sp_fwa_recoil_pistol_9mm_medium: sp_fwa_recoil_base
	{
		kickBack[] = {0.03,0.06};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.2,1,0.2,0.3};
		permanent = 0.1;
		temporary = 0.03;
	};
	class sp_fwa_recoil_pistol_45_medium: sp_fwa_recoil_base
	{
		kickBack[] = {0.033,0.066};
		muzzleInner[] = {0,0,0.11,0.11};
		muzzleOuter[] = {0.22,1.1,0.22,0.33};
		permanent = 0.11;
		temporary = 0.033;
	};
	class sp_fwa_recoil_smg_9mm_medium: sp_fwa_recoil_base
	{
		kickBack[] = {0.02,0.04};
		muzzleInner[] = {0,0,0.1,0.1};
		muzzleOuter[] = {0.2,0.4,0.3,0.3};
		permanent = 0.1;
		temporary = 0.01;
	};
	class sp_fwa_recoil_smg_9mm_light: sp_fwa_recoil_smg_9mm_medium
	{
		kickBack[] = {0.018,0.036};
		muzzleInner[] = {0,0,0.09,0.09};
		muzzleOuter[] = {0.18,0.36,0.27,0.27};
		permanent = 0.09;
		temporary = 0.009;
	};
	class sp_fwa_recoil_smg_9mm_heavy: sp_fwa_recoil_smg_9mm_medium
	{
		kickBack[] = {0.022,0.044};
		muzzleInner[] = {0,0,0.11,0.11};
		muzzleOuter[] = {0.22,0.44,0.33,0.33};
		permanent = 0.11;
		temporary = 0.011;
	};
	class sp_fwa_recoil_riflegrenade
	{
		kickBack[] = {0.2,0.24};
		permanent = 0.5;
		muzzleOuter[] = {7,6,0.2,0.2};
		temporary = 0.025;
	};
};
class sp_fwa_ComplexEffect_fire_gas_rifleGrenade
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_fire_gas_rifleGrenade";
	};
	class Sub2
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_fire_gas_rifleGrenadeRocketBack";
	};
	class Sub3
	{
		enabled = "isWaterSurface * (humidity interpolate [0.2,0.20001,1,0]) * (distToSurface interpolate [3,3.1,1,0])";
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "RifleAssaultDust1";
	};
};
class sp_fwa_ComplexEffect_fire_gas_smallarms
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "RifleAssaultCloud2";
	};
	class Sub2
	{
		enabled = "isWaterSurface * (humidity interpolate [0.2,0.20001,1,0]) * (distToSurface interpolate [1.3,1.31,1,0])";
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "RifleAssaultDust1";
	};
};
class sp_fwa_ComplexEffect_fire_gas_smallarms_small
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_fire_gas_smallarms_small";
	};
};
class sp_fwa_ComplexEffect_fire_gas_smallarms_subtle
{
	class Sub1
	{
		intensity = 0.2;
		interval = 1;
		lifeTime = 0.01;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_fire_gas_smallarms_subtle";
	};
};
class sp_fwa_ComplexEffect_fire_haze_smallarms
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_fire_haze_smallarms";
	};
};
class sp_fwa_ComplexEffect_fire_haze_smallarms_556
{
	class Sub1
	{
		intensity = 0.5;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_fire_haze_smallarms";
	};
};
class sp_fwa_ComplexEffect_eject_link
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_cloudlet_eject_link";
	};
};
class sp_fwa_ComplexEffect_eject_762
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		qualityLevel = 2;
		simulation = "particles";
		type = "sp_fwa_cloudlet_eject_762";
	};
};
class sp_fwa_ComplexEffect_eject_762_mag58
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		qualityLevel = 2;
		simulation = "particles";
		type = "sp_fwa_cloudlet_eject_762_mag58";
	};
};
class sp_fwa_ComplexEffect_eject_762_m60
{
	class Sub1
	{
		intensity = 1;
		interval = 1;
		lifeTime = 0.05;
		position[] = {0,0,0};
		qualityLevel = 2;
		simulation = "particles";
		type = "sp_fwa_cloudlet_eject_762_m60";
	};
};
class FlareShell;
class sp_fwa_flareshell_whitesmoke: FlareShell
{
	class Flare1
	{
		intensity = 1;
		interval = 1;
		position[] = {0,0,0};
		simulation = "particles";
		type = "sp_fwa_flareshell_whitesmoke_cloudlet";
	};
	class Light1
	{
		intensity = 1;
		interval = 1;
		position[] = {0,0,0};
		simulation = "light";
		type = "FlareLight";
	};
};
class sp_fwa_explosioneffect_wp
{
	class WPSmoke1
	{
		intensity = 0.5;
		interval = 1;
		lifeTime = 30;
		type = "sp_fwa_explosionparticles_wp";
		simulation = "particles";
		position[] = {0,0,0};
	};
};
class sp_fwa_explosioneffect_prac
{
	class WPSmoke1
	{
		intensity = 0.5;
		interval = 1;
		lifeTime = 3;
		type = "sp_fwa_explosionparticles_prac";
		simulation = "particles";
		position[] = {0,0,0};
	};
};
class cfgCloudlets
{
	class RifleAssaultCloud1;
	class sp_fwa_cloudlet_fire_gas_smallarms: RifleAssaultCloud1{};
	class sp_fwa_cloudlet_fire_gas_rifleGrenade: sp_fwa_cloudlet_fire_gas_smallarms
	{
		color[] = {{0.9,0.9,0.9,0.14},{0.9,0.9,0.9,0.028},{0.9,0.9,0.9,0.016},{0.9,0.9,0.9,0.001}};
		size[] = {0.3,1.5,3};
		lifeTime = 5;
		moveVelocity[] = {"1.5*directionX","1.5*directionY","1.5*directionZ"};
	};
	class sp_fwa_cloudlet_fire_gas_rifleGrenadeRocketBack: sp_fwa_cloudlet_fire_gas_smallarms
	{
		color[] = {{0.9,0.9,0.9,0.14},{0.9,0.9,0.9,0.028},{0.9,0.9,0.9,0.016},{0.9,0.9,0.9,0.001}};
		size[] = {0.3,1.5,3};
		lifeTime = 2;
		moveVelocity[] = {"-1.5*directionX","-1.5*directionY","-1.5*directionZ"};
	};
	class CaselessAmmoCloud1;
	class sp_fwa_cloudlet_fire_gas_smallarms_small: CaselessAmmoCloud1
	{
		moveVelocity[] = {".25*directionX",".25*directionY",".25*directionZ"};
		moveVelocityVarConst[] = {0.1,0.1,0.1};
		color[] = {{0.35,0.35,0.35,0.2},{0.35,0.35,0.35,0.1},{0.35,0.35,0.35,0.05},{0.35,0.35,0.35,0.01}};
		sizeVar = 0.2;
	};
	class sp_fwa_cloudlet_fire_gas_smallarms_subtle: sp_fwa_cloudlet_fire_gas_smallarms_small
	{
		color[] = {{0.35,0.35,0.35,0.1},{0.35,0.35,0.35,0.05},{0.35,0.35,0.35,0.025},{0.35,0.35,0.35,0.01}};
	};
	class MachineGunCartridge338;
	class sp_fwa_cloudlet_eject_762: MachineGunCartridge338
	{
		moveVelocity[] = {"directionX","directionY","directionZ"};
		size[] = {1};
		lifeTime = 10;
	};
	class sp_fwa_cloudlet_eject_762_mag58: sp_fwa_cloudlet_eject_762
	{
		moveVelocity[] = {"0","0","-.5*directionZ"};
	};
	class sp_fwa_cloudlet_eject_762_m60: sp_fwa_cloudlet_eject_762{};
	class sp_fwa_cloudlet_eject_link: MachineGunCartridge338
	{
		moveVelocity[] = {"directionX","directionY","directionZ"};
		particleShape = "\A3\data_f\ParticleEffects\Universal\AmmoBelt_Links.p3d";
		lifeTime = 10;
	};
	class sp_fwa_cloudlet_fire_haze_smallarms
	{
		angle = 0;
		angleVar = 1;
		animationName = "";
		animationSpeed[] = {2,1};
		animationSpeedCoef = 1;
		beforeDestroyScript = "";
		blockAIVisibility = 0;
		circleRadius = 0;
		circleVelocity[] = {0,0,0};
		color[] = {{0.06,0.06,0.06,0.22},{0.3,0.3,0.3,0.18},{0.3,0.3,0.3,0.15},{0.3,0.3,0.3,0.12},{0.3,0.3,0.3,0.05}};
		colorCoef[] = {1,1,1,1};
		colorVar[] = {0,0,0,0};
		destroyOnWaterSurface = 1;
		interval = 0.005;
		lifeTime = 0.75;
		lifeTimeVar = 0;
		moveVelocity[] = {"-0.15*directionX","-0.15*directionY","-0.15*directionZ"};
		moveVelocityVar[] = {0,0,0};
		MoveVelocityVarConst[] = {0,0,0};
		onTimerScript = "";
		particleFSFrameCount = 1;
		particleFSIndex = 0;
		particleFSLoop = 0;
		particleFSNtieth = 1;
		particleShape = "\a3\data_f\ParticleEffects\Universal\Refract";
		particleType = "Billboard";
		position[] = {"positionX","positionY","positionZ"};
		positionVar[] = {0,0,0};
		positionVarConst[] = {0,0,0};
		randomDirectionIntensity = 0.05;
		randomDirectionIntensityVar = 0;
		randomDirectionPeriod = 0.1;
		randomDirectionPeriodVar = 0;
		rotationVelocity = 1;
		rotationVelocityVar = 20;
		rubbing = 0.1;
		size[] = {0.2};
		sizeCoef = 0.5;
		sizeVar = 0.05;
		timerPeriod = 1.1;
		volume = 1;
		weight = 1.2;
	};
	class sp_fwa_cloudlet_fire_haze_rifleGrenade: sp_fwa_cloudlet_fire_haze_smallarms
	{
		lifeTime = 2;
		size[] = {0.75};
	};
	class FlareShell;
	class sp_fwa_flareshell_whitesmoke_cloudlet: FlareShell
	{
		interval = 0.015;
		lifeTime = 7;
		lifeTimeVar = 1;
		sizeVar = 0.5;
		size[] = {0.1,1,3};
		color[] = {{0.75,0.75,0.75,1},{0.75,0.75,0.75,0.75},{0.75,0.75,0.75,0}};
	};
	class sp_fwa_flareshell_whitesmoke_cloudlet_small: FlareShell
	{
		interval = 0.015;
		lifeTime = 7;
		lifeTimeVar = 1;
		sizeVar = 0.5;
		size[] = {0.1,0.2,1};
		color[] = {{0.75,0.75,0.75,1},{0.75,0.75,0.75,0.75},{0.75,0.75,0.75,0}};
	};
	class WPCloud;
	class sp_fwa_explosionparticles_wp: WPCloud
	{
		color[] = {{0.75,0.75,0.75,1},{0.75,0.75,0.75,0.75},{0.75,0.75,0.75,0}};
		size[] = {2,5,6.5,7,8,9,10,11};
		lifeTime = 15;
		lifeTimeVar = 2;
		blockAIVisibility = "true";
		positionVar[] = {0.5,0.5,0};
		particleEffects = "";
		damageType = "Fire";
		coreIntensity = 100;
		coreDistance = 3;
		damageTime = 0.1;
	};
	class sp_fwa_explosionparticles_prac: WPCloud
	{
		color[] = {{0.75,0.75,0.75,1},{0.75,0.75,0.75,0.75},{0.75,0.75,0.75,0}};
		size[] = {1,2};
		lifeTime = 3;
		lifeTimeVar = 2;
		blockAIVisibility = "true";
		positionVar[] = {0.5,0.25,0};
		particleEffects = "";
	};
};
class cfgMods
{
	author = "TepacheLoco";
	timepacked = "1645293576";
};
