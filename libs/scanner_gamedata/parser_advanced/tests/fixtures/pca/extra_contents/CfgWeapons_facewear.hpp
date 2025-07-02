class pca_nvg_dummy_base: NVGoggles
{
	scope = 1;
	author = "pca";
	descriptionShort = "No Armor";
	modelOptics = "\A3\Weapons_F\empty";
	visionMode[] = 
	{
		"Normal",
		"Normal"
	};
};

class pca_nvg_cigarette: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Cigarette";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_g_cigarette_ca.paa";
	model = "\A3_Aegis\Characters_F_Aegis\Facewear\G_Cigarette.p3d";
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		uniformModel = "\A3_Aegis\Characters_F_Aegis\Facewear\G_Cigarette.p3d";
		modelOff = "\A3_Aegis\Characters_F_Aegis\Facewear\G_Cigarette.p3d";
		mass = 2;
	};
};

class pca_nvg_shemagh_lowered_khk: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Khaki)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_g_shemag_khk_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_khk_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_oli: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Olive)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_g_shemag_oli_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_oli_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_red: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Red)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_g_shemag_red_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_red_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_tan: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (Tan)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_g_shemag_tan_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_tan_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		mass = 8;
	};
};

class pca_nvg_shemagh_lowered_white: pca_nvg_dummy_base
{
	scope = 2;
	displayName = "Shemagh Lowered (White)";
	picture = "\a3_aegis\characters_f_aegis\facewear\data\ui\icon_g_shemag_white_ca.paa";
	model = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
	hiddenSelections[] =
	{
		"camo"
	};
	hiddenSelectionsTextures[] =
	{
		"\a3_aegis\characters_f_aegis\facewear\data\shemag_white_co.paa"
	};
	class ItemInfo
	{
		type = 616;
		hmdType = 0;
		hiddenSelections[] =
		{
			"camo"
		};
		uniformModel = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		modelOff = "\a3_aegis\characters_f_aegis\facewear\g_shemag.p3d";
		mass = 8;
	};
};