// test de s�curit� pour �viter l'inclusion multiple des d�finitions
#ifndef _FCTMATHMESURE_H_
#define _FCTMATHMESURE_H_

#include "CMesure.h"


///////////////////////////// Fonctions math�matiques //////////////////////////

// Les fonctions suivantes n�cessite l'utilisation
// de la loi de propagation des incertitudes.
// cf norme NF ENV 13005 (Guide d'Utilisation de la M�trologie : GUM)

// 1 - ce groupe de fonction est � une seule variable,
// ce qui donne, pour l'�quation y = f(x), la formule suivante:
// U�(y) = [df(x)/dx]� * U�(x)

	CMesure fabs (CMesure& M);
	CMesure sin  (CMesure& M);
	CMesure cos  (CMesure& M);
	CMesure tan  (CMesure& M);
	CMesure acos (CMesure& M);
	CMesure asin (CMesure& M);
	CMesure atan (CMesure& M);
	CMesure cosh (CMesure& M);
	CMesure sinh (CMesure& M);
	CMesure tanh (CMesure& M);
	CMesure log  (CMesure& M);
	CMesure log10(CMesure& M);
	CMesure exp  (CMesure& M);
	CMesure sqrt (CMesure& M);

	CMesure ceil (CMesure& M);
	CMesure floor(CMesure& M);


// 2 - ce groupe de fonction est � deux variables,
// ce qui donne, pour l'�quation y = f(x,y), la formule suivante:
// U�(y) = [df(x)/dx]� * U�(x) + [df(y)/dy]� * U�(y)

	CMesure pow  (CMesure& Base, CMesure& Puiss);
	CMesure pow  (double   Base, CMesure& Puiss);
	CMesure pow  (CMesure& Base, double   Puiss);

	CMesure atan2(CMesure& X, CMesure& Y);
	CMesure modf (CMesure& X, CMesure* Fpart);

// 3 - ce groupe de fonction me pose probl�me pour d�terminer
// comment calculer l'incertitude type. Je vais certainement faire
// un choix personnel unilat�rale parfaitement arbitraire.

	CMesure MIN  (CMesure& A, CMesure& B);
	CMesure MAX  (CMesure& A, CMesure& B);
	CMesure SIGN (CMesure& A);

	
#endif	// fin de _FCTMATHMESURE_H_