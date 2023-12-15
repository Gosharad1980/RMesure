// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo


use core::f64;
use std::{ops,fmt,cmp};
use std::cmp::Ordering;
//use sscanf::sscanf;



pub const RMESURE_EPS: f64 = f64::EPSILON;
pub const RMESURE_MAX: f64 = 9223371500000000000.0_f64; //f32::MAX.sqrt()/2.0;

//#[derive(Debug)]
#[derive(Debug, Clone, Copy)]
pub struct RMesure
{
	valeur: f64,
	epsilon: f64,
    alpha: f64,
}

/*
impl Copy for RMesure {}

impl Clone for RMesure 
{
    fn clone(&self) -> Self
	{
		Self 
		{ 
			valeur: self.valeur, 
			epsilon: self.epsilon, 
			alpha: self.alpha 
		}
    }
}

impl Drop for RMesure { fn drop(&mut self) { } }
*/

/// (valeur +/- IT() | alpha%)
impl fmt::Display for RMesure 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "({} +/- {} | {}%)", self.valeur, self.IT(), self.alpha)
    }
}

impl RMesure 
{
    pub fn new(valeur: f64, epsilon: f64, alpha: f64) -> Self 
	{
        Self { valeur, epsilon, alpha }
    }

	pub fn zero() -> Self 
	{
		Self { valeur: 0.0, epsilon: RMESURE_EPS, alpha: 95.45 }
    }

	pub fn scalaire(valeur: f64) -> Self
	{
		Self { valeur, epsilon: RMESURE_EPS, alpha: 95.45 }
	}

	/*
	pub fn load(msg: &str) -> Self
	{
		let valeur_loc: f64;
		let epsilon_loc: f64;
		let alpha_loc: f64;

		(valeur_loc, epsilon_loc, alpha_loc) = sscanf::sscanf!(msg, "({f64} +/- {f64} | {f64}%)").unwrap();

		Self 
		{
			valeur: valeur_loc, 
			epsilon: epsilon_loc.abs() / RMesure::K_alpha(alpha_loc), //fabs(it/this->K())
			alpha: alpha_loc 
		}
	}
	*/

	/// Dans le cadre de mesures effectuées dans des conditions bien identifiées,
	/// il est possible d'estimer l'incertitude type directement à partir de
	/// l'intervalle de tolérance à l'aide des lois suivante
	///
	///		1) 'R' : Résolution d'un indicateur numérique       : epsilon = it / rac(12.0)
	///		2) 'H' : Hystérésis tel que it = MAXI - MINI        : epsilon = it / rac(12.0)
	///		3) 'S' : évolution Sinusoïdale sur it = MAXI - MINI : epsilon = it / 1.4
	///		4) 'N' : loi Normale par défaut, K = 2              : epsilon = it / 2.0
	///		5) 'C' : appareil de Classe +/- it                  : epsilon = it / rac(3.0)
	pub fn loi(valeur: f64, it: f64, loi: char) -> Self 
	{
		let inner_epsilon: f64;

		match loi
		{
			'R' => inner_epsilon = it.abs() / 12.0_f64.sqrt(),
			'H' => inner_epsilon = it.abs() / 12.0_f64.sqrt(),
			'S' => inner_epsilon = it.abs() / 1.4_f64,
			'C' => inner_epsilon = it.abs() / 3.0_f64.sqrt(),
			// c'est la loi par défaut dans tout bon certificat d'étalonnage qui se respecte
			'N' => inner_epsilon = it.abs() / 2.0_f64, 
			_ => inner_epsilon = it.abs() / 2.0_f64, 
		}
		
		Self { valeur, epsilon: inner_epsilon, alpha: 95.45 }

    }

	fn K_alpha(alpha_loc: f64) -> f64 
	{
		// Calcul par interpolation du coeff d'élargissement à l'aide
		// des valeurs décrites dans la norme "NF ENV 13005"
		// Boîte à moustache :
		// - la boîte --> 50% --> K = 0.6745
		// - les moustaches --> 99.3% --> K = 2.698
		let p: [f64; 13] = [99.95 , 99.73 , 99.30, 99.00 , 98.76, 95.45 , 95.00 , 90.00 , 86.64, 68.27 , 50.000, 38.29, 0.000];
		let k: [f64; 13] = [3.500 , 3.000 , 2.698, 2.576 , 2.500, 2.000 , 1.960 , 1.645 , 1.500, 1.000 , 0.6745, 0.500, 0.000];

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

		return a * alpha_loc + b
	}
	
}

impl RMesure
{	
	pub fn Val(&self) -> f64	{ self.valeur 	}	// LA mesure en cours de traitement
	pub fn Alpha(&self) -> f64	{ self.alpha 	}	// Taux de confiance
	pub fn Eps(&self) -> f64 	{ self.epsilon	}	// Incertitude type.
	pub fn IT(&self) -> f64 	{ self.epsilon * self.K() }	// Intervalle de tolérance = Eps x K

	/// Retourne : (min , 1er quartile , médiane , 3e quartile , max)
	/// https://fr.wikipedia.org/wiki/Loi_normale#Loi_normale_centr%C3%A9e_r%C3%A9duite
	pub fn BoxPlot(&self) -> (f64,f64,f64,f64,f64) 	
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
	fn K(&self) -> f64 
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

// RMesure = RMesure + RMesure
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
			epsilon: (self.epsilon.powf(2.0_f64) + RMesure_rhs.epsilon.powf(2.0_f64)).sqrt(),
			alpha: self.alpha.max(RMesure_rhs.alpha)
		}
    }
}

// RMesure = constante_f64 + RMesure
// f64.add(RMesure)
impl ops::Add<RMesure> for f64 
{
    type Output = RMesure;
	/// U²(self + M) = U²(self) + U²(M)
    fn add(self: f64, RMesure_rhs: RMesure) -> RMesure 
	{ RMesure::scalaire(self) + RMesure_rhs }
}

// RMesure = RMesure + constante_f64
// RMesure.add(f64)
impl ops::Add<f64> for RMesure 
{
    type Output = RMesure;
	/// U²(self + M) = U²(self) + U²(M)
    fn add(self: RMesure, f64_rhs: f64) -> RMesure 
	{ self + RMesure::scalaire(f64_rhs) }
}

// RMesure += RMesure
impl ops::AddAssign<RMesure> for RMesure
{
	fn add_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = *self + RMesure_rhs }
}

// RMesure += constante_f64
impl ops::AddAssign<f64> for RMesure
{
	fn add_assign(&mut self, f64_rhs: f64)
	{ *self = *self + RMesure::scalaire(f64_rhs) }
}




// RMesure = - RMesure
// -------------------
impl ops::Neg for RMesure
{
	type Output = RMesure;
	fn neg(self) -> RMesure
	{ RMesure::new(-1.0_f64 * self.valeur, self.epsilon, self.alpha) }
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
			epsilon: (self.epsilon.powf(2.0_f64) + RMesure_rhs.epsilon.powf(2.0_f64)).sqrt(),
			alpha: self.alpha.max(RMesure_rhs.alpha)
		}
    }
}

// RMesure = constante_f64 - RMesure
// f64.sub(RMesure)
impl ops::Sub<RMesure> for f64 
{
    type Output = RMesure;
	/// U²(self - M) = U²(self) + U²(M)
    fn sub(self: f64, RMesure_rhs: RMesure) -> RMesure 
	{ RMesure::scalaire(self) - RMesure_rhs }
}

// RMesure = RMesure - constante_f64
// RMesure.sub(f64)
impl ops::Sub<f64> for RMesure 
{
    type Output = RMesure;
	/// U²(self - M) = U²(self) + U²(M)
    fn sub(self: RMesure, f64_rhs: f64) -> RMesure 
	{ self - RMesure::scalaire(f64_rhs) }
}

// RMesure -= RMesure
impl ops::SubAssign<RMesure> for RMesure
{
	fn sub_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = *self - RMesure_rhs }
}

// RMesure -= constante_f64
impl ops::SubAssign<f64> for RMesure
{
	fn sub_assign(&mut self, f64_rhs: f64)
	{ *self = *self - RMesure::scalaire(f64_rhs) }
}




// RMesure = RMesure * RMesure
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
			epsilon: (self.Val().powf(2.0_f64) * RMesure_rhs.epsilon.powf(2.0_f64)) + (self.epsilon.powf(2.0_f64) * RMesure_rhs.valeur.powf(2.0_f64)).sqrt(),
			alpha: self.alpha.max(RMesure_rhs.alpha)
		}
    }
}

// RMesure = constante_f64 * RMesure
// f64.mul(RMesure)
impl ops::Mul<RMesure> for f64 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²))
    fn mul(self: f64, RMesure_rhs: RMesure) -> RMesure 
	{ RMesure::scalaire(self) * RMesure_rhs }
}

// RMesure = RMesure * constante_f64
// RMesure.mul(f64)
impl ops::Mul<f64> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²))
    fn mul(self: RMesure, f64_rhs: f64) -> RMesure 
	{ self - RMesure::scalaire(f64_rhs) }
}

// RMesure *= RMesure
impl ops::MulAssign<RMesure> for RMesure
{
	fn mul_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = *self * RMesure_rhs }
}

// RMesure *= constante_f64
impl ops::MulAssign<f64> for RMesure
{
	fn mul_assign(&mut self, f64_rhs: f64)
	{ *self = *self * RMesure::scalaire(f64_rhs) }
}





// RMesure = RMesure / RMesure
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
		//if RMesure_rhs.valeur == 0.0_f64
		if RMesure_rhs == RMesure::scalaire(0.0_f64)
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
				epsilon: ((self.epsilon.powf(2.0_f64) * RMesure_rhs.valeur.powf(2.0_f64)) + (RMesure_rhs.epsilon.powf(2.0_f64) * self.Val().powf(2.0_f64))).sqrt() / RMesure_rhs.valeur.powf(2.0_f64),
				alpha: self.alpha.max(RMesure_rhs.alpha)
			}
		}
    }
}

// RMesure = constante_f64 / RMesure
// f64.div(RMesure)
impl ops::Div<RMesure> for f64 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
    fn div(self: f64, RMesure_rhs: RMesure) -> RMesure 
	{ RMesure::scalaire(self) / RMesure_rhs }
}

// RMesure = RMesure / constante_f64
// RMesure.div(f64)
impl ops::Div<f64> for RMesure 
{
    type Output = RMesure;
	/// U(R) = sqrt((U(this)² * M²) + (this² * U(M)²)) * (1 / M²) 
    fn div(self: RMesure, f64_rhs: f64) -> RMesure 
	{ self / RMesure::scalaire(f64_rhs) }
}

// RMesure /= RMesure
impl ops::DivAssign<RMesure> for RMesure
{
	fn div_assign(&mut self, RMesure_rhs: RMesure)
	{ *self = *self / RMesure_rhs }
}

// RMesure /= constante_f64
impl ops::DivAssign<f64> for RMesure
{
	fn div_assign(&mut self, f64_rhs: f64)
	{ *self = *self / RMesure::scalaire(f64_rhs) }
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

//     bool operator==(const CMesure& M) const;		PartialEq eq
//     bool operator!=(const CMesure& M) const;		PartialEq ne
//     bool operator<=(const CMesure& M) const;		PartialOrd partial_cmp
//     bool operator>=(const CMesure& M) const;		PartialOrd partial_cmp
//     bool operator< (const CMesure& M) const;		PartialOrd partial_cmp
//     bool operator> (const CMesure& M) const;		PartialOrd partial_cmp

impl cmp::PartialEq<RMesure> for RMesure 
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
	fn eq(&self, RMesure_rhs: &RMesure) -> bool
	{ 
		let D: RMesure = *self - *RMesure_rhs;
		D.valeur.abs() <= D.IT()		 
	}
}

impl cmp::PartialOrd<RMesure> for RMesure 
{
	fn partial_cmp(&self, RMesure_rhs: &RMesure) -> Option<Ordering>
	{
		let D: RMesure = *self - *RMesure_rhs;

		if D.valeur.abs() <= D.IT() { Some(Ordering::Equal)   }
		else if D.valeur < -D.IT()  { Some(Ordering::Less)    }
		else                        { Some(Ordering::Greater) }
	}
}