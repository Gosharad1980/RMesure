// test de s�curit� pour �viter l'inclusion multiple des d�finitions
#ifndef _CMESURE_H_
#define _CMESURE_H_

#include <iostream>
using namespace std;


///////////// Classe de gestion simultan�e d'une mesure et de son incertitude ///////////

// Dans la suite, tous les calculs sont efectu�s en consid�rant les donn�es ind�pendantes
// Il n'y a donc pas de gestion des covariances ... et allez vous faire foutre.


class CMesure
{
private:

	double valeur;      // Valeur de la mesure
    double epsilon;     // Valeur de l'incertitude type (�cart type de la variable al�atoire)
    double alpha;       // taux de confiance pour le calcul du coeff d'�largissement
                        // => par d�faut � 95.45% �quivalent � un coeff d'�largissement
						// de 2.00 en loi normale

public:

//////////////////////////// Constructeurs et destructeur ////////////////////////////

	// constructeurs
    CMesure();							   // valeur = 0.0; epsilon = CMESURE_EPS;	alpha = 95.45;
    CMesure(double v);                     // valeur = v;   epsilon = CMESURE_EPS;	alpha = 95.45;
    CMesure(double v, double e, double a); // valeur = v;   epsilon = fabs(e);		alpha = fabs(a);
	CMesure(char* m);					   // pour passer en param�tre la chaine ( valeur +/- IT | alpha% )

	CMesure(double v, double it, char loi = 'N');
	// Dans le cadre de mesures effectu�es dans des conditions bien identifi�es,
	// il est possible d'estimer l'incertitude type (epsilon) directement � partir
	// de l'intervalle de tol�rance (it) � l'aide des lois suivante
	//
	//		1) 'R' : R�solution d'un indicateur num�rique     : epsilon = it / rac(12.0)
	//		2) 'H' : Hyst�r�sis tel que it = MAX - MIN        : epsilon = it / rac(12.0)
	//		3) 'S' : �volution Sinuso�dale sur it = MAX - MIN : epsilon = it / 1.4
	//		4) 'N' : loi Normale par d�faut, K = 2            : epsilon = it / 2.0
	//		5) 'C' : appareil de Classe +/- it                : epsilon = it / rac(3.0)
	//
	// REM : la loi N est la loi par d�faut dans tout bon certificat d'�talonnage qui se respecte

	// constructeur de recopie
	CMesure(const CMesure& X); 

	// destructeur
    ~CMesure();                             



////////////////// Accesseurs aux donn�es priv�es de la classe /////////////////

	// Read only
	double Val(void);	// LA mesure en cours de traitement
    double Alpha(void);	// Taux de confiance
	double Eps(void);	// Incertitude type.
	double IT(void);	// Intervalle de tol�rance = Eps x K
    double K(void);		// Coeff d'�largissement


    
    // surd�finition des op�rateurs
    
    CMesure  operator+ (const CMesure& M) const;
    CMesure& operator+=(const CMesure& M);

	CMesure  operator+ (const double&  V) const;
	CMesure& operator+=(const double&  V);

    CMesure  operator- (const CMesure& M) const;
    CMesure& operator-=(const CMesure& M);

	CMesure  operator- (const double&  V) const;
	CMesure& operator-=(const double&  V);

    CMesure  operator* (const CMesure& M) const;
    CMesure& operator*=(const CMesure& M);

	CMesure  operator* (const double&  V) const;
	CMesure& operator*=(const double&  V);

    CMesure  operator/ (const CMesure& M) const;
    CMesure& operator/=(const CMesure& M);

	CMesure  operator/ (const double&  V) const;
	CMesure& operator/=(const double&  V);

    CMesure& operator= (const CMesure& M);
	CMesure& operator= (const double&  V);

    bool operator==(const CMesure& M) const;
    bool operator!=(const CMesure& M) const;
    bool operator<=(const CMesure& M) const;
    bool operator>=(const CMesure& M) const;
    bool operator< (const CMesure& M) const;
    bool operator> (const CMesure& M) const;


////////////////////////// Fonctions amies de la classe /////////////////////////

	friend CMesure operator-(CMesure& M);
	friend CMesure operator+(double V, const CMesure& M);
	friend CMesure operator-(double V, const CMesure& M);
	friend CMesure operator*(double V, const CMesure& M);
	friend CMesure operator/(double V, const CMesure& M);

    // affichage au format (valeur +/- IG)
    friend ostream& operator<<(ostream& s, CMesure& M);
	
	// fonctions math�matique perso
	friend CMesure Conjug(CMesure& M);
};

#endif	// fin de _CMESURE_H_
