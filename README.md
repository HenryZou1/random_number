# random_number
main file generates two random numbers, first one 0-255, 0-231 \n
1st number created through getting a u8 integer between 0-1 oring it with answer which starts at 0 then left shifing the number by 1. \n
process is repeated 8 times to construct a number between 0-255.\n
2nd number is generated by gen_range(0, 231) xor with integer between 0-1 \n
