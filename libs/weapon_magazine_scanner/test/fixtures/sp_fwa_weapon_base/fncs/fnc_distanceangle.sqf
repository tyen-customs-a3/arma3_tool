player addEventHandler ["Fired", {
	params ["_unit", "_weapon", "_muzzle", "_mode", "_ammo", "_magazine", "_projectile", "_gunner"];
	_pos = getPosATL _projectile;
	_pitch = (_projectile call BIS_fnc_getPitchBank) select 0;
	while {alive _projectile} do {_pos = getPosATL _projectile;};
	hint format["Angle %1, Distance %2",_pitch,_pos distance player];
}];
