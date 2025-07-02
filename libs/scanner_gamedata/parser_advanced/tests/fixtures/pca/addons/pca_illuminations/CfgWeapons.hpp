class asdg_SlotInfo;
class asdg_FrontSideRail: asdg_SlotInfo
{
	class compatibleItems
	{
		pca_flashlight_led = 1;
		pca_flashlight_led_beam = 1;
		pca_flashlight_led_wide = 1;
		pca_flashlight_led_cqb = 1;
		pca_flashlight_sf = 1;
		pca_flashlight_sf_beam = 1;
		pca_flashlight_sf_wide = 1;
		pca_flashlight_sf_cqb = 1;
		pca_flashlight_sf_od = 1;
		pca_flashlight_sf_od_beam = 1;
		pca_flashlight_sf_od_wide = 1;
		pca_flashlight_sf_od_cqb = 1;
		pca_flashlight_sf_tan = 1;
		pca_flashlight_sf_tan_beam = 1;
		pca_flashlight_sf_tan_wide = 1;
		pca_flashlight_sf_tan_cqb = 1;
	};
};

class PointerSlot;
class PointerSlot_Rail: PointerSlot
{
	class compatibleItems
	{
		pca_flashlight_led = 1;
		pca_flashlight_led_beam = 1;
		pca_flashlight_led_wide = 1;
		pca_flashlight_led_cqb = 1;
		pca_flashlight_sf = 1;
		pca_flashlight_sf_beam = 1;
		pca_flashlight_sf_wide = 1;
		pca_flashlight_sf_cqb = 1;
		pca_flashlight_sf_od = 1;
		pca_flashlight_sf_od_beam = 1;
		pca_flashlight_sf_od_wide = 1;
		pca_flashlight_sf_od_cqb = 1;
		pca_flashlight_sf_tan = 1;
		pca_flashlight_sf_tan_beam = 1;
		pca_flashlight_sf_tan_wide = 1;
		pca_flashlight_sf_tan_cqb = 1;
	};
};

class CfgWeapons 
{
	class acc_flashlight;
	class InventoryFlashLightItem_Base_F;
	
	class pca_flashlight_led: acc_flashlight
	{
		author = "PCA";
		scope = 2;
		displayName = "Flashlight LED";
		descriptionUse = "Flashlight LED (Normal)";
		descriptionShort = "Flashlight LED (Normal)";
		class ItemInfo: InventoryFlashLightItem_Base_F 
		{
			mass = 5;
			RMBhint = "Flashlight LED (Normal)";
			class Flashlight 
			{
				ambient[] = {0.58,0.72,0.82};
				color[] = {148,186,208};
				coneFadeCoef = 10;
				dayLight = 1;
				direction = "flash";
				flareMaxDistance = 500;
				flareSize = 3;
				innerAngle = 10;
				intensity = 600;
				outerAngle = 80;
				position = "flash dir";
				scale[] = {0};
				size = 1;
				useFlare = 1;
				class Attenuation
				{
					start = 0;
					constant = 40;
					linear = 1;
					quadratic = 0.2;
					hardLimitEnd = 80;
					hardLimitStart = 20;
				};
			};
		};
		MRT_SwitchItemNextClass="pca_flashlight_led_wide";
		MRT_SwitchItemPrevClass="pca_flashlight_led_cqb";
		MRT_switchItemHintText="Flashlight LED (Normal)";
	};
	class pca_flashlight_led_wide: pca_flashlight_led
	{
		author = "PCA";
		scope = 2;
		displayName = "Flashlight LED (Wide)";
		descriptionUse = "Flashlight LED (Wide)";
		descriptionShort = "Flashlight LED (Wide)";
		class ItemInfo: InventoryFlashLightItem_Base_F 
		{
			mass = 5;
			RMBhint = "Flashlight LED (Wide)";
			class Flashlight 
			{
				ambient[] = {0.58,0.72,0.82};
				color[] = {148,186,208};
				coneFadeCoef = 12;
				dayLight = 1;
				direction = "flash";
				flareMaxDistance = 500;
				flareSize = 3;
				innerAngle = 40;
				intensity = 400;
				outerAngle = 140;
				position = "flash dir";
				scale[] = {0};
				size = 1;
				useFlare = 1;
				class Attenuation
				{
					start = 0;
					constant = 40;
					linear = 1;
					quadratic = 0.2;
					hardLimitEnd = 80;
					hardLimitStart = 15;
				};
			};
		};
		MRT_SwitchItemNextClass="pca_flashlight_led_beam";
		MRT_SwitchItemPrevClass="pca_flashlight_led";
		MRT_switchItemHintText="Flashlight LED (Wide)";
	};
	class pca_flashlight_led_beam: pca_flashlight_led
	{
		author = "PCA";
		scope = 2;
		displayName = "Flashlight LED (Beam)";
		descriptionUse = "Flashlight LED (Beam)";
		descriptionShort = "Flashlight LED (Beam)";
		class ItemInfo: InventoryFlashLightItem_Base_F 
		{
			mass = 5;
			RMBhint = "Flashlight LED (Beam)";
			class Flashlight 
			{
				ambient[] = {0.58,0.72,0.82};
				color[] = {148,186,208};
				coneFadeCoef = 32;
				dayLight = 1;
				direction = "flash";
				flareMaxDistance = 500;
				flareSize = 4;
				innerAngle = 10;
				intensity = 1000;
				outerAngle = 60;
				position = "flash dir";
				scale[] = {0};
				size = 1;
				useFlare = 1;
				class Attenuation
				{
					start = 0;
					constant = 10;
					linear = 6;
					quadratic = 0.01;
					hardLimitEnd = 100;
					hardLimitStart = 80;
				};
			};
		};
		MRT_SwitchItemNextClass="pca_flashlight_led_cqb";
		MRT_SwitchItemPrevClass="pca_flashlight_led_wide";
		MRT_switchItemHintText="Flashlight LED (Focused Beam)";
	};
	class pca_flashlight_led_cqb: pca_flashlight_led
	{
		author = "PCA";
		scope = 2;
		displayName = "Flashlight LED (CQB)";
		descriptionUse = "Flashlight LED (CQB)";
		descriptionShort = "Flashlight LED (CQB)";
		class ItemInfo: InventoryFlashLightItem_Base_F 
		{
			mass = 5;
			RMBhint = "Flashlight LED (CQB)";
			class Flashlight 
			{
				ambient[] = {0.58,0.72,0.82};
				color[] = {148,186,208};
				coneFadeCoef = 12;
				dayLight = 1;
				direction = "flash";
				flareMaxDistance = 500;
				flareSize = 2;
				innerAngle = 40;
				intensity = 100;
				outerAngle = 120;
				position = "flash dir";
				scale[] = {0};
				size = 1;
				useFlare = 1;
				class Attenuation
				{
					start = 0;
					constant = 32;
					linear = 1;
					quadratic = 0.2;
					hardLimitEnd = 60;
					hardLimitStart = 1;
				};
			};
		};
		MRT_SwitchItemNextClass="pca_flashlight_led";
		MRT_SwitchItemPrevClass="pca_flashlight_led_beam";
		MRT_switchItemHintText="Flashlight LED (Low Intensity Light)";
	};
	class pca_flashlight_sf: pca_flashlight_led
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire LED";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_black_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_wide";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_cqb";
	};
	class pca_flashlight_sf_wide: pca_flashlight_led_wide
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire LED (Wide)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_black_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_beam";
		MRT_SwitchItemPrevClass="pca_flashlight_sf";
	};
	class pca_flashlight_sf_beam: pca_flashlight_led_beam
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire LED (Beam)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_black_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_cqb";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_wide";
	};
	class pca_flashlight_sf_cqb: pca_flashlight_led_cqb
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire LED (CQB)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_black_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_beam";
	};
	class pca_flashlight_sf_od: pca_flashlight_led
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire OD LED";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_olive_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_od.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_od_wide";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_od_cqb";
	};
	class pca_flashlight_sf_od_wide: pca_flashlight_led_wide
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire OD LED (Wide)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_olive_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_od.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_od_beam";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_od";
	};
	class pca_flashlight_sf_od_beam: pca_flashlight_led_beam
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire OD LED (Beam)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_olive_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_od.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_od_cqb";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_od_wide";
	};
	class pca_flashlight_sf_od_cqb: pca_flashlight_led_cqb
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire OD LED (CQB)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_olive_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_od.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_od";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_od_beam";
	};
	class pca_flashlight_sf_tan: pca_flashlight_led
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire Tan LED";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_tan_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_tan.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_tan_wide";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_tan_cqb";
	};
	class pca_flashlight_sf_tan_wide: pca_flashlight_led_wide
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire Tan LED (Wide)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_tan_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_tan.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_tan_beam";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_tan";
	};
	class pca_flashlight_sf_tan_beam: pca_flashlight_led_beam
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire Tan LED (Beam)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_tan_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_tan.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_tan_cqb";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_tan_wide";
	};
	class pca_flashlight_sf_tan_cqb: pca_flashlight_led_cqb
	{
		author = "PCA";
		scope = 2;
		displayName = "Surefire Tan LED (CQB)";
		picture = "\cup\weapons\cup_weapons_west_attachments\flashlight\data\ui\gear_acc_flashlight_tan_ca.paa";
		model = "\cup\weapons\cup_weapons_west_attachments\flashlight\cup_surefire_flashlight_tan.p3d";
		MRT_SwitchItemNextClass="pca_flashlight_sf_tan";
		MRT_SwitchItemPrevClass="pca_flashlight_sf_tan_beam";
	};
};