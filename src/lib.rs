// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//#![allow(unused_variables)]
#![allow(dead_code)]
//#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo


use core::{f32,convert};
use std::{ops,fmt,cmp};
use std::cmp::Ordering;

pub const RMESURE_EPS: f32 = f32::EPSILON;
pub const RMESURE_MAX: f32 = 9223371500000000000.0_f32; //f32::MAX.sqrt()/2.0;


#[cfg_attr(
    all(not(feature = "MagicEverywhere")),
    derive(Debug, Clone)
)]


#[cfg_attr(
    all(feature = "MagicEverywhere"),
    derive(Debug, Clone, Copy)
)]
pub struct RMesure
{
	valeur: f32,
	variance: f32,
    alpha: f32,
}

/*
#[cfg(not(feature = "MagicEverywhere"))]
impl Drop for RMesure { fn drop(&mut self) { } }

#[cfg(not(feature = "MagicEverywhere"))]
impl Clone for RMesure 
{
	fn clone(&self) -> RMesure
	{
		RMesure 
		{ 
			valeur: self.valeur, 
			epsilon: self.epsilon, 
			alpha: self.alpha 
		}
    }
}
*/


impl convert::From<f32> for RMesure 
{
    fn from(scalaire: f32) -> RMesure
	{
		RMesure::scalaire(scalaire)
    }
}


/*
use sscanf::sscanf;
impl convert::From<&str> for RMesure 
{
	fn from(msg: &str) -> Self
	{
		let valeur_loc: f32;
		let epsilon_loc: f32;
		let alpha_loc: f32;

		(valeur_loc, epsilon_loc, alpha_loc) = sscanf::sscanf!(msg, "({f32} +/- {f32} | {f32}%)").unwrap();

		Self 
		{
			valeur: valeur_loc, 
			epsilon: epsilon_loc.abs() / RMesure::K_alpha(alpha_loc), //fabs(it/this->K())
			alpha: alpha_loc 
		}
	}
}
*/


/// (valeur +/- IT() | alpha%)
impl fmt::Display for RMesure 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "({} +/- {} | {}%)", self.Val(), self.IT(), self.Alpha())
    }
}


impl Default for RMesure { fn default() -> RMesure { RMesure::zero() } }

impl RMesure 
{
    pub fn new(valeur: f32, sigma: f32, alpha: f32) -> RMesure 	{ Self { valeur,      variance:       sigma.powf(2.0_f32), alpha        } }
	pub fn zero() -> RMesure 									{ Self { valeur: 0.0, variance: RMESURE_EPS.powf(2.0_f32), alpha: 95.45 } }
	pub fn scalaire(valeur: f32) -> RMesure						{ Self { valeur,      variance: RMESURE_EPS.powf(2.0_f32), alpha: 95.45 } }

	/// Dans le cadre de mesures effectuées dans des conditions bien identifiées,
	/// il est possible d'estimer l'incertitude type directement à partir de
	/// l'intervalle de tolérance à l'aide des lois suivante
	///
	///		1) 'R' : Résolution d'un indicateur numérique       : epsilon = it / rac(12.0)
	///		2) 'H' : Hystérésis tel que it = MAXI - MINI        : epsilon = it / rac(12.0)
	///		3) 'S' : évolution Sinusoïdale sur it = MAXI - MINI : epsilon = it / 1.4
	///		4) 'N' : loi Normale par défaut, K = 2              : epsilon = it / 2.0
	///		5) 'C' : appareil de classe +/- it                  : epsilon = it / rac(3.0)
	/// 	6) 'P' : appareil de classe it = p%					: epsilon = (valeur * p / 100.0) / K_alpha(95.45)
	pub fn loi(valeur: f32, it: f32, loi: char) -> RMesure 
	{
		let inner_epsilon: f32;

		match loi
		{
			'R' => inner_epsilon = it.abs() / 12.0_f32.sqrt(),
			'H' => inner_epsilon = it.abs() / 12.0_f32.sqrt(),
			'S' => inner_epsilon = it.abs() / 2.0_f32.sqrt(),
			'C' => inner_epsilon = it.abs() / 3.0_f32.sqrt(),
			'P' => inner_epsilon = (valeur * it.abs() / 100.0_f32) / RMesure::K_alpha(95.45_f32),
			// c'est la loi par défaut dans tout bon certificat d'étalonnage qui se respecte
			'N' => inner_epsilon = it.abs() / RMesure::K_alpha(95.45_f32), 
			_ => inner_epsilon = it.abs() / RMesure::K_alpha(95.45_f32), 
		}
		
		Self { valeur, variance: inner_epsilon.powf(2.0_f32), alpha: 95.45 }
    }

	fn K_alpha(alpha_loc: f32) -> f32 
	{
		// Calcul par interpolation du coeff d'élargissement à l'aide
		// des valeurs décrites dans la norme "NF ENV 13005"
		let p: [f32; 13] = [99.95 , 99.73 , 99.30, 99.00 , 98.76 , 95.45 , 95.00 , 90.00 , 86.64 , 68.27 , 50.000 , 38.29 , 0.000];
		let k: [f32; 13] = [3.500 , 3.000 , 2.698, 2.576 , 2.500 , 2.000 , 1.960 , 1.645 , 1.500 , 1.000 , 0.6745 , 0.500 , 0.000];

		let mut i = 0;

		// Recherche du cadran dans lequel on se situe
		for j in 1..p.len()
		{
			if alpha_loc >= p[j] 
			{ i = j; break; }
		}

		// Interpolation de la valeur du coefficient d'élargissement
		let a = (k[i] - k[i-1]) / (p[i] - p[i-1]);
		let b = k[i-1] - (a * p[i-1]);

		//return a * alpha_loc + b
		a * alpha_loc + b
	}
	
}

impl RMesure
{	
	pub fn Val(&self) -> f32	  { self.valeur   }	// LA mesure en cours de traitement
	pub fn Alpha(&self) -> f32	  { self.alpha 	  }	// Taux de confiance
	pub fn Variance(&self) -> f32 { self.variance }
	pub fn Eps(&self) -> f32 	  { self.variance.sqrt() }	// Incertitude type.
	pub fn IT(&self) -> f32 	  { self.Eps() * self.K() }	// Intervalle de tolérance = Eps x K

	/// Retourne : (min , 1er quartile , médiane , 3e quartile , max)
	/// Boîte à moustache :
	///		- la boîte --> 50% --> K = 0.6745
	///		- les moustaches --> 99.3% --> K = 2.698
	/// https://fr.wikipedia.org/wiki/Loi_normale#Loi_normale_centr%C3%A9e_r%C3%A9duite
	pub fn BoxPlot(&self) -> (f32,f32,f32,f32,f32) 	
	{ 
		(
			self.valeur - (self.Eps() * RMesure::K_alpha(99.3)),
			self.valeur - (self.Eps() * RMesure::K_alpha(50.0)),
			self.valeur,
			self.valeur + (self.Eps() * RMesure::K_alpha(50.0)),
			self.valeur + (self.Eps() * RMesure::K_alpha(99.3)),
		)
	}	

	// Coeff d'élargissement
	fn K(&self) -> f32 
	{
		RMesure::K_alpha(self.alpha)
	}
}



/************************************************************************/
/*                                                                      */
/*                                                                      */
/*                                                                      */
/*                                                                      */
/*                 Surdéfinition des opérateurs                         */
/*                                                                      */
/*                                                                      */
/*                                                                      */
/*                                                                      */
/************************************************************************/



// ---------------------------
// ---------------------------
// RMesure = RMesure + RMesure
// ---------------------------
// ---------------------------

impl ops::Add<RMesure> for RMesure 
{
    type Output = RMesure;

	/// U²(self + M) = U²(self) + U²(M)
    fn add(self: RMesure, RMesure_rhs: RMesure) -> RMesure
	{
		Self
		{
			valeur: self.Val() + RMesure_rhs.Val(),
			variance: self.Variance() + RMesure_rhs.Variance(),
			alpha: self.Alpha().max(RMesure_rhs.Alpha())
		}
    }
}

// RMesure = constante_f32 + RMesure
// f32.add(RMesure)
impl ops::Add<RMesure> for f32 
{
    type Output = RMesure;
	/// U²(self + M) = U²(self) + U²(M)
    fn add(self: f32, RMesure_rhs: RMesure) -> RMesure 
	{ 
		RMesure::scalaire(self) + RMesure_rhs
	}
}

// RMesure = RMesure + constante_f32
// RMesure.add(f32)
impl ops::Add<f32> for RMesure 
{
    type Output = RMesure;
	/// U²(self + M) = U²(self) + U²(M)
    fn add(self: RMesure, f32_rhs: f32) -> RMesure 
	{
		self + RMesure::scalaire(f32_rhs)
	}
}

// RMesure += RMesure
impl ops::AddAssign<RMesure> for RMesure
{
	fn add_assign(&mut self, RMesure_rhs: RMesure)
	{ 
		*self = self.clone() + RMesure_rhs
	}
}

// RMesure += constante_f32
impl ops::AddAssign<f32> for RMesure
{
	fn add_assign(&mut self, f32_rhs: f32)
	{
		*self = self.clone() + RMesure::scalaire(f32_rhs)
	}
}





// ---------------------------
// ---------------------------
// RMesure = - RMesure
// ---------------------------
// ---------------------------

impl ops::Neg for RMesure
{
	type Output = RMesure;
	fn neg(self) -> RMesure
	{
		RMesure::new(-1.0_f32 * self.Val(), self.Eps(), self.Alpha())
	}
}

// RMesure = RMesure - RMesure
// ---------------------------

impl ops::Sub<RMesure> for RMesure 
{
    type Output = RMesure;
	/// U²(self - M) = U²(self) + U²(M)
    fn sub(self: RMesure, RMesure_rhs: RMesure) -> RMesure
	{
		Self
		{
			valeur: self.Val() - RMesure_rhs.Val(),
			variance: self.Variance() + RMesure_rhs.Variance(),
			alpha: self.Alpha().max(RMesure_rhs.Alpha())
		}
    }
}

// RMesure = constante_f32 - RMesure
// f32.sub(RMesure)
impl ops::Sub<RMesure> for f32 
{
    type Output = RMesure;
	/// U²(self - M) = U²(self) + U²(M)
    fn sub(self: f32, RMesure_rhs: RMesure) -> RMesure 
	{
		RMesure::scalaire(self) - RMesure_rhs
	}
}

// RMesure = RMesure - constante_f32
// RMesure.sub(f32)
impl ops::Sub<f32> for RMesure 
{
    type Output = RMesure;
	/// U²(self - M) = U²(self) + U²(M)
    fn sub(self: RMesure, f32_rhs: f32) -> RMesure 
	{
		self - RMesure::scalaire(f32_rhs)
	}
}

// RMesure -= RMesure
impl ops::SubAssign<RMesure> for RMesure
{
	fn sub_assign(&mut self, RMesure_rhs: RMesure)
	{
		*self = self.clone() - RMesure_rhs
	}
}

// RMesure -= constante_f32
impl ops::SubAssign<f32> for RMesure
{
	fn sub_assign(&mut self, f32_rhs: f32)
	{
		*self = self.clone() - RMesure::scalaire(f32_rhs)
	}
}





// ---------------------------
// ---------------------------
// RMesure = RMesure * RMesure
// ---------------------------
// ---------------------------

impl ops::Mul<RMesure> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²))

    fn mul(self: RMesure, RMesure_rhs: RMesure) -> RMesure
	{
		Self
		{
			valeur: self.Val() * RMesure_rhs.Val(),
			variance: (self.Val().powf(2.0_f32) * RMesure_rhs.Variance()) + (self.Variance() * RMesure_rhs.Val().powf(2.0_f32)),
			alpha: self.Alpha().max(RMesure_rhs.Alpha())
		}
    }
}

// RMesure = constante_f32 * RMesure
// f32.mul(RMesure)
impl ops::Mul<RMesure> for f32 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²))
    fn mul(self: f32, RMesure_rhs: RMesure) -> RMesure 
	{
		RMesure::scalaire(self) * RMesure_rhs
	}
}

// RMesure = RMesure * constante_f32
// RMesure.mul(f32)
impl ops::Mul<f32> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²))
    fn mul(self: RMesure, f32_rhs: f32) -> RMesure 
	{
		self * RMesure::scalaire(f32_rhs)
	}
}

// RMesure *= RMesure
impl ops::MulAssign<RMesure> for RMesure
{
	fn mul_assign(&mut self, RMesure_rhs: RMesure)
	{
		*self = self.clone() * RMesure_rhs
	}
}

// RMesure *= constante_f32
impl ops::MulAssign<f32> for RMesure
{
	fn mul_assign(&mut self, f32_rhs: f32)
	{
		*self = self.clone() * RMesure::scalaire(f32_rhs)
	}
}





// ---------------------------
// ---------------------------
// RMesure = RMesure / RMesure
// ---------------------------
// ---------------------------

impl ops::Div<RMesure> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
	/// CAS DE LA DIVISION DE/PAR ZERO !!! (traite l'infinie comme une valeur)
	///		R.valeur = +/-inf si dénominateur nul
	///		eps = +inf si dénom est nul

    fn div(self: RMesure, RMesure_rhs: RMesure) -> RMesure
	{
		// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
		// CAS DE LA DIVISION DE/PAR ZERO !!! (traite l'infinie comme une valeur)
		//		R.valeur = +/-inf si dénominateur nul
		//		eps = +inf si dénom est nul
		//if RMesure_rhs == RMesure::scalaire(0.0_f32)
		if RMesure_rhs == RMesure::zero()
		{
			Self
			{
				valeur: self.Val().signum() * RMESURE_MAX,
				variance: RMESURE_MAX,
				alpha: self.Alpha().max(RMesure_rhs.Alpha())
			}
		}
		else
		{
			Self
			{
				valeur: self.Val() / RMesure_rhs.Val(),
				variance: ((self.Val().powf(2.0_f32) * RMesure_rhs.Variance()) + (RMesure_rhs.Val().powf(2.0_f32) * self.Variance())) / RMesure_rhs.Val().powf(4.0_f32),
				alpha: self.Alpha().max(RMesure_rhs.Alpha())
			}
		}
    }
}

// RMesure = constante_f32 / RMesure
// f32.div(RMesure)
impl ops::Div<RMesure> for f32 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
    fn div(self: f32, RMesure_rhs: RMesure) -> RMesure 
	{
		RMesure::scalaire(self) / RMesure_rhs
	}
}

// RMesure = RMesure / constante_f32
// RMesure.div(f32)
impl ops::Div<f32> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
    fn div(self: RMesure, f32_rhs: f32) -> RMesure 
	{
		self / RMesure::scalaire(f32_rhs)
	}
}

// RMesure /= RMesure
impl ops::DivAssign<RMesure> for RMesure
{
	fn div_assign(&mut self, RMesure_rhs: RMesure)
	{
		*self = self.clone() / RMesure_rhs
	}
}

// RMesure /= constante_f32
impl ops::DivAssign<f32> for RMesure
{
	fn div_assign(&mut self, f32_rhs: f32)
	{
		*self = self.clone() / RMesure::scalaire(f32_rhs)
	}
}

/////////////////////////////////////////////////////////////////////////////////
//////////////////////// Fonctions de tests pour les VA /////////////////////////
/////////////////////////////////////////////////////////////////////////////////

//
// Dans cette classe, les mesures sont considérées comme des
// variables aléatoires, effectuer un test d'ordonnancement
// entre deux mesures revient à effectuer un test statistique
// entre deux variables aléatoires.
//

/////////////////////////////////////////////////////////////////////////////////
//                                                                             //
//           PRINCIPE DE RESOLUTION DES TESTS D'ORDONNANCEMENT                 //
//           -------------------------------------------------                 //
//                                                                             //
//   -> Calcul de R = A - B                                                    //
//   -> contrôle de la position du résultat par rapport à son propre IT        //
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

//     bool operator==(const CMesure& M) const;		PartialEq eq -> partial_cmp Some(Equal)
//     bool operator!=(const CMesure& M) const;		PartialEq ne -> !eq
//     bool operator<=(const CMesure& M) const;		PartialOrd partial_cmp -> Some(Less | Equal)
//     bool operator>=(const CMesure& M) const;		PartialOrd partial_cmp -> Some(Greater | Equal)
//     bool operator< (const CMesure& M) const;		PartialOrd partial_cmp -> Some(Less)
//     bool operator> (const CMesure& M) const;		PartialOrd partial_cmp -> Some(Greater)



impl cmp::PartialOrd<RMesure> for RMesure 
{
	///
	/// Dans cette classe, les mesures sont considérées comme des
	/// variables aléatoires : tester si A == B revient à effectuer,
	/// ce que l'on nomme en statistique, un test bilatéral.
	///
	/// Cela consiste à calculer la VA équivalente à la différence des
	/// deux VA testées et à vérifier que sa moyenne est comprise dans
	/// son propre intervalle de tolérance centré en zéro.
	///

	fn partial_cmp(&self, RMesure_rhs: &RMesure) -> Option<Ordering>
	{
		let D: RMesure = self.clone() - RMesure_rhs.clone();

		if D.Val().abs() <= D.IT() { Some(Ordering::Equal)   }
		else if D.Val() < -D.IT()  { Some(Ordering::Less)    }
		else                       { Some(Ordering::Greater) }
	}
}

impl cmp::PartialEq<RMesure> for RMesure 
{
	fn eq(&self, RMesure_rhs: &RMesure) -> bool
	{ 
		matches!(self.partial_cmp(RMesure_rhs), Some(Ordering::Equal))
	}
}




/************************************************************************/
/*                                                                      */
/*                                                                      */
/*                                                                      */
/*                                                                      */
/*          Surdéfinition des Fonctions mathématiques                  */
/*                                                                      */
/*                                                                      */
/*                                                                      */
/*                                                                      */
/************************************************************************/


// Les fonctions suivantes nécessite l'utilisation
// de la loi de propagation des incertitudes.
// cf norme NF ENV 13005 (Guide d'Utilisation de la Métrologie : GUM)


// 1 - ce groupe de fonction est à une seule variable,
// ce qui donne, pour l'équation y = f(x), la formule suivante:
// U²(y) = [df(x)/dx]² * U²(x)

impl RMesure 
{
	pub fn abs(self) -> RMesure 
	{
		// si M.Val() <  0 => fabs(M.Val()) = -1.0 * M.Val() et df = -1
		// si M.Val() >= 0 => fabs(M.Val()) =  1.0 * M.Val() et df =  1
		// dans tous les cas, lorsque df sera élevé au carré, df² = 1
		// => U²(x) = sqrt(M.Eps() * M.Eps()) = fabs(M. Eps())

		Self
		{
			valeur: self.Val().abs(),
			variance: self.Variance(),
			alpha: self.Alpha()
		}
	}

	pub fn recip(self) -> RMesure { 1.0_f32 / self }

	pub fn sin(self) -> RMesure
	{
		// d[sin(x)] = cos(x)
		Self
		{
			valeur: self.Val().sin(),
			variance: self.Val().cos().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	}

    pub fn cos(self) -> RMesure 
	{
		// d[cos(x)] = -sin(x)
		// df² = (-1 * sin(x) )² = sin(x)²
		Self
		{
			valeur: self.Val().cos(),
			variance: self.Val().sin().powf(2.0_f32) * self.Variance(), 
			alpha: self.Alpha()
		}
	}

    pub fn tan(self) -> RMesure 
	{
		// d[tan(x)] = 1 + tan²(x)
		Self
		{
			valeur: self.Val().tan(),
			variance: (1.0_f32 + self.Val().tan().powf(2.0_f32)).powf(2.0_f32) * self.Variance(), 
			alpha: self.Alpha()
		}
	}

    pub fn asin(self) -> RMesure 
	{
		// d[asin(x)] = 1 / sqrt(1 - x²)
		// df² = 1² / sqrt(1 - x²)²
		// df² = 1 / (1 - x²)
		Self
		{
			valeur: self.Val().asin(),
			variance: (1.0_f32 - self.Val().powf(2.0_f32)).recip() * self.Variance(), 
			alpha: self.Alpha()
		}		
	}

    pub fn acos(self) -> RMesure 
	{ 	
		// d[acos(x)] = -1 / rac(1 - x²) 
		// df² = (-1)² / sqrt(1 - x²)²
		// df² = 1 / (1 - x²)
		Self
		{
			valeur: self.Val().acos(),
			variance: (1.0_f32 - self.Val().powf(2.0_f32)).recip() * self.Variance(),
			alpha: self.Alpha()
		}
	}

    pub fn atan(self) -> RMesure
	{
		// d[atan(x)] = 1 / (1 - x²)
		Self
		{
			valeur: self.Val().atan(),
			variance: (1.0_f32 - self.Val().powf(2.0_f32)).recip().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 
	
	pub fn sinh(self) -> RMesure
	{
		// d[sinh(x)] = cosh(x)
		Self
		{
			valeur: self.Val().sinh(),
			variance: self.Val().cosh().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 
    
	pub fn cosh(self) -> RMesure 
	{
		// d[cosh(x)] = sinh(x)
		Self
		{
			valeur: self.Val().cosh(),
			variance: self.Val().sinh().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 

    pub fn tanh(self) -> RMesure 
	{
		// d[tanh(x)] = 1 + tanh²(x)
		Self
		{
			valeur: self.Val().tanh(),
			variance: (1.0_f32 + self.Val().tanh().powf(2.0_f32)).powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 

	pub fn asinh(self) -> RMesure 
	{
		// d[asinh(x)] = 1 / sqrt(x² + 1)
		Self
		{
			valeur: self.Val().asinh(),
			variance: (self.Val().powf(2.0_f32) + 1.0_f32).sqrt().recip().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	}

    pub fn acosh(self) -> RMesure 
	{
		// d[acosh(x)] = 1 / sqrt(x² - 1)
		Self
		{
			valeur: self.Val().acosh(),
			variance: (self.Val().powf(2.0_f32) - 1.0_f32).sqrt().recip().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 

    pub fn atanh(self) -> RMesure 
	{
		// d[atanh(x)] = 1 / (1 - x²)
		Self
		{
			valeur: self.Val().atanh(),
			variance: (1.0_f32 - self.Val().powf(2.0_f32)).recip().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	}


	pub fn sin_cos(self) -> (RMesure, RMesure) { (self.clone().sin() , self.cos()) }

	pub fn ln(self) -> RMesure 
	{
		// d[log(x)] = 1 / x
		Self
		{
			valeur: self.Val().ln(),
			variance: self.Val().recip().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 

    pub fn log2(self)  -> RMesure { self.log(2.0_f32.into()) }
	pub fn log10(self) -> RMesure { self.log(10.0_f32.into()) }
	pub fn ln_1p(self) -> RMesure { (1.0_f32 + self).ln() }


    pub fn exp(self) -> RMesure 
	{
		// d[exp(x)] = exp(x)
		Self
		{
			valeur: self.Val().exp(),
			variance: self.Val().exp().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 
    
	pub fn exp2(self)   -> RMesure { RMesure::from(2.0_f32).powf(self) }
    pub fn exp_m1(self) -> RMesure { self.exp() - RMesure::from(1.0_f32) }

	pub fn sqrt(self) -> RMesure
	{
		// d[rac(x)] = 1 / (2 * rac(x))
		Self
		{
			valeur: self.Val().sqrt(),
			variance: (2.0_f32 * self.Val().sqrt()).recip().powf(2.0_f32) * self.Variance(),
			alpha: self.Alpha()
		}
	} 

	// racine cubique de x
	//pub fn cbrt(self) -> RMesure { self.powf(RMesure::from(1.0_f32 / 3.0_f32)) }
	pub fn cbrt(self) -> RMesure { self.powf((1.0_f32 / 3.0_f32).into()) }

}


// 2 - ce groupe de fonction est à deux variables,
// ce qui donne, pour l'équation y = f(x,y), la formule suivante:
// U²(y) = [df(x)/dx]² * U²(x) + [df(y)/dy]² * U²(y)

impl RMesure 
{
	pub fn log(self, base: RMesure) -> RMesure 
	{
		// WxMaxima
		// kill(all);
		// LogN(x, n) := ln(x) / ln(base);
		// diff (LogN(x, base), x, 1); 
		// diff (LogN(x, base), base, 1);

		// df(x)/dx = 1/(ln(base)*x)
		// df(y)/dy = -ln(x)/(base*ln(base)^2)
		Self
		{
			valeur: self.Val().log(base.Val()),
			variance: ((base.Val().ln() * self.Val()).recip().powf(2.0_f32) * self.Variance()) + ((self.Val().ln() / (base.Val() * base.Val().ln().powf(2.0_f32))).powf(2.0_f32) * base.Variance()),
			alpha: self.Alpha()
		}
	}

	// pub fn powi(self, puiss: i32) -> RMesure { self.powf(RMesure::from(puiss as f32)) }
	pub fn powi(self, puiss: i32) -> RMesure { self.powf((puiss as f32).into()) }
    
	pub fn powf(self, puiss: RMesure) -> RMesure
	{
		// [df(x)/dx]² * U²(x) + [df(y)/dy]² * U²(y)
		//
		// d[pow(p,base)] = d[b^p]/db + d[b^p]/dp
		//
		// d[b^p]/db = b^(p-1) * p
		// d[b^p]/dp = b^p * ln(b)

		Self
		{
			valeur: self.Val().powf(puiss.Val()),
			variance: (self.Val().powf(puiss.Val() - 1.0_f32) * puiss.Val()).powf(2.0_f32) * self.Variance() + (self.Val().powf(puiss.Val()) * self.Val().ln()).powf(2.0_f32) * puiss.Variance(),
			alpha: self.Alpha()
		}
	}

	pub fn atan2(self, Y: RMesure) -> RMesure 
	{
		// atan((CMesure&) (Y / X) )
		if self == RMesure::zero() && Y == RMesure::zero() 
		{ RMesure::zero() }
		else if self >= RMesure::zero()
		{ (Y / self).atan() }
		else if Y >= RMesure::zero()
		{ (Y / self).atan() + RMesure::from(std::f32::consts::PI) }
		else
		{ (Y / self).atan() - RMesure::from(std::f32::consts::PI) }
	}	

	pub fn hypot(self, Y: RMesure) -> RMesure { (self.powi(2) + Y.powi(2)).sqrt() }

}




// 3 - ce groupe de fonction me pose problème pour déterminer
// comment calculer l'incertitude type. Je vais certainement faire
// un choix personnel unilatérale parfaitement arbitraire.

impl RMesure 
{
    // Le calcul d'epsilon pour floor et ceil a posé plusieurs questions:
	//		1) Application d'un coeff de proportionnalité newVal/oldVal ?
	//		2) Considérer qu'une valeur seuillée possède un epsilon nul 
	//		   car c'est une valeure certaine ?
	//		3) Augmenter epsilon de suffisament pour conserver l'ancien IT
	//		   dans un nouveau centré en newVal ?
	//
	// Résultat de mes réflexions : 
	//		Solution 1 : problème identifier si floor ou ceil retourne 0.0 car 
	//		cela provoque un epsilon infini => REFUSEE
	//		Solution 2 : supprimer de façon artificielle une incertitude sur
	//		une mesure est contraire à la philosophie de cette classe => REFUSEE
	//		Solution 3 : c'est la moins pire, selon moi, elle offre un compromis 
	//		acceptable entre la philosophie de cette classe et les loies math�matiques
	//		sous jacente

	pub fn ceil(self) -> RMesure 
	{
		Self
		{
			valeur: self.Val().ceil(),
			variance: ((self.IT() + (self.Val().ceil() - self.Val()).abs()) / self.K()).powf(2.0_f32),
			alpha: self.Alpha()
		}
	}

	pub fn floor(self) -> RMesure 
	{
		Self
		{
			valeur: self.Val().floor(),
			variance: ((self.IT() + (self.Val().floor() - self.Val()).abs()) / self.K()).powf(2.0_f32),
			alpha: self.Alpha()
		}
	}
		
	pub fn max(self, Y: RMesure) -> RMesure
	{
		if self == Y 		{ (self + Y) / 2.0_f32 }	// cas indécidable
		else if self < Y	{ Y                    }	// self < Y
		else 				{ self                 }	// self > Y
	}

    pub fn min(self, Y: RMesure) -> RMesure 
	{
		if self == Y 		{ (self + Y) / 2.0_f32 }	// cas indécidable
		else if self < Y	{ self                 }	// self < Y
		else 				{ Y                    }	// self > Y
	}

	pub fn signum(self) -> RMesure { RMesure::from(self.Val().signum()) }

}




/*
impl Float for RMesure 
{
	pub fn round(self)  -> RMesure { todo!() }
    pub fn trunc(self)  -> RMesure { todo!() }
    pub fn fract(self)  -> RMesure { todo!() }
		
	
	
	fn nan() -> Self { todo!() }
    fn infinity() -> Self { todo!() }
    fn neg_infinity() -> Self { todo!() }
    fn neg_zero() -> Self { todo!() }

    fn is_nan(self) -> bool { todo!() }
    fn is_infinite(self) -> bool { todo!() }
    fn is_finite(self) -> bool { todo!() }
    fn is_normal(self) -> bool { todo!() }
    fn classify(self) -> FpCategory { todo!() }

	fn is_sign_positive(self) -> bool { todo!() }
    fn is_sign_negative(self) -> bool { todo!() }

	fn min_value() -> Self { todo!() }
    fn min_positive_value() -> Self { todo!() }
    fn max_value() -> Self { todo!() }

    fn mul_add(self, _: Self, _: Self) -> Self { todo!() }
    fn abs_sub(self, _: Self) -> Self { todo!() }
    fn integer_decode(self) -> (u64, i16, i8) { todo!() }
}
*/