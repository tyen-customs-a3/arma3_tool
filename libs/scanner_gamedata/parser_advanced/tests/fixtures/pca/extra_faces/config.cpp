////////////////////////////////////////////////////////////////////
//DeRap: config.bin
//Produced from mikero's Dos Tools Dll version 9.98
//https://mikero.bytex.digital/Downloads
//'now' is Sat May 24 10:29:39 2025 : 'file' last modified on Thu Jan 01 13:00:00 1970
////////////////////////////////////////////////////////////////////

#define _ARMA_

class CfgPatches
{
	class pca_extra_faces
	{
		name = "Extra Faces";
		author = "PCA";
		units[] = {};
		weapons[] = {};
		requiredAddons[] = {"pca_main","female3_iceman"};
		requiredVersion = 1.6;
	};
};
class CfgFaces
{
	class Default;
	class Man_A3: Default
	{
		class Default;
		class B_female_bun_05: Default
		{
			displayname = "Emma";
			head = "bun_female_bun_01";
			texture = "x\pca\extra_faces\face_01\face1_co.paa";
			material = "\A3_female_heads\female_head_ICEMAN\female_01\f_white.rvmat";
			materialWounded1 = "\A3_female_heads\female_head_ICEMAN\female_01\f_white.rvmat";
			materialWounded2 = "\A3_female_heads\female_head_ICEMAN\female_01\f_white.rvmat";
			textureHL = "\A3_female_heads\female_head_ICEMAN\female_03\F_HL_co.paa";
			materialHL = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_white.rvmat";
			textureHL2 = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_co.paa";
			materialHL2 = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_white.rvmat";
		};
		class B_female_bun_06: Default
		{
			displayname = "Olivia";
			head = "bun_female_bun_01";
			texture = "x\pca\extra_faces\face_02\face2_co.paa";
			material = "\A3_female_heads\female_head_ICEMAN\female_01\f_white.rvmat";
			materialWounded1 = "\A3_female_heads\female_head_ICEMAN\female_01\f_white.rvmat";
			materialWounded2 = "\A3_female_heads\female_head_ICEMAN\female_01\f_white.rvmat";
			textureHL = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_co.paa";
			materialHL = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_white.rvmat";
			textureHL2 = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_co.paa";
			materialHL2 = "\A3_female_heads\female_head_ICEMAN\female_01\F_HL_white.rvmat";
		};
	};
};
class CfgHeads
{
	class bun_female_bun_01;
	class bun_female_bun_02;
	class bun_female_bun_03;
};
