////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:43 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_custom_cup_illumination
	{
		name = "PCA - Custom - CUP Illuminations";
		author = "PCA";
		units[] = {};
		weapons[] = {};
		requiredAddons[] = {"A3_Weapons_F","cba_jr","pca_mods_main","CUP_Weapons_Ammunition"};
		requiredVersion = 1.6;
	};
};
class CfgAmmo
{
	class F_40mm_White;
	class F_40mm_Red;
	class F_40mm_Green;
	class CUP_F_40mm_Star_White: F_40mm_White
	{
		timeToLive = 150;
		coefGravity = 0.1;
	};
	class CUP_F_40mm_StarCluster_White: CUP_F_40mm_Star_White
	{
		timeToLive = 60;
	};
	class CUP_F_40mm_StarCluster_Green: CUP_F_40mm_Star_White
	{
		timeToLive = 60;
	};
	class CUP_F_40mm_StarCluster_Red: CUP_F_40mm_Star_White
	{
		timeToLive = 60;
	};
};
