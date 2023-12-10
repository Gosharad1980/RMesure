// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo
//use core::f64::{EPSILON, MAX};
//use core::f32::{EPSILON, MAX};

use core::f64;

pub const RMESURE_EPS: f64 = f64::EPSILON;
pub const RMESURE_MAX: f64 = 9223371500000000000.0; //f32::MAX.sqrt()/2.0;

//#[derive(Eq)]
#[derive(Debug)]
pub struct RMesure
{
	valeur: f64,
	epsilon: f64,
    alpha: f64,
}

// impl Copy for RMesure {}

impl Clone for RMesure 
{
    fn clone(&self) -> Self
	{
		Self { valeur: self.valeur, epsilon: self.epsilon, alpha: self.alpha }
    }
}

impl Drop for RMesure
{
    fn drop(&mut self)
	{
        //println!("Toto est détruit")
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


	pub fn loi(valeur: f64, it: f64, loi: char) -> Self 
	{
		// Dans le cadre de mesures effectuées dans des conditions bien identifiées,
		// il est possible d'estimer l'incertitude type directement à partir de
		// l'intervalle de tolérance à l'aide des lois suivante
		//
		//		1) 'R' : Résolution d'un indicateur numérique       : epsilon = it / rac(12.0)
		//		2) 'H' : Hystérésis tel que it = MAXI - MINI        : epsilon = it / rac(12.0)
		//		3) 'S' : évolution Sinusoïdale sur it = MAXI - MINI : epsilon = it / 1.4
		//		4) 'N' : loi Normale par défaut, K = 2              : epsilon = it / 2.0
		//		5) 'C' : appareil de Classe +/- it                  : epsilon = it / rac(3.0)

		let mut inner_epsilon: f64 = 0.0;

		/*
		match loi
		{
			'R' => inner_epsilon = f64::abs(it) / f64::sqrt(12.0),
			'H' => inner_epsilon = f64::abs(it) / f64::sqrt(12.0),
			'S' => inner_epsilon = f64::abs(it) / 1.4,
			'C' => inner_epsilon = f64::abs(it) / f64::sqrt(3.0),
			// c'est la loi par défaut dans tout bon certificat d'étalonnage qui se respecte
			'N' => inner_epsilon = f64::abs(it) / 2.0, 
			_ => inner_epsilon = f64::abs(it) / 2.0, 
		}
		*/

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
	
}

impl RMesure
{	
	pub fn Val(&self) -> f64	{ self.valeur 	}	// LA mesure en cours de traitement
	pub fn Alpha(&self) -> f64	{ self.alpha 	}	// Taux de confiance
	pub fn Eps(&self) -> f64 	{ self.epsilon	}	// Incertitude type.
	pub fn IT(&self) -> f64 	{ self.epsilon * self.K() }	// Intervalle de tolérance = Eps x K

	// Coeff d'élargissement
	fn K(&self) -> f64 
	{
		// Calcul par interpolation du coeff d'élargissement à l'aide
		// des valeurs décrites dans la norme "NF ENV 13005"
		let p: [f64; 8] = [99.95 , 99.73 , 99.00 , 95.45 , 95.00 , 90.00 , 68.27 , 0.000];
		let k: [f64; 8] = [4.000 , 3.000 , 2.576 , 2.000 , 1.960 , 1.645 , 1.000 , 0.000];

		let mut a: f64 = 0.0;
		let mut b: f64 = 0.0;
		let mut i = 0;

		// Recherche du cadran dans lequel on se situe
		for j in 1..p.len()
		{
			if self.alpha >= p[j]
			{
				i = j;
				break;
			}
		}

		// Interpolation de la valeur du coefficient d'élargissement
		a = (k[i] - k[i-1]) / (p[i] - p[i-1]);
		b = k[i-1] - (a * p[i-1]);

		return a * self.alpha + b

	}

}