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
		let resultat = addition.Val() == 24.0_f32 && addition.Eps() == (0.005_f32.powf(2.0_f32) + 0.005_f32.powf(2.0_f32)).sqrt() && addition.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_add_f32() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');

		let addition = U + 2.0; 
		let resultat = addition.Val() == 14.0_f32 && addition.IT() == 0.01_f32 && addition.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_f32_add_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');

		let addition = 2.0 + U; 
		let resultat = addition.Val() == 14.0_f32 && addition.IT() == 0.01_f32 && addition.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

    #[test]
    fn test_mesure_addassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U += V;
		
		let resultat = U.Val() == 24.0_f32 && U.Eps() == (0.005_f32.powf(2.0_f32) + 0.005_f32.powf(2.0_f32)).sqrt() && U.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_addassign_f32() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U += 2.0;  

		let resultat = U.Val() == 14.0_f32 && U.IT() == 0.01_f32 && U.Alpha() == 95.45_f32;

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

		let resultat = soustraction.Val() == 2.0_f32 && soustraction.Eps() == (0.005_f32.powf(2.0_f32) + 0.005_f32.powf(2.0_f32)).sqrt() && soustraction.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 	
    }

	#[test]
    fn test_mesure_sub_f32() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let soustraction = U - 2.0; 

		let resultat = soustraction.Val() == 10.0_f32 && soustraction.IT() == 0.01_f32 && soustraction.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_f32_sub_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let soustraction = 2.0 - U; 

		let resultat = soustraction.Val() == -10.0_f32 && soustraction.IT() == 0.01_f32 && soustraction.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

    #[test]
    fn test_mesure_subassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(10.0, 0.01, 'N');
		U -= V;

		let resultat = U.Val() == 2.0_f32 && U.Eps() == (0.005_f32.powf(2.0_f32) + 0.005_f32.powf(2.0_f32)).sqrt() && U.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_subassign_f32() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U -= 2.0; 

		let resultat = U.Val() == 10.0_f32 && U.IT() == 0.01_f32 && U.Alpha() == 95.45_f32;

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
		
		let resultat = multiplication.Val() == 6.0_f32 && multiplication.Eps() == ((U.Val().powf(2.0_f32) * V.Eps().powf(2.0_f32)) + (U.Eps().powf(2.0_f32) * V.Val().powf(2.0_f32))).sqrt() && multiplication.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_mul_f32() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let multiplication = U.clone() * 2.0; 
		 
		let resultat = multiplication.Val() == 4.0_f32 && multiplication.IT() == 0.02_f32 && multiplication.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_f32_mul_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let multiplication = 2.0 * U.clone(); 
		 
		let resultat = multiplication.Val() == 4.0_f32 && multiplication.IT() == 0.02_f32 && multiplication.Alpha() == 95.45_f32;
		
		assert_eq!(resultat , true); 
    }

    #[test]
    fn test_mesure_mulassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');

		let multiplication_Eps = ((U.Val().powf(2.0_f32) * V.Eps().powf(2.0_f32)) + (U.Eps().powf(2.0_f32) * V.Val().powf(2.0_f32))).sqrt();

		U *= V;

		let resultat = U.Val() == 6.0_f32 && U.Eps() == multiplication_Eps && U.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }

	#[test]
    fn test_mesure_mulassign_f32() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		U *= 2.0;

		let resultat = U.Val() == 4.0_f32 && U.IT() == 0.02_f32 && U.Alpha() == 95.45_f32;
		
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

		let resultat_val = 2.0_f32/3.0_f32;
		let resultat_eps = ((U.Eps().powf(2.0_f32) * V.Val().powf(2.0_f32)) + (V.Eps().powf(2.0_f32) * U.Val().powf(2.0_f32))).sqrt() / V.Val().powf(2.0_f32);
		let resultat_alf = 95.45_f32;

		let resultat = division.Val() == resultat_val && division.Eps() == resultat_eps && division.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_mesure_div_f32() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let division = U.clone() / 3.0_f32;

		let resultat_val = 2.0_f32/3.0_f32;
		let resultat_eps = (U.Eps().powf(2.0_f32) * 3.0_f32.powf(2.0_f32)).sqrt() / 3.0_f32.powf(2.0_f32);
		let resultat_alf = 95.45_f32;

		let resultat = division.Val() == resultat_val && division.Eps() == resultat_eps && division.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_f32_div_mesure() 
	{
		let U: RMesure = RMesure::loi(3.0, 0.01, 'N');
		let division = 2.0_f32 / U.clone();

		let resultat_val = 2.0_f32/3.0_f32;
		let resultat_eps = (U.Eps().powf(2.0_f32) * 2.0_f32.powf(2.0_f32)).sqrt() / U.Val().powf(2.0_f32);
		let resultat_alf = 95.45_f32;

		let resultat = division.Val() == resultat_val && division.Eps() == resultat_eps && division.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

    #[test]
    fn test_mesure_divassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		
		let resultat_val = 2.0_f32/3.0_f32;
		let resultat_eps = ((U.Eps().powf(2.0_f32) * V.Val().powf(2.0_f32)) + (V.Eps().powf(2.0_f32) * U.Val().powf(2.0_f32))).sqrt() / V.Val().powf(2.0_f32);
		let resultat_alf = 95.45_f32;

		U /= V.clone();
		
		let resultat = U.Val() == resultat_val && U.Eps() == resultat_eps && U.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_mesure_divassign_f32() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');

		let resultat_val = 2.0_f32/3.0_f32;
		let resultat_eps = (U.Eps().powf(2.0_f32) * 3.0_f32.powf(2.0_f32)).sqrt() / 3.0_f32.powf(2.0_f32);
		let resultat_alf = 95.45_f32;

		U /= 3.0_f32;

		let resultat = U.Val() == resultat_val && U.Eps() == resultat_eps && U.Alpha() == resultat_alf;

		assert_eq!(resultat , true);
    }

	#[test]
    fn test_mesure_div_00() 
	{
		let U: RMesure = RMesure::loi(-2.0, 0.01, 'N');
		let division = U.clone() / 0.0_f32;

		let resultat = division.Val() == (U.Val().signum() * RMESURE_MAX) && division.Eps() == RMESURE_MAX && division.Alpha() == 95.45_f32;

		assert_eq!(resultat , true); 
    }
}