class CfgAmmo
{
	class FlareCore;
	class FlareBase: FlareCore
	{
		timeToLive = 120; //Vanilla - 25, ACE - 60
		intensity = 100000; //10000
		coefGravity = 0.25;
	};
	class Flare_82mm_AMOS_White: FlareCore
	{
		timeToLive = 180; //Vanilla - 45, ACE - 60
		intensity = 100000; //10000
		coefGravity = 0.1;
	};
	
	class F_40mm_White;
	class F_40mm_Red;
	class F_40mm_Green;
	
	class CUP_F_40mm_Star_White: F_40mm_White
	{
		timeToLive = 150; //43
		coefGravity = 0.1;
	};
	class CUP_F_40mm_StarCluster_White: CUP_F_40mm_Star_White
	{
		timeToLive = 60; //7.5
	};
	class CUP_F_40mm_StarCluster_Green: CUP_F_40mm_Star_White
	{
		timeToLive = 60; //7.5
	};
	class CUP_F_40mm_StarCluster_Red: CUP_F_40mm_Star_White
	{
		timeToLive = 60; //7.5
	};
};