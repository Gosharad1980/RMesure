// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo


#[cfg(test)]
mod tests 
{
    // use ::RMesure::RMESURE_EPS;
    // Import the necessary modules
	use RMesure::RMesure;

	#[test]
	fn test_abs() 
	{
		let mesure:   RMesure = RMesure::new(-1.0, 2.0 , 95.45);
		let resultat: RMesure = RMesure::new( 1.0, 2.0 , 95.45);
		// let resultat = (-4.396000000000015,-0.3490000000000002,1.0,2.349,6.396000000000015) == mesure.BoxPlot();
		// let resultat = (-4.395996, -0.34899986, 1.0, 2.349, 6.395996) == mesure.BoxPlot();
		assert_eq!(resultat==mesure.abs() , true);     
	}

	#[test]
	fn test_recip() 
	{
		let mesure:		RMesure = RMesure::new(2.0, 0.1 , 95.45);
		let division:	RMesure = RMesure::new(2.0, 0.1 , 95.45).recip();

		let resultat_val = 1.0_f64 / 2.0_f64;
		let resultat_eps = (mesure.Eps().powf(2.0_f64) * 1.0_f64.powf(2.0_f64)).sqrt() / mesure.Val().powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = division.Val() == resultat_val && division.Eps() == resultat_eps && division.Alpha() == resultat_alf;
	}

	#[test]
	fn test_sin() 
	{
		let resultat:	RMesure = RMesure::new(1.570796_f64, 0.001_f64 , 95.45_f64).sin();

		let resultat_val = 1.570796_f64.sin();
		let resultat_eps = (1.570796_f64.cos().powf(2.0_f64) * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_cos() 
	{
		let resultat:	RMesure = RMesure::new(1.570796_f64, 0.001_f64 , 95.45_f64).cos();

		let resultat_val = 1.570796_f64.cos();
		let resultat_eps = (1.570796_f64.sin().powf(2.0_f64) * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_tan() 
	{
		let resultat:	RMesure = RMesure::new(1.570796_f64, 0.001_f64 , 95.45_f64).tan();

		let resultat_val = 1.570796_f64.tan();
		let resultat_eps = ((1.0_f64 + 1.570796_f64.tan().powf(2.0_f64)) * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_asin() 
	{
		let resultat:	RMesure = RMesure::new(2.0_f64.sqrt() / 2.0_f64, 0.001_f64 , 95.45_f64).asin();

		let resultat_val = (2.0_f64.sqrt() / 2.0_f64).asin();
		let resultat_eps = ((1.0_f64 - (2.0_f64.sqrt() / 2.0_f64).sqrt()).recip() * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_acos() 
	{
		let resultat:	RMesure = RMesure::new(2.0_f64.sqrt() / 2.0_f64, 0.001_f64 , 95.45_f64).acos();

		let resultat_val = (2.0_f64.sqrt() / 2.0_f64).acos();
		let resultat_eps = ((1.0_f64 - (2.0_f64.sqrt() / 2.0_f64).sqrt()).recip() * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_atan() 
	{
		let resultat:	RMesure = RMesure::new(2.0_f64.sqrt() / 2.0_f64, 0.001_f64 , 95.45_f64).atan();

		let resultat_val = (2.0_f64.sqrt() / 2.0_f64).atan();
		let resultat_eps = ((1.0_f64 - (2.0_f64.sqrt() / 2.0_f64)).recip() * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_sinh() 
	{
		let resultat:	RMesure = RMesure::new(1.570796_f64, 0.001_f64 , 95.45_f64).sinh();

		let resultat_val = 1.570796_f64.sinh();
		let resultat_eps = (1.570796_f64.cosh().powf(2.0_f64) * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}
	
	#[test]
	fn test_cosh() 
	{
		let resultat:	RMesure = RMesure::new(1.570796_f64, 0.001_f64 , 95.45_f64).cosh();

		let resultat_val = 1.570796_f64.cosh();
		let resultat_eps = (1.570796_f64.sinh().powf(2.0_f64) * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_tanh() 
	{
		let resultat:	RMesure = RMesure::new(1.570796_f64, 0.001_f64 , 95.45_f64).tanh();

		let resultat_val = 1.570796_f64.tanh();
		let resultat_eps = ((1.0_f64 + 1.570796_f64.tanh().powf(2.0_f64)).powf(2.0_f64) * 0.001_f64.powf(2.0_f64)).sqrt();
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_asinh() 
	{
		let resultat:	RMesure = RMesure::new(2.0_f64.sqrt() / 2.0_f64, 0.001_f64 , 95.45_f64).asinh();

		let resultat_val = (2.0_f64.sqrt() / 2.0_f64).asinh();
		let resultat_eps = ((2.0_f64.sqrt() / 2.0_f64).powf(2.0_f64) + 1.0_f64).recip() * 0.001_f64.powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_acosh() 
	{
		let resultat:	RMesure = RMesure::new(2.0_f64.sqrt() / 2.0_f64, 0.001_f64 , 95.45_f64).acosh();

		let resultat_val = (2.0_f64.sqrt() / 2.0_f64).acosh();
		let resultat_eps = ((2.0_f64.sqrt() / 2.0_f64).powf(2.0_f64) - 1.0_f64).recip() * 0.001_f64.powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

	#[test]
	fn test_atanh() 
	{
		let resultat:	RMesure = RMesure::new(2.0_f64.sqrt() / 2.0_f64, 0.001_f64 , 95.45_f64).atanh();

		let resultat_val = (2.0_f64.sqrt() / 2.0_f64).atanh();
		let resultat_eps = (1.0_f64 - (2.0_f64.sqrt() / 2.0_f64).powf(2.0_f64)).recip().powf(2.0_f64) * 0.001_f64.powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = resultat.Val() == resultat_val && resultat.Eps() == resultat_eps && resultat.Alpha() == resultat_alf;    
	}

}