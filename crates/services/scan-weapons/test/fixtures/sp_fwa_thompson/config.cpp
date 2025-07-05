////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 11:18:15 2025 : 'file' last modified on Fri Feb 18 23:48:22 2022
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class sp_fwa_thompson
	{
		requiredaddons[] = {"A3_Data_F_Tank_Loadorder","A3_Data_F","A3_UI_F","A3_Anims_F","A3_Anims_F_Config_Sdr","A3_Weapons_F","sp_fwa_weapon_base"};
		requiredversion = 0.1;
		units[] = {};
		weapons[] = {"sp_fwa_smg_thompson_m1a1","sp_fwa_smg_thompson_m1928a1","sp_fwa_smg_thompson_m1928"};
		magazines[] = {"sp_fwa_30Rnd_45acp_thompson_m1a1","sp_fwa_30Rnd_45acp_thompson_m1a1_Tracer","sp_fwa_30Rnd_45acp_thompson_m1a1_Ball","sp_fwa_20Rnd_45acp_thompson_m1a1","sp_fwa_20Rnd_45acp_thompson_m1a1_Tracer","sp_fwa_20Rnd_45acp_thompson_m1a1_Ball","sp_fwa_50Rnd_45acp_thompson_m1a1","sp_fwa_50Rnd_45acp_thompson_m1a1_Tracer","sp_fwa_50Rnd_45acp_thompson_m1a1_Ball"};
	};
};
class CfgSoundSets
{
	class SMGVermin_Shot_SoundSet;
	class sp_fwa_thompson_Shot_SoundSet: SMGVermin_Shot_SoundSet
	{
		soundShaders[] = {"sp_fwa_thompson_Closure_SoundShader","sp_fwa_thompson_closeShot_SoundShader","sp_fwa_thompson_midShot_SoundShader","sp_fwa_thompson_distShot_SoundShader"};
	};
};
class CfgSoundShaders
{
	class SMGVermin_Closure_SoundShader;
	class sp_fwa_thompson_Closure_SoundShader: SMGVermin_Closure_SoundShader{};
	class SMGVermin_closeShot_SoundShader;
	class sp_fwa_thompson_closeShot_SoundShader: SMGVermin_closeShot_SoundShader
	{
		samples[] = {{"sp_fwa_thompson\sound\thompson_single_close_01.wav",0.25},{"sp_fwa_thompson\sound\thompson_single_close_02.wav",0.25},{"sp_fwa_thompson\sound\thompson_single_close_03.wav",0.25},{"sp_fwa_thompson\sound\thompson_single_close_04.wav",0.25}};
		volume = 1;
	};
	class SMGVermin_midShot_SoundShader;
	class sp_fwa_thompson_midShot_SoundShader: SMGVermin_midShot_SoundShader{};
	class SMGVermin_distShot_SoundShader;
	class sp_fwa_thompson_distShot_SoundShader: SMGVermin_distShot_SoundShader{};
};
class CfgWeapons
{
	class SlotInfo;
	class ItemCore;
	class sp_fwa_rifle_base;
	class sp_fwa_smg_9mm_base: sp_fwa_rifle_base
	{
		class SemiAuto;
		class FullAuto;
		class Burst;
		class BurstMid;
		class WeaponSlotsInfo;
	};
	class sp_fwa_smg_thompson_m1a1: sp_fwa_smg_9mm_base
	{
		displayName = "Auto Ordnance M1A1 Thompson";
		author = "Free World Armoury & Luchador";
		baseWeapon = "sp_fwa_smg_thompson_m1a1";
		magazines[] = {"sp_fwa_20Rnd_45acp_thompson_m1a1"};
		magazineWell[] = {"CBA_45ACP_Thompson_Stick"};
		model = "sp_fwa_thompson\sp_fwa_thompson_m1a1";
		recoil = "sp_fwa_recoil_smg_9mm_heavy";
		picture = "\sp_fwa_thompson\icons\thompson_m1a1_icon_ca.paa";
		handAnim[] = {"OFP2_ManSkeleton","\sp_fwa_thompson\anims\m1a1_hand_01.rtm"};
		scope = 2;
		magazineReloadSwitchPhase = 0.3;
		discreteDistance[] = {150};
		discreteDistanceInitIndex = 0;
		selectionFireAnim = "muzzleFlash";
		reloadTime = 0.13;
		class SemiAuto: SemiAuto
		{
			reloadTime = 0.0916;
			maxRange = 300;
			maxRangeProbab = 0.05;
			midRange = 220;
			midRangeProbab = 0.7;
			minRange = 230;
			minRangeProbab = 0.3;
			class BaseSoundModeType;
			class StandardSound: BaseSoundModeType
			{
				SoundSetShot[] = {"sp_fwa_thompson_Shot_SoundSet","SMGVermin_Tail_SoundSet","SMGVermin_InteriorTail_SoundSet"};
			};
		};
		class FullAuto: FullAuto
		{
			reloadTime = 0.0916;
			class BaseSoundModeType;
			class StandardSound: BaseSoundModeType
			{
				SoundSetShot[] = {"sp_fwa_thompson_Shot_SoundSet","SMGVermin_Tail_SoundSet","SMGVermin_InteriorTail_SoundSet"};
			};
		};
		class Burst: Burst
		{
			reloadTime = 0.0916;
			class BaseSoundModeType;
			class StandardSound: BaseSoundModeType
			{
				SoundSetShot[] = {"sp_fwa_thompson_Shot_SoundSet","SMGVermin_Tail_SoundSet","SMGVermin_InteriorTail_SoundSet"};
			};
		};
		class BurstMid: BurstMid
		{
			maxRange = 250;
			maxRangeProbab = 0.05;
			midRange = 220;
			midRangeProbab = 0.7;
			reloadTime = 0.0916;
			class BaseSoundModeType;
			class StandardSound: BaseSoundModeType
			{
				SoundSetShot[] = {"sp_fwa_thompson_Shot_SoundSet","SMGVermin_Tail_SoundSet","SMGVermin_InteriorTail_SoundSet"};
			};
		};
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			mass = 110;
		};
	};
	class sp_fwa_smg_thompson_m1928a1: sp_fwa_smg_thompson_m1a1
	{
		displayName = "Auto Ordnance M1928A1 Thompson";
		baseWeapon = "sp_fwa_smg_thompson_m1928a1";
		model = "sp_fwa_thompson\sp_fwa_thompson_m1928a1";
		recoil = "sp_fwa_recoil_smg_9mm_medium";
		magazines[] = {"sp_fwa_30Rnd_45acp_thompson_m1a1"};
		magazineWell[] = {"CBA_45ACP_Thompson_Stick","CBA_45ACP_Thompson_Drum"};
		discreteDistance[] = {135,90,135,180,230,275,320,365,410,455,500,545};
		discreteDistanceInitIndex = 0;
		maxZeroing = 550;
		handAnim[] = {"OFP2_ManSkeleton","\sp_fwa_thompson\anims\m1a1_hand_01.rtm"};
		picture = "\sp_fwa_thompson\icons\thompson_m1928a1_icon_ca.paa";
		class FullAuto: FullAuto
		{
			reloadTime = 0.0722;
		};
		class Burst: Burst
		{
			reloadTime = 0.0722;
		};
		class BurstMid: BurstMid
		{
			reloadTime = 0.0722;
		};
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			mass = 120;
		};
	};
	class sp_fwa_smg_thompson_m1928: sp_fwa_smg_thompson_m1928a1
	{
		displayName = "Auto Ordnance M1928 Thompson";
		baseWeapon = "sp_fwa_smg_thompson_m1928";
		model = "sp_fwa_thompson\sp_fwa_thompson_m1928";
		handAnim[] = {"OFP2_ManSkeleton","\sp_fwa_thompson\anims\m1928_hand_01.rtm"};
		picture = "\sp_fwa_thompson\icons\thompson_m1928_icon_ca.paa";
		class WeaponSlotsInfo: WeaponSlotsInfo
		{
			mass = 125;
		};
	};
};
class CfgMagazines
{
	class 30Rnd_9x21_Mag_SMG_02;
	class sp_fwa_30Rnd_45acp_thompson_m1a1: 30Rnd_9x21_Mag_SMG_02
	{
		author = "Free World Armoury & Luchador";
		descriptionshort = "Caliber: 45acp<br />Rounds: 30<br />Used in: Thompson";
		displayname = "45acp Thompson 30rnd 4B/1T";
		mass = 8;
		displaynameshort = "4B/1T";
		count = 30;
		modelSpecial = "sp_fwa_thompson\sp_fwa_thompson_30_mag";
		modelSpecialIsProxy = 1;
		tracersEvery = 5;
		lastRoundsTracer = 3;
		ammo = "sp_fwa_B_45ACP_Ball_Red";
		picture = "\sp_fwa_thompson\icons\mag_thompson_30_icon_ca.paa";
	};
	class sp_fwa_30Rnd_45acp_thompson_m1a1_Tracer: sp_fwa_30Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		displayname = "45acp Thompson 30rnd Tracer";
		tracersEvery = 1;
		displaynameshort = "Tracer";
	};
	class sp_fwa_30Rnd_45acp_thompson_m1a1_Ball: sp_fwa_30Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		displayname = "45acp Thompson 30rnd Ball";
		tracersEvery = 0;
		lastRoundsTracer = 0;
		displaynameshort = "Ball";
	};
	class sp_fwa_20Rnd_45acp_thompson_m1a1: sp_fwa_30Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		descriptionshort = "Caliber: 45acp<br />Rounds: 20<br />Used in: Thompson";
		displayname = "45acp Thompson 20rnd 3B/1T";
		mass = 6;
		count = 20;
		displaynameshort = "3B/1T";
		tracersEvery = 4;
		modelSpecial = "sp_fwa_thompson\sp_fwa_thompson_20_mag";
		picture = "\sp_fwa_thompson\icons\mag_thompson_20_icon_ca.paa";
	};
	class sp_fwa_20Rnd_45acp_thompson_m1a1_Tracer: sp_fwa_20Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		displayname = "45acp Thompson 20rnd Tracer";
		tracersEvery = 1;
		displaynameshort = "Tracer";
	};
	class sp_fwa_20Rnd_45acp_thompson_m1a1_Ball: sp_fwa_20Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		displayname = "45acp Thompson 20rnd Ball";
		tracersEvery = 0;
		lastRoundsTracer = 0;
		displaynameshort = "Ball";
	};
	class sp_fwa_50Rnd_45acp_thompson_m1a1: sp_fwa_30Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		descriptionshort = "Caliber: 45acp<br />Rounds: 50<br />Used in: Thompson";
		displayname = "45acp Thompson 50rnd 4B/1T";
		mass = 20;
		count = 50;
		modelSpecial = "sp_fwa_thompson\sp_fwa_thompson_50_mag";
		picture = "\sp_fwa_thompson\icons\mag_thompson_50_icon_ca.paa";
	};
	class sp_fwa_50Rnd_45acp_thompson_m1a1_Tracer: sp_fwa_50Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		displayname = "45acp Thompson 50rnd Tracer";
		tracersEvery = 1;
		displaynameshort = "Tracer";
	};
	class sp_fwa_50Rnd_45acp_thompson_m1a1_Ball: sp_fwa_50Rnd_45acp_thompson_m1a1
	{
		author = "Free World Armoury & Luchador";
		displayname = "45acp Thompson 50rnd Ball";
		tracersEvery = 0;
		lastRoundsTracer = 0;
		displaynameshort = "Ball";
	};
};
class CfgMagazineWells
{
	class CBA_45ACP_Thompson_Stick
	{
		sp_fwa_Magazines[] += {"sp_fwa_30Rnd_45acp_thompson_m1a1","sp_fwa_30Rnd_45acp_thompson_m1a1_Tracer","sp_fwa_30Rnd_45acp_thompson_m1a1_Ball","sp_fwa_20Rnd_45acp_thompson_m1a1","sp_fwa_20Rnd_45acp_thompson_m1a1_Tracer","sp_fwa_20Rnd_45acp_thompson_m1a1_Ball"};
	};
	class CBA_45ACP_Thompson_Drum
	{
		sp_fwa_Magazines[] += {"sp_fwa_50Rnd_45acp_thompson_m1a1","sp_fwa_50Rnd_45acp_thompson_m1a1_Tracer","sp_fwa_50Rnd_45acp_thompson_m1a1_Ball"};
	};
};
class cfgMods
{
	author = "TepacheLoco";
	timepacked = "1645181302";
};
