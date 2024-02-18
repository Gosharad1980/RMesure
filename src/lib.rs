// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//#![allow(unused_variables)]
//#![allow(dead_code)]
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
	epsilon: f32,
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
    pub fn new(valeur: f32, epsilon: f32, alpha: f32) -> RMesure 	{ Self { valeur,      epsilon,              alpha        } }
	pub fn zero() -> RMesure 										{ Self { valeur: 0.0, epsilon: RMESURE_EPS, alpha: 95.45 } }
	pub fn scalaire(valeur: f32) -> RMesure							{ Self { valeur,      epsilon: RMESURE_EPS, alpha: 95.45 } }

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
			'P' => inner_epsilon = (valeur * it.abs() / 100.0_f32) / RMesure::K_alpha(95.45),
			// c'est la loi par défaut dans tout bon certificat d'étalonnage qui se respecte
			'N' => inner_epsilon = it.abs() / RMesure::K_alpha(95.45), 
			_ => inner_epsilon = it.abs() / RMesure::K_alpha(95.45), 
		}
		
		Self { valeur, epsilon: inner_epsilon, alpha: 95.45 }

    }

	fn K_alpha(alpha_loc: f32) -> f32 
	{
		// Calcul par interpolation du coeff d'élargissement à l'aide
		// des valeurs décrites dans la norme "NF ENV 13005"
		// Boîte à moustache :
		// - la boîte --> 50% --> K = 0.6745
		// - les moustaches --> 99.3% --> K = 2.698
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
	pub fn Val(&self) -> f32	{ self.valeur 	}	// LA mesure en cours de traitement
	pub fn Alpha(&self) -> f32	{ self.alpha 	}	// Taux de confiance
	pub fn Eps(&self) -> f32 	{ self.epsilon	}	// Incertitude type.
	pub fn IT(&self) -> f32 	{ self.epsilon * self.K() }	// Intervalle de tolérance = Eps x K

	/// Retourne : (min , 1er quartile , médiane , 3e quartile , max)
	/// https://fr.wikipedia.org/wiki/Loi_normale#Loi_normale_centr%C3%A9e_r%C3%A9duite
	pub fn BoxPlot(&self) -> (f32,f32,f32,f32,f32) 	
	{ 
		(
			self.valeur - (self.epsilon * RMesure::K_alpha(99.3)),
			self.valeur - (self.epsilon * RMesure::K_alpha(50.0)),
			self.valeur,
			self.valeur + (self.epsilon * RMesure::K_alpha(50.0)),
			self.valeur + (self.epsilon * RMesure::K_alpha(99.3)),
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
			valeur: self.valeur + RMesure_rhs.valeur,
			epsilon: (self.epsilon.powf(2.0_f32) + RMesure_rhs.epsilon.powf(2.0_f32)).sqrt(),
			alpha: self.alpha.max(RMesure_rhs.alpha)
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
	{ RMesure::scalaire(self) + RMesure_rhs }
}

// RMesure = RMesure + constante_f32
// RMesure.add(f32)
impl ops::Add<f32> for RMesure 
{
    type Output = RMesure;
	/// U²(self + M) = U²(self) + U²(M)
    fn add(self: RMesure, f32_rhs: f32) -> RMesure 
	{ self + RMesure::scalaire(f32_rhs) }
}

// RMesure += RMesure
impl ops::AddAssign<RMesure> for RMesure
{
	fn add_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = self.clone() + RMesure_rhs }
}

// RMesure += constante_f32
impl ops::AddAssign<f32> for RMesure
{
	fn add_assign(&mut self, f32_rhs: f32)
	{ *self = self.clone() + RMesure::scalaire(f32_rhs) }
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
	{ RMesure::new(-1.0_f32 * self.valeur, self.epsilon, self.alpha) }
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
			valeur: self.valeur - RMesure_rhs.valeur,
			epsilon: (self.epsilon.powf(2.0_f32) + RMesure_rhs.epsilon.powf(2.0_f32)).sqrt(),
			alpha: self.alpha.max(RMesure_rhs.alpha)
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
	{ RMesure::scalaire(self) - RMesure_rhs }
}

// RMesure = RMesure - constante_f32
// RMesure.sub(f32)
impl ops::Sub<f32> for RMesure 
{
    type Output = RMesure;
	/// U²(self - M) = U²(self) + U²(M)
    fn sub(self: RMesure, f32_rhs: f32) -> RMesure 
	{ self - RMesure::scalaire(f32_rhs) }
}

// RMesure -= RMesure
impl ops::SubAssign<RMesure> for RMesure
{
	fn sub_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = self.clone() - RMesure_rhs }
}

// RMesure -= constante_f32
impl ops::SubAssign<f32> for RMesure
{
	fn sub_assign(&mut self, f32_rhs: f32)
	{ *self = self.clone() - RMesure::scalaire(f32_rhs) }
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
			valeur: self.valeur * RMesure_rhs.valeur,
			epsilon: ((self.Val().powf(2.0_f32) * RMesure_rhs.epsilon.powf(2.0_f32)) + (self.epsilon.powf(2.0_f32) * RMesure_rhs.valeur.powf(2.0_f32))).sqrt(),
			alpha: self.alpha.max(RMesure_rhs.alpha)
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
	{ RMesure::scalaire(self) * RMesure_rhs }
}

// RMesure = RMesure * constante_f32
// RMesure.mul(f32)
impl ops::Mul<f32> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²))
    fn mul(self: RMesure, f32_rhs: f32) -> RMesure 
	{ self * RMesure::scalaire(f32_rhs) }
}

// RMesure *= RMesure
impl ops::MulAssign<RMesure> for RMesure
{
	fn mul_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = self.clone() * RMesure_rhs }
}

// RMesure *= constante_f32
impl ops::MulAssign<f32> for RMesure
{
	fn mul_assign(&mut self, f32_rhs: f32)
	{ *self = self.clone() * RMesure::scalaire(f32_rhs) }
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
				valeur: self.valeur.signum() * RMESURE_MAX,
				epsilon: RMESURE_MAX,
				alpha: self.alpha.max(RMesure_rhs.alpha)
			}
		}
		else
		{
			Self
			{
				valeur: self.valeur / RMesure_rhs.valeur,
				epsilon: ((self.epsilon.powf(2.0_f32) * RMesure_rhs.valeur.powf(2.0_f32)) + (RMesure_rhs.epsilon.powf(2.0_f32) * self.Val().powf(2.0_f32))).sqrt() / RMesure_rhs.valeur.powf(2.0_f32),
				alpha: self.alpha.max(RMesure_rhs.alpha)
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
	{ RMesure::scalaire(self) / RMesure_rhs }
}

// RMesure = RMesure / constante_f32
// RMesure.div(f32)
impl ops::Div<f32> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
    fn div(self: RMesure, f32_rhs: f32) -> RMesure 
	{ self / RMesure::scalaire(f32_rhs) }
}

// RMesure /= RMesure
impl ops::DivAssign<RMesure> for RMesure
{
	fn div_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = self.clone() / RMesure_rhs }
}

// RMesure /= constante_f32
impl ops::DivAssign<f32> for RMesure
{
	fn div_assign(&mut self, f32_rhs: f32)
	{ *self = self.clone() / RMesure::scalaire(f32_rhs) }
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

		if D.valeur.abs() <= D.IT() { Some(Ordering::Equal)   }
		else if D.valeur < -D.IT()  { Some(Ordering::Less)    }
		else                        { Some(Ordering::Greater) }
	}
}

impl cmp::PartialEq<RMesure> for RMesure 
{
	fn eq(&self, RMesure_rhs: &RMesure) -> bool
	{ 
		matches!(self.partial_cmp(RMesure_rhs), Some(Ordering::Equal))
	}
}