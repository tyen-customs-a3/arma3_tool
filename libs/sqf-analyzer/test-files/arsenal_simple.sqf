/**
	* Adds curated arsenal to player that disables itself under specified conditions.
	*
	* Faction:
	*
	* Usage - under initPlayerLocal.sqf
	* 0 = execVM 'loadouts\arsenal.sqf';
*/

//Variables

arsenal = "building" createVehicleLocal [0,0,0];
player setVariable ["startpos", getPosASL player];

//Define Arsenal items
private _itemEquipment = 
[
	//Radios
	"ACRE_PRC148",
	"ACRE_PRC152",
	"ACRE_PRC117F"
];


private _itemMod =
[	
	//Bipod & Foregrips
	"rhsusf_acc_grip1",
	"rhsusf_acc_grip2",
	"rhsusf_acc_grip3",
	"rhsusf_acc_grip4",
	"rhsusf_acc_grip4_bipod",
	"rhsusf_acc_saw_lw_bipod"
];

private _itemWeaponRifle =
[
	"rhs_weap_hk416d145",
];

private _itemWeaponLAT = 
[
	"rhs_weap_M136"
];

private _itemWeaponAmmo =
[
	//Rifle Ammo
	"rhs_mag_30Rnd_556x45_M855A1_Stanag",
	"greenmag_ammo_556x45_M855A1_60Rnd",
	"rhsusf_200Rnd_556x45_M855_mixed_soft_pouch",
];

//Add Existing Player Items
{
    _itemEquipment pushBackUnique _x;
}forEach (primaryWeaponItems player);

{
    _itemEquipment pushBackUnique _x;
}forEach (handgunItems player);

_itemEquipment pushBack uniform player;
_itemEquipment pushBack vest player;
_itemEquipment pushBack backpack player;
_itemEquipment pushBack headgear player;

{
    _itemEquipment pushBackUnique _x;
} forEach (assignedItems player);

//Match unitrole name with the classnames in loadout.
[arsenal, (_itemEquipment + _itemMod + _itemWeaponLAT + _itemWeaponRifle + _itemWeaponAmmo)] call ace_arsenal_fnc_initBox; 