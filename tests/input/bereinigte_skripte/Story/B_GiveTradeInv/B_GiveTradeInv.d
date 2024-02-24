// **********************************************
// B_GiveTradeInventory
// --------------------
// Verteiler. Aufruf aller B_GiveTradeInv-Befehle
// **********************************************

func void B_GiveTradeInv (var C_NPC slf)
{
//******************************************************************
//	Hier muss jeder Händler eingetragen werden!!!!!!!
//******************************************************************	

	
	// GOTHIC2
	
//********************************************************************
//		Hier auch!!!
//********************************************************************

	B_ClearRuneInv (slf);
	
	if (slf.aivar[AIV_ChapterInv] <= Kapitel)
	{
		
		
		
		
	};		
	
	////////////////////////////////////////////////////////////////////////////////////////////////////
	//	Handelsware Clearen und Ambientstuff in die Tasche, wenn Trader Unconscious oder Dead
	////////////////////////////////////////////////////////////////////////////////////////////////////
	
	if 	(Npc_IsInState	(slf, ZS_Dead))
	||	(Npc_IsInState	(slf, ZS_Unconscious ))
	{
		
	
	};

};
	
	

