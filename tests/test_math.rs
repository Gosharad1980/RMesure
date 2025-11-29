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
   
}