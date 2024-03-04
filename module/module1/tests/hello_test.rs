use module1::*;

/// Tests

#[ test ]
fn example_test()
{
	assert_eq!( "hello world!".to_string(), hello() );
	assert_eq!( "hello world!123".to_string(), hello() );
	
}
