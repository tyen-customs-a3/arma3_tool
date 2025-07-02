////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:43 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_ace_tracer_compat
	{
		name = "ACE Tracer Compatibility";
		author = "PCA";
		units[] = {};
		weapons[] = {};
		requiredVersion = 1.6;
		requiredAddons[] = {"pca_mods_main","ace_tracers","CUP_Weapons_Ammunition"};
	};
};
class CfgAmmo
{
	class BulletBase;
	class B_556x45_Ball: BulletBase{};
	class B_762x51_Ball: BulletBase{};
	class B_20mm;
	class B_30mm_AP;
	class B_30mm_HE;
	class B_35mm_AA;
	class CUP_B_545x39_Ball: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_545x39_Ball_Subsonic: CUP_B_545x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_545x39_Ball_Tracer_Green: CUP_B_545x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_545x39_Ball_Tracer_Red: CUP_B_545x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_545x39_Ball_Tracer_White: CUP_B_545x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_545x39_Ball_Tracer_Yellow: CUP_B_545x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_762x39_Ball: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_762x39_Ball_Tracer_Green: CUP_B_762x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_762x39_Ball_Tracer_Red: CUP_B_762x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_762x39_Ball_Tracer_Yellow: CUP_B_762x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_762x39_Ball_Subsonic: CUP_B_762x39_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_762x51_noTracer: B_762x51_Ball{};
	class CUP_B_762x51_Tracer_Green: CUP_B_762x51_noTracer
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_762x51_Tracer_Red: CUP_B_762x51_noTracer
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_762x51_Tracer_Yellow: CUP_B_762x51_noTracer
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_762x51_Tracer_White: CUP_B_762x51_noTracer
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_762x51_Red_Tracer_3RndBurst: CUP_B_762x51_noTracer
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_762x51_White_Tracer_3RndBurst: CUP_B_762x51_noTracer
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_762x51_Tracer_Red_Splash: CUP_B_762x51_Tracer_Red{};
	class CUP_B_762x51_Tracer_White_Splash: CUP_B_762x51_Tracer_Red_Splash
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_303_Ball: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_762x54_Ball_White_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_762x54_Ball_Red_Tracer: CUP_B_762x54_Ball_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_762x54_Ball_Green_Tracer: CUP_B_762x54_Ball_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_762x54_Ball_Yellow_Tracer: CUP_B_762x54_Ball_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class B_127x107_Ball: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_127x107_Ball_Green_Tracer: B_127x107_Ball
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_127x108_Ball_Green_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_145x115_AP_Green_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_20mm_AP_Tracer_Red: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_20mm_AP_Tracer_Green: CUP_B_20mm_AP_Tracer_Red
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_20mm_AP_Tracer_Yellow: CUP_B_20mm_AP_Tracer_Red
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_20mm_APHE_Tracer_Red: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_20mm_APHE_Tracer_Green: CUP_B_20mm_APHE_Tracer_Red
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_20mm_APHE_Tracer_Yellow: CUP_B_20mm_APHE_Tracer_Red
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_20mm_AA: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_23mm_APHE_Tracer_Green: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_23mm_APHE_Tracer_Yellow: CUP_B_23mm_APHE_Tracer_Green
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_23mm_APHE_Tracer_Red: CUP_B_23mm_APHE_Tracer_Green
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_23mm_AA: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_23mm_APHE_No_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_23mm_APHE_Green_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_23mm_APHE_Yellow_Tracer: CUP_B_23mm_APHE_Green_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_30x113mm_M789_HEDP_Red_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_30x113mm_M789_HEDP_Green_Tracer: CUP_B_30x113mm_M789_HEDP_Red_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_30x113mm_M789_HEDP_Yellow_Tracer: CUP_B_30x113mm_M789_HEDP_Red_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_30x113mm_M789_HEDP_White_Tracer: CUP_B_30x113mm_M789_HEDP_Red_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_30mm_HE_Red_Tracer: B_30mm_HE
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_30mm_HE_Green_Tracer: B_30mm_HE
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_30mm_HE_Yellow_Tracer: B_30mm_HE
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_30mm_HE_White_Tracer: B_30mm_HE
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_30mm_AP_Red_Tracer: B_30mm_AP
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_30mm_AP_Green_Tracer: B_30mm_AP
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_30mm_AP_Yellow_Tracer: B_30mm_AP
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_30mm_AP_White_Tracer: B_30mm_AP
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_30mm_CAS_Red_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_25mm_HE_White_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_25mm_HE_Red_Tracer: CUP_B_25mm_HE_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_25mm_HE_Green_Tracer: CUP_B_25mm_HE_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_25mm_HE_Yellow_Tracer: CUP_B_25mm_HE_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
	class CUP_B_25mm_APFSDS_White_Tracer: BulletBase
	{
		model = "\z\ace\addons\tracers\ace_TracerWhite2.p3d";
	};
	class CUP_B_25mm_APFSDS_Red_Tracer: CUP_B_25mm_APFSDS_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerRed2.p3d";
	};
	class CUP_B_25mm_APFSDS_Green_Tracer: CUP_B_25mm_APFSDS_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerGreen2.p3d";
	};
	class CUP_B_25mm_APFSDS_Yellow_Tracer: CUP_B_25mm_APFSDS_White_Tracer
	{
		model = "\z\ace\addons\tracers\ace_TracerYellow2.p3d";
	};
};
