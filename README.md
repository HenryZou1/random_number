# random_number
main file generates two random numbers, first one 0-255, 0-231.  
answer is set to zero to begin  
1st number created through getting a u8 integer between 0-1 oring it with answer then left shifting the number by 1.  
process is repeated 8 times to construct a number between 0-255.  
2nd number is generated by gen_range(0, 231) xor with an integer between 0-1  
