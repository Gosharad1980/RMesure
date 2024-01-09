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
    use ::RMesure::RMESURE_MAX;
    // Import the necessary modules
    use RMesure::RMesure;


	//////////////
	// Addition //
	//////////////
    #[test]
    fn test_mesure_add_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(12.0, 0.01, 'N');

		let addition = U + V;
		let resultat = addition.Val() == 24.0_f64 && addition.Uc() == (0.005_f64.powf(2.0_f64) + 0.005_f64.powf(2.0_f64)).sqrt() && addition.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_add_f64() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');

		let addition = U + 2.0; 
		let resultat = addition.Val() == 14.0_f64 && addition.IT() == 0.01_f64 && addition.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_f64_add_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');

		let addition = 2.0 + U; 
		let resultat = addition.Val() == 14.0_f64 && addition.IT() == 0.01_f64 && addition.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

    #[test]
    fn test_mesure_addassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U += V;
		
		let resultat = U.Val() == 24.0_f64 && U.Uc() == (0.005_f64.powf(2.0_f64) + 0.005_f64.powf(2.0_f64)).sqrt() && U.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_addassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U += 2.0;  

		let resultat = U.Val() == 14.0_f64 && U.IT() == 0.01_f64 && U.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	//////////////////
	// Soustraction //
	//////////////////
	#[test]
    fn test_mesure_sub_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(10.0, 0.01, 'N');
		let soustraction = U - V;

		let resultat = soustraction.Val() == 2.0_f64 && soustraction.Uc() == (0.005_f64.powf(2.0_f64) + 0.005_f64.powf(2.0_f64)).sqrt() && soustraction.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 	
    }

	#[test]
    fn test_mesure_sub_f64() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let soustraction = U - 2.0; 

		let resultat = soustraction.Val() == 10.0_f64 && soustraction.IT() == 0.01_f64 && soustraction.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_f64_sub_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let soustraction = 2.0 - U; 

		let resultat = soustraction.Val() == -10.0_f64 && soustraction.IT() == 0.01_f64 && soustraction.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

    #[test]
    fn test_mesure_subassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(10.0, 0.01, 'N');
		U -= V;

		let resultat = U.Val() == 2.0_f64 && U.Uc() == (0.005_f64.powf(2.0_f64) + 0.005_f64.powf(2.0_f64)).sqrt() && U.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_subassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U -= 2.0; 

		let resultat = U.Val() == 10.0_f64 && U.IT() == 0.01_f64 && U.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }



	////////////////////
	// Multiplication //
	////////////////////

	#[test]
    fn test_mesure_mul_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		let multiplication = U.clone() * V.clone();
		
		let resultat = multiplication.Val() == 6.0_f64 && multiplication.Uc() == ((U.Val().powf(2.0_f64) * V.Uc().powf(2.0_f64)) + (U.Uc().powf(2.0_f64) * V.Val().powf(2.0_f64))).sqrt() && multiplication.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_mul_f64() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let multiplication = U.clone() * 2.0; 
		 
		let resultat = multiplication.Val() == 4.0_f64 && multiplication.IT() == 0.02_f64 && multiplication.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_f64_mul_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let multiplication = 2.0 * U.clone(); 
		 
		let resultat = multiplication.Val() == 4.0_f64 && multiplication.IT() == 0.02_f64 && multiplication.Alpha() == 95.45_f64;
		
		assert_eq!(resultat , true); 
    }

    #[test]
    fn test_mesure_mulassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');

		let multiplication_Eps = ((U.Val().powf(2.0_f64) * V.Uc().powf(2.0_f64)) + (U.Uc().powf(2.0_f64) * V.Val().powf(2.0_f64))).sqrt();

		U *= V;

		let resultat = U.Val() == 6.0_f64 && U.Uc() == multiplication_Eps && U.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_mulassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		U *= 2.0;

		let resultat = U.Val() == 4.0_f64 && U.IT() == 0.02_f64 && U.Alpha() == 95.45_f64;
		
		assert_eq!(resultat , true); 
    }

	//////////////
	// Division //
	//////////////
	#[test]
    fn test_mesure_div_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');

		let division = U.clone() / V.clone();

		let resultat_val = 2.0_f64/3.0_f64;
		let resultat_eps = ((U.Uc().powf(2.0_f64) * V.Val().powf(2.0_f64)) + (V.Uc().powf(2.0_f64) * U.Val().powf(2.0_f64))).sqrt() / V.Val().powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = division.Val() == resultat_val && division.Uc() == resultat_eps && division.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_mesure_div_f64() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let division = U.clone() / 3.0_f64;

		let resultat_val = 2.0_f64/3.0_f64;
		let resultat_eps = (U.Uc().powf(2.0_f64) * 3.0_f64.powf(2.0_f64)).sqrt() / 3.0_f64.powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = division.Val() == resultat_val && division.Uc() == resultat_eps && division.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_f64_div_mesure() 
	{
		let U: RMesure = RMesure::loi(3.0, 0.01, 'N');
		let division = 2.0_f64 / U.clone();

		let resultat_val = 2.0_f64/3.0_f64;
		let resultat_eps = (U.Uc().powf(2.0_f64) * 2.0_f64.powf(2.0_f64)).sqrt() / U.Val().powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		let resultat = division.Val() == resultat_val && division.Uc() == resultat_eps && division.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

    #[test]
    fn test_mesure_divassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		
		let resultat_val = 2.0_f64/3.0_f64;
		let resultat_eps = ((U.Uc().powf(2.0_f64) * V.Val().powf(2.0_f64)) + (V.Uc().powf(2.0_f64) * U.Val().powf(2.0_f64))).sqrt() / V.Val().powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		U /= V.clone();
		
		let resultat = U.Val() == resultat_val && U.Uc() == resultat_eps && U.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_mesure_divassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');

		let resultat_val = 2.0_f64/3.0_f64;
		let resultat_eps = (U.Uc().powf(2.0_f64) * 3.0_f64.powf(2.0_f64)).sqrt() / 3.0_f64.powf(2.0_f64);
		let resultat_alf = 95.45_f64;

		U /= 3.0_f64;

		let resultat = U.Val() == resultat_val && U.Uc() == resultat_eps && U.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_mesure_div_00() 
	{
		let U: RMesure = RMesure::loi(-2.0, 0.01, 'N');
		let division = U.clone() / 0.0_f64;

		let resultat = division.Val() == (U.Val().signum() * RMESURE_MAX) && division.Uc() == RMESURE_MAX && division.Alpha() == 95.45_f64;

		assert_eq!(resultat , true); 
    }
}