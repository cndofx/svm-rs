; Selection sort for SVM

; Read N
"Please enter the number of elements in the sequence\n"
print
in

dup ; Store N at address 0
0
stor 

; Input the sequence to be sorted
"Enter the sequence:\n"
print
:input_array
	;See if we must read the next value
	dup 0 @input_array_end je ; Jump to sort if all elements have been read 

	;Read the next value and place it in the memory
	dup ; Generate address for the new value 
	in  ; Read it
	swp ; Swap the value and its address so that we can use STOR 
	stor

	;Loop
	dec
	@input_array
	jmp
:input_array_end
pop ; Get rid of junk on stack

; Sort the given sequence with the selection sort algorithm
0 load ; Init outer loop
:sort ; Outer loop start
	dup 0 @sort_end je ; Check outer loop condition
	dup ; Inner loop init
	dup 0 load inc stor ; Store the number of min. element at address N+1
	:find_min ; Inner loop
		dup 0 @find_min_end je ; Check inner loop condition
		0 load inc load load   ; Load a[min]
		ovr load ; Load a[j]   ; Load a[j]
		@no_min jg             ; if a[min] < a[j] then min = j
		dup 0 load inc stor    ; Store the new minimum
		:no_min
		dec
		@find_min jmp ; Proceed to next iter of inloop
	:find_min_end
	pop ; Get rid of junk on stack
	; Now to swap the minimal and current elements (a[i] and a[min])
	; unless they have the same adress.
	dup 0 load inc load 
	@noswap je
		dup 0 load inc load load swp ; The stack will be <a[min] i>
		dup load 0 load inc load     ; The stack will be <a[min] i a[i] min>
		stor stor                    ; Voila! we've swapped the two values
	:noswap
	dec
	@sort jmp ; Proceed to next iter of outloop
:sort_end	

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
	
