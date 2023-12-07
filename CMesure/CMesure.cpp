// Cette ligne est sp�cifique � Visual C++
// pour pouvoir utiliser le sscanf
#define _CRT_SECURE_NO_DEPRECATE

#include <cmath>
#include <stdio.h>
#include <float.h>
/*
=> float.h est utilis�e pour :
	#define DBL_EPSILON     2.2204460492503131e-016 // smallest such that 1.0+DBL_EPSILON != 1.0
	#define DBL_MAX         1.7976931348623158e+308 // MAXI value
*/

#include "CMesure.h"

using namespace std;

#define CMESURE_MAX (sqrt(DBL_MAX)/2.0)
#define CMESURE_EPS DBL_EPSILON

/////////////////////////////////////////////////////////////////////////////////
/////////////////// Pour �viter les probl�mes de compilation ////////////////////
/////////////////////////////////////////////////////////////////////////////////

double MINI(double a, double b) { return (((a)<(b)) ? (a) : (b)); }
double MAXI(double a, double b) { return (((a)>(b)) ? (a) : (b)); }
double SIGN(double a)			{ return (((a)>=0) ? (1) : (-1)); }


////////////////////////////////////////////////////////////////////////////////
///////////////////////// Constructeurs et destructeur /////////////////////////
////////////////////////////////////////////////////////////////////////////////

// constructeur par d�faut
CMesure::CMesure()
{
	this->valeur  = 0.0;
	this->epsilon = CMESURE_EPS;
	this->alpha   = 95.45;
}

// constructeur pour une constante : epsilon MINI
CMesure::CMesure(double v)
{
	this->valeur  = v;
	this->epsilon = CMESURE_EPS;
	this->alpha   = 95.45;
}

// construction d'une mesure en passant tous les param�tres
CMesure::CMesure(double v, double e, double a)
{
	this->valeur  = v;
	this->epsilon = e;
	this->alpha   = a;
}

// Permet de creer une mesure � partir d'une sauvegarde texte
CMesure::CMesure(char* m)                     
{
	double v, it, a;
	sscanf(m, "( %lf +/- %lf | %lf% )", &v, &it, &a);
	this->valeur  = v;
	this->alpha   = a;
	this->epsilon = fabs(it/this->K());
}

// permet de cr�er une mesure � partir d'une loi de distribution connue
CMesure::CMesure(double v, double it, char loi)
{
	// Dans le cadre de mesures effectu�es dans des conditions bien identifi�es,
	// il est possible d'estimer l'incertitude type directement � partir de
	// l'intervalle de tol�rance � l'aide des lois suivante
	//
	//		1) 'R' : R�solution d'un indicateur num�rique     : epsilon = it / rac(12.0)
	//		2) 'H' : Hyst�r�sis tel que it = MAXI - MINI        : epsilon = it / rac(12.0)
	//		3) 'S' : �volution Sinuso�dale sur it = MAXI - MINI : epsilon = it / 1.4
	//		4) 'N' : loi Normale par d�faut, K = 2            : epsilon = it / 2.0
	//		5) 'C' : appareil de Classe +/- it                : epsilon = it / rac(3.0)

	this->valeur = v;
	this->alpha  = 95.45;

	switch(loi)
	{
		case 'R' : this->epsilon = fabs(it / sqrt(12.0)); break;
		case 'H' : this->epsilon = fabs(it / sqrt(12.0)); break;
		case 'S' : this->epsilon = fabs(it / 1.4)       ; break;
		case 'C' : this->epsilon = fabs(it / sqrt(3.0)) ; break;
		case 'N' : // c'est la loi par d�faut dans tout bon certificat d'�talonnage qui se respecte
		default  : this->epsilon = fabs(it / 2.0)       ; break;
	}
}


// construceteur de recopie
CMesure::CMesure(const CMesure& X)
{
	this->valeur    = X.valeur;
	this->epsilon   = X.epsilon;
	this->alpha     = X.alpha;
}

// destructeur => vide car il n'y a aucune allocations dynamiques 
CMesure::~CMesure() 
{
	// Pas d'allocations dynamiques ... donc rien � d�truire.
}


////////////////////////////////////////////////////////////////////////////////
////////////////// Accesseurs aux donn�es priv�es de la classe /////////////////
////////////////////////////////////////////////////////////////////////////////

double CMesure::Val  (void) { return this->valeur;              }
double CMesure::Alpha(void) { return this->alpha;               }
double CMesure::Eps  (void) { return this->epsilon;             }
double CMesure::IT   (void) { return this->epsilon * this->K(); }

// Coeff d'�largissement calcul� en fct de alpha
double CMesure::K(void)
{
	// Calcul par interpolation du coeff d'�largissement � l'aide
	// des valeurs d�crites dans la norme "NF ENV 13005"
    double p[8] = { 99.95 , 99.73 , 99.00 , 95.45 , 95.00 , 90.00 , 68.27 , 0.000 };
    double k[8] = { 4.000 , 3.000 , 2.576 , 2.000 , 1.960 , 1.645 , 1.000 , 0.000 };

    double a, b;
	int i;
    
    // Recherche du cadran dans lequel on se situe
	for(i=1; i<8; i++) if(this->alpha >= p[i]) break;

	// Interpolation de la valeur du coefficient d'�largissement
    a = (k[i] - k[i-1]) / (p[i] - p[i-1]);
    b = k[i-1] - (a * p[i-1]);

    return (a * this->alpha + b);
}
    
////////////////////////////////////////////////////////////////////////////////
//////////////////////// surd�finition des op�rateurs //////////////////////////
////////////////////////////////////////////////////////////////////////////////

CMesure  CMesure::operator+ (const CMesure& M) const
{
	// U�(this + M) = U�(this) + U�(M)
	
	CMesure R;

	R.valeur  = this->valeur + M.valeur;
	R.epsilon = sqrt(this->epsilon * this->epsilon + M.epsilon * M.epsilon);
	R.alpha   = MAXI(this->alpha, M.alpha);

    return R;
}

CMesure& CMesure::operator+=(const CMesure& M)
{
    // U�(this + M) = U�(this) + U�(M)
	
	(*this) = (*this) + M;
    return (*this);
}

CMesure  CMesure::operator+ (const double&  V) const
{
	// une constante est une mesure dont l'incertitude est minimale
	return (*this) + CMesure(V);
}

CMesure& CMesure::operator+=(const double&  V)
{
	// une constante est une mesure dont l'incertitude est minimale
	(*this) = ((*this) + CMesure(V));
	return (*this);
}

CMesure  CMesure::operator- (const CMesure& M) const
{
    // U�(this - M) = U�(this) + U�(M)
	
	CMesure R;

	R.valeur  = this->valeur - M.valeur;
	R.epsilon = sqrt(this->epsilon * this->epsilon + M.epsilon * M.epsilon);
	R.alpha   = MAXI(this->alpha, M.alpha);

    return R;
}
CMesure& CMesure::operator-=(const CMesure& M)
{
    // U�(this - M) = U�(this) + U�(M)

	(*this) = (*this) - M;
    return (*this);    
}

CMesure  CMesure::operator- (const double&  V) const
{
	// une constante est une mesure dont l'incertitude est minimale
	return (*this) - CMesure(V);
}

CMesure& CMesure::operator-=(const double&  V)
{
	// une constante est une mesure dont l'incertitude est minimale
	(*this) = ((*this) - CMesure(V));
	return (*this);
}

CMesure  CMesure::operator* (const CMesure& M) const
{
    // U(R) = sqrt((U(this)� * M�) + (this� * U(M)�))

	CMesure R;
	
	R.valeur  = this->valeur * M.valeur;
	R.epsilon = sqrt((this->valeur * M.epsilon * this->valeur * M.epsilon) + (M.valeur * this->epsilon * M.valeur * this->epsilon));
	R.alpha   = MAXI(this->alpha, M.alpha);

    return R;
}

CMesure& CMesure::operator*=(const CMesure& M)
{
    // U(R) = sqrt((U(this)� * M�) + (this� * U(M)�))

	(*this) = (*this) * M;
    return (*this);
}

CMesure  CMesure::operator* (const double&  V) const
{
	// une constante est une mesure dont l'incertitude est minimale
	return (*this) * CMesure(V);
}

CMesure& CMesure::operator*=(const double&  V)
{
	// une constante est une mesure dont l'incertitude est minimale
	(*this) = (*this) * CMesure(V);
	return (*this);
}

CMesure  CMesure::operator/ (const CMesure& M) const
{

	// U(R) = sqrt((U(this)� * M�) + (this� * U(M)�)) * (1 / M�) 
	// CAS DE LA DIVISION DE/PAR ZERO !!! (traite l'infinie comme une valeur)
	//		R.valeur = +/-inf si d�nominateur nul
	//		eps = +inf si d�nom est nul

	CMesure R;
	
	if(M == CMesure(0.0))	
		R = CMesure(
					(double)SIGN(this->valeur) * CMESURE_MAX, 
					CMESURE_MAX, 
					MAXI(this->alpha, M.alpha) 
				   );
	else					
		R = CMesure(
					(this->valeur / M.valeur),
					sqrt((this->valeur * M.epsilon * this->valeur * M.epsilon) + (M.valeur * this->epsilon * M.valeur * this->epsilon)) * (1.0 / (M.valeur * M.valeur)),
					MAXI(this->alpha, M.alpha)
				   );

    return R;
}

CMesure& CMesure::operator/=(const CMesure& M)
{
	(*this) = (*this) / M;
    return (*this);
}

CMesure  CMesure::operator/ (const double&  V) const
{
	// une constante est une mesure dont l'incertitude vaut z�ro
	return (*this) / CMesure(V);
}

CMesure& CMesure::operator/=(const double&  V)
{
	// une constante est une mesure dont l'incertitude vaut z�ro
	(*this) = (*this) / CMesure(V);
	return (*this);
}

CMesure& CMesure::operator= (const CMesure& M)
{
    this->valeur  = M.valeur;
    this->epsilon = M.epsilon;
    this->alpha   = M.alpha;
    return (*this);
}
CMesure& CMesure::operator= (const double&  V)
{
	// une constante est une mesure dont l'incertitude est minimale
	(*this) = CMesure(V);
	return (*this);
}

/////////////////////////////////////////////////////////////////////////////////
//////////////////////// Fonctions de tests pour les VA /////////////////////////
/////////////////////////////////////////////////////////////////////////////////

//
// Dans cette classe, les mesures sont consid�r�es comme des
// variables al�atoires, effectuer un test d'ordonnancement
// entre deux mesures revient � effectuer un test statistique
// entre deux variables al�atoires.
//

/////////////////////////////////////////////////////////////////////////////////
//                                                                             //
//           PRINCIPE DE RESOLUTION DES TESTS D'ORDONNANCEMENT                 //
//           -------------------------------------------------                 //
//                                                                             //
//   -> Calcul de R = A - B                                                    //
//   -> contr�le de la position du r�sultat par rapport � son propre IT        //
//                                                                             //
//                                                                             //
//                     -IT(A-B)       0      +IT(A-B)                          //
//  -inf ------------------+----------+----------+-----------------> (A - B)   //
//                                                                             //
//              (A!=B)             (A==B)                (A!=B)                //
//  -inf ------------------[----------+----------]------------------ +inf      //
//                                                                             //
//                      (A<=B)                             (A>B)               //
//  -inf ----------------------------------------]------------------ +inf      //
//                                                                             //
//               (A<B)                         (A>=B)                          //
//  -inf ------------------[---------------------------------------- +inf      //
//                                                                             //
//                                                                             //
/////////////////////////////////////////////////////////////////////////////////


bool CMesure::operator==(const CMesure& M) const
{ 
	//
	// Dans cette classe, les mesures sont consid�r�es comme des
	// variables al�atoires : tester si A == B revient � effectuer,
	// ce que l'on nomme en statistique, un test bilat�ral.
	//
	// Cela consiste � calculer la VA �quivalente � la diff�rence des
	// deux VA test�es et � v�rifier que sa moyenne est comprise dans
	// son propre intervalle de tol�rance centr� en z�ro.
	//
	CMesure D;
	D = (*this) - M;
	return ( fabs(D.valeur) <= D.IT() );
}

bool CMesure::operator!=(const CMesure& M) const { return !((*this) == M); }
bool CMesure::operator<=(const CMesure& M) const { return !((*this) >  M); }
bool CMesure::operator>=(const CMesure& M) const { return !((*this) <  M); }

bool CMesure::operator< (const CMesure& M) const 
{
	// tester est-ce que (A < B) consiste � v�rifier si
	// 1) A et B sont significativement diffentes
	// 2) A.valeur < B.valeur

	CMesure D;
	D = (*this) - M;
	return ( D.valeur < (-1.0 * D.IT()) );
}

bool CMesure::operator>( const CMesure& M) const
{
	// tester est-ce que (A > B) consiste � v�rifier si
	// 1) A et B sont significativement diffentes
	// 2) A.valeur > B.valeur

	CMesure D;
	D = (*this) - M;
	return ( D.valeur > D.IT() );
}

/////////////////////////////////////////////////////////////////////////////////
////////////////////////// Fonctions amies de la classe /////////////////////////
/////////////////////////////////////////////////////////////////////////////////

CMesure  operator- (CMesure& M) 
{ 
	// return CMesure(-M.valeur, M.epsilon, M.alpha);
	return (CMesure(0.0) - M);
}

CMesure  operator+ (double V, const CMesure& M) { return CMesure(V) + M; }
CMesure  operator- (double V, const CMesure& M) { return CMesure(V) - M; }
CMesure  operator* (double V, const CMesure& M) { return CMesure(V) * M; }
CMesure  operator/ (double V, const CMesure& M) { return CMesure(V) / M; }

ostream& operator<<(ostream& s, CMesure& M)
{
	s << "( " << M.valeur << " +/- " << M.epsilon*M.K() << " | " << M.alpha << "% )";
	return s;
}

CMesure Conjug(CMesure& M) { M = CMesure(M.valeur, M.epsilon, M.alpha); return M; }


