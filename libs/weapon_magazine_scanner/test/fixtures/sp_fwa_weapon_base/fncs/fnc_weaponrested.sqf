/* hint "Working woo!"; */
private _unit = missionNamespace getVariable ["bis_fnc_moduleRemoteControl_unit", player]; //Get player unit

if(!(local _unit))exitWith{};

_unit addEventHandler ["WeaponDeployed", { //Deploying plays animation for holding stock
	params ["_unitDeploying", "_isDeployed"];

 	private _anim = "sp_fwa_GestureDeployedWeapon";

	if(_isDeployed) then {
		_unitDeploying playAction _anim;
  }else{
		_unitDeploying playAction "gestureNod";
  };
}];


_unit addEventHandler ["Take", { //When reloading, check if weapon is deployed and if so play animation for holding stock
	params ["_unitTaking"];
 	private _anim = "sp_fwa_GestureDeployedWeapon";

	if(isWeaponDeployed _unitTaking) then {
		_unitTaking playAction _anim;
  };
}];
