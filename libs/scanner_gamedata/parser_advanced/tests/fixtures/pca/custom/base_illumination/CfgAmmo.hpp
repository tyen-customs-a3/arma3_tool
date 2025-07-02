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
};