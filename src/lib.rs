// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo


#[derive(Debug)]
pub struct RMesure
{
	pub valeur: f64,
	pub epsilon: f64,
    pub alpha: f64,
}

impl RMesure
{
	
	pub fn Val(&self) -> f64	{ self.valeur 	}	// LA mesure en cours de traitement
	pub fn Alpha(&self) -> f64	{ self.alpha 	}	// Taux de confiance
	pub fn Eps(&self) -> f64 	{ self.epsilon	}	// Incertitude type.
	pub fn IT(&self) -> f64 	{ self.epsilon * self.K() }	// Intervalle de tol�rance = Eps x K

	// Coeff d'élargissement
	fn K(&self) -> f64 
	{
		// Calcul par interpolation du coeff d'�largissement � l'aide
		// des valeurs d�crites dans la norme "NF ENV 13005"
		let p: [f64; 8] = [99.95 , 99.73 , 99.00 , 95.45 , 95.00 , 90.00 , 68.27 , 0.000];
		let k: [f64; 8] = [4.000 , 3.000 , 2.576 , 2.000 , 1.960 , 1.645 , 1.000 , 0.000];

		let mut a: f64 = 0.0;
		let mut b: f64 = 0.0;
		let mut i: usize = 0;

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