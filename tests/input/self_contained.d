// Look for the function "DMG_OnDmg" to modify

class oSDamageDescriptor {
	var int validFields; 		// zDWORD 0x00
	
	var int attackerVob; 		// zCVob* 0x04
	var int attackerNpc; 		// oCNpc* 0x08
	var int hitVob; 			// zCVob* 0x0C
	var int hitPfx;				// oCVisualFX* 0x10
	var int itemWeapon; 		// oCItem* 0x14
		
	var int spellID;			// zDWORD 0x18
	var int spellCat; 			// zDWORD 0x1C
	var int spellLevel;			// zDWORD 0x20
};

class C_Npc {};
func C_Npc _^(var int ptr) {};
func int MEM_ReadInt(var int ptr) { return 0; };
func void HookEngineF(var int ptr, var func f) {};

var int EDI, ESP, EDI;

func int DMG_OnDmg(var int victimPtr, var int attackerPtr, var int dmg, var int dmgDescriptorPtr) {
	var oSDamageDescriptor dmgDesc; dmgDesc = _^(dmgDescriptorPtr);
	var c_npc attackerNpc; attackerNpc = _^(attackerptr);
    var c_npc victimNpc; victimNpc = _^(victimPtr);

	// Diese Funktion anpassen, wenn ihr den Schaden verändern wollt! 'dmg' ist der von Gothic berechnete Schaden
	return dmg;
};
	

var int _DMG_DmgDesc;

func void _DMG_OnDmg_Post() {
	EDI = DMG_OnDmg(EBP, MEM_ReadInt(MEM_ReadInt(ESP+644)+8), EDI, _DMG_DmgDesc);
};

func void _DMG_OnDmg_Pre() {
	_DMG_DmgDesc = ESI; // I'm preeeeetty sure it won't get moved in the meantime...
};

func void InitDamage() {
	const int dmg = 0;
	if (dmg) { return; };
	HookEngineF(6736583/*0x66CAC7*/, 5, _DMG_OnDmg_Post);
	const int oCNpc__OnDamage_Hit = 6710800;
	HookEngineF(oCNpc__OnDamage_Hit, 7, _DMG_OnDmg_Pre);
	dmg = 1;
};