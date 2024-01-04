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
		let resultat = U + V;
		assert_eq!(resultat , RMesure::new(24.0, 0.005 * 2.0_f64.sqrt(), 95.45)); 
    }

	#[test]
    fn test_mesure_add_f64() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let resultat = U + 2.0; 
		assert_eq!(resultat , RMesure::loi(14.0, 0.01, 'N')); 
    }

	#[test]
    fn test_f64_add_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let resultat = 2.0 + U; 
		assert_eq!(resultat , RMesure::loi(14.0, 0.01, 'N')); 
    }

    #[test]
    fn test_mesure_addassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U += V;
		assert_eq!(U , RMesure::new(24.0, 0.005 * 2.0_f64.sqrt(), 95.45)); 
    }

	#[test]
    fn test_mesure_addassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U += 2.0;  
		assert_eq!(U , RMesure::loi(14.0, 0.01, 'N')); 
    }

	//////////////////
	// Soustraction //
	//////////////////
	#[test]
    fn test_mesure_sub_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(10.0, 0.01, 'N');
		let resultat = U - V;
		assert_eq!(resultat , RMesure::new(2.0, 0.005 * 2.0_f64.sqrt(), 95.45)); 
    }

	#[test]
    fn test_mesure_sub_f64() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let resultat = U - 2.0; 
		assert_eq!(resultat , RMesure::loi(10.0, 0.01, 'N')); 
    }

	#[test]
    fn test_f64_sub_mesure() 
	{
		let U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let resultat = 2.0 - U; 
		assert_eq!(resultat , RMesure::loi(-10.0, 0.01, 'N')); 
    }

    #[test]
    fn test_mesure_subassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(10.0, 0.01, 'N');
		U -= V;
		assert_eq!(U , RMesure::new(2.0, 0.005 * 2.0_f64.sqrt(), 95.45)); 
    }

	#[test]
    fn test_mesure_subassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(12.0, 0.01, 'N');
		U -= 2.0; 
		assert_eq!(U , RMesure::loi(10.0, 0.01, 'N')); 
    }



	////////////////////
	// Multiplication //
	////////////////////

	#[test]
    fn test_mesure_mul_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		let resultat = U * V;
		assert_eq!(resultat , RMesure::new(6.0, 0.005 * 13.0_f64.sqrt(), 95.45)); 
    }

	#[test]
    fn test_mesure_mul_f64() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let resultat = U * 2.0; 
		assert_eq!(resultat , RMesure::loi(4.0, 0.01, 'N')); 
    }

	#[test]
    fn test_f64_mul_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let resultat = 2.0 * U; 
		assert_eq!(resultat , RMesure::loi(4.0, 0.01, 'N')); 
    }

    #[test]
    fn test_mesure_mulassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		U *= V;
		assert_eq!(U , RMesure::new(6.0, 0.005 * 13.0_f64.sqrt(), 95.45));  
    }

	#[test]
    fn test_mesure_mulassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		U *= 2.0;
		assert_eq!(U , RMesure::loi(4.0, 0.01, 'N')); 
    }

	//////////////
	// Division //
	//////////////
	#[test]
    fn test_mesure_div_mesure() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		let resultat = U.clone() / V.clone();
		assert_eq!(resultat , RMesure::loi(
			2.0_f64/3.0_f64, 
			((U.Eps().powf(2.0_f64) * V.Val().powf(2.0_f64)) + (V.Eps().powf(2.0_f64) * U.Val().powf(2.0_f64))).sqrt() / V.Val().powf(2.0_f64)*2.0_f64, 
			'N')); 
    }

	#[test]
    fn test_mesure_div_f64() 
	{
		let U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let resultat = U.clone() / 3.0_f64;
		assert_eq!(resultat , RMesure::loi(
			2.0_f64/3.0_f64, 
			(U.Eps().powf(2.0_f64) * 3.0_f64.powf(2.0_f64)).sqrt() / 3.0_f64.powf(2.0_f64)*2.0_f64, 
			'N')); 
    }

	#[test]
    fn test_f64_div_mesure() 
	{
		let U: RMesure = RMesure::loi(3.0, 0.01, 'N');
		let resultat = 2.0_f64 / U.clone();
		assert_eq!(resultat , RMesure::loi(
			2.0_f64/3.0_f64, 
			(U.Eps().powf(2.0_f64) * 3.0_f64.powf(2.0_f64)).sqrt() / 3.0_f64.powf(2.0_f64)*2.0_f64, 
			'N')); 
    }

    #[test]
    fn test_mesure_divassign_mesure() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		let V: RMesure = RMesure::loi(3.0, 0.01, 'N');
		U /= V.clone();
		assert_eq!(U , RMesure::loi(
			2.0_f64/3.0_f64, 
			((U.Eps().powf(2.0_f64) * V.Val().powf(2.0_f64)) + (V.Eps().powf(2.0_f64) * U.Val().powf(2.0_f64))).sqrt() / V.Val().powf(2.0_f64)*2.0_f64, 
			'N')); 
    }

	#[test]
    fn test_mesure_divassign_f64() 
	{
		let mut U: RMesure = RMesure::loi(2.0, 0.01, 'N');
		U /= 3.0_f64;
		assert_eq!(U , RMesure::loi(
			2.0_f64/3.0_f64, 
			(U.Eps().powf(2.0_f64) * 3.0_f64.powf(2.0_f64)).sqrt() / 3.0_f64.powf(2.0_f64)*2.0_f64, 
			'N')); 
    }

	#[test]
    fn test_mesure_div_00() 
	{
		let U: RMesure = RMesure::loi(-2.0, 0.01, 'N');
		let resultat = U.clone() / 0.0_f64;
		assert_eq!(resultat , RMesure::loi(U.Val().signum() * RMESURE_MAX, RMESURE_MAX ,'N')); 
    }



}