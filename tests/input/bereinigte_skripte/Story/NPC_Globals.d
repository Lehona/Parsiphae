//*******************************
//			NPC Globals
//*******************************

//Die Schiffsbesatzung
var C_NPC Gorn;
var C_NPC Diego;
var C_NPC Milten;
var C_NPC Lester;
var C_NPC Vatras;
var C_NPC Torlof;
var C_NPC Lee;
var C_NPC Lares;
var C_NPC Bennet;
var C_NPC Angar;
var C_NPC Wolf_DI;
var C_NPC Girion;
var C_NPC Biff;

//kleiner Bauernhof
var C_NPC Haggis;	//Bauer
var C_NPC Tana;		//Bäuerin
var C_NPC Lena;		//kleine Tochter
var C_NPC Liara;	//große Tochter
var C_NPC Auric;	//Sohn
var C_NPC Calina;	//Magd1
var C_NPC Magd2;
var C_NPC Arbeiter1;
var C_NPC Arbeiter2;
var C_NPC Arbeiter3;
var C_NPC Arbeiter4;
var C_NPC Arbeiter5;
var C_NPC Arbeiter6;
var C_NPC Arbeiter7;

//*******************************************************
//			NPC Globals füllen
//*******************************************************
 
func void  B_InitNpcGlobals ()
{
 	// **********************
 	if (Kapitel == 0)
	{
		Kapitel = 1; //HACK - wenn man mal wieder Xardas nicht anquatscht...
	};
	// **********************
/*
	//Die Schiffsbesatzung
	Diego			= Hlp_GetNpc(PC_Diego);
	Gorn			= Hlp_GetNpc(PC_Gorn);
	Milten			= Hlp_GetNpc(PC_Milten);
	Lester			= Hlp_GetNpc(PC_Lester);
	Torlof			= Hlp_GetNpc(PC_Torlof);
	Vatras			= Hlp_GetNpc(PC_Vatras);
	Lee			= Hlp_GetNpc(PC_Lee);
	Lares			= Hlp_GetNpc(PC_Lares);
	Bennet			= Hlp_GetNpc(PC_Bennet);
	Angar			= Hlp_GetNpc(PC_Angar);
	Wolf_DI			= Hlp_GetNpc(PC_Wolf);
	Girion			= Hlp_GetNpc(PC_Girion);
	Biff			= Hlp_GetNpc(PC_Biff);

	//kleiner Bauernhof
	Haggis			= Hlp_GetNpc(BAU_Haggis);
	Lena			= Hlp_GetNpc(BAU_Lena);
	Liara			= Hlp_GetNpc(BAU_Liara);
	Tana			= Hlp_GetNpc(BAU_Tana);
	Auric			= Hlp_GetNpc(BAU_Auric);
	Calina			= Hlp_GetNpc(BAU_Calina);
	Arbeiter1		= Hlp_GetNpc(BAU_Arbeiter1);
   	
*/
//Ende von B_InitNpcGlobals	 
};
