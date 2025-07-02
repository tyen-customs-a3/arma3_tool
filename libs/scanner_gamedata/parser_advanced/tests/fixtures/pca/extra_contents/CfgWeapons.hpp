class CfgWeapons 
{
	class ItemCore;
	class Uniform_Base;
	class HeadGearItem;
	
	class Vest_Camo_Base: ItemCore
	{
		class ItemInfo;
	};
	
	class Binocular;
	class NVGoggles: Binocular
	{
		class ItemInfo;
	};
	
	#include "CfgWeapons_facewear.hpp"
};