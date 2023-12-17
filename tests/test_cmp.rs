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
    // Import the necessary modules
    use RMesure::RMesure;

    #[test]
    fn test_eq_false() 
	{
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(a==d , false);    
    }

	#[test]
    fn test_eq_true() 
	{
		let b: RMesure = RMesure::loi(1.0, 0.1, 'N');
		let c: RMesure = RMesure::loi(1.1, 0.1, 'N');
		assert_eq!(b==c , true);      
    }

	#[test]
    fn test_ne_false() 
	{
		let b: RMesure = RMesure::loi(1.0, 0.1, 'N');
		let c: RMesure = RMesure::loi(1.1, 0.1, 'N');
		assert_eq!(b!=c , false);    
    }

	#[test]
    fn test_ne_true() 
	{
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(a!=d , true);      
    }

    #[test]
    fn test_lt_false() 
	{
		let b: RMesure = RMesure::loi(1.0, 0.1, 'N');
		let c: RMesure = RMesure::loi(1.1, 0.1, 'N');
		assert_eq!(c<b , false); 
   
    }

	#[test]
	fn test_lt_true() 
	{ 
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(a<d , true); 
    }

	#[test]
    fn test_gt_false() 
	{  
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(a>d , false); 
    }

	#[test]
	fn test_gt_true() 
	{ 
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(d>a , true); 
    }

    #[test]
    fn test_le_false() 
	{ 
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(d<=a , false); 
    }

	#[test]
	fn test_le_true() 
	{  
		let b: RMesure = RMesure::loi(1.0, 0.1, 'N');
		let c: RMesure = RMesure::loi(1.1, 0.1, 'N');
		assert_eq!(c<=b , true); 
    }

    #[test]
    fn test_ge_false() 
	{
		let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
		let d: RMesure = RMesure::loi(1.5, 0.1, 'N');
		assert_eq!(a>=d , false);
		
    }

	#[test]
	fn test_ge_true() 
	{
		let b: RMesure = RMesure::loi(1.0, 0.1, 'N');
		let c: RMesure = RMesure::loi(1.1, 0.1, 'N');
		assert_eq!(b>=c , true); 
    }


}