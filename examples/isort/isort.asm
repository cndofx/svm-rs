;Insertion sort

;Read N
"Please enter the number of elements in the sequence\n"
print
in
dup
0
stor ;N is stored at address 0

"Enter the sequence:\n"
print

;Input the sequence to be sorted
:input_array
	;See if we must read the next value
	dup
	0
	@sort
	je 

	;Read the next value and place it in the memory
	dup 
	in 
	swp
	stor

	;Loop
	dec
	@input_array
	jmp

;Sort the sequence using the insertion sort algorithm
:sort
	pop ;get rid of an unneeded 0

	0 load ;Init outer loop
	:outer_loop
		dup 0 @end je ;Check loop condition
		dup ;Init inner loop
		:inner_loop
			;Check loop conditions
			dup 0 load @end_inner_loop je ;j < N
			dup load ovr inc load @end_inner_loop jle ;a[j] > a[j+1]

			;Now swap a[j] with a[j+1]
			dup dup inc load swp
			dup inc ovr load swp
			stor stor

			;Proceed to the next iteration of the inner loop
			inc
			@inner_loop jmp
		:end_inner_loop
		pop ;Get rid of the inner loop counter
		dec ;Proceed to the next iteration of the outer loop
		@outer_loop jmp

;Print out the sorted sequence
:end
	;Print the result 
	0
	load
	:print
		dup
		0
		@exit
		je
		dup
		load	
		out
		dec
		@print
		jmp
:exit

