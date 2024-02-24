func void B_BeliarsWeaponSpecialDamage (var C_NPC oth, var C_NPC slf) //other ist angreifer, slf ist victim
{
	if (Hlp_GetInstanceID(oth) == Hlp_GetInstanceID(hero))
	{
		var int DamageRandy;
		DamageRandy = Hlp_Random (100);

		if (C_ScHasReadiedBeliarsWeapon())
		&& (DamageRandy <= BeliarDamageChance) 
			{
				if (slf.aivar[AIV_MM_REAL_ID] == ID_DRAGON_UNDEAD) //beim untoten Drachen nimmt der SC Schaden
				{
					Wld_PlayEffect("spellFX_BELIARSRAGE", oth, oth, 0, 0, 0, FALSE );
					B_MagicHurtNpc (slf, oth, 100); 
				}
				else if (slf.flags != NPC_FLAG_IMMORTAL)
				{
					Wld_PlayEffect("spellFX_BELIARSRAGE", slf, slf, 0, 0, 0, FALSE );
					B_MagicHurtNpc (oth, slf, 100);  			
				};
				//Ambient Pfx
				Wld_PlayEffect("spellFX_BELIARSRAGE_COLLIDE", hero, hero, 0, 0, 0, FALSE );
			};
	
		if (C_ScHasReadiedBeliarsWeapon())
		&& (DamageRandy <= 50) // Effekt
		{
				//Ambient Pfx
				Wld_PlayEffect("spellFX_BELIARSRAGE_COLLIDE", hero, hero, 0, 0, 0, FALSE );
		};
	};
};
