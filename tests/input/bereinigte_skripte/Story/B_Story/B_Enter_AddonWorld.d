// ***************************************************
//  			B_ENTER_ADDONWORLD			
// ***************************************************

// B_ENTER_MDWORLD_Kapitel_1
//****************************************************
var int Enter_Kapitel1;
FUNC VOID B_ENTER_ADDONWORLD_Kapitel_1 ()
{
	if (Enter_Kapitel1 == FALSE)
	{
	
		// ------ Gilden-Attit�den �ndern ------

		// ------ Immortal-Flags l�schen ------

		// ------ TAs �ndern ------

		// ------ Respawn ------
                Enter_Kapitel1 = TRUE;
		
	};

};

// ******************************************************************************************************************************************************************
// B_ENTER_ADDONWORLD			 (wird �ber INIT_ADDONWORLD)
// ******************************************************************************************************************************************************************

FUNC VOID B_ENTER_ADDONWORLD ()	
{
	B_InitNpcGlobals (); 
	if (Kapitel == 1)	{B_ENTER_ADDONWORLD_Kapitel_1 ();	};
	
	CurrentLevel = ADDONWORLD_ZEN; 
	B_InitNpcGlobals ();
};
