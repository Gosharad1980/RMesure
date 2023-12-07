#include <cmath>

#include "CMesure.h"

#include "FctMathMesure.h"

using namespace std;

////////////////////////////////////////////////////////////////////////////////
///////////////////////////// Fonctions math�matiques //////////////////////////
////////////////////////////////////////////////////////////////////////////////

// fonctions � une variable //
// U�(y) = [df(x)/dx]� * U�(x)

CMesure fabs (CMesure& M) 
{
	// si M.Val() <  0 => fabs(M.Val()) = -1.0 * M.Val() et df = -1
	// si M.Val() >= 0 => fabs(M.Val()) =  1.0 * M.Val() et df =  1
	// dans tous les cas, lorsque df sera �lev� au carr�, df� = 1
	// => U�(x) = sqrt(M.Eps() * M.Eps()) = fabs(M. Eps())

	return CMesure(
		(double)fabs(M.Val()), 
		(double)fabs(M.Eps()), 
		M.Alpha());
}

CMesure sin  (CMesure& M)
{
	// d[sin(x)] = cos(x)
	
	double df;
	df = (double)cos(M.Val());
	
	return CMesure(
		(double)sin(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure cos  (CMesure& M)
{
	// d[cos(x)] = -sin(x)
	
	double df;
	df = (-1.0) * (double)sin(M.Val());
	
	return CMesure(
		(double)cos(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure tan  (CMesure& M)
{
	// d[tan(x)] = 1 + tan�(x)
	
	double df;
	df = 1.0 + (double)tan(M.Val()) * (double)tan(M.Val());

	return CMesure(
		(double)tan(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure acos (CMesure& M)
{
	// d[acos(x)] = -1 / rac(1 - x�)
	
	double df;
	df = (double) ( (-1.0) / ((double)sqrt(1.0 - M.Val() * M.Val())) );

	return CMesure(
		(double)acos(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure asin (CMesure& M)
{
	// d[asin(x)] = 1 / rac(1 - x�)
	
	double df;
	df = (double) ( (1.0) / ((double)sqrt(1.0 - M.Val() * M.Val())) );

	return CMesure(
		(double)asin(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure atan (CMesure& M)
{
	// d[atan(x)] = 1 / (1 - x�)
	
	double df;
	df = (double) ( (1.0) / (1.0 - M.Val() * M.Val()) );

	return CMesure(
		(double)atan(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());	
}

CMesure cosh (CMesure& M)
{
	// d[acos(x)] = sinh(x)
	
	double df;
	df = (double) sinh(M.Val());

	return CMesure(
		(double)cosh(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure sinh (CMesure& M)
{
	// d[sinh(x)] = cosh(x)
	
	double df;
	df = (double) cosh(M.Val());

	return CMesure(
		(double)sinh(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure tanh (CMesure& M)
{
	// d[tanh(x)] = 1 + tanh�(x)
	
	double df;
	df = 1.0 + (double)tanh(M.Val()) * (double)tanh(M.Val());

	return CMesure(
		(double)tanh(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure log  (CMesure& M)
{
	// d[log(x)] = 1 / x
	
	double df;
	df = 1.0 / M.Val();

	return CMesure(
		(double)log(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure log10(CMesure& M)
{
	// d[log10(x)] = d[log(x)/log(10)]
	//             = 1/log(10) * d[log(x)]
	//             = 1/log(10) * 1/x

	double df;
	df = (1.0 / log(10.0)) * (1.0 / M.Val());

	return CMesure(
		(double)log10(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure exp  (CMesure& M)
{
	// d[exp(x)] = exp(x)

	double df;
	df = exp(M.Val());

	return CMesure(
		(double)exp(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

CMesure sqrt (CMesure& M)
{
	// d[rac(x)] = 1 / (2 * rac(x))

	double df;
	df = 1.0 / (2.0 * sqrt(M.Val()));
	
	return CMesure(
		(double)sqrt(M.Val()), 
		(double)sqrt(df * df * M.Eps() * M.Eps()), 
		M.Alpha());
}

// Le calcul d'epsilon pour floor et ceil a pos� plusieurs questions:
//		1) Application d'un coeff de proportionnalit� newVal/oldVal ?
//		2) Consid�rer qu'une valeur seuill�e poss�de un epsilon nul 
//		   car c'est une valeure certaine ?
//		3) Augmenter epsilon de suffisament pour conserver l'ancien IT
//		   dans un nouveau centr� en newVal ?
//
// R�sultat de mes r�flexions : 
//		Solution 1 : probl�me identifier si floor ou ceil retourne 0.0 car 
//		cela provoque un epsilon infini => REFUSEE
//		Solution 2 : supprimer de fa�on artificielle une incertitude sur
//		une mesure est contraire � la philosophie de cette classe => REFUSEE
//		Solution 3 : c'est la moins pire, selon moi, elle offre un compromis 
//		acceptable entre la philosophie de cette classe et les loies math�matiques
//		sous jacente

CMesure ceil (CMesure& M)
{ 
	double valeur, epsilon, alpha;
	valeur  = ceil(M.Val());
	epsilon = (M.IT() + fabs(valeur - M.Val())) / M.K();
	alpha   = M.Alpha();
	return CMesure(valeur, epsilon, alpha);
}

CMesure floor(CMesure& M)
{
	double valeur, epsilon, alpha;
	valeur  = floor(M.Val());
	epsilon = (M.IT() + fabs(M.Val() - valeur)) / M.K();
	alpha   = M.Alpha();
	return CMesure(valeur, epsilon, alpha);
}

// fonctions � deux variables //
// U�(y) = [df(x)/dx]� * U�(x) + [df(y)/dy]� * U�(y)

CMesure pow(CMesure& Base, CMesure& Puiss)
{
	// d[pow(b,p)] = d[b^p]/db + d[b^p]/dp
	//
	// d[b^p]/db = p * b^(p-1)
	//
	// d[b^p]/dp = d[e^(p*ln(b))]
	//           = b^(p-1) * (b*1*ln(b) + 1*b)
	//           = b(1+ln(b))*b^(p-1)
	//           = (1+ln(b))*b^p
	
	double dfB, dfP;

	dfB = Puiss.Val() * pow(Base.Val(), Puiss.Val() - 1.0);
	dfP = (1.0 + log(Base.Val())) * pow(Base.Val(), Puiss.Val());

	return CMesure(
		(double)pow(Base.Val(), Puiss.Val()), 
		(double)sqrt(dfB * dfB * Base.Eps() * Base.Eps() + dfP * dfP * Puiss.Eps() * Puiss.Eps()), 
		(Base.Alpha() > Puiss.Alpha() ? Base.Alpha() : Puiss.Alpha()) 
				  );
}

CMesure pow  (double Base, CMesure& Puiss) { return pow((CMesure&) CMesure(Base), (CMesure&) Puiss);          }
CMesure pow  (CMesure& Base, double Puiss) { return pow((CMesure&) Base,          (CMesure&) CMesure(Puiss)); }
CMesure atan2(CMesure& X, CMesure& Y)      { return atan((CMesure&) (Y / X) );             }

CMesure modf (CMesure& X, CMesure* Fpart)
{
	(*Fpart) = X - floor(X);
	return floor(X);
}


// Ce groupe de fonction me pose probl�me pour d�terminer
// comment calculer l'incertitude type. Je vais certainement faire
// un choix personnel unilat�rale parfaitement arbitraire.

// Pour une incertitude de mesure les MIN et MAX ne sont pas sym�triques
// Etant donn� que les in�galit�s font intervenir les IT, il y a trois cas :
// -> A<B
// -> A>B
// -> A potentiellement �gal � B : donc A ni inff ni supp � B mais A.val != B.val
// Dans ce troisi�me cas, faut-il
// 1) retourner A ? (puisque ind�cidable donc potentiellement �quivalent)
// 2) retourner B ? (puisque ind�cidable donc potentiellement �quivalent)
// 3) une autre valeur => choix unilat�ral personnel parfaitement arbitraire de retourner la moyenne des deux ... et je vous emmerde

CMesure MIN (CMesure& A, CMesure& B)
{ 
	CMesure R;

	if (A == B)	    { R = (A + B) / 2.0; }	// cas ind�cidable
	else if (A < B) { R = A; }				// A < B
	else            { R = B; }				// A > B
	
	return R; 
}

CMesure MAX (CMesure& A, CMesure& B) 
{ 
	CMesure R;

	if (A == B)	    { R = (A + B) / 2.0; }	// cas ind�cidable
	else if (A < B) { R = B; }				// A < B
	else            { R = A; }				// A > B
	
	return R; 
}

CMesure SIGN(CMesure& A)
{ 
	return ((A.Val() >= 0) ? CMesure(1.0) : CMesure(-1.0)); 
}

