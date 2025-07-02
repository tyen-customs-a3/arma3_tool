////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:39 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_invisible_backpack
	{
		name = "PCA Invisible Backpack";
		author = "PCA";
		units[] = {};
		weapons[] = {};
		requiredVersion = 1.6;
	};
};
class CfgVehicles
{
	class Bag_Base;
	class pca_backpack_invisible: Bag_Base
	{
		author = "PCA";
		scope = 2;
		displayName = "Invisible Backpack";
		model = "\a3\weapons_f\empty";
		picture = "";
		maximumLoad = 240;
		mass = 20;
	};
	class pca_backpack_invisible_large: Bag_Base
	{
		author = "PCA";
		scope = 2;
		displayName = "Invisible Backpack (Large)";
		model = "\a3\weapons_f\empty";
		picture = "";
		maximumLoad = 320;
		mass = 40;
	};
};
