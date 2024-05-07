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
    use ::RMesure::RMESURE_EPS;
    // Import the necessary modules
    use RMesure::RMesure;

    #[test]
    fn test_new() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 3.0);
		let resultat = (mesure.Val() == 1.0) && (mesure.Eps() == 2.0) && (mesure.Alpha() == 3.0);
		println!("valeur = {}", mesure.Val());
		println!("sigma = {}", mesure.Eps());
		println!("alpha = {}", mesure.Alpha());
		assert_eq!(resultat , true);    
    }
    #[test]
    fn test_zero() 
	{
		let mesure: RMesure = RMesure::zero();
		let resultat = (mesure.Val() == 0.0) && (mesure.Eps() == RMESURE_EPS) && (mesure.Alpha() == 95.45);
		assert_eq!(resultat , true);    
    }

	#[test]
    fn test_from() 
	{
		let mesure: RMesure = RMesure::from(1.618);
		let resultat = (mesure.Val() == 1.618) && (mesure.Eps() == RMESURE_EPS) && (mesure.Alpha() == 95.45);
		assert_eq!(resultat , true);    
    }

	#[test]
    fn test_scalaire() 
	{
		let mesure: RMesure = RMesure::scalaire(1.618);
		let resultat = (mesure.Val() == 1.618) && (mesure.Eps() == RMESURE_EPS) && (mesure.Alpha() == 95.45);
		assert_eq!(resultat , true);    
    }

	#[test]
    fn test_loi() 
	{
		print!("Loi R -> ");
		let U: RMesure = RMesure::loi(1.0, 0.01, 'R');
		let resultat = (U.Val() == 1.0) && (U.Eps() == (0.01_f64/12.0_f64.sqrt()) ) && (U.Alpha() == 95.45);
		assert_eq!(resultat , true); 
		println!("done");

		print!("Loi H -> ");
		let U: RMesure = RMesure::loi(2.0, 0.01, 'H');
		let resultat = (U.Val() == 2.0) && (U.Eps() == (0.01_f64/12.0_f64.sqrt()) ) && (U.Alpha() == 95.45);
		assert_eq!(resultat , true); 
		println!("done");
		
		print!("Loi S -> ");
		let U: RMesure = RMesure::loi(3.0, 0.01, 'S');
		let resultat = (U.Val() == 3.0) && (U.Eps() == 0.01_f64/2.0_f64.sqrt()) && (U.Alpha() == 95.45);
		assert_eq!(resultat , true); 
		println!("done");

		print!("Loi N -> ");
		let U: RMesure = RMesure::loi(4.0, 0.01, 'N');
		let resultat = (U.Val() == 4.0) && (U.IT() == 0.01) && (U.Alpha() == 95.45);
		assert_eq!(resultat , true);
		println!("done");

		print!("Loi C -> ");
		let U: RMesure = RMesure::loi(5.0, 0.01, 'C');
		let resultat = (U.Val() == 5.0) && (U.Eps() == (0.01_f64/3.0_f64.sqrt()) ) && (U.Alpha() == 95.45);
		assert_eq!(resultat , true); 
		println!("done");

		print!("Loi P -> ");
		let U: RMesure = RMesure::loi(5.0, -2.0, 'P');
		let resultat = (U.Val() == 5.0) && (U.Eps() == (5.0_f64 * 2.0_f64 / 100.0_f64 / 2.0_f64) ) && (U.Alpha() == 95.45);
		assert_eq!(resultat , true); 
		println!("done");

    }

	#[test]
    fn test_display() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 95.45); 
		println!("mesure = {mesure}");  
		// ouais pas d'assert ... parce que je ne sais pas trop comment le tester
    }

	#[test]
    fn test_K_alpha() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 95.45);
		let resultat = mesure.IT() == (2.0_f64 * mesure.Eps());
		assert_eq!(resultat , true);         
    }

	#[test]
    fn test_BoxPlot() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 95.45);
		println!("BoxPlot I = {:?}", mesure.BoxPlot());
		// let resultat = (-4.396000000000015,-0.3490000000000002,1.0,2.349,6.396000000000015) == mesure.BoxPlot();
		let resultat = (-4.395996, -0.34899986, 1.0, 2.349, 6.395996) == mesure.BoxPlot();
		assert_eq!(resultat , true);      
    }
	#[test]
    fn test_Val() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 3.0);
		let resultat = mesure.Val() == 1.0;
		assert_eq!(resultat , true);      
    }

	#[test]
    fn test_Eps() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 3.0);
		let resultat = mesure.Eps() == 2.0;
		assert_eq!(resultat , true);       
    }

	#[test]
    fn test_Alpha() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 3.0);
		let resultat = mesure.Alpha() == 3.0;
		assert_eq!(resultat , true);      
    }

	#[test]
    fn test_IT() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 95.45);
		let resultat = mesure.IT() == 4.0_f64; //2.0_f64 * 2.0_f64;
		assert_eq!(resultat , true);     
    }

	#[test]
    fn test_K() 
	{
		let mesure: RMesure = RMesure::new(1.0, 2.0 , 95.45);
		let resultat = mesure.IT() == (2.0_f64 * mesure.Eps());
		assert_eq!(resultat , true);     
    }

}