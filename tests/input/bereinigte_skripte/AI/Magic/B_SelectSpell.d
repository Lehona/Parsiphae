// *************************************************************
// B_SelectSpell
// -------------
// Magieauswahl für Menschen und Monster
// Wenn Auswahlkriterien erfüllt (je nach Gilde unterschiedlich)
// --> TRUE, sonst FALSE
// Humans zaubern nur als KdF, PAL oder DMT
// *************************************************************

func int B_SelectSpell (var C_NPC slf, var C_NPC oth)
{	
	// ------ Npctype_Friend macht höchstens Sleep ------
	if (slf.npctype == NPCTYPE_FRIEND)
	&& (Npc_IsPlayer (oth))
	&& (oth.guild < GIL_SEPERATOR_HUM) //nicht gegen verwandelten Spieler
	{
		if (slf.guild == GIL_KDF)
		|| (slf.aivar[AIV_MagicUser] == MAGIC_ALWAYS)
		{
			if (Npc_HasItems (slf, ItRu_Sleep) == 0)
			{
				CreateInvItems (slf, ItRu_Sleep, 1);
			};
		
			B_ReadySpell (slf, SPL_Sleep, SPL_Cost_Sleep);
			return TRUE;
		}
		else //Nicht-KdF
		{
			return FALSE;
		};
	};


	// ------ Magier ------
	if (slf.guild == GIL_KDF)
	|| (slf.aivar[AIV_MagicUser] == MAGIC_ALWAYS)
	{
		if (Npc_HasItems (slf, ItRu_Concussionbolt) == 0)
		{
			CreateInvItems (slf, ItRu_Concussionbolt, 1);
		};
		
		if (Npc_HasItems (slf, ItRu_InstantFireBall) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireBall, 1);
		};
		
		if (Npc_HasItems (slf, ItRu_Deathball) == 0)
		{
			CreateInvItems (slf, ItRu_Deathball, 1);
		};
		
		if (Npc_HasItems (slf, ItRu_FullHeal) == 0)
		{
			CreateInvItems (slf, ItRu_FullHeal, 1);
		};
		
		if (self.attribute[ATR_HITPOINTS] < 100) 
		{
			B_ReadySpell (slf, SPL_FullHeal, SPL_Cost_FullHeal);
			return TRUE;
		}
		else if (C_NpcHasAttackReasonToKill (self))
		{
			if (self.flags == NPC_FLAG_IMMORTAL)
			{
				B_ReadySpell (slf, SPL_Deathball, SPL_Cost_Deathball);
			}
			else
			{
				B_ReadySpell (slf, SPL_InstantFireball, SPL_Cost_InstantFireBall);
			};
			return TRUE;
		}
		else
		{
			B_ReadySpell (slf, SPL_Concussionbolt, SPL_Cost_Concussionbolt);
			return TRUE;
		};
	};
	
	// ------ Paladin ------
	if (slf.guild == GIL_PAL)
	{
		if (slf.fight_tactic == FAI_NAILED) //AL-Burgwachen auf den Zinnen
		{
			return FALSE;
		};
			
		if (Npc_HasItems (slf, ItRu_PalHolyBolt) == 0)
		{
			CreateInvItems (slf, ItRu_PalHolyBolt, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_MELEE)
		&& (C_NpcIsEvil(oth))
		{
			B_ReadySpell (slf, SPL_PalHolyBolt, SPL_Cost_PalHolyBolt);
			return TRUE;
		}
		else
		{
			return FALSE; //Angriff mit Waffen
		};
	};
	
	// ------ Skelett Magier ------
	if (slf.guild == GIL_SKELETON_MAGE)
	{
		if (Npc_HasItems (slf, ItRu_SumSkel) == 0)
		{
			CreateInvItems (slf, ItRu_SumSkel, 1);
		};
		
		if (Npc_HasItems (slf, ItRu_IceCube) == 0)
		{
			CreateInvItems (slf, ItRu_IceCube, 1);
		};
		
		if (Npc_HasItems (slf, ItRu_Icebolt) == 0)
		{
			CreateInvItems (slf, ItRu_Icebolt, 1);
		};
		
			// ------ Spruchzyklus bei SUMMON beginnen ------
			if (slf.aivar[AIV_SelectSpell] >= 6)
			{
				slf.aivar[AIV_SelectSpell] = 1;
			};
		
		if (!Npc_IsInState (oth, ZS_MagicFreeze))
		&& (slf.aivar[AIV_SelectSpell] == 0)
		{
			B_ReadySpell (slf, SPL_IceCube,	SPL_Cost_IceCube);
			return TRUE;
		}
		else if (slf.aivar[AIV_SelectSpell] == 1)
		{
			B_ReadySpell (slf, SPL_SummonSkeleton, SPL_Cost_SummonSkeleton);
			return TRUE;
		}
		else
		{
			B_ReadySpell (slf, SPL_Icebolt, SPL_Cost_Icebolt);
			return TRUE;
		};
	};
	
	// ------ Eisgolem ------
	if (slf.guild == GIL_ICEGOLEM)
	{
		if (Npc_HasItems (slf, ItRu_IceCube) == 0)
		{
			CreateInvItems (slf, ItRu_IceCube, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) < FIGHT_DIST_MELEE) 
		|| (Npc_IsInState (oth, ZS_MagicFreeze))								
		{
			return FALSE; //Nahkampfangriff
		}
		else
		{
			B_ReadySpell (slf, SPL_IceCube,	SPL_Cost_IceCube);
			return TRUE;
			
		};
	};
	
	// ------ Feuergolem ------
	if (slf.guild == GIL_FIREGOLEM)
	{
		if (Npc_HasItems (slf, ItRu_InstantFireball) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireball, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_MELEE)
		{
			B_ReadySpell (slf, SPL_InstantFireball,	SPL_Cost_InstantFireball);
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	

	// ------ Sumpfdrache ------
	if (slf.aivar[AIV_MM_REAL_ID] == ID_DRAGON_SWAMP)
	{
		if (Npc_HasItems (slf, ItRu_InstantFireball) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireball, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_DRAGON_MAGIC)
		{
			B_ReadySpell (slf, SPL_InstantFireball, SPL_Cost_InstantFireball);
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	
	// ------ Felsdrache ------
	if (slf.aivar[AIV_MM_REAL_ID] == ID_DRAGON_ROCK)
	{
		if (Npc_HasItems (slf, ItRu_InstantFireball) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireball, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_DRAGON_MAGIC)
		{
			B_ReadySpell (slf, SPL_InstantFireball, SPL_Cost_InstantFireball);
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	
	// ------ Feuerdrache ------
	if (slf.aivar[AIV_MM_REAL_ID] == ID_DRAGON_FIRE)
	{
		if (Npc_HasItems (slf, ItRu_InstantFireball) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireball, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_DRAGON_MAGIC)
		{
			B_ReadySpell (slf, SPL_InstantFireball, SPL_Cost_InstantFireball);
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	
	// ------ Eisdrache ------
	if (slf.aivar[AIV_MM_REAL_ID] == ID_DRAGON_ICE)
	{
		if (Npc_HasItems (slf, ItRu_InstantFireball) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireball, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_DRAGON_MAGIC)
		{
			B_ReadySpell (slf, SPL_InstantFireball, SPL_Cost_InstantFireball);
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	
	// ------ Untoter Drache (ENDGEGNER) ------
	if (slf.aivar[AIV_MM_REAL_ID] == ID_DRAGON_UNDEAD)
	{
		Npc_ClearAIQueue(self);
		if (Npc_HasItems (slf, ItRu_Deathball) == 0)
		{
			CreateInvItems (slf, ItRu_Deathball, 1);
		};
		
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_DRAGON_MAGIC)
		{
			B_ReadySpell (slf, SPL_Deathball, SPL_Cost_Deathball);	
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	
	// ------ Ork Schamane ------
	if (slf.aivar[AIV_MM_REAL_ID] == ID_ORCSHAMAN)
	{
		if (Npc_HasItems (slf, ItRu_InstantFireball) == 0)
		{
			CreateInvItems (slf, ItRu_InstantFireball, 1);
		};
				
		if (Npc_GetDistToNpc(slf,oth) > FIGHT_DIST_MELEE)
		{
			B_ReadySpell (slf, SPL_InstantFireball, SPL_Cost_InstantFireball);
			return TRUE;
		}
		else
		{
			return FALSE; //Nahkampfangriff
		};
	};
	
	return FALSE; //alle anderen Gilden		
};   
