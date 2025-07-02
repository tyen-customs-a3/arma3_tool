////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:43 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_custom_base_illumination
	{
		name = "PCA - Custom - BASE Illuminations";
		author = "PCA";
		units[] = {};
		weapons[] = {};
		requiredAddons[] = {"A3_Weapons_F","cba_jr"};
		requiredVersion = 1.6;
	};
};
class CfgAmmo
{
	class FlareCore;
	class FlareBase: FlareCore
	{
		timeToLive = 120;
		intensity = 100000;
		coefGravity = 0.25;
	};
	class Flare_82mm_AMOS_White: FlareCore
	{
		timeToLive = 180;
		intensity = 100000;
		coefGravity = 0.1;
	};
};
