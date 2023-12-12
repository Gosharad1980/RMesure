// Cette ligne est sp�cifique � Visual C++
// pour pouvoir utiliser le scanf
#define _CRT_SECURE_NO_WARNINGS

#include <stdio.h>
#include <iostream>
#include <fstream>
#include <time.h>
#include <cmath>

#include "CMesure/CMesure.h"
#include "CMesure/FctMathMesure.h"

#define MAXCHAR 255

using namespace std;
void benchMark1(void)
{

	CMesure a(0.23, 0.07);               // CMesure(double v, double it, char loi = 'N');
	CMesure b(2.0, 0.1, 'R');            // CMesure(double v, double it, char loi);
	CMesure c(-1.0, 0.19);               // CMesure(double v, double it, char loi = 'N');
	CMesure d(1.0, 0.1, 120.0);          // CMesure(double v, double e, double a);
	CMesure e((char*)"(3.0+/-0.00005|95.45%)"); // CMesure(char* m);
	CMesure f;                           // CMesure();
	CMesure g(0.0);

	char ReInit[MAXCHAR];

	cout << "a = " << a << endl;
	cout << "b = " << b << endl;
	cout << "c = " << c << endl;
	cout << "d = " << d << endl;
	cout << "e = " << e << endl;
	cout << "f = " << f << endl;
	cout << "g = " << g << endl;

	cout << endl;

    cout << "fabs(a)   = " << (CMesure&)fabs(a)   << endl;
	cout << "sin(a)    = " << (CMesure&)sin(a)    << endl;
	cout << "cos(a)    = " << (CMesure&)cos(a)    << endl;
	cout << "tan(a)    = " << (CMesure&)tan(a)    << endl;
	cout << "acos(a)   = " << (CMesure&)acos(a)   << endl;
	cout << "asin(a)   = " << (CMesure&)asin(a)   << endl;
	cout << "atan(a)   = " << (CMesure&)atan(a)   << endl;
	cout << "cosh(a)   = " << (CMesure&)cosh(a)   << endl;
	cout << "sinh(a)   = " << (CMesure&)sinh(a)   << endl;
	cout << "tanh(a)   = " << (CMesure&)tanh(a)   << endl;
	cout << "log(a)    = " << (CMesure&)log(a)    << endl;
	cout << "log10(a)  = " << (CMesure&)log10(a)  << endl;
	cout << "exp(a)    = " << (CMesure&)exp(a)    << endl;
	cout << "sqrt(a)   = " << (CMesure&)sqrt(a)   << endl;

	cout << "ceil(a)   = " << (CMesure)ceil(a)   << endl;
	cout << "floor(a)  = " << (CMesure)floor(a)  << endl;

	cout << "pow(a, b) = "       << (CMesure&)pow(a, b)       << endl;
	cout << "pow(a.Val(), b) = " << (CMesure&)pow(a.Val(), b) << endl;
	cout << "pow(a, b.Val()) = " << (CMesure&)pow(a, b.Val()) << endl;

	cout << "atan2(a, b) = " << (CMesure&)atan2(a, b) << endl;
	cout << "modf(a, &e) = " << (CMesure&)modf(a, &e) << endl;
	cout << "e = "           << e           << endl;
    
	cout << endl;

                cout << "a+b        = " << (CMesure&)(a+b)       << endl;
    a+=b;       cout << "a+=b       = " << a         << endl;
	            cout << "a+b.Val()  = " << (a+b).Val() << endl;
	a+=b.Val(); cout << "a+=b.Val() = " << a         << endl;
                cout << "a-b        = " << (CMesure&)(a-b)       << endl;
    a-=b;       cout << "a-=b       = " << a         << endl;
	            cout << "a-b.Val()  = " << (a-b).Val() << endl;
	a-=b.Val(); cout << "a-=b.Val() = " << a         << endl;
                cout << "a*b        = " << (CMesure&)(a*b)       << endl;
    a*=b;       cout << "a*=b       = " << a         << endl;
	            cout << "a*b.Val()  = " << (a*b).Val() << endl;
	a*=b.Val(); cout << "a*=b.Val() = " << a         << endl;
                cout << "a/b        = " << (CMesure&)(a/b)       << endl;
    a/=b;       cout << "a/=b       = " << a         << endl;
	            cout << "a/b.Val()  = " << (a/b).Val() << endl;
	a/=b.Val(); cout << "a/=b.Val() = " << a         << endl;
    a=b;        cout << "a=b        = " << a         << endl;
	a=b.Val();  cout << "a=b.Val()  = " << a         << endl;

	cout << endl;

	/* cout << system("DIR .") << endl; */

	fstream fin("input.txt");
	fin.getline(ReInit, MAXCHAR, '\n');	a = CMesure(ReInit);
	fin.getline(ReInit, MAXCHAR, '\n'); b = CMesure(ReInit);
	fin.getline(ReInit, MAXCHAR, '\n');	c = CMesure(ReInit);
	fin.getline(ReInit, MAXCHAR, '\n');	d = CMesure(ReInit);
	fin.close();

	cout << "a = " << a << endl;
	cout << "b = " << b << endl;
	cout << "c = " << c << endl;
	cout << "d = " << d << endl;


	/*
	fstream fin("input.txt");
	cout << "�a chie dans la colle ?" << fin.fail() << endl;
	fin >> ReInit;	cout << "a=" << ReInit << endl;		a = CMesure(ReInit); 	cout << "a = " << a << endl;
	fin >> ReInit;	cout << "b=" << ReInit << endl;		b = CMesure(ReInit);	cout << "b = " << b << endl;
	fin >> ReInit;	cout << "c=" << ReInit << endl;		c = CMesure(ReInit);	cout << "c = " << c << endl;
	fin >> ReInit;	cout << "d=" << ReInit << endl;		d = CMesure(ReInit);	cout << "d = " << d << endl;
	fin.close();
	*/

	cout << endl;

	cout << "a == d [ FAUX ] => " << (a==d ? "VRAI" : "FAUX") << endl; // significativement diff�rents
	cout << "b == c [ VRAI ] => " << (b==c ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents

	cout << "a != d [ VRAI ] => " << (a!=d ? "VRAI" : "FAUX") << endl; // significativement diff�rents
	cout << "b != c [ FAUX ] => " << (b!=c ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents

	cout << "a < d  [ VRAI ] => " << (a<d  ? "VRAI" : "FAUX") << endl; // a.valeur < d.valeur et a!=d
	cout << "d < a  [ FAUX ] => " << (d<a  ? "VRAI" : "FAUX") << endl; // d.valeur > a.valeur et d!=a
	cout << "c < b  [ FAUX ] => " << (c<b  ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents
	cout << "b < c  [ FAUX ] => " << (b<c  ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents

	cout << "a > d  [ FAUX ] => " << (a>d  ? "VRAI" : "FAUX") << endl; // a.valeur < d.valeur et a!=d
	cout << "d > a  [ VRAI ] => " << (d>a  ? "VRAI" : "FAUX") << endl; // d.valeur > a.valeur et d!=a
	cout << "c > b  [ FAUX ] => " << (c>b  ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents
	cout << "b > c  [ FAUX ] => " << (b>c  ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents

	cout << "a <= d [ VRAI ] => " << (a<=d ? "VRAI" : "FAUX") << endl; // a.valeur < d.valeur et a!=d
	cout << "d <= a [ FAUX ] => " << (d<=a ? "VRAI" : "FAUX") << endl; // d.valeur > a.valeur et d!=a
	cout << "c <= b [ VRAI ] => " << (c<=b ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents
	cout << "b <= c [ VRAI ] => " << (b<=c ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents

	cout << "a >= d [ FAUX ] => " << (a>=d ? "VRAI" : "FAUX") << endl; // a.valeur < d.valeur et a!=d
	cout << "d >= a [ VRAI ] => " << (d>=a ? "VRAI" : "FAUX") << endl; // d.valeur > a.valeur et d!=a
	cout << "c >= b [ VRAI ] => " << (c>=b ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents
	cout << "b >= c [ VRAI ] => " << (b>=c ? "VRAI" : "FAUX") << endl; // insignificativement diff�rents

	cout << endl;

    cout << "Conjug(c) = " << (CMesure&)Conjug(c) << endl;
	// cout << "Nulle(c)  = " << Nulle(c)  << endl;

	cout << endl;

	cout << "-b  = "         << (CMesure&)(-b)          << endl;
	cout << "a.Val() + b = " << (CMesure&)(a.Val() + b) << endl;
	cout << "a.Val() - b = " << (CMesure&)(a.Val() - b) << endl;
	cout << "a.Val() * b = " << (CMesure&)(a.Val() * b) << endl;
	cout << "a.Val() / b = " << (CMesure&)(a.Val() / b) << endl;

	cout << endl;
}

void benchMark2(void)
{
	
	CMesure A(0, 0);
	CMesure B(1, 1);
	CMesure C(0, 0);
	CMesure D(0, 0);

	C = A * B;	// C = (0, 1.0e260)
	D = C - A;	// D = (0, 1.0e260)


	cout << "A = " << A << endl;
	cout << "B = " << B << endl;
	cout << "C = A * B = " << C << endl;
	cout << "D = C - A = " << D << endl;

	cout << "C == A => " << (C == A ? "VRAI" : "FAUX") << endl;
	cout << "C != A => " << (C != A ? "VRAI" : "FAUX") << endl;
	
	cout << endl;
}
int main(int argc, char *argv[])
{

	benchMark1();
	benchMark2();

	// { 99.95 , 99.73 , 99.00 , 95.45 , 95.00 , 90.00 , 68.27 , 0.000 };
    // { 4.000 , 3.000 , 2.576 , 2.000 , 1.960 , 1.645 , 1.000 , 0.000 };

	/*
	for(int i=1; i<=110; i++)
	{
		CMesure A(1.0, 1.0e-12, (double)i);
		cout << "alpha = " << A.Alpha() << "\tK = " << A.K() << endl;
	}
	*/

	getchar();
}
