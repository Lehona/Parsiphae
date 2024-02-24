// ******************************************************************************************************
//  			B_NPC_IsAliveCheck			(für NPCs die eine Levelchange vollziehen)
// ******************************************************************************************************

FUNC VOID B_NPC_IsAliveCheck (var int Zen)
{
	if (Zen == NEWWORLD_ZEN )
	{
		if (Kapitel >= 2)
		{
		};
		
		if (Kapitel >= 3)
		{
		};
		
		if (MIS_ReadyforChapter4 == TRUE)	//Joly: letzter Pyrokar Dialog im 3. Kapitel
		{
		};
		
		if (Kapitel >= 5)
		{
		};
		
		if (Kapitel >= 6)
		{
		};
	};

	if (Zen == OLDWORLD_ZEN )
	{
		if (Kapitel >= 2)
		{
		};
		
		if (Kapitel >= 3)
		{
		};
		
		if (Kapitel >= 4)
		{

		};
		
		if (Kapitel >= 5)
		{
		};
		
		if (Kapitel >= 6)
		{
		};
	};
};

