
#[derive(Copy,Clone,Debug)]
pub enum IntelOp32 {
// AAA--ASCII Adjust After Addition.
	///
	/// 'aaa;' ASCII adjust AL after addition.
	AAA,
// AAD--ASCII Adjust AX Before Division.
	///
	/// 'aad;' ASCII adjust AX before division.
	///
	/// 'aad imm8;' Adjust AX before division to number base imm8.
	AAD,
// AAM--ASCII Adjust AX After Multiply.
	///
	/// 'aam;' ASCII adjust AX after multiply.
	///
	/// 'aam imm8;' Adjust AX after multiply to number base imm8.
	AAM,
// AAS--ASCII Adjust AL After Subtraction.
	///
	/// 'aas;' ASCII adjust AL after subtraction.
	AAS,
// ADC--Add with Carry.
	///
	/// 'adc AL,imm8;' Add with carry imm8 to AL.
	///
	/// 'adc AX,imm16;' Add with carry imm16 to AX.
	///
	/// 'adc EAX,imm32;' Add with carry imm32 to EAX.
	///
	/// 'adc RAX,imm32;' Add with carry imm32 sign extended to 64bits to RAX.
	///
	/// 'adc r/m8,imm8*;' Add with carry imm8 to r/m8.
	///
	/// 'adc r/m8,imm8;' Add with carry imm8 to r/m8.
	///
	/// 'adc r/m16,imm16;' Add with carry imm16 to r/m16.
	///
	/// 'adc r/m32,imm32;' Add with CF imm32 to r/m32.
	///
	/// 'adc r/m64,imm32;' Add with CF imm32 sign extended to 64-bits to r/m64.
	///
	/// 'adc r/m16,imm8;' Add with CF sign-extended imm8 to r/m16.
	///
	/// 'adc r/m32,imm8;' Add with CF sign-extended imm8 into r/m32.
	///
	/// 'adc r/m64,imm8;' Add with CF sign-extended imm8 into r/m64.
	///
	/// 'adc r/m8,r8**;' Add with carry byte register to r/m8.
	///
	/// 'adc r/m8,r8;' Add with carry byte register to r/m64.
	///
	/// 'adc r/m16,r16;' Add with carry r16 to r/m16.
	///
	/// 'adc r/m32,r32;' Add with CF r32 to r/m32.
	///
	/// 'adc r/m64,r64;' Add with CF r64 to r/m64.
	///
	/// 'adc r8,r/m8**;' Add with carry r/m8 to byte register.
	///
	/// 'adc r8,r/m8;' Add with carry r/m64 to byte register.
	///
	/// 'adc r16,r/m16;' Add with carry r/m16 to r16.
	///
	/// 'adc r32,r/m32;' Add with CF r/m32 to r32.
	///
	/// 'adc r64,r/m64;' Add with CF r/m64 to r64.
	ADC,
// ADCX--Unsigned Integer Addition of Two Operands with Carry Flag.
	///
	/// 'adcx r32,r/m32;' Unsigned addition of r32 with CF, r/m32 to r32, writes CF.
	///
	/// 'adcx r64,r/m64;' Unsigned addition of r64 with CF, r/m64 to r64, writes CF.
	ADCX,
// ADD--Add.
	///
	/// 'add AL,imm8;' Add imm8 to AL.
	///
	/// 'add AX,imm16;' Add imm16 to AX.
	///
	/// 'add EAX,imm32;' Add imm32 to EAX.
	///
	/// 'add RAX,imm32;' Add imm32 sign-extended to 64-bits to RAX.
	///
	/// 'add r/m8,imm8*;' Add imm8 to r/m8.
	///
	/// 'add r/m8,imm8;' Add sign-extended imm8 to r/m64.
	///
	/// 'add r/m16,imm16;' Add imm16 to r/m16.
	///
	/// 'add r/m32,imm32;' Add imm32 to r/m32.
	///
	/// 'add r/m64,imm32;' Add imm32 sign-extended to 64-bits to r/m64.
	///
	/// 'add r/m16,imm8;' Add sign-extended imm8 to r/m16.
	///
	/// 'add r/m32,imm8;' Add sign-extended imm8 to r/m32.
	///
	/// 'add r/m64,imm8;' Add sign-extended imm8 to r/m64.
	///
	/// 'add r/m8,r8**;' Add r8 to r/m8.
	///
	/// 'add r/m8,r8;' Add r8 to r/m8.
	///
	/// 'add r/m16,r16;' Add r16 to r/m16.
	///
	/// 'add r/m32,r32;' Add r32 to r/m32.
	///
	/// 'add r/m64,r64;' Add r64 to r/m64.
	///
	/// 'add r8,r/m8**;' Add r/m8 to r8.
	///
	/// 'add r8,r/m8;' Add r/m8 to r8.
	///
	/// 'add r16,r/m16;' Add r/m16 to r16.
	///
	/// 'add r32,r/m32;' Add r/m32 to r32.
	///
	/// 'add r64,r/m64;' Add r/m64 to r64.
	ADD,
// ADDPD--Add Packed Double-Precision Floating-Point Values.
	///
	/// 'vaddpd xmm1,xmm2,xmm3/m128;' Add packed double-precision floating-point values from xmm3/mem to xmm2 and stores result in xmm1.
	///
	/// 'vaddpd ymm1,ymm2,ymm3/m256;' Add packed double-precision floating-point values from ymm3/mem to ymm2 and stores result in ymm1.
	VADDPD,
	///
	/// 'addpd xmm1,xmm2/m128;' Add packed double-precision floating-point values from xmm2/m128 to xmm1.
	ADDPD,
// ADDPS--Add Packed Single-Precision Floating-Point Values.
	///
	/// 'vaddps xmm1,xmm2,xmm3/m128;' Add packed single-precision floating-point values from xmm3/mem to xmm2 and stores result in xmm1.
	///
	/// 'vaddps ymm1,ymm2,ymm3/m256;' Add packed single-precision floating-point values from ymm3/mem to ymm2 and stores result in ymm1.
	VADDPS,
	///
	/// 'addps xmm1,xmm2/m128;' Add packed single-precision floating-point values from xmm2/m128 to xmm1 and stores result in xmm1.
	ADDPS,
// ADDSD--Add Scalar Double-Precision Floating-Point Values.
	///
	/// 'vaddsd xmm1,xmm2,xmm3/m64;' Add the low double-precision floating-point value from xmm3/mem to xmm2 and store the result in xmm1.
	VADDSD,
	///
	/// 'addsd xmm1,xmm2/m64;' Add the low double-precision floating-point value from xmm2/m64 to xmm1.
	ADDSD,
// ADDSS--Add Scalar Single-Precision Floating-Point Values.
	///
	/// 'vaddss xmm1,xmm2,xmm3/m32;' Add the low single-precision floating-point value from xmm3/mem to xmm2 and store the result in xmm1.
	VADDSS,
	///
	/// 'addss xmm1,xmm2/m32;' Add the low single-precision floating-point value from xmm2/m32 to xmm1.
	ADDSS,
// ADDSUBPD--Packed Double-FP Add/Subtract.
	///
	/// 'addsubpd xmm1,xmm2/m128;' Add/subtract double-precision floating-point values from xmm2/m128 to xmm1.
	ADDSUBPD,
	///
	/// 'vaddsubpd xmm1,xmm2,xmm3/m128;' Add/subtract packed double-precision floating-point values from xmm3/mem to xmm2 and stores result in xmm1.
	///
	/// 'vaddsubpd ymm1,ymm2,ymm3/m256;' Add / subtract packed double-precision floating-point values from ymm3/mem to ymm2 and stores result in ymm1.
	VADDSUBPD,
// ADDSUBPS--Packed Single-FP Add/Subtract.
	///
	/// 'vaddsubps xmm1,xmm2,xmm3/m128;' Add/subtract single-precision floating-point values from xmm3/mem to xmm2 and stores result in xmm1.
	///
	/// 'vaddsubps ymm1,ymm2,ymm3/m256;' Add / subtract single-precision floating-point values from ymm3/mem to ymm2 and stores result in ymm1.
	VADDSUBPS,
	///
	/// 'addsubps xmm1,xmm2/m128;' Add/subtract single-precision floating-point values from xmm2/m128 to xmm1.
	ADDSUBPS,
// ADOX--Unsigned Integer Addition of Two Operands with Overflow Flag.
	///
	/// 'adox r32,r/m32;' Unsigned addition of r32 with OF, r/m32 to r32, writes OF.
	///
	/// 'adox r64,r/m64;' Unsigned addition of r64 with OF, r/m64 to r64, writes OF.
	ADOX,
// AESDEC--Perform One Round of an AES Decryption Flow.
	///
	/// 'vaesdec xmm1,xmm2,xmm3/m128;' Perform one round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from xmm2 with a 128-bit round key from xmm3/m128; store the result in xmm1.
	VAESDEC,
	///
	/// 'aesdec xmm1,xmm2/m128;' Perform one round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from xmm1 with a 128-bit round key from xmm2/m128.
	AESDEC,
// AESDECLAST--Perform Last Round of an AES Decryption Flow.
	///
	/// 'aesdeclast xmm1,xmm2/m128;' Perform the last round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from xmm1 with a 128-bit round key from xmm2/m128.
	AESDECLAST,
	///
	/// 'vaesdeclast xmm1,xmm2,xmm3/m128;' Perform the last round of an AES decryption flow, using the Equivalent Inverse Cipher, operating on a 128-bit data (state) from xmm2 with a 128-bit round key from xmm3/m128; store the result in xmm1.
	VAESDECLAST,
// AESENC--Perform One Round of an AES Encryption Flow.
	///
	/// 'aesenc xmm1,xmm2/m128;' Perform one round of an AES encryption flow, operating on a 128-bit data (state) from xmm1 with a 128-bit round key from xmm2/m128.
	AESENC,
	///
	/// 'vaesenc xmm1,xmm2,xmm3/m128;' Perform one round of an AES encryption flow, operating on a 128-bit data (state) from xmm2 with a 128-bit round key from the xmm3/m128; store the result in xmm1.
	VAESENC,
// AESENCLAST--Perform Last Round of an AES Encryption Flow.
	///
	/// 'vaesenclast xmm1,xmm2,xmm3/m128;' Perform the last round of an AES encryption flow, operating on a 128-bit data (state) from xmm2 with a 128 bit round key from xmm3/m128; store the result in xmm1.
	VAESENCLAST,
	///
	/// 'aesenclast xmm1,xmm2/m128;' Perform the last round of an AES encryption flow, operating on a 128-bit data (state) from xmm1 with a 128-bit round key from xmm2/m128.
	AESENCLAST,
// AESIMC--Perform the AES InvMixColumn Transformation.
	///
	/// 'aesimc xmm1,xmm2/m128;' Perform the InvMixColumn transformation on a 128-bit round key from xmm2/m128 and store the result in xmm1.
	AESIMC,
	///
	/// 'vaesimc xmm1,xmm2/m128;' Perform the InvMixColumn transformation on a 128-bit round key from xmm2/m128 and store the result in xmm1.
	VAESIMC,
// AESKEYGENASSIST--AES Round Key Generation Assist.
	///
	/// 'aeskeygenassist xmm1,xmm2/m128,imm8;' Assist in AES round key generation using an 8 bits Round Constant (RCON) specified in the immediate byte, operating on 128 bits of data specified in xmm2/m128 and stores the result in xmm1.
	AESKEYGENASSIST,
	///
	/// 'vaeskeygenassist xmm1,xmm2/m128,imm8;' Assist in AES round key generation using 8 bits Round Constant (RCON) specified in the immediate byte, operating on 128 bits of data specified in xmm2/m128 and stores the result in xmm1.
	VAESKEYGENASSIST,
// AND--Logical AND.
	///
	/// 'and AL,imm8;' AL AND imm8.
	///
	/// 'and AX,imm16;' AX AND imm16.
	///
	/// 'and EAX,imm32;' EAX AND imm32.
	///
	/// 'and RAX,imm32;' RAX AND imm32 sign-extended to 64-bits.
	///
	/// 'and r/m8,imm8*;' r/m8 AND imm8.
	///
	/// 'and r/m8,imm8;' r/m8 AND imm8.
	///
	/// 'and r/m16,imm16;' r/m16 AND imm16.
	///
	/// 'and r/m32,imm32;' r/m32 AND imm32.
	///
	/// 'and r/m64,imm32;' r/m64 AND imm32 sign extended to 64-bits.
	///
	/// 'and r/m16,imm8;' r/m16 AND imm8 (sign-extended).
	///
	/// 'and r/m32,imm8;' r/m32 AND imm8 (sign-extended).
	///
	/// 'and r/m64,imm8;' r/m64 AND imm8 (sign-extended).
	///
	/// 'and r/m8,r8**;' r/m8 AND r8.
	///
	/// 'and r/m8,r8;' r/m64 AND r8 (sign-extended).
	///
	/// 'and r/m16,r16;' r/m16 AND r16.
	///
	/// 'and r/m32,r32;' r/m32 AND r32.
	///
	/// 'and r/m64,r64;' r/m64 AND r32.
	///
	/// 'and r8,r/m8**;' r8 AND r/m8.
	///
	/// 'and r8,r/m8;' r/m64 AND r8 (sign-extended).
	///
	/// 'and r16,r/m16;' r16 AND r/m16.
	///
	/// 'and r32,r/m32;' r32 AND r/m32.
	///
	/// 'and r64,r/m64;' r64 AND r/m64.
	AND,
// ANDN--Logical AND NOT.
	///
	/// 'andn r32a,r32b,r/m32;' Bitwise AND of inverted r32b with r/m32, store result in r32a.
	///
	/// 'andn r64a,r64b,r/m64;' Bitwise AND of inverted r64b with r/m64, store result in r64a.
	ANDN,
// ANDPD--Bitwise Logical AND of Packed Double-Precision Floating-Point Values.
	///
	/// 'vandpd xmm1,xmm2,xmm3/m128;' Return the bitwise logical AND of packed double-precision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vandpd ymm1,ymm2,ymm3/m256;' Return the bitwise logical AND of packed double-precision floating-point values in ymm2 and ymm3/mem.
	VANDPD,
	///
	/// 'andpd xmm1,xmm2/m128;' Return the bitwise logical AND of packed double-precision floating-point values in xmm1 and xmm2/m128.
	ANDPD,
// ANDPS--Bitwise Logical AND of Packed Single-Precision Floating-Point Values.
	///
	/// 'vandps xmm1,xmm2,xmm3/m128;' Return the bitwise logical AND of packed single-precision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vandps ymm1,ymm2,ymm3/m256;' Return the bitwise logical AND of packed single-precision floating-point values in ymm2 and ymm3/mem.
	VANDPS,
	///
	/// 'andps xmm1,xmm2/m128;' Bitwise logical AND of xmm2/m128 and xmm1.
	ANDPS,
// ANDNPD--Bitwise Logical AND NOT of Packed Double-Precision Floating-Point Values.
	///
	/// 'andnpd xmm1,xmm2/m128;' Bitwise logical AND NOT of xmm2/m128 and xmm1.
	ANDNPD,
	///
	/// 'vandnpd xmm1,xmm2,xmm3/m128;' Return the bitwise logical AND NOT of packed double-precision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vandnpd ymm1,ymm2,ymm3/m256;' Return the bitwise logical AND NOT of packed double-precision floating-point values in ymm2 and ymm3/mem.
	VANDNPD,
// ANDNPS--Bitwise Logical AND NOT of Packed Single-Precision Floating-Point Values.
	///
	/// 'vandnps xmm1,xmm2,xmm3/m128;' Return the bitwise logical AND NOT of packed single-precision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vandnps ymm1,ymm2,ymm3/m256;' Return the bitwise logical AND NOT of packed single-precision floating-point values in ymm2 and ymm3/mem.
	VANDNPS,
	///
	/// 'andnps xmm1,xmm2/m128;' Bitwise logical AND NOT of xmm2/m128 and xmm1.
	ANDNPS,
// ARPL--Adjust RPL Field of Segment Selector.
	///
	/// 'arpl r/m16,r16;' Adjust RPL of r/m16 to not less than RPL of r16.
	ARPL,
// BLENDPD--Blend Packed Double Precision Floating-Point Values.
	///
	/// 'vblendpd xmm1,xmm2,xmm3/m128,imm8;' Select packed double-precision floating-point Values from xmm2 and xmm3/m128 from mask in imm8 and store the values in xmm1.
	///
	/// 'vblendpd ymm1,ymm2,ymm3/m256,imm8;' Select packed double-precision floating-point Values from ymm2 and ymm3/m256 from mask in imm8 and store the values in ymm1.
	VBLENDPD,
	///
	/// 'blendpd xmm1,xmm2/m128,imm8;' Select packed DP-FP values from xmm1 and xmm2/m128 from mask specified in imm8 and store the values into xmm1.
	BLENDPD,
// BEXTR--Bit Field Extract.
	///
	/// 'bextr r32a,r/m32,r32b;' Contiguous bitwise extract from r/m32 using r32b as control; store result in r32a.
	///
	/// 'bextr r64a,r/m64,r64b;' Contiguous bitwise extract from r/m64 using r64b as control; store result in r64a.
	BEXTR,
// BLENDPS--Blend Packed Single Precision Floating-Point Values.
	///
	/// 'blendps xmm1,xmm2/m128,imm8;' Select packed single precision floating-point values from xmm1 and xmm2/m128 from mask specified in imm8 and store the values into xmm1.
	BLENDPS,
	///
	/// 'vblendps xmm1,xmm2,xmm3/m128,imm8;' Select packed single-precision floating-point values from xmm2 and xmm3/m128 from mask in imm8 and store the values in xmm1.
	///
	/// 'vblendps ymm1,ymm2,ymm3/m256,imm8;' Select packed single-precision floating-point values from ymm2 and ymm3/m256 from mask in imm8 and store the values in ymm1.
	VBLENDPS,
// BLENDVPD--Variable Blend Packed Double Precision Floating-Point Values.
	///
	/// 'vblendvpd xmm1,xmm2,xmm3/m128,xmm4;' Conditionally copy double-precision floatingpoint values from xmm2 or xmm3/m128 to xmm1, based on mask bits in the mask operand, xmm4.
	///
	/// 'vblendvpd ymm1,ymm2,ymm3/m256,ymm4;' Conditionally copy double-precision floatingpoint values from ymm2 or ymm3/m256 to ymm1, based on mask bits in the mask operand, ymm4.
	VBLENDVPD,
	///
	/// 'blendvpd xmm1,xmm2/m128,<XMM0>;' Select packed DP FP values from xmm1 and xmm2 from mask specified in XMM0 and store the values in xmm1.
	BLENDVPD,
// BLENDVPS--Variable Blend Packed Single Precision Floating-Point Values.
	///
	/// 'vblendvps xmm1,xmm2,xmm3/m128,xmm4;' Conditionally copy single-precision floatingpoint values from xmm2 or xmm3/m128 to xmm1, based on mask bits in the specified mask operand, xmm4.
	///
	/// 'vblendvps ymm1,ymm2,ymm3/m256,ymm4;' Conditionally copy single-precision floatingpoint values from ymm2 or ymm3/m256 to ymm1, based on mask bits in the specified mask register, ymm4.
	VBLENDVPS,
	///
	/// 'blendvps xmm1,xmm2/m128,<XMM0>;' Select packed single precision floating-point values from xmm1 and xmm2/m128 from mask specified in XMM0 and store the values into xmm1.
	BLENDVPS,
// BLSI--Extract Lowest Set Isolated Bit.
	///
	/// 'blsi r32,r/m32;' Extract lowest set bit from r/m32 and set that bit in r32.
	///
	/// 'blsi r64,r/m64;' Extract lowest set bit from r/m64, and set that bit in r64.
	BLSI,
// BLSMSK--Get Mask Up to Lowest Set Bit.
	///
	/// 'blsmsk r32,r/m32;' Set all lower bits in r32 to '1' starting from bit 0 to lowest set bit in r/m32.
	///
	/// 'blsmsk r64,r/m64;' Set all lower bits in r64 to '1' starting from bit 0 to lowest set bit in r/m64.
	BLSMSK,
// BLSR--Reset Lowest Set Bit.
	///
	/// 'blsr r32,r/m32;' Reset lowest set bit of r/m32, keep all other bits of r/m32 and write result to r32.
	///
	/// 'blsr r64,r/m64;' Reset lowest set bit of r/m64, keep all other bits of r/m64 and write result to r64.
	BLSR,
// BNDCL--Check Lower Bound.
	///
	/// 'bndcl bnd,r/m32;' Generate a #BR if the address in r/m32 is lower than the lower bound in bnd.LB.
	///
	/// 'bndcl bnd,r/m64;' Generate a #BR if the address in r/m64 is lower than the lower bound in bnd.LB.
	BNDCL,
// BNDCU/BNDCN--Check Upper Bound.
	///
	/// 'bndcu bnd,r/m32;' Generate a #BR if the address in r/m32 is higher than the upper bound in bnd.UB (bnb.UB in 1's complement form).
	///
	/// 'bndcu bnd,r/m64;' Generate a #BR if the address in r/m64 is higher than the upper bound in bnd.UB (bnb.UB in 1's complement form).
	BNDCU,
	///
	/// 'bndcn bnd,r/m32;' Generate a #BR if the address in r/m32 is higher than the upper bound in bnd.UB (bnb.UB not in 1's complement form).
	///
	/// 'bndcn bnd,r/m64;' Generate a #BR if the address in r/m64 is higher than the upper bound in bnd.UB (bnb.UB not in 1's complement form).
	BNDCN,
// BNDLDX--Load Extended Bounds Using Address Translation.
	///
	/// 'bndldx bnd,mib;' Load the bounds stored in a bound table entry (BTE) into bnd with address translation using the base of mib and conditional on the index of mib matching the pointer value in the BTE.
	BNDLDX,
// BNDMK--Make Bounds.
	///
	/// 'bndmk bnd,m32;' Make lower and upper bounds from m32 and store them in bnd.
	///
	/// 'bndmk bnd,m64;' Make lower and upper bounds from m64 and store them in bnd.
	BNDMK,
// BNDMOV--Move Bounds.
	///
	/// 'bndmov bnd1,bnd2/m64;' Move lower and upper bound from bnd2/m64 to bound register bnd1.
	///
	/// 'bndmov bnd1,bnd2/m128;' Move lower and upper bound from bnd2/m128 to bound register bnd1.
	///
	/// 'bndmov bnd1/m64,bnd2;' Move lower and upper bound from bnd2 to bnd1/m64.
	///
	/// 'bndmov bnd1/m128,bnd2;' Move lower and upper bound from bnd2 to bound register bnd1/m128.
	BNDMOV,
// BNDSTX--Store Extended Bounds Using Address Translation.
	///
	/// 'bndstx mib,bnd;' Store the bounds in bnd and the pointer value in the index register of mib to a bound table entry (BTE) with address translation using the base of mib.
	BNDSTX,
// BOUND--Check Array Index Against Bounds.
	///
	/// 'bound r16,m16&16;' Check if r16 (array index) is within bounds specified by m16&16.
	///
	/// 'bound r32,m32&32;' Check if r32 (array index) is within bounds specified by m32&32.
	BOUND,
// BSF--Bit Scan Forward.
	///
	/// 'bsf r16,r/m16;' Bit scan forward on r/m16.
	///
	/// 'bsf r32,r/m32;' Bit scan forward on r/m32.
	///
	/// 'bsf r64,r/m64;' Bit scan forward on r/m64.
	BSF,
// BSR--Bit Scan Reverse.
	///
	/// 'bsr r16,r/m16;' Bit scan reverse on r/m16.
	///
	/// 'bsr r32,r/m32;' Bit scan reverse on r/m32.
	///
	/// 'bsr r64,r/m64;' Bit scan reverse on r/m64.
	BSR,
// BSWAP--Byte Swap.
	///
	/// 'bswap r32;' Reverses the byte order of a 32-bit register.
	///
	/// 'bswap r64;' Reverses the byte order of a 64-bit register.
	BSWAP,
// BT--Bit Test.
	///
	/// 'bt r/m16,r16;' Store selected bit in CF flag.
	///
	/// 'bt r/m32,r32;' Store selected bit in CF flag.
	///
	/// 'bt r/m64,r64;' Store selected bit in CF flag.
	///
	/// 'bt r/m16,imm8;' Store selected bit in CF flag.
	///
	/// 'bt r/m32,imm8;' Store selected bit in CF flag.
	///
	/// 'bt r/m64,imm8;' Store selected bit in CF flag.
	BT,
// BTC--Bit Test and Complement.
	///
	/// 'btc r/m16,r16;' Store selected bit in CF flag and complement.
	///
	/// 'btc r/m32,r32;' Store selected bit in CF flag and complement.
	///
	/// 'btc r/m64,r64;' Store selected bit in CF flag and complement.
	///
	/// 'btc r/m16,imm8;' Store selected bit in CF flag and complement.
	///
	/// 'btc r/m32,imm8;' Store selected bit in CF flag and complement.
	///
	/// 'btc r/m64,imm8;' Store selected bit in CF flag and complement.
	BTC,
// BTR--Bit Test and Reset.
	///
	/// 'btr r/m16,r16;' Store selected bit in CF flag and clear.
	///
	/// 'btr r/m32,r32;' Store selected bit in CF flag and clear.
	///
	/// 'btr r/m64,r64;' Store selected bit in CF flag and clear.
	///
	/// 'btr r/m16,imm8;' Store selected bit in CF flag and clear.
	///
	/// 'btr r/m32,imm8;' Store selected bit in CF flag and clear.
	///
	/// 'btr r/m64,imm8;' Store selected bit in CF flag and clear.
	BTR,
// BTS--Bit Test and Set.
	///
	/// 'bts r/m16,r16;' Store selected bit in CF flag and set.
	///
	/// 'bts r/m32,r32;' Store selected bit in CF flag and set.
	///
	/// 'bts r/m64,r64;' Store selected bit in CF flag and set.
	///
	/// 'bts r/m16,imm8;' Store selected bit in CF flag and set.
	///
	/// 'bts r/m32,imm8;' Store selected bit in CF flag and set.
	///
	/// 'bts r/m64,imm8;' Store selected bit in CF flag and set.
	BTS,
// BZHI--Zero High Bits Starting with Specified Bit Position.
	///
	/// 'bzhi r32a,r/m32,r32b;' Zero bits in r/m32 starting with the position in r32b, write result to r32a.
	///
	/// 'bzhi r64a,r/m64,r64b;' Zero bits in r/m64 starting with the position in r64b, write result to r64a.
	BZHI,
// CALL--Call Procedure.
	///
	/// 'call rel16;' Call near, relative, displacement relative to next instruction.
	///
	/// 'call rel32;' Call near, relative, displacement relative to next instruction. 32-bit displacement sign extended to 64-bits in 64-bit mode.
	///
	/// 'call r/m16;' Call near, absolute indirect, address given in r/m16.
	///
	/// 'call r/m32;' Call near, absolute indirect, address given in r/m32.
	///
	/// 'call r/m64;' Call near, absolute indirect, address given in r/m64.
	///
	/// 'call ptr16:16;' Call far, absolute, address given in operand.
	///
	/// 'call ptr16:32;' Call far, absolute, address given in operand.
	///
	/// 'call m16:16;' Call far, absolute indirect address given in m16:16. In 32-bit mode: if selector points to a gate, then RIP = 32-bit zero extended displacement taken from gate; else RIP = zero extended 16instruction.
	///
	/// 'call m16:32;' In 64-bit mode: If selector points to a gate, then RIP = 64-bit displacement taken from gate; else RIP = zero extended 32-bit offset from far pointer referenced in the instruction.
	///
	/// 'call m16:64;' In 64-bit mode: If selector points to a gate, then RIP = 64-bit displacement taken from gate; else RIP = 64-bit offset from far pointer referenced in the instruction.
	CALL,
// CBW/CWDE/CDQE--Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword to Quadword.
	///
	/// 'cwde;' EAX <-- sign-extend of AX.
	CWDE,
	///
	/// 'cbw;' AX <-- sign-extend of AL.
	CBW,
	///
	/// 'cdqe;' RAX <-- sign-extend of EAX.
	CDQE,
// CLAC--Clear AC Flag in EFLAGS Register.
	///
	/// 'clac;' Clear the AC flag in the EFLAGS register.
	CLAC,
// CLC--Clear Carry Flag.
	///
	/// 'clc;' Clear CF flag.
	CLC,
// CLD--Clear Direction Flag.
	///
	/// 'cld;' Clear DF flag.
	CLD,
// CLFLUSH--Flush Cache Line.
	///
	/// 'clflush m8;' Flushes cache line containing m8.
	CLFLUSH,
// CLI--Clear Interrupt Flag.
	///
	/// 'cli;' Clear interrupt flag; interrupts disabled when interrupt flag cleared.
	CLI,
// CLTS--Clear Task-Switched Flag in CR0.
	///
	/// 'clts;' Clears TS flag in CR0.
	CLTS,
// CMC--Complement Carry Flag.
	///
	/// 'cmc;' Complement CF flag.
	CMC,
// CMOVcc--Conditional Move.
	///
	/// 'cmovc r16,r/m16;' Move if carry (CF=1).
	///
	/// 'cmovc r32,r/m32;' Move if carry (CF=1).
	///
	/// 'cmovc r64,r/m64;' Move if carry (CF=1).
	CMOVC,
	///
	/// 'cmovz r16,r/m16;' Move if zero (ZF=1).
	///
	/// 'cmovz r32,r/m32;' Move if zero (ZF=1).
	///
	/// 'cmovz r64,r/m64;' Move if zero (ZF=1).
	CMOVZ,
	///
	/// 'cmovnbe r16,r/m16;' Move if not below or equal (CF=0 and ZF=0).
	///
	/// 'cmovnbe r32,r/m32;' Move if not below or equal (CF=0 and ZF=0).
	///
	/// 'cmovnbe r64,r/m64;' Move if not below or equal (CF=0 and ZF=0).
	CMOVNBE,
	///
	/// 'cmovl r16,r/m16;' Move if less (SF != OF).
	///
	/// 'cmovl r32,r/m32;' Move if less (SF != OF).
	///
	/// 'cmovl r64,r/m64;' Move if less (SF != OF).
	CMOVL,
	///
	/// 'cmovne r16,r/m16;' Move if not equal (ZF=0).
	///
	/// 'cmovne r32,r/m32;' Move if not equal (ZF=0).
	///
	/// 'cmovne r64,r/m64;' Move if not equal (ZF=0).
	CMOVNE,
	///
	/// 'cmovnle r16,r/m16;' Move if not less or equal (ZF=0 and SF=OF).
	///
	/// 'cmovnle r32,r/m32;' Move if not less or equal (ZF=0 and SF=OF).
	///
	/// 'cmovnle r64,r/m64;' Move if not less or equal (ZF=0 and SF=OF).
	CMOVNLE,
	///
	/// 'cmovpo r16,r/m16;' Move if parity odd (PF=0).
	///
	/// 'cmovpo r32,r/m32;' Move if parity odd (PF=0).
	///
	/// 'cmovpo r64,r/m64;' Move if parity odd (PF=0).
	CMOVPO,
	///
	/// 'cmovle r16,r/m16;' Move if less or equal (ZF=1 or SF != OF).
	///
	/// 'cmovle r32,r/m32;' Move if less or equal (ZF=1 or SF != OF).
	///
	/// 'cmovle r64,r/m64;' Move if less or equal (ZF=1 or SF != OF).
	CMOVLE,
	///
	/// 'cmovo r16,r/m16;' Move if overflow (OF=1).
	///
	/// 'cmovo r32,r/m32;' Move if overflow (OF=1).
	///
	/// 'cmovo r64,r/m64;' Move if overflow (OF=1).
	CMOVO,
	///
	/// 'cmovnc r16,r/m16;' Move if not carry (CF=0).
	///
	/// 'cmovnc r32,r/m32;' Move if not carry (CF=0).
	///
	/// 'cmovnc r64,r/m64;' Move if not carry (CF=0).
	CMOVNC,
	///
	/// 'cmovae r16,r/m16;' Move if above or equal (CF=0).
	///
	/// 'cmovae r32,r/m32;' Move if above or equal (CF=0).
	///
	/// 'cmovae r64,r/m64;' Move if above or equal (CF=0).
	CMOVAE,
	///
	/// 'cmovns r16,r/m16;' Move if not sign (SF=0).
	///
	/// 'cmovns r32,r/m32;' Move if not sign (SF=0).
	///
	/// 'cmovns r64,r/m64;' Move if not sign (SF=0).
	CMOVNS,
	///
	/// 'cmovpe r16,r/m16;' Move if parity even (PF=1).
	///
	/// 'cmovpe r32,r/m32;' Move if parity even (PF=1).
	///
	/// 'cmovpe r64,r/m64;' Move if parity even (PF=1).
	CMOVPE,
	///
	/// 'cmove r16,r/m16;' Move if equal (ZF=1).
	///
	/// 'cmove r32,r/m32;' Move if equal (ZF=1).
	///
	/// 'cmove r64,r/m64;' Move if equal (ZF=1).
	CMOVE,
	///
	/// 'cmovnge r16,r/m16;' Move if not greater or equal (SF != OF).
	///
	/// 'cmovnge r32,r/m32;' Move if not greater or equal (SF != OF).
	///
	/// 'cmovnge r64,r/m64;' Move if not greater or equal (SF != OF).
	CMOVNGE,
	///
	/// 'cmovge r16,r/m16;' Move if greater or equal (SF=OF).
	///
	/// 'cmovge r32,r/m32;' Move if greater or equal (SF=OF).
	///
	/// 'cmovge r64,r/m64;' Move if greater or equal (SF=OF).
	CMOVGE,
	///
	/// 'cmovnae r16,r/m16;' Move if not above or equal (CF=1).
	///
	/// 'cmovnae r32,r/m32;' Move if not above or equal (CF=1).
	///
	/// 'cmovnae r64,r/m64;' Move if not above or equal (CF=1).
	CMOVNAE,
	///
	/// 'cmovna r16,r/m16;' Move if not above (CF=1 or ZF=1).
	///
	/// 'cmovna r32,r/m32;' Move if not above (CF=1 or ZF=1).
	///
	/// 'cmovna r64,r/m64;' Move if not above (CF=1 or ZF=1).
	CMOVNA,
	///
	/// 'cmovno r16,r/m16;' Move if not overflow (OF=0).
	///
	/// 'cmovno r32,r/m32;' Move if not overflow (OF=0).
	///
	/// 'cmovno r64,r/m64;' Move if not overflow (OF=0).
	CMOVNO,
	///
	/// 'cmovs r16,r/m16;' Move if sign (SF=1).
	///
	/// 'cmovs r32,r/m32;' Move if sign (SF=1).
	///
	/// 'cmovs r64,r/m64;' Move if sign (SF=1).
	CMOVS,
	///
	/// 'cmovng r16,r/m16;' Move if not greater (ZF=1 or SF != OF).
	///
	/// 'cmovng r32,r/m32;' Move if not greater (ZF=1 or SF != OF).
	///
	/// 'cmovng r64,r/m64;' Move if not greater (ZF=1 or SF != OF).
	CMOVNG,
	///
	/// 'cmovnz r16,r/m16;' Move if not zero (ZF=0).
	///
	/// 'cmovnz r32,r/m32;' Move if not zero (ZF=0).
	///
	/// 'cmovnz r64,r/m64;' Move if not zero (ZF=0).
	CMOVNZ,
	///
	/// 'cmova r16,r/m16;' Move if above (CF=0 and ZF=0).
	///
	/// 'cmova r32,r/m32;' Move if above (CF=0 and ZF=0).
	///
	/// 'cmova r64,r/m64;' Move if above (CF=0 and ZF=0).
	CMOVA,
	///
	/// 'cmovb r16,r/m16;' Move if below (CF=1).
	///
	/// 'cmovb r32,r/m32;' Move if below (CF=1).
	///
	/// 'cmovb r64,r/m64;' Move if below (CF=1).
	CMOVB,
	///
	/// 'cmovnp r16,r/m16;' Move if not parity (PF=0).
	///
	/// 'cmovnp r32,r/m32;' Move if not parity (PF=0).
	///
	/// 'cmovnp r64,r/m64;' Move if not parity (PF=0).
	CMOVNP,
	///
	/// 'cmovnb r16,r/m16;' Move if not below (CF=0).
	///
	/// 'cmovnb r32,r/m32;' Move if not below (CF=0).
	///
	/// 'cmovnb r64,r/m64;' Move if not below (CF=0).
	CMOVNB,
	///
	/// 'cmovbe r16,r/m16;' Move if below or equal (CF=1 or ZF=1).
	///
	/// 'cmovbe r32,r/m32;' Move if below or equal (CF=1 or ZF=1).
	///
	/// 'cmovbe r64,r/m64;' Move if below or equal (CF=1 or ZF=1).
	CMOVBE,
	///
	/// 'cmovg r16,r/m16;' Move if greater (ZF=0 and SF=OF).
	///
	/// 'cmovg r32,r/m32;' Move if greater (ZF=0 and SF=OF).
	///
	/// 'cmovg r64,r/m64;' Move if greater (ZF=0 and SF=OF).
	CMOVG,
	///
	/// 'cmovp r16,r/m16;' Move if parity (PF=1).
	///
	/// 'cmovp r32,r/m32;' Move if parity (PF=1).
	///
	/// 'cmovp r64,r/m64;' Move if parity (PF=1).
	CMOVP,
	///
	/// 'cmovnl r16,r/m16;' Move if not less (SF=OF).
	///
	/// 'cmovnl r32,r/m32;' Move if not less (SF=OF).
	///
	/// 'cmovnl r64,r/m64;' Move if not less (SF=OF).
	CMOVNL,
// CMP--Compare Two Operands.
	///
	/// 'cmp AL,imm8;' Compare imm8 with AL.
	///
	/// 'cmp AX,imm16;' Compare imm16 with AX.
	///
	/// 'cmp EAX,imm32;' Compare imm32 with EAX.
	///
	/// 'cmp RAX,imm32;' Compare imm32 sign-extended to 64-bits with RAX.
	///
	/// 'cmp r/m8,imm8*;' Compare imm8 with r/m8.
	///
	/// 'cmp r/m8,imm8;' Compare imm8 with r/m8.
	///
	/// 'cmp r/m16,imm16;' Compare imm16 with r/m16.
	///
	/// 'cmp r/m32,imm32;' Compare imm32 with r/m32.
	///
	/// 'cmp r/m64,imm32;' Compare imm32 sign-extended to 64-bits with r/m64.
	///
	/// 'cmp r/m16,imm8;' Compare imm8 with r/m16.
	///
	/// 'cmp r/m32,imm8;' Compare imm8 with r/m32.
	///
	/// 'cmp r/m64,imm8;' Compare imm8 with r/m64.
	///
	/// 'cmp r/m8,r8**;' Compare r8 with r/m8.
	///
	/// 'cmp r/m8,r8;' Compare r8 with r/m8.
	///
	/// 'cmp r/m16,r16;' Compare r16 with r/m16.
	///
	/// 'cmp r/m32,r32;' Compare r32 with r/m32.
	///
	/// 'cmp r/m64,r64;' Compare r64 with r/m64.
	///
	/// 'cmp r8,r/m8**;' Compare r/m8 with r8.
	///
	/// 'cmp r8,r/m8;' Compare r/m8 with r8.
	///
	/// 'cmp r16,r/m16;' Compare r/m16 with r16.
	///
	/// 'cmp r32,r/m32;' Compare r/m32 with r32.
	///
	/// 'cmp r64,r/m64;' Compare r/m64 with r64.
	CMP,
// CMPPD--Compare Packed Double-Precision Floating-Point Values.
	///
	/// 'cmppd xmm1,xmm2/m128,imm8;' Compare packed double-precision floatingpoint values in xmm2/m128 and xmm1 using imm8 as comparison predicate.
	CMPPD,
	///
	/// 'vcmppd xmm1,xmm2,xmm3/m128,imm8;' Compare packed double-precision floatingpoint values in xmm3/m128 and xmm2 using bits 4:0 of imm8 as a comparison predicate.
	///
	/// 'vcmppd ymm1,ymm2,ymm3/m256,imm8;' Compare packed double-precision floatingpoint values in ymm3/m256 and ymm2 using bits 4:0 of imm8 as a comparison predicate.
	VCMPPD,
// CMPPS--Compare Packed Single-Precision Floating-Point Values.
	///
	/// 'vcmpps xmm1,xmm2,xmm3/m128,imm8;' Compare packed single-precision floatingpoint values in xmm3/m128 and xmm2 using bits 4:0 of imm8 as a comparison predicate.
	///
	/// 'vcmpps ymm1,ymm2,ymm3/m256,imm8;' Compare packed single-precision floatingpoint values in ymm3/m256 and ymm2 using bits 4:0 of imm8 as a comparison predicate.
	VCMPPS,
	///
	/// 'cmpps xmm1,xmm2/m128,imm8;' Compare packed single-precision floatingpoint values in xmm2/mem and xmm1 using imm8 as comparison predicate.
	CMPPS,
// CMPS/CMPSB/CMPSW/CMPSD/CMPSQ--Compare String Operands.
	///
	/// 'cmpsd;' For legacy mode, compare dword at address DS:(E)SI with dword at address ES:(E)DI; For 64-bit mode compare dword at address (R|E)SI with dword at address (R|E)DI. The status flags are set accordingly.
	CMPSD,
	///
	/// 'cmpsw;' For legacy mode, compare word at address DS:(E)SI with word at address ES:(E)DI; For 64word at address (R|E)DI. The status flags are set accordingly.
	CMPSW,
	///
	/// 'cmps m8,m8;' For legacy mode, compare byte at address DS:(E)SI with byte at address ES:(E)DI; For 64byte at address (R|E)DI. The status flags are set accordingly.
	///
	/// 'cmps m16,m16;' For legacy mode, compare word at address DS:(E)SI with word at address ES:(E)DI; For 64word at address (R|E)DI. The status flags are set accordingly.
	///
	/// 'cmps m32,m32;' For legacy mode, compare dword at address DS:(E)SI at dword at address ES:(E)DI; For 64dword at address (R|E)DI. The status flags are set accordingly.
	///
	/// 'cmps m64,m64;' Compares quadword at address (R|E)SI with quadword at address (R|E)DI and sets the status flags accordingly.
	CMPS,
	///
	/// 'cmpsb;' For legacy mode, compare byte at address DS:(E)SI with byte at address ES:(E)DI; For 64byte at address (R|E)DI. The status flags are set accordingly.
	CMPSB,
	///
	/// 'cmpsq;' Compares quadword at address (R|E)SI with quadword at address (R|E)DI and sets the status flags accordingly.
	CMPSQ,
// CMPSD--Compare Scalar Double-Precision Floating-Point Values.
	///
	/// 'cmpsd xmm1,xmm2/m64,imm8;' Compare low double-precision floating-point value in xmm2/m64 and xmm1 using imm8 as comparison predicate.
	CMPSD,
	///
	/// 'vcmpsd xmm1,xmm2,xmm3/m64,imm8;' Compare low double precision floating-point value in xmm3/m64 and xmm2 using bits 4:0 of imm8 as comparison predicate.
	VCMPSD,
// CMPSS--Compare Scalar Single-Precision Floating-Point Values.
	///
	/// 'cmpss xmm1,xmm2/m32,imm8;' Compare low single-precision floating-point value in xmm2/m32 and xmm1 using imm8 as comparison predicate.
	CMPSS,
	///
	/// 'vcmpss xmm1,xmm2,xmm3/m32,imm8;' Compare low single precision floating-point value in xmm3/m32 and xmm2 using bits 4:0 of imm8 as comparison predicate.
	VCMPSS,
// CMPXCHG--Compare and Exchange.
	///
	/// 'cmpxchg r/m8,r8;' Compare AL with r/m8. If equal, ZF is set and r8 is loaded into r/m8. Else, clear ZF and load r/m8 into AL.
	///
	/// 'cmpxchg r/m8**,r8;' Compare AL with r/m8. If equal, ZF is set and r8 is loaded into r/m8. Else, clear ZF and load r/m8 into AL.
	///
	/// 'cmpxchg r/m16,r16;' Compare AX with r/m16. If equal, ZF is set and r16 is loaded into r/m16. Else, clear ZF and load r/m16 into AX.
	///
	/// 'cmpxchg r/m32,r32;' Compare EAX with r/m32. If equal, ZF is set and r32 is loaded into r/m32. Else, clear ZF and load r/m32 into EAX.
	///
	/// 'cmpxchg r/m64,r64;' Compare RAX with r/m64. If equal, ZF is set and r64 is loaded into r/m64. Else, clear ZF and load r/m64 into RAX.
	CMPXCHG,
// CMPXCHG8B/CMPXCHG16B--Compare and Exchange Bytes.
	///
	/// 'cmpxchg8b m64;' Compare EDX:EAX with m64. If equal, set ZF and load ECX:EBX into m64. Else, clear ZF and load m64 into EDX:EAX.
	CMPXCHG8B,
	///
	/// 'cmpxchg16b m128;' Compare RDX:RAX with m128. If equal, set ZF and load RCX:RBX into m128. Else, clear ZF and load m128 into RDX:RAX.
	CMPXCHG16B,
// COMISD--Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS.
	///
	/// 'comisd xmm1,xmm2/m64;' Compare low double-precision floating-point values in xmm1 and xmm2/mem64 and set the EFLAGS flags accordingly.
	COMISD,
	///
	/// 'vcomisd xmm1,xmm2/m64;' Compare low double precision floating-point values in xmm1 and xmm2/mem64 and set the EFLAGS flags accordingly.
	VCOMISD,
// COMISS--Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS.
	///
	/// 'comiss xmm1,xmm2/m32;' Compare low single-precision floating-point values in xmm1 and xmm2/mem32 and set the EFLAGS flags accordingly.
	COMISS,
	///
	/// 'vcomiss xmm1,xmm2/m32;' Compare low single precision floating-point values in xmm1 and xmm2/mem32 and set the EFLAGS flags accordingly.
	VCOMISS,
// CPUID--CPU Identification.
	///
	/// 'cpuid;' Returns processor identification and feature information to the EAX, EBX, ECX, and EDX registers, as determined by input entered in EAX (in some cases, ECX as well).
	CPUID,
// CRC32--Accumulate CRC32 Value.
	///
	/// 'crc32 r32,r/m8;' Accumulate CRC32 on r/m8.
	///
	/// 'crc32 r32,r/m8*;' Accumulate CRC32 on r/m8.
	///
	/// 'crc32 r32,r/m16;' Accumulate CRC32 on r/m16.
	///
	/// 'crc32 r32,r/m32;' Accumulate CRC32 on r/m32.
	///
	/// 'crc32 r64,r/m8;' Accumulate CRC32 on r/m8.
	///
	/// 'crc32 r64,r/m64;' Accumulate CRC32 on r/m64.
	CRC32,
// CVTDQ2PD--Convert Packed Dword Integers to Packed Double-Precision FP Values.
	///
	/// 'vcvtdq2pd xmm1,xmm2/m64;' Convert two packed signed doubleword integers from xmm2/mem to two packed double-precision floating-point values in xmm1.
	///
	/// 'vcvtdq2pd ymm1,xmm2/m128;' Convert four packed signed doubleword integers from xmm2/mem to four packed double-precision floating-point values in ymm1.
	VCVTDQ2PD,
	///
	/// 'cvtdq2pd xmm1,xmm2/m64;' Convert two packed signed doubleword integers from xmm2/m128 to two packed double-precision floating-point values in xmm1.
	CVTDQ2PD,
// CVTDQ2PS--Convert Packed Dword Integers to Packed Single-Precision FP Values.
	///
	/// 'vcvtdq2ps xmm1,xmm2/m128;' Convert four packed signed doubleword integers from xmm2/mem to four packed single-precision floating-point values in xmm1.
	///
	/// 'vcvtdq2ps ymm1,ymm2/m256;' Convert eight packed signed doubleword integers from ymm2/mem to eight packed single-precision floating-point values in ymm1.
	VCVTDQ2PS,
	///
	/// 'cvtdq2ps xmm1,xmm2/m128;' Convert four packed signed doubleword integers from xmm2/m128 to four packed single-precision floating-point values in xmm1.
	CVTDQ2PS,
// CVTPD2DQ--Convert Packed Double-Precision FP Values to Packed Dword Integers.
	///
	/// 'vcvtpd2dq xmm1,xmm2/m128;' Convert two packed double-precision floatingpoint values in xmm2/mem to two signed doubleword integers in xmm1.
	///
	/// 'vcvtpd2dq xmm1,ymm2/m256;' Convert four packed double-precision floatingpoint values in ymm2/mem to four signed doubleword integers in xmm1.
	VCVTPD2DQ,
	///
	/// 'cvtpd2dq xmm1,xmm2/m128;' Convert two packed double-precision floatingpoint values from xmm2/m128 to two packed signed doubleword integers in xmm1.
	CVTPD2DQ,
// CVTPD2PI--Convert Packed Double-Precision FP Values to Packed Dword Integers.
	///
	/// 'cvtpd2pi mm,xmm/m128;' Convert two packed double-precision floatingpoint values from xmm/m128 to two packed signed doubleword integers in mm.
	CVTPD2PI,
// CVTPD2PS--Convert Packed Double-Precision FP Values to Packed Single-Precision FP Values.
	///
	/// 'vcvtpd2ps xmm1,xmm2/m128;' Convert two packed double-precision floatingpoint values in xmm2/mem to two singleprecision floating-point values in xmm1.
	///
	/// 'vcvtpd2ps xmm1,ymm2/m256;' Convert four packed double-precision floatingpoint values in ymm2/mem to four singleprecision floating-point values in xmm1.
	VCVTPD2PS,
	///
	/// 'cvtpd2ps xmm1,xmm2/m128;' Convert two packed double-precision floatingpoint values in xmm2/m128 to two packed single-precision floating-point values in xmm1.
	CVTPD2PS,
// CVTPI2PD--Convert Packed Dword Integers to Packed Double-Precision FP Values.
	///
	/// 'cvtpi2pd xmm,mm/m64*;' Convert two packed signed doubleword integers from mm/mem64 to two packed double-precision floating-point values in xmm.
	CVTPI2PD,
// CVTPI2PS--Convert Packed Dword Integers to Packed Single-Precision FP Values.
	///
	/// 'cvtpi2ps xmm,mm/m64;' Convert two signed doubleword integers from mm/m64 to two single-precision floating-point values in xmm.
	CVTPI2PS,
// CVTPS2DQ--Convert Packed Single-Precision FP Values to Packed Dword Integers.
	///
	/// 'vcvtps2dq xmm1,xmm2/m128;' Convert four packed single precision floatingpoint values from xmm2/mem to four packed signed doubleword values in xmm1.
	///
	/// 'vcvtps2dq ymm1,ymm2/m256;' Convert eight packed single precision floatingpoint values from ymm2/mem to eight packed signed doubleword values in ymm1.
	VCVTPS2DQ,
	///
	/// 'cvtps2dq xmm1,xmm2/m128;' Convert four packed single-precision floatingpoint values from xmm2/m128 to four packed signed doubleword integers in xmm1.
	CVTPS2DQ,
// CVTPS2PD--Convert Packed Single-Precision FP Values to Packed Double-Precision FP Values.
	///
	/// 'cvtps2pd xmm1,xmm2/m64;' Convert two packed single-precision floatingpoint values in xmm2/m64 to two packed double-precision floating-point values in xmm1.
	CVTPS2PD,
	///
	/// 'vcvtps2pd xmm1,xmm2/m64;' Convert two packed single-precision floatingpoint values in xmm2/mem to two packed double-precision floating-point values in xmm1.
	///
	/// 'vcvtps2pd ymm1,xmm2/m128;' Convert four packed single-precision floatingpoint values in xmm2/mem to four packed double-precision floating-point values in ymm1.
	VCVTPS2PD,
// CVTPS2PI--Convert Packed Single-Precision FP Values to Packed Dword Integers.
	///
	/// 'cvtps2pi mm,xmm/m64;' Convert two packed single-precision floatingpoint values from xmm/m64 to two packed signed doubleword integers in mm.
	CVTPS2PI,
// CVTSD2SI--Convert Scalar Double-Precision FP Value to Integer.
	///
	/// 'vcvtsd2si r32,xmm1/m64;' Convert one double precision floating-point value from xmm1/m64 to one signed doubleword integer r32.
	///
	/// 'vcvtsd2si r64,xmm1/m64;' Convert one double precision floating-point value from xmm1/m64 to one signed quadword integer sign-extended into r64.
	VCVTSD2SI,
	///
	/// 'cvtsd2si r32,xmm/m64;' Convert one double-precision floating-point value from xmm/m64 to one signed doubleword integer r32.
	///
	/// 'cvtsd2si r64,xmm/m64;' Convert one double-precision floating-point value from xmm/m64 to one signed quadword integer sign-extended into r64.
	CVTSD2SI,
// CVTSD2SS--Convert Scalar Double-Precision FP Value to Scalar Single-Precision FP Value.
	///
	/// 'cvtsd2ss xmm1,xmm2/m64;' Convert one double-precision floating-point value in xmm2/m64 to one single-precision floating-point value in xmm1.
	CVTSD2SS,
	///
	/// 'vcvtsd2ss xmm1,xmm2,xmm3/m64;' Convert one double-precision floating-point value in xmm3/m64 to one single-precision floating-point value and merge with high bits in xmm2.
	VCVTSD2SS,
// CVTSI2SD--Convert Dword Integer to Scalar Double-Precision FP Value.
	///
	/// 'vcvtsi2sd xmm1,xmm2,r/m32;' Convert one signed doubleword integer from r/m32 to one double-precision floating-point value in xmm1.
	///
	/// 'vcvtsi2sd xmm1,xmm2,r/m64;' Convert one signed quadword integer from r/m64 to one double-precision floating-point value in xmm1.
	VCVTSI2SD,
	///
	/// 'cvtsi2sd xmm,r/m32;' Convert one signed doubleword integer from r/m32 to one double-precision floating-point value in xmm.
	///
	/// 'cvtsi2sd xmm,r/m64;' Convert one signed quadword integer from r/m64 to one double-precision floating-point value in xmm.
	CVTSI2SD,
// CVTSI2SS--Convert Dword Integer to Scalar Single-Precision FP Value.
	///
	/// 'cvtsi2ss xmm,r/m32;' Convert one signed doubleword integer from r/m32 to one single-precision floating-point value in xmm.
	///
	/// 'cvtsi2ss xmm,r/m64;' Convert one signed quadword integer from r/m64 to one single-precision floating-point value in xmm.
	CVTSI2SS,
	///
	/// 'vcvtsi2ss xmm1,xmm2,r/m32;' Convert one signed doubleword integer from r/m32 to one single-precision floating-point value in xmm1.
	///
	/// 'vcvtsi2ss xmm1,xmm2,r/m64;' Convert one signed quadword integer from r/m64 to one single-precision floating-point value in xmm1.
	VCVTSI2SS,
// CVTSS2SD--Convert Scalar Single-Precision FP Value to Scalar Double-Precision FP Value.
	///
	/// 'cvtss2sd xmm1,xmm2/m32;' Convert one single-precision floating-point value in xmm2/m32 to one double-precision floating-point value in xmm1.
	CVTSS2SD,
	///
	/// 'vcvtss2sd xmm1,xmm2,xmm3/m32;' Convert one single-precision floating-point value in xmm3/m32 to one double-precision floating-point value and merge with high bits of xmm2.
	VCVTSS2SD,
// CVTSS2SI--Convert Scalar Single-Precision FP Value to Dword Integer.
	///
	/// 'vcvtss2si r32,xmm1/m32;' Convert one single-precision floating-point value from xmm1/m32 to one signed doubleword integer in r32.
	///
	/// 'vcvtss2si r64,xmm1/m32;' Convert one single-precision floating-point value from xmm1/m32 to one signed quadword integer in r64.
	VCVTSS2SI,
	///
	/// 'cvtss2si r32,xmm/m32;' Convert one single-precision floating-point value from xmm/m32 to one signed doubleword integer in r32.
	///
	/// 'cvtss2si r64,xmm/m32;' Convert one single-precision floating-point value from xmm/m32 to one signed quadword integer in r64.
	CVTSS2SI,
// CVTTPD2DQ--Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers.
	///
	/// 'cvttpd2dq xmm1,xmm2/m128;' Convert two packed double-precision floatingpoint values from xmm2/m128 to two packed signed doubleword integers in xmm1 using truncation.
	CVTTPD2DQ,
	///
	/// 'vcvttpd2dq xmm1,xmm2/m128;' Convert two packed double-precision floatingpoint values in xmm2/mem to two signed doubleword integers in xmm1 using truncation.
	///
	/// 'vcvttpd2dq xmm1,ymm2/m256;' Convert four packed double-precision floatingpoint values in ymm2/mem to four signed doubleword integers in xmm1 using truncation.
	VCVTTPD2DQ,
// CVTTPD2PI--Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers.
	///
	/// 'cvttpd2pi mm,xmm/m128;' Convert two packer double-precision floatingpoint values from xmm/m128 to two packed signed doubleword integers in mm using truncation.
	CVTTPD2PI,
// CVTTPS2DQ--Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers.
	///
	/// 'vcvttps2dq xmm1,xmm2/m128;' Convert four packed single precision floatingpoint values from xmm2/mem to four packed signed doubleword values in xmm1 using truncation.
	///
	/// 'vcvttps2dq ymm1,ymm2/m256;' Convert eight packed single precision floatingpoint values from ymm2/mem to eight packed signed doubleword values in ymm1 using truncation.
	VCVTTPS2DQ,
	///
	/// 'cvttps2dq xmm1,xmm2/m128;' Convert four single-precision floating-point values from xmm2/m128 to four signed doubleword integers in xmm1 using truncation.
	CVTTPS2DQ,
// CVTTPS2PI--Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers.
	///
	/// 'cvttps2pi mm,xmm/m64;' Convert two single-precision floating-point values from xmm/m64 to two signed doubleword signed integers in mm using truncation.
	CVTTPS2PI,
// CVTTSD2SI--Convert with Truncation Scalar Double-Precision FP Value to Signed Integer.
	///
	/// 'vcvttsd2si r32,xmm1/m64;' Convert one double-precision floating-point value from xmm1/m64 to one signed doubleword integer in r32 using truncation.
	///
	/// 'vcvttsd2si r64,xmm1/m64;' Convert one double precision floating-point value from xmm1/m64 to one signed quadword integer in r64 using truncation.
	VCVTTSD2SI,
	///
	/// 'cvttsd2si r32,xmm/m64;' Convert one double-precision floating-point value from xmm/m64 to one signed doubleword integer in r32 using truncation.
	///
	/// 'cvttsd2si r64,xmm/m64;' Convert one double precision floating-point value from xmm/m64 to one signedquadword integer in r64 using truncation.
	CVTTSD2SI,
// CVTTSS2SI--Convert with Truncation Scalar Single-Precision FP Value to Dword Integer.
	///
	/// 'vcvttss2si r32,xmm1/m32;' Convert one single-precision floating-point value from xmm1/m32 to one signed doubleword integer in r32 using truncation.
	///
	/// 'vcvttss2si r64,xmm1/m32;' Convert one single-precision floating-point value from xmm1/m32 to one signed quadword integer in r64 using truncation.
	VCVTTSS2SI,
	///
	/// 'cvttss2si r32,xmm/m32;' Convert one single-precision floating-point value from xmm/m32 to one signed doubleword integer in r32 using truncation.
	///
	/// 'cvttss2si r64,xmm/m32;' Convert one single-precision floating-point value from xmm/m32 to one signed quadword integer in r64 using truncation.
	CVTTSS2SI,
// CWD/CDQ/CQO--Convert Word to Doubleword/Convert Doubleword to Quadword.
	///
	/// 'cdq;' EDX:EAX <-- sign-extend of EAX.
	CDQ,
	///
	/// 'cwd;' DX:AX <-- sign-extend of AX.
	CWD,
	///
	/// 'cqo;' RDX:RAX<-- sign-extend of RAX.
	CQO,
// DAA--Decimal Adjust AL after Addition.
	///
	/// 'daa;' Decimal adjust AL after addition.
	DAA,
// DAS--Decimal Adjust AL after Subtraction.
	///
	/// 'das;' Decimal adjust AL after subtraction.
	DAS,
// DEC--Decrement by 1.
	///
	/// 'dec r/m8*;' Decrement r/m8 by 1.
	///
	/// 'dec r/m8;' Decrement r/m8 by 1.
	///
	/// 'dec r/m16;' Decrement r/m16 by 1.
	///
	/// 'dec r/m32;' Decrement r/m32 by 1.
	///
	/// 'dec r/m64;' Decrement r/m64 by 1.
	///
	/// 'dec r16;' Decrement r16 by 1.
	///
	/// 'dec r32;' Decrement r32 by 1.
	DEC,
// DIV--Unsigned Divide.
	///
	/// 'div r/m8*;' Unsigned divide AX by r/m8, with result stored in AL <-- Quotient, AH ? Remainder.
	///
	/// 'div r/m8;' Unsigned divide AX by r/m8, with result stored in AL <-- Quotient, AH ? Remainder.
	///
	/// 'div r/m16;' Unsigned divide DX:AX by r/m16, with result stored in AX <-- Quotient, DX ? Remainder.
	///
	/// 'div r/m32;' Unsigned divide EDX:EAX by r/m32, with result stored in EAX <-- Quotient, EDX ? Remainder.
	///
	/// 'div r/m64;' Unsigned divide RDX:RAX by r/m64, with result stored in RAX <-- Quotient, RDX ? Remainder.
	DIV,
// DIVPD--Divide Packed Double-Precision Floating-Point Values.
	///
	/// 'vdivpd xmm1,xmm2,xmm3/m128;' Divide packed double-precision floating-point values in xmm2 by packed double-precision floating-point values in xmm3/mem.
	///
	/// 'vdivpd ymm1,ymm2,ymm3/m256;' Divide packed double-precision floating-point values in ymm2 by packed double-precision floating-point values in ymm3/mem.
	VDIVPD,
	///
	/// 'divpd xmm1,xmm2/m128;' Divide packed double-precision floating-point values in xmm1 by packed double-precision floating-point values xmm2/m128.
	DIVPD,
// DIVPS--Divide Packed Single-Precision Floating-Point Values.
	///
	/// 'divps xmm1,xmm2/m128;' Divide packed single-precision floating-point values in xmm1 by packed single-precision floating-point values xmm2/m128.
	DIVPS,
	///
	/// 'vdivps xmm1,xmm2,xmm3/m128;' Divide packed single-precision floating-point values in xmm2 by packed double-precision floating-point values in xmm3/mem.
	///
	/// 'vdivps ymm1,ymm2,ymm3/m256;' Divide packed single-precision floating-point values in ymm2 by packed double-precision floating-point values in ymm3/mem.
	VDIVPS,
// DIVSD--Divide Scalar Double-Precision Floating-Point Values.
	///
	/// 'vdivsd xmm1,xmm2,xmm3/m64;' Divide low double-precision floating point values in xmm2 by low double precision floating-point value in xmm3/mem64.
	VDIVSD,
	///
	/// 'divsd xmm1,xmm2/m64;' Divide low double-precision floating-point value in xmm1 by low double-precision floating-point value in xmm2/mem64.
	DIVSD,
// DIVSS--Divide Scalar Single-Precision Floating-Point Values.
	///
	/// 'divss xmm1,xmm2/m32;' Divide low single-precision floating-point value in xmm1 by low single-precision floating-point value in xmm2/m32.
	DIVSS,
	///
	/// 'vdivss xmm1,xmm2,xmm3/m32;' Divide low single-precision floating point value in xmm2 by low single precision floating-point value in xmm3/m32.
	VDIVSS,
// DPPD--Dot Product of Packed Double Precision Floating-Point Values.
	///
	/// 'dppd xmm1,xmm2/m128,imm8;' Selectively multiply packed DP floating-point values from xmm1 with packed DP floatingpoint values from xmm2, add and selectively store the packed DP floating-point values to xmm1.
	DPPD,
	///
	/// 'vdppd xmm1,xmm2,xmm3/m128,imm8;' Selectively multiply packed DP floating-point values from xmm2 with packed DP floatingpoint values from xmm3, add and selectively store the packed DP floating-point values to xmm1.
	VDPPD,
// DPPS--Dot Product of Packed Single Precision Floating-Point Values.
	///
	/// 'dpps xmm1,xmm2/m128,imm8;' Selectively multiply packed SP floating-point values from xmm1 with packed SP floatingpoint values from xmm2, add and selectively store the packed SP floating-point values or zero values to xmm1.
	DPPS,
	///
	/// 'vdpps xmm1,xmm2,xmm3/m128,imm8;' Multiply packed SP floating point values from xmm1 with packed SP floating point values from xmm2/mem selectively add and store to xmm1.
	///
	/// 'vdpps ymm1,ymm2,ymm3/m256,imm8;' Multiply packed single-precision floating-point values from ymm2 with packed SP floating point values from ymm3/mem, selectively add pairs of elements and store to ymm1.
	VDPPS,
// EMMS--Empty MMX Technology State.
	///
	/// 'emms;' Set the x87 FPU tag word to empty.
	EMMS,
// ENTER--Make Stack Frame for Procedure Parameters.
	///
	/// 'enter imm16,0;' Create a stack frame for a procedure.
	///
	/// 'enter imm16,1;' Create a stack frame with a nested pointer for a procedure.
	///
	/// 'enter imm16,imm8;' Create a stack frame with nested pointers for a procedure.
	ENTER,
// EXTRACTPS--Extract Packed Single Precision Floating-Point Value.
	///
	/// 'vextractps r/m32,xmm1,imm8;' Extract one single-precision floating-point value from xmm1 at the offset specified by imm8 and store the result in reg or m32. Zero extend the results in 64-bit register if applicable.
	VEXTRACTPS,
	///
	/// 'extractps reg/m32,xmm2,imm8;' Extract a single-precision floating-point value from xmm2 at the source offset specified by imm8 and store the result to reg or m32. The upper 32 bits of r64 is zeroed if reg is r64.
	EXTRACTPS,
// F2XM1--Compute 2  -1.
	///
	/// 'f2xm1;' Replace ST(0) with (2ST(0) 1).
	F2XM1,
// FABS--Absolute Value.
	///
	/// 'fabs;' Replace ST with its absolute value.
	FABS,
// FADD/FADDP/FIADD--Add.
	///
	/// 'fadd m32fp;' Add m32fp to ST(0) and store result in ST(0).
	///
	/// 'fadd m64fp;' Add m64fp to ST(0) and store result in ST(0).
	///
	/// 'fadd ST(0),ST(i);' Add ST(0) to ST(i) and store result in ST(0).
	///
	/// 'fadd ST(i),ST(0);' Add ST(i) to ST(0) and store result in ST(i).
	FADD,
	///
	/// 'faddp ST(i),ST(0);' Add ST(0) to ST(i), store result in ST(i), and pop the register stack.
	///
	/// 'faddp;' Add ST(0) to ST(1), store result in ST(1), and pop the register stack.
	FADDP,
	///
	/// 'fiadd m32int;' Add m32int to ST(0) and store result in ST(0).
	///
	/// 'fiadd m16int;' Add m16int to ST(0) and store result in ST(0).
	FIADD,
// FBLD--Load Binary Coded Decimal.
	///
	/// 'fbld m80dec;' Convert BCD value to floating-point and push onto the FPU stack.
	FBLD,
// FBSTP--Store BCD Integer and Pop.
	///
	/// 'fbstp m80bcd;' Store ST(0) in m80bcd and pop ST(0).
	FBSTP,
// FCHS--Change Sign.
	///
	/// 'fchs;' Complements sign of ST(0).
	FCHS,
// FCLEX/FNCLEX--Clear Exceptions.
	///
	/// 'fnclex;' Clear floating-point exception flags without checking for pending unmasked floating-point exceptions.
	FNCLEX,
	///
	/// 'fclex*;' Clear floating-point exception flags after checking for pending unmasked floating-point exceptions.
	FCLEX*,
// FCMOVcc--Floating-Point Conditional Move.
	///
	/// 'fcmovne ST(0),ST(i);' Move if not equal (ZF=0).
	FCMOVNE,
	///
	/// 'fcmovnb ST(0),ST(i);' Move if not below (CF=0).
	FCMOVNB,
	///
	/// 'fcmovu ST(0),ST(i);' Move if unordered (PF=1).
	FCMOVU,
	///
	/// 'fcmovbe ST(0),ST(i);' Move if below or equal (CF=1 or ZF=1).
	FCMOVBE,
	///
	/// 'fcmove ST(0),ST(i);' Move if equal (ZF=1).
	FCMOVE,
	///
	/// 'fcmovb ST(0),ST(i);' Move if below (CF=1).
	FCMOVB,
	///
	/// 'fcmovnbe ST(0),ST(i);' Move if not below or equal (CF=0 and ZF=0).
	FCMOVNBE,
	///
	/// 'fcmovnu ST(0),ST(i);' Move if not unordered (PF=0).
	FCMOVNU,
// FCOM/FCOMP/FCOMPP--Compare Floating Point Values.
	///
	/// 'fcomip ST,ST(i);' Compare ST(0) with ST(i), set status flags accordingly, and pop register stack.
	FCOMIP,
	///
	/// 'fcom m32fp;' Compare ST(0) with m32fp.
	///
	/// 'fcom m64fp;' Compare ST(0) with m64fp.
	///
	/// 'fcom ST(i);' Compare ST(0) with ST(i).
	///
	/// 'fcom;' Compare ST(0) with ST(1).
	FCOM,
	///
	/// 'fcomp m32fp;' Compare ST(0) with m32fp and pop register stack.
	///
	/// 'fcomp m64fp;' Compare ST(0) with m64fp and pop register stack.
	///
	/// 'fcomp ST(i);' Compare ST(0) with ST(i) and pop register stack.
	///
	/// 'fcomp;' Compare ST(0) with ST(1) and pop register stack.
	FCOMP,
	///
	/// 'fcompp;' Compare ST(0) with ST(1) and pop register stack twice.
	FCOMPP,
	///
	/// 'fucomip ST,ST(i);' Compare ST(0) with ST(i), check for ordered values, set status flags accordingly, and pop register stack.
	FUCOMIP,
	///
	/// 'fcomi ST,ST(i);' Compare ST(0) with ST(i) and set status flags accordingly.
	FCOMI,
	///
	/// 'fucomi ST,ST(i);' Compare ST(0) with ST(i), check for ordered values, and set status flags accordingly.
	FUCOMI,
// FCOS--Cosine.
	///
	/// 'fcos;' Replace ST(0) with its approximate cosine.
	FCOS,
// FDECSTP--Decrement Stack-Top Pointer.
	///
	/// 'fdecstp;' Decrement TOP field in FPU status word.
	FDECSTP,
// FDIV/FDIVP/FIDIV--Divide.
	///
	/// 'fdivp ST(i),ST(0);' Divide ST(i) by ST(0), store result in ST(i), and pop the register stack.
	///
	/// 'fdivp;' Divide ST(1) by ST(0), store result in ST(1), and pop the register stack.
	FDIVP,
	///
	/// 'fidiv m32int;' Divide ST(0) by m32int and store result in ST(0).
	///
	/// 'fidiv m16int;' Divide ST(0) by m16int and store result in ST(0).
	FIDIV,
	///
	/// 'fdiv m32fp;' Divide ST(0) by m32fp and store result in ST(0).
	///
	/// 'fdiv m64fp;' Divide ST(0) by m64fp and store result in ST(0).
	///
	/// 'fdiv ST(0),ST(i);' Divide ST(0) by ST(i) and store result in ST(0).
	///
	/// 'fdiv ST(i),ST(0);' Divide ST(i) by ST(0) and store result in ST(i).
	FDIV,
// FDIVR/FDIVRP/FIDIVR--Reverse Divide.
	///
	/// 'fidivr m32int;' Divide m32int by ST(0) and store result in ST(0).
	///
	/// 'fidivr m16int;' Divide m16int by ST(0) and store result in ST(0).
	FIDIVR,
	///
	/// 'fdivrp ST(i),ST(0);' Divide ST(0) by ST(i), store result in ST(i), and pop the register stack.
	///
	/// 'fdivrp;' Divide ST(0) by ST(1), store result in ST(1), and pop the register stack.
	FDIVRP,
	///
	/// 'fdivr m32fp;' Divide m32fp by ST(0) and store result in ST(0).
	///
	/// 'fdivr m64fp;' Divide m64fp by ST(0) and store result in ST(0).
	///
	/// 'fdivr ST(0),ST(i);' Divide ST(i) by ST(0) and store result in ST(0).
	///
	/// 'fdivr ST(i),ST(0);' Divide ST(0) by ST(i) and store result in ST(i).
	FDIVR,
// FFREE--Free Floating-Point Register.
	///
	/// 'ffree ST(i);' Sets tag for ST(i) to empty.
	FFREE,
// FICOM/FICOMP--Compare Integer.
	///
	/// 'ficom m16int;' Compare ST(0) with m16int.
	///
	/// 'ficom m32int;' Compare ST(0) with m32int.
	FICOM,
	///
	/// 'ficomp m16int;' Compare ST(0) with m16int and pop stack register.
	///
	/// 'ficomp m32int;' Compare ST(0) with m32int and pop stack register.
	FICOMP,
// FILD--Load Integer.
	///
	/// 'fild m16int;' Push m16int onto the FPU register stack.
	///
	/// 'fild m32int;' Push m32int onto the FPU register stack.
	///
	/// 'fild m64int;' Push m64int onto the FPU register stack.
	FILD,
// FINCSTP--Increment Stack-Top Pointer.
	///
	/// 'fincstp;' Increment the TOP field in the FPU status register.
	FINCSTP,
// FINIT/FNINIT--Initialize Floating-Point Unit.
	///
	/// 'finit*;' Initialize FPU after checking for pending unmasked floating-point exceptions.
	FINIT*,
	///
	/// 'fninit;' Initialize FPU without checking for pending unmasked floating-point exceptions.
	FNINIT,
// FIST/FISTP--Store Integer.
	///
	/// 'fist m16int;' Store ST(0) in m16int.
	///
	/// 'fist m32int;' Store ST(0) in m32int.
	FIST,
	///
	/// 'fistp m16int;' Store ST(0) in m16int and pop register stack.
	///
	/// 'fistp m32int;' Store ST(0) in m32int and pop register stack.
	///
	/// 'fistp m64int;' Store ST(0) in m64int and pop register stack.
	FISTP,
// FISTTP--Store Integer with Truncation.
	///
	/// 'fisttp m16int;' Store ST(0) in m16int with truncation.
	///
	/// 'fisttp m32int;' Store ST(0) in m32int with truncation.
	///
	/// 'fisttp m64int;' Store ST(0) in m64int with truncation.
	FISTTP,
// FLD--Load Floating Point Value.
	///
	/// 'fld m32fp;' Push m32fp onto the FPU register stack.
	///
	/// 'fld m64fp;' Push m64fp onto the FPU register stack.
	///
	/// 'fld m80fp;' Push m80fp onto the FPU register stack.
	///
	/// 'fld ST(i);' Push ST(i) onto the FPU register stack.
	FLD,
// FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ--Load Constant.
	///
	/// 'fldln2;' Push loge2 onto the FPU register stack.
	FLDLN2,
	///
	/// 'fldl2e;' Push log2e onto the FPU register stack.
	FLDL2E,
	///
	/// 'fldpi;' Push p onto the FPU register stack.
	FLDPI,
	///
	/// 'fldz;' Push +0.0 onto the FPU register stack.
	FLDZ,
	///
	/// 'fld1;' Push +1.0 onto the FPU register stack.
	FLD1,
	///
	/// 'fldlg2;' Push log102 onto the FPU register stack.
	FLDLG2,
	///
	/// 'fldl2t;' Push log210 onto the FPU register stack.
	FLDL2T,
// FLDCW--Load x87 FPU Control Word.
	///
	/// 'fldcw m2byte;' Load FPU control word from m2byte.
	FLDCW,
// FLDENV--Load x87 FPU Environment.
	///
	/// 'fldenv m14/28byte;' Load FPU environment from m14byte or m28byte.
	FLDENV,
// FMUL/FMULP/FIMUL--Multiply.
	///
	/// 'fmul m32fp;' Multiply ST(0) by m32fp and store result in ST(0).
	///
	/// 'fmul m64fp;' Multiply ST(0) by m64fp and store result in ST(0).
	///
	/// 'fmul ST(0),ST(i);' Multiply ST(0) by ST(i) and store result in ST(0).
	///
	/// 'fmul ST(i),ST(0);' Multiply ST(i) by ST(0) and store result in ST(i).
	FMUL,
	///
	/// 'fmulp ST(i),ST(0);' Multiply ST(i) by ST(0), store result in ST(i), and pop the register stack.
	///
	/// 'fmulp;' Multiply ST(1) by ST(0), store result in ST(1), and pop the register stack.
	FMULP,
	///
	/// 'fimul m32int;' Multiply ST(0) by m32int and store result in ST(0).
	///
	/// 'fimul m16int;' Multiply ST(0) by m16int and store result in ST(0).
	FIMUL,
// FNOP--No Operation.
	///
	/// 'fnop;' No operation is performed.
	FNOP,
// FPATAN--Partial Arctangent.
	///
	/// 'fpatan;' Replace ST(1) with arctan(ST(1)/ST(0)) and pop the register stack.
	FPATAN,
// FPREM--Partial Remainder.
	///
	/// 'fprem;' Replace ST(0) with the remainder obtained from dividing ST(0) by ST(1).
	FPREM,
// FPREM1--Partial Remainder.
	///
	/// 'fprem1;' Replace ST(0) with the IEEE remainder obtained from dividing ST(0) by ST(1).
	FPREM1,
// FPTAN--Partial Tangent.
	///
	/// 'fptan;' Replace ST(0) with its approximate tangent and push 1 onto the FPU stack.
	FPTAN,
// FRNDINT--Round to Integer.
	///
	/// 'frndint;' Round ST(0) to an integer.
	FRNDINT,
// FRSTOR--Restore x87 FPU State.
	///
	/// 'frstor m94/108byte;' Load FPU state from m94byte or m108byte.
	FRSTOR,
// FSAVE/FNSAVE--Store x87 FPU State.
	///
	/// 'fsave m94/108byte*;' Store FPU state to m94byte or m108byte after checking for pending unmasked floating-point exceptions. Then re-initialize the FPU.
	FSAVE,
	///
	/// 'fnsave m94/108byte;' Store FPU environment to m94byte or m108byte without checking for pending unmasked floatingpoint exceptions. Then re-initialize the FPU.
	FNSAVE,
// FSCALE--Scale.
	///
	/// 'fscale;' Scale ST(0) by ST(1).
	FSCALE,
// FSIN--Sine.
	///
	/// 'fsin;' Replace ST(0) with the approximate of its sine.
	FSIN,
// FSINCOS--Sine and Cosine.
	///
	/// 'fsincos;' Compute the sine and cosine of ST(0); replace ST(0) with the approximate sine, and push the approximate cosine onto the register stack.
	FSINCOS,
// FSQRT--Square Root.
	///
	/// 'fsqrt;' Computes square root of ST(0) and stores the result in ST(0).
	FSQRT,
// FST/FSTP--Store Floating Point Value.
	///
	/// 'fst m32fp;' Copy ST(0) to m32fp.
	///
	/// 'fst m64fp;' Copy ST(0) to m64fp.
	///
	/// 'fst ST(i);' Copy ST(0) to ST(i).
	FST,
	///
	/// 'fstp m32fp;' Copy ST(0) to m32fp and pop register stack.
	///
	/// 'fstp m64fp;' Copy ST(0) to m64fp and pop register stack.
	///
	/// 'fstp m80fp;' Copy ST(0) to m80fp and pop register stack.
	///
	/// 'fstp ST(i);' Copy ST(0) to ST(i) and pop register stack.
	FSTP,
// FSTCW/FNSTCW--Store x87 FPU Control Word.
	///
	/// 'fnstcw m2byte;' Store FPU control word to m2byte without checking for pending unmasked floating-point exceptions.
	FNSTCW,
	///
	/// 'fstcw m2byte*;' Store FPU control word to m2byte after checking for pending unmasked floating-point exceptions.
	FSTCW,
// FSTENV/FNSTENV--Store x87 FPU Environment.
	///
	/// 'fstenv m14/28byte*;' Store FPU environment to m14byte or m28byte after checking for pending unmasked floating-point exceptions. Then mask all floating-point exceptions.
	FSTENV,
	///
	/// 'fnstenv m14/28byte;' Store FPU environment to m14byte or m28byte without checking for pending unmasked floatingpoint exceptions. Then mask all floatingpoint exceptions.
	FNSTENV,
// FSTSW/FNSTSW--Store x87 FPU Status Word.
	///
	/// 'fnstsw m2byte*;' Store FPU status word at m2byte without checking for pending unmasked floating-point exceptions.
	///
	/// 'fnstsw AX;' Store FPU status word in AX register without checking for pending unmasked floating-point exceptions.
	FNSTSW,
	///
	/// 'fstsw m2byte;' Store FPU status word at m2byte after checking for pending unmasked floating-point exceptions.
	///
	/// 'fstsw AX*;' Store FPU status word in AX register after checking for pending unmasked floating-point exceptions.
	FSTSW,
// FSUB/FSUBP/FISUB--Subtract.
	///
	/// 'fsub m32fp;' Subtract m32fp from ST(0) and store result in ST(0).
	///
	/// 'fsub m64fp;' Subtract m64fp from ST(0) and store result in ST(0).
	///
	/// 'fsub ST(0),ST(i);' Subtract ST(i) from ST(0) and store result in ST(0).
	///
	/// 'fsub ST(i),ST(0);' Subtract ST(0) from ST(i) and store result in ST(i).
	FSUB,
	///
	/// 'fsubp ST(i),ST(0);' Subtract ST(0) from ST(i), store result in ST(i), and pop register stack.
	///
	/// 'fsubp;' Subtract ST(0) from ST(1), store result in ST(1), and pop register stack.
	FSUBP,
	///
	/// 'fisub m32int;' Subtract m32int from ST(0) and store result in ST(0).
	///
	/// 'fisub m16int;' Subtract m16int from ST(0) and store result in ST(0).
	FISUB,
// FSUBR/FSUBRP/FISUBR--Reverse Subtract.
	///
	/// 'fisubr m32int;' Subtract ST(0) from m32int and store result in ST(0).
	///
	/// 'fisubr m16int;' Subtract ST(0) from m16int and store result in ST(0).
	FISUBR,
	///
	/// 'fsubr m32fp;' Subtract ST(0) from m32fp and store result in ST(0).
	///
	/// 'fsubr m64fp;' Subtract ST(0) from m64fp and store result in ST(0).
	///
	/// 'fsubr ST(0),ST(i);' Subtract ST(0) from ST(i) and store result in ST(0).
	///
	/// 'fsubr ST(i),ST(0);' Subtract ST(i) from ST(0) and store result in ST(i).
	FSUBR,
	///
	/// 'fsubrp ST(i),ST(0);' Subtract ST(i) from ST(0), store result in ST(i), and pop register stack.
	///
	/// 'fsubrp;' Subtract ST(1) from ST(0), store result in ST(1), and pop register stack.
	FSUBRP,
// FTST--TEST.
	///
	/// 'ftst;' Compare ST(0) with 0.0.
	FTST,
// FUCOM/FUCOMP/FUCOMPP--Unordered Compare Floating Point Values.
	///
	/// 'fucompp;' Compare ST(0) with ST(1) and pop register stack twice.
	FUCOMPP,
	///
	/// 'fucomp ST(i);' Compare ST(0) with ST(i) and pop register stack.
	///
	/// 'fucomp;' Compare ST(0) with ST(1) and pop register stack.
	FUCOMP,
	///
	/// 'fucom ST(i);' Compare ST(0) with ST(i).
	///
	/// 'fucom;' Compare ST(0) with ST(1).
	FUCOM,
// FXAM--Examine ModR/M.
	///
	/// 'fxam;' Classify value or number in ST(0).
	FXAM,
// FXCH--Exchange Register Contents.
	///
	/// 'fxch ST(i);' Exchange the contents of ST(0) and ST(i).
	///
	/// 'fxch;' Exchange the contents of ST(0) and ST(1).
	FXCH,
// FXRSTOR--Restore x87 FPU, MMX, XMM, and MXCSR State.
	///
	/// 'fxrstor m512byte;' Restore the x87 FPU, MMX, XMM, and MXCSR register state from m512byte.
	FXRSTOR,
	///
	/// 'fxrstor64 m512byte;' Restore the x87 FPU, MMX, XMM, and MXCSR register state from m512byte.
	FXRSTOR64,
// FXSAVE--Save x87 FPU, MMX Technology, and SSE State.
	///
	/// 'fxsave m512byte;' Save the x87 FPU, MMX, XMM, and MXCSR register state to m512byte.
	FXSAVE,
	///
	/// 'fxsave64 m512byte;' Save the x87 FPU, MMX, XMM, and MXCSR register state to m512byte.
	FXSAVE64,
// FXTRACT--Extract Exponent and Significand.
	///
	/// 'fxtract;' Separate value in ST(0) into exponent and significand, store exponent in ST(0), and push the significand onto the register stack.
	FXTRACT,
// FYL2X--Compute y * log  x 2.
	///
	/// 'fyl2x;' Replace ST(1) with (ST(1) * log2ST(0)) and pop the register stack.
	FYL2X,
// FYL2XP1--Compute y * log  (x + 1) 2.
	///
	/// 'fyl2xp1;' Replace ST(1) with ST(1) * log2(ST(0) + 1.0) and pop the register stack.
	FYL2XP1,
// HADDPD--Packed Double-FP Horizontal Add.
	///
	/// 'haddpd xmm1,xmm2/m128;' Horizontal add packed double-precision floating-point values from xmm2/m128 to xmm1.
	HADDPD,
	///
	/// 'vhaddpd xmm1,xmm2,xmm3/m128;' Horizontal add packed double-precision floating-point values from xmm2 and xmm3/mem.
	///
	/// 'vhaddpd ymm1,ymm2,ymm3/m256;' Horizontal add packed double-precision floating-point values from ymm2 and ymm3/mem.
	VHADDPD,
// HADDPS--Packed Single-FP Horizontal Add.
	///
	/// 'vhaddps xmm1,xmm2,xmm3/m128;' Horizontal add packed single-precision floating-point values from xmm2 and xmm3/mem.
	///
	/// 'vhaddps ymm1,ymm2,ymm3/m256;' Horizontal add packed single-precision floating-point values from ymm2 and ymm3/mem.
	VHADDPS,
	///
	/// 'haddps xmm1,xmm2/m128;' Horizontal add packed single-precision floating-point values from xmm2/m128 to xmm1.
	HADDPS,
// HLT--Halt.
	///
	/// 'hlt;' Halt.
	HLT,
// HSUBPD--Packed Double-FP Horizontal Subtract.
	///
	/// 'vhsubpd xmm1,xmm2,xmm3/m128;' Horizontal subtract packed double-precision floating-point values from xmm2 and xmm3/mem.
	///
	/// 'vhsubpd ymm1,ymm2,ymm3/m256;' Horizontal subtract packed double-precision floating-point values from ymm2 and ymm3/mem.
	VHSUBPD,
	///
	/// 'hsubpd xmm1,xmm2/m128;' Horizontal subtract packed double-precision floating-point values from xmm2/m128 to xmm1.
	HSUBPD,
// HSUBPS--Packed Single-FP Horizontal Subtract.
	///
	/// 'vhsubps xmm1,xmm2,xmm3/m128;' Horizontal subtract packed single-precision floating-point values from xmm2 and xmm3/mem.
	///
	/// 'vhsubps ymm1,ymm2,ymm3/m256;' Horizontal subtract packed single-precision floating-point values from ymm2 and ymm3/mem.
	VHSUBPS,
	///
	/// 'hsubps xmm1,xmm2/m128;' Horizontal subtract packed single-precision floating-point values from xmm2/m128 to xmm1.
	HSUBPS,
// IDIV--Signed Divide.
	///
	/// 'idiv r/m8;' Signed divide AX by r/m8, with result stored in: AL <-- Quotient, AH ? Remainder.
	///
	/// 'idiv r/m8*;' Signed divide AX by r/m8, with result stored in AL <-- Quotient, AH ? Remainder.
	///
	/// 'idiv r/m16;' Signed divide DX:AX by r/m16, with result stored in AX <-- Quotient, DX ? Remainder.
	///
	/// 'idiv r/m32;' Signed divide EDX:EAX by r/m32, with result stored in EAX <-- Quotient, EDX ? Remainder.
	///
	/// 'idiv r/m64;' Signed divide RDX:RAX by r/m64, with result stored in RAX <-- Quotient, RDX ? Remainder.
	IDIV,
// IMUL--Signed Multiply.
	///
	/// 'imul r/m8*;' AX<-- AL * r/m byte.
	///
	/// 'imul r/m16;' DX:AX <-- AX * r/m word.
	///
	/// 'imul r/m32;' EDX:EAX <-- EAX * r/m32.
	///
	/// 'imul r/m64;' RDX:RAX <-- RAX * r/m64.
	///
	/// 'imul r16,r/m16;' word register <-- word register * r/m16.
	///
	/// 'imul r32,r/m32;' doubleword register <-- doubleword register * r/m32.
	///
	/// 'imul r64,r/m64;' Quadword register <-- Quadword register * r/m64.
	///
	/// 'imul r16,r/m16,imm8;' word register <-- r/m16 * sign-extended immediate byte.
	///
	/// 'imul r32,r/m32,imm8;' doubleword register <-- r/m32 * signextended immediate byte.
	///
	/// 'imul r64,r/m64,imm8;' Quadword register <-- r/m64 * sign-extended immediate byte.
	///
	/// 'imul r16,r/m16,imm16;' word register <-- r/m16 * immediate word.
	///
	/// 'imul r32,r/m32,imm32;' doubleword register <-- r/m32 * immediate doubleword.
	///
	/// 'imul r64,r/m64,imm32;' Quadword register <-- r/m64 * immediate doubleword.
	IMUL,
// IN--Input from Port.
	///
	/// 'in AL,imm8;' Input byte from imm8 I/O port address into AL.
	///
	/// 'in AX,imm8;' Input word from imm8 I/O port address into AX.
	///
	/// 'in EAX,imm8;' Input dword from imm8 I/O port address into EAX.
	///
	/// 'in AL,DX;' Input byte from I/O port in DX into AL.
	///
	/// 'in AX,DX;' Input word from I/O port in DX into AX.
	///
	/// 'in EAX,DX;' Input doubleword from I/O port in DX into EAX.
	IN,
// INC--Increment by 1.
	///
	/// 'inc r/m8*;' Increment r/m byte by 1.
	///
	/// 'inc r/m8;' Increment r/m byte by 1.
	///
	/// 'inc r/m16;' Increment r/m word by 1.
	///
	/// 'inc r/m32;' Increment r/m doubleword by 1.
	///
	/// 'inc r/m64**;' Increment r/m quadword by 1.
	///
	/// 'inc r16;' Increment word register by 1.
	///
	/// 'inc r32;' Increment doubleword register by 1.
	INC,
// INS/INSB/INSW/INSD--Input from Port to String.
	///
	/// 'insd;' Input doubleword from I/O port specified in DX into memory location specified in ES:(E)DI or RDI.1.
	INSD,
	///
	/// 'insw;' Input word from I/O port specified in DX into memory location specified in ES:(E)DI or RDI.1.
	INSW,
	///
	/// 'ins m8,DX;' Input byte from I/O port specified in DX into memory location specified in ES:(E)DI or RDI.*.
	///
	/// 'ins m16,DX;' Input word from I/O port specified in DX into memory location specified in ES:(E)DI or RDI.1.
	///
	/// 'ins m32,DX;' Input doubleword from I/O port specified in DX into memory location specified in ES:(E)DI or RDI.1.
	INS,
	///
	/// 'insb;' Input byte from I/O port specified in DX into memory location specified with ES:(E)DI or RDI.1.
	INSB,
// INSERTPS--Insert Packed Single Precision Floating-Point Value.
	///
	/// 'insertps xmm1,xmm2/m32,imm8;' Insert a single precision floating-point value selected by imm8 from xmm2/m32 into xmm1 at the specified destination element specified by imm8 and zero out destination elements in xmm1 as indicated in imm8.
	INSERTPS,
	///
	/// 'vinsertps xmm1,xmm2,xmm3/m32,imm8;' Insert a single precision floating point value selected by imm8 from xmm3/m32 and merge into xmm2 at the specified destination element specified by imm8 and zero out destination elements in xmm1 as indicated in imm8.
	VINSERTPS,
// INTn/INTO/INT3--Call to Interrupt Procedure.
	///
	/// 'into;' Interrupt 4--if overflow flag is 1.
	INTO,
	///
	/// 'int 3;' Interrupt 3--trap to debugger.
	///
	/// 'int imm8;' Interrupt vector specified by immediate byte.
	INT,
// INVD--Invalidate Internal Caches.
	///
	/// 'invd;' Flush internal caches; initiate flushing of external caches.
	INVD,
// INVLPG--Invalidate TLB Entries.
	///
	/// 'invlpg m;' Invalidate TLB entries for page containing m.
	INVLPG,
// INVPCID--Invalidate Process-Context Identifier.
	///
	/// 'invpcid r32,m128;' Invalidates entries in the TLBs and paging-structure caches based on invalidation type in r32 and descriptor in m128.
	///
	/// 'invpcid r64,m128;' Invalidates entries in the TLBs and paging-structure caches based on invalidation type in r64 and descriptor in m128.
	INVPCID,
// IRET/IRETD--Interrupt Return.
	///
	/// 'iret;' Interrupt return (16-bit operand size).
	IRET,
	///
	/// 'iretd;' Interrupt return (32-bit operand size).
	IRETD,
	///
	/// 'iretq;' Interrupt return (64-bit operand size).
	IRETQ,
// Jcc--Jump if Condition Is Met.
	///
	/// 'jecxz rel8;' Jump short if ECX register is 0.
	JECXZ,
	///
	/// 'jnge rel8;' Jump short if not greater or equal (SF != OF).
	///
	/// 'jnge rel16;' Jump near if not greater or equal (SF != OF). Not supported in 64-bit mode.
	///
	/// 'jnge rel32;' Jump near if not greater or equal (SF != OF).
	JNGE,
	///
	/// 'jnle rel8;' Jump short if not less or equal (ZF=0 and SF=OF).
	///
	/// 'jnle rel16;' Jump near if not less or equal (ZF=0 and SF=OF). Not supported in 64-bit mode.
	///
	/// 'jnle rel32;' Jump near if not less or equal (ZF=0 and SF=OF).
	JNLE,
	///
	/// 'jle rel8;' Jump short if less or equal (ZF=1 or SF != OF).
	///
	/// 'jle rel16;' Jump near if less or equal (ZF=1 or SF != OF). Not supported in 64-bit mode.
	///
	/// 'jle rel32;' Jump near if less or equal (ZF=1 or SF != OF).
	JLE,
	///
	/// 'jns rel8;' Jump short if not sign (SF=0).
	///
	/// 'jns rel16;' Jump near if not sign (SF=0). Not supported in 64-bit mode.
	///
	/// 'jns rel32;' Jump near if not sign (SF=0).
	JNS,
	///
	/// 'je rel8;' Jump short if equal (ZF=1).
	///
	/// 'je rel16;' Jump near if equal (ZF=1). Not supported in 64-bit mode.
	///
	/// 'je rel32;' Jump near if equal (ZF=1).
	JE,
	///
	/// 'jng rel8;' Jump short if not greater (ZF=1 or SF != OF).
	///
	/// 'jng rel16;' Jump near if not greater (ZF=1 or SF != OF). Not supported in 64-bit mode.
	///
	/// 'jng rel32;' Jump near if not greater (ZF=1 or SF != OF).
	JNG,
	///
	/// 'jnc rel8;' Jump short if not carry (CF=0).
	///
	/// 'jnc rel16;' Jump near if not carry (CF=0). Not supported in 64-bit mode.
	///
	/// 'jnc rel32;' Jump near if not carry (CF=0).
	JNC,
	///
	/// 'jnl rel8;' Jump short if not less (SF=OF).
	///
	/// 'jnl rel16;' Jump near if not less (SF=OF). Not supported in 64-bit mode.
	///
	/// 'jnl rel32;' Jump near if not less (SF=OF).
	JNL,
	///
	/// 'jnb rel8;' Jump short if not below (CF=0).
	///
	/// 'jnb rel16;' Jump near if not below (CF=0). Not supported in 64-bit mode.
	///
	/// 'jnb rel32;' Jump near if not below (CF=0).
	JNB,
	///
	/// 'jne rel8;' Jump short if not equal (ZF=0).
	///
	/// 'jne rel16;' Jump near if not equal (ZF=0). Not supported in 64-bit mode.
	///
	/// 'jne rel32;' Jump near if not equal (ZF=0).
	JNE,
	///
	/// 'jge rel8;' Jump short if greater or equal (SF=OF).
	///
	/// 'jge rel16;' Jump near if greater or equal (SF=OF). Not supported in 64-bit mode.
	///
	/// 'jge rel32;' Jump near if greater or equal (SF=OF).
	JGE,
	///
	/// 'ja rel8;' Jump short if above (CF=0 and ZF=0).
	///
	/// 'ja rel16;' Jump near if above (CF=0 and ZF=0). Not supported in 64-bit mode.
	///
	/// 'ja rel32;' Jump near if above (CF=0 and ZF=0).
	JA,
	///
	/// 'jnbe rel8;' Jump short if not below or equal (CF=0 and ZF=0).
	///
	/// 'jnbe rel16;' Jump near if not below or equal (CF=0 and ZF=0). Not supported in 64-bit mode.
	///
	/// 'jnbe rel32;' Jump near if not below or equal (CF=0 and ZF=0).
	JNBE,
	///
	/// 'jrcxz rel8;' Jump short if RCX register is 0.
	JRCXZ,
	///
	/// 'jnz rel8;' Jump short if not zero (ZF=0).
	///
	/// 'jnz rel16;' Jump near if not zero (ZF=0). Not supported in 64-bit mode.
	///
	/// 'jnz rel32;' Jump near if not zero (ZF=0).
	JNZ,
	///
	/// 'jg rel8;' Jump short if greater (ZF=0 and SF=OF).
	///
	/// 'jg rel16;' Jump near if greater (ZF=0 and SF=OF). Not supported in 64-bit mode.
	///
	/// 'jg rel32;' Jump near if greater (ZF=0 and SF=OF).
	JG,
	///
	/// 'jo rel8;' Jump short if overflow (OF=1).
	///
	/// 'jo rel16;' Jump near if overflow (OF=1). Not supported in 64-bit mode.
	///
	/// 'jo rel32;' Jump near if overflow (OF=1).
	JO,
	///
	/// 'jpe rel8;' Jump short if parity even (PF=1).
	///
	/// 'jpe rel16;' Jump near if parity even (PF=1). Not supported in 64-bit mode.
	///
	/// 'jpe rel32;' Jump near if parity even (PF=1).
	JPE,
	///
	/// 'jc rel8;' Jump short if carry (CF=1).
	///
	/// 'jc rel16;' Jump near if carry (CF=1). Not supported in 64-bit mode.
	///
	/// 'jc rel32;' Jump near if carry (CF=1).
	JC,
	///
	/// 'jnae rel8;' Jump short if not above or equal (CF=1).
	///
	/// 'jnae rel16;' Jump near if not above or equal (CF=1). Not supported in 64-bit mode.
	///
	/// 'jnae rel32;' Jump near if not above or equal (CF=1).
	JNAE,
	///
	/// 'jnp rel8;' Jump short if not parity (PF=0).
	///
	/// 'jnp rel16;' Jump near if not parity (PF=0). Not supported in 64-bit mode.
	///
	/// 'jnp rel32;' Jump near if not parity (PF=0).
	JNP,
	///
	/// 'jl rel8;' Jump short if less (SF != OF).
	///
	/// 'jl rel16;' Jump near if less (SF != OF). Not supported in 64-bit mode.
	///
	/// 'jl rel32;' Jump near if less (SF != OF).
	JL,
	///
	/// 'jcxz rel8;' Jump short if CX register is 0.
	JCXZ,
	///
	/// 'jbe rel8;' Jump short if below or equal (CF=1 or ZF=1).
	///
	/// 'jbe rel16;' Jump near if below or equal (CF=1 or ZF=1). Not supported in 64-bit mode.
	///
	/// 'jbe rel32;' Jump near if below or equal (CF=1 or ZF=1).
	JBE,
	///
	/// 'jpo rel8;' Jump short if parity odd (PF=0).
	///
	/// 'jpo rel16;' Jump near if parity odd (PF=0). Not supported in 64-bit mode.
	///
	/// 'jpo rel32;' Jump near if parity odd (PF=0).
	JPO,
	///
	/// 'jna rel8;' Jump short if not above (CF=1 or ZF=1).
	///
	/// 'jna rel16;' Jump near if not above (CF=1 or ZF=1). Not supported in 64-bit mode.
	///
	/// 'jna rel32;' Jump near if not above (CF=1 or ZF=1).
	JNA,
	///
	/// 'jae rel8;' Jump short if above or equal (CF=0).
	///
	/// 'jae rel16;' Jump near if above or equal (CF=0). Not supported in 64-bit mode.
	///
	/// 'jae rel32;' Jump near if above or equal (CF=0).
	JAE,
	///
	/// 'jno rel8;' Jump short if not overflow (OF=0).
	///
	/// 'jno rel16;' Jump near if not overflow (OF=0). Not supported in 64-bit mode.
	///
	/// 'jno rel32;' Jump near if not overflow (OF=0).
	JNO,
	///
	/// 'jb rel8;' Jump short if below (CF=1).
	///
	/// 'jb rel16;' Jump near if below (CF=1). Not supported in 64-bit mode.
	///
	/// 'jb rel32;' Jump near if below (CF=1).
	JB,
	///
	/// 'js rel8;' Jump short if sign (SF=1).
	///
	/// 'js rel16;' Jump near if sign (SF=1). Not supported in 64.
	///
	/// 'js rel32;' Jump near if sign (SF=1).
	JS,
	///
	/// 'jp rel8;' Jump short if parity (PF=1).
	///
	/// 'jp rel16;' Jump near if parity (PF=1). Not supported in 64-bit mode.
	///
	/// 'jp rel32;' Jump near if parity (PF=1).
	JP,
	///
	/// 'jz rel8;' Jump short if zero (ZF = 1).
	///
	/// 'jz rel16;' Jump near if 0 (ZF=1). Not supported in 64-bit mode.
	///
	/// 'jz rel32;' Jump near if 0 (ZF=1).
	///
	/// 'jz rel16;' Jump near if 0 (ZF=1). Not supported in 64-bit mode.
	///
	/// 'jz rel32;' Jump near if 0 (ZF=1).
	JZ,
// JMP--Jump.
	///
	/// 'jmp rel8;' Jump short, RIP = RIP + 8-bit displacement sign extended to 64-bits.
	///
	/// 'jmp rel16;' Jump near, relative, displacement relative to next instruction. Not supported in 64-bit mode.
	///
	/// 'jmp rel32;' Jump near, relative, RIP = RIP + 32-bit displacement sign extended to 64-bits.
	///
	/// 'jmp r/m16;' Jump near, absolute indirect, address = zeroextended r/m16. Not supported in 64-bit mode.
	///
	/// 'jmp r/m32;' Jump near, absolute indirect, address given in r/m32. Not supported in 64-bit mode.
	///
	/// 'jmp r/m64;' Jump near, absolute indirect, RIP = 64-Bit offset from register or memory.
	///
	/// 'jmp ptr16:16;' Jump far, absolute, address given in operand.
	///
	/// 'jmp ptr16:32;' Jump far, absolute, address given in operand.
	///
	/// 'jmp m16:16;' Jump far, absolute indirect, address given in m16:16.
	///
	/// 'jmp m16:32;' Jump far, absolute indirect, address given in m16:32.
	///
	/// 'jmp m16:64;' Jump far, absolute indirect, address given in m16:64.
	JMP,
// LAHF--Load Status Flags into AH Register.
	///
	/// 'lahf;' Load: AH <-- EFLAGS(SF:ZF:0:AF:0:PF:1:CF).
	LAHF,
// LAR--Load Access Rights Byte.
	///
	/// 'lar r16,r16/m16;' r16 <-- access rights referenced by r16/m16.
	///
	/// 'lar reg,r32/m16 1;' reg <-- access rights referenced by r32/m16.
	LAR,
// LDDQU--Load Unaligned Integer 128 Bits.
	///
	/// 'vlddqu xmm1,m128;' Load unaligned packed integer values from mem to xmm1.
	///
	/// 'vlddqu ymm1,m256;' Load unaligned packed integer values from mem to ymm1.
	VLDDQU,
	///
	/// 'lddqu xmm1,mem;' Load unaligned data from mem and return double quadword in xmm1.
	LDDQU,
// LDMXCSR--Load MXCSR Register.
	///
	/// 'ldmxcsr m32;' Load MXCSR register from m32.
	LDMXCSR,
	///
	/// 'vldmxcsr m32;' Load MXCSR register from m32.
	VLDMXCSR,
// LDS/LES/LFS/LGS/LSS--Load Far Pointer.
	///
	/// 'lfs r16,m16:16;' Load FS:r16 with far pointer from memory.
	///
	/// 'lfs r32,m16:32;' Load FS:r32 with far pointer from memory.
	///
	/// 'lfs r64,m16:64;' Load FS:r64 with far pointer from memory.
	LFS,
	///
	/// 'lss r16,m16:16;' Load SS:r16 with far pointer from memory.
	///
	/// 'lss r32,m16:32;' Load SS:r32 with far pointer from memory.
	///
	/// 'lss r64,m16:64;' Load SS:r64 with far pointer from memory.
	LSS,
	///
	/// 'les r16,m16:16;' Load ES:r16 with far pointer from memory.
	///
	/// 'les r32,m16:32;' Load ES:r32 with far pointer from memory.
	LES,
	///
	/// 'lgs r16,m16:16;' Load GS:r16 with far pointer from memory.
	///
	/// 'lgs r32,m16:32;' Load GS:r32 with far pointer from memory.
	///
	/// 'lgs r64,m16:64;' Load GS:r64 with far pointer from memory.
	LGS,
	///
	/// 'lds r16,m16:16;' Load DS:r16 with far pointer from memory.
	///
	/// 'lds r32,m16:32;' Load DS:r32 with far pointer from memory.
	LDS,
// LEA--Load Effective Address.
	///
	/// 'lea r16,m;' Store effective address for m in register r16.
	///
	/// 'lea r32,m;' Store effective address for m in register r32.
	///
	/// 'lea r64,m;' Store effective address for m in register r64.
	LEA,
// LEAVE--High Level Procedure Exit.
	///
	/// 'leave;' Set SP to BP, then pop BP.
	///
	/// 'leave;' Set ESP to EBP, then pop EBP.
	///
	/// 'leave;' Set RSP to RBP, then pop RBP.
	LEAVE,
// LFENCE--Load Fence.
	///
	/// 'lfence;' Serializes load operations.
	LFENCE,
// LGDT/LIDT--Load Global/Interrupt Descriptor Table Register.
	///
	/// 'lgdt m16&32;' Load m into GDTR.
	///
	/// 'lgdt m16&64;' Load m into GDTR.
	LGDT,
	///
	/// 'lidt m16&32;' Load m into IDTR.
	///
	/// 'lidt m16&64;' Load m into IDTR.
	LIDT,
// LLDT--Load Local Descriptor Table Register.
	///
	/// 'lldt r/m16;' Load segment selector r/m16 into LDTR.
	LLDT,
// LMSW--Load Machine Status Word.
	///
	/// 'lmsw r/m16;' Loads r/m 16 in machine status word of CR0.
	LMSW,
// LOCK--Assert LOCK# Signal Prefix.
	///
	/// 'lock;'  signal for duration of the accompanying instruction.
	LOCK,
// LODS/LODSB/LODSW/LODSD/LODSQ--Load String.
	///
	/// 'lodsd;' For legacy mode, Load dword at address DS:(E)SI into EAX. For 64-bit mode load dword at address (R)SI into EAX.
	LODSD,
	///
	/// 'lods m8;' For legacy mode, Load byte at address DS:(E)SI into AL. For 64-bit mode load byte at address (R)SI into AL.
	///
	/// 'lods m16;' For legacy mode, Load word at address DS:(E)SI into AX. For 64-bit mode load word at address (R)SI into AX.
	///
	/// 'lods m32;' For legacy mode, Load dword at address DS:(E)SI into EAX. For 64-bit mode load dword at address (R)SI into EAX.
	///
	/// 'lods m64;' Load qword at address (R)SI into RAX.
	LODS,
	///
	/// 'lodsb;' For legacy mode, Load byte at address DS:(E)SI into AL. For 64-bit mode load byte at address (R)SI into AL.
	LODSB,
	///
	/// 'lodsq;' Load qword at address (R)SI into RAX.
	LODSQ,
	///
	/// 'lodsw;' For legacy mode, Load word at address DS:(E)SI into AX. For 64-bit mode load word at address (R)SI into AX.
	LODSW,
// LOOP/LOOPcc--Loop According to ECX Counter.
	///
	/// 'loop rel8;' Decrement count; jump short if count != 0.
	LOOP,
	///
	/// 'loopne rel8;' Decrement count; jump short if count != 0 and ZF = 0.
	LOOPNE,
	///
	/// 'loope rel8;' Decrement count; jump short if count != 0 and ZF = 1.
	LOOPE,
// LSL--Load Segment Limit.
	///
	/// 'lsl r16,r16/m16*;' Load: r16 <-- segment limit, selector r16/m16.
	///
	/// 'lsl r32,r32/m16*;' Load: r32 <-- segment limit, selector r32/m16.
	///
	/// 'lsl r64,r32/m16;' Load: r64 <-- segment limit, selector r32/m16.
	LSL,
// LTR--Load Task Register.
	///
	/// 'ltr r/m16;' Load r/m16 into task register.
	LTR,
// LZCNT--Count the Number of Leading Zero Bits.
	///
	/// 'lzcnt r16,r/m16;' Count the number of leading zero bits in r/m16, return result in r16.
	///
	/// 'lzcnt r32,r/m32;' Count the number of leading zero bits in r/m32, return result in r32.
	///
	/// 'lzcnt r64,r/m64;' Count the number of leading zero bits in r/m64, return result in r64.
	LZCNT,
// MASKMOVDQU--Store Selected Bytes of Double Quadword.
	///
	/// 'vmaskmovdqu xmm1,xmm2;' Selectively write bytes from xmm1 to memory location using the byte mask in xmm2. The default memory location is specified by DS:DI/EDI/RDI.
	VMASKMOVDQU,
	///
	/// 'maskmovdqu xmm1,xmm2;' Selectively write bytes from xmm1 to memory location using the byte mask in xmm2. The default memory location is specified by DS:DI/EDI/RDI.
	MASKMOVDQU,
// MASKMOVQ--Store Selected Bytes of Quadword.
	///
	/// 'maskmovq mm1,mm2;' Selectively write bytes from mm1 to memory location using the byte mask in mm2. The default memory location is specified by DS:DI/EDI/RDI.
	MASKMOVQ,
// MAXPD--Return Maximum Packed Double-Precision Floating-Point Values.
	///
	/// 'vmaxpd xmm1,xmm2,xmm3/m128;' Return the maximum double-precision floating-point values between xmm2 and xmm3/mem.
	///
	/// 'vmaxpd ymm1,ymm2,ymm3/m256;' Return the maximum packed double-precision floating-point values between ymm2 and ymm3/mem.
	VMAXPD,
	///
	/// 'maxpd xmm1,xmm2/m128;' Return the maximum double-precision floating-point values between xmm2/m128 and xmm1.
	MAXPD,
// MAXPS--Return Maximum Packed Single-Precision Floating-Point Values.
	///
	/// 'vmaxps xmm1,xmm2,xmm3/m128;' Return the maximum single-precision floatingpoint values between xmm2 and xmm3/mem.
	///
	/// 'vmaxps ymm1,ymm2,ymm3/m256;' Return the maximum single double-precision floating-point values between ymm2 and ymm3/mem.
	VMAXPS,
	///
	/// 'maxps xmm1,xmm2/m128;' Return the maximum single-precision floatingpoint values between xmm2/m128 and xmm1.
	MAXPS,
// MAXSD--Return Maximum Scalar Double-Precision Floating-Point Value.
	///
	/// 'vmaxsd xmm1,xmm2,xmm3/m64;' Return the maximum scalar double-precision floating-point value between xmm3/mem64 and xmm2.
	VMAXSD,
	///
	/// 'maxsd xmm1,xmm2/m64;' Return the maximum scalar double-precision floating-point value between xmm2/mem64 and xmm1.
	MAXSD,
// MAXSS--Return Maximum Scalar Single-Precision Floating-Point Value.
	///
	/// 'maxss xmm1,xmm2/m32;' Return the maximum scalar single-precision floating-point value between xmm2/mem32 and xmm1.
	MAXSS,
	///
	/// 'vmaxss xmm1,xmm2,xmm3/m32;' Return the maximum scalar single-precision floating-point value between xmm3/mem32 and xmm2.
	VMAXSS,
// MFENCE--Memory Fence.
	///
	/// 'mfence;' Serializes load and store operations.
	MFENCE,
// MINPD--Return Minimum Packed Double-Precision Floating-Point Values.
	///
	/// 'minpd xmm1,xmm2/m128;' Return the minimum double-precision floating-point values between xmm2/m128 and xmm1.
	MINPD,
	///
	/// 'vminpd xmm1,xmm2,xmm3/m128;' Return the minimum double-precision floatingpoint values between xmm2 and xmm3/mem.
	///
	/// 'vminpd ymm1,ymm2,ymm3/m256;' Return the minimum packed double-precision floating-point values between ymm2 and ymm3/mem.
	VMINPD,
// MINPS--Return Minimum Packed Single-Precision Floating-Point Values.
	///
	/// 'vminps xmm1,xmm2,xmm3/m128;' Return the minimum single-precision floatingpoint values between xmm2 and xmm3/mem.
	///
	/// 'vminps ymm1,ymm2,ymm3/m256;' Return the minimum single double-precision floating-point values between ymm2 and ymm3/mem.
	VMINPS,
	///
	/// 'minps xmm1,xmm2/m128;' Return the minimum single-precision floatingpoint values between xmm2/m128 and xmm1.
	MINPS,
// MINSD--Return Minimum Scalar Double-Precision Floating-Point Value.
	///
	/// 'minsd xmm1,xmm2/m64;' Return the minimum scalar double-precision floating-point value between xmm2/mem64 and xmm1.
	MINSD,
	///
	/// 'vminsd xmm1,xmm2,xmm3/m64;' Return the minimum scalar double precision floating-point value between xmm3/mem64 and xmm2.
	VMINSD,
// MINSS--Return Minimum Scalar Single-Precision Floating-Point Value.
	///
	/// 'minss xmm1,xmm2/m32;' Return the minimum scalar single-precision floating-point value between xmm2/mem32 and xmm1.
	MINSS,
	///
	/// 'vminss xmm1,xmm2,xmm3/m32;' Return the minimum scalar single precision floating-point value between xmm3/mem32 and xmm2.
	VMINSS,
// MONITOR--Set Up Monitor Address.
	///
	/// 'monitor;' Sets up a linear address range to be monitored by hardware and activates the monitor. The address range should be a writeback memory caching type. The address is DS:EAX (DS:RAX in 64-bit mode).
	MONITOR,
// MOV--Move.
	///
	/// 'mov r/m8,r8;' Move r8 to r/m8.
	///
	/// 'mov r/m8***,r8***;' Move r8 to r/m8.
	///
	/// 'mov r/m16,r16;' Move r16 to r/m16.
	///
	/// 'mov r/m32,r32;' Move r32 to r/m32.
	///
	/// 'mov r/m64,r64;' Move r64 to r/m64.
	///
	/// 'mov r8,r/m8;' Move r/m8 to r8.
	///
	/// 'mov r8***,r/m8***;' Move r/m8 to r8.
	///
	/// 'mov r16,r/m16;' Move r/m16 to r16.
	///
	/// 'mov r32,r/m32;' Move r/m32 to r32.
	///
	/// 'mov r64,r/m64;' Move r/m64 to r64.
	///
	/// 'mov r/m16,Sreg**;' Move segment register to r/m16.
	///
	/// 'mov r/m64,Sreg**;' Move zero extended 16-bit segment register to r/m64.
	///
	/// 'mov Sreg,r/m16**;' Move r/m16 to segment register.
	///
	/// 'mov Sreg,r/m64**;' Move lower 16 bits of r/m64 to segment register.
	///
	/// 'mov AL,moffs8*;' Move byte at (seg:offset) to AL.
	///
	/// 'mov AL,moffs8*;' Move byte at (offset) to AL.
	///
	/// 'mov AX,moffs16*;' Move word at (seg:offset) to AX.
	///
	/// 'mov EAX,moffs32*;' Move doubleword at (seg:offset) to EAX.
	///
	/// 'mov RAX,moffs64*;' Move quadword at (offset) to RAX.
	///
	/// 'mov moffs8,AL***;' Move AL to (seg:offset).
	///
	/// 'mov moffs8,AL;' Move AL to (offset).
	///
	/// 'mov moffs16*,AX;' Move AX to (seg:offset).
	///
	/// 'mov moffs32*,EAX;' Move EAX to (seg:offset).
	///
	/// 'mov moffs64*,RAX;' Move RAX to (offset).
	///
	/// 'mov r8,imm8***;' Move imm8 to r8.
	///
	/// 'mov r8,imm8;' Move imm8 to r8.
	///
	/// 'mov r16,imm16;' Move imm16 to r16.
	///
	/// 'mov r32,imm32;' Move imm32 to r32.
	///
	/// 'mov r64,imm64;' Move imm64 to r64.
	///
	/// 'mov r/m8,imm8;' Move imm8 to r/m8.
	///
	/// 'mov r/m8***,imm8;' Move imm8 to r/m8.
	///
	/// 'mov r/m16,imm16;' Move imm16 to r/m16.
	///
	/// 'mov r/m32,imm32;' Move imm32 to r/m32.
	///
	/// 'mov r/m64,imm32;' Move imm32 sign extended to 64-bits to r/m64.
	MOV,
// MOV--Move to/from Control Registers.
	///
	/// 'mov r32,CR0-CR7;' Move control register to r32.
	///
	/// 'mov r64,CR0-CR7;' Move extended control register to r64. 1.
	///
	/// 'mov r64,CR8;' Move extended CR8 to r64.
	///
	/// 'mov CR0-CR7,r32;' Move r32 to control register.
	///
	/// 'mov CR0-CR7,r64;' Move r64 to extended control register. 1.
	///
	/// 'mov CR8,r64;' Move r64 to extended CR8.
	MOV,
// MOV--Move to/from Debug Registers.
	///
	/// 'mov r32,DR0-DR7;' Move debug register to r32.
	///
	/// 'mov r64,DR0-DR7;' Move extended debug register to r64.
	///
	/// 'mov DR0-DR7,r32;' Move r32 to debug register.
	///
	/// 'mov DR0-DR7,r64;' Move r64 to extended debug register.
	MOV,
// MOVAPD--Move Aligned Packed Double-Precision Floating-Point Values.
	///
	/// 'vmovapd xmm1,xmm2/m128;' Move aligned packed double-precision floatingpoint values from xmm2/mem to xmm1.
	///
	/// 'vmovapd xmm2/m128,xmm1;' Move aligned packed double-precision floatingpoint values from xmm1 to xmm2/mem.
	///
	/// 'vmovapd ymm1,ymm2/m256;' Move aligned packed double-precision floatingpoint values from ymm2/mem to ymm1.
	///
	/// 'vmovapd ymm2/m256,ymm1;' Move aligned packed double-precision floatingpoint values from ymm1 to ymm2/mem.
	VMOVAPD,
	///
	/// 'movapd xmm1,xmm2/m128;' Move packed double-precision floating-point values from xmm2/m128 to xmm1.
	///
	/// 'movapd xmm2/m128,xmm1;' Move packed double-precision floating-point values from xmm1 to xmm2/m128.
	MOVAPD,
// MOVAPS--Move Aligned Packed Single-Precision Floating-Point Values.
	///
	/// 'movaps xmm1,xmm2/m128;' Move packed single-precision floating-point values from xmm2/m128 to xmm1.
	///
	/// 'movaps xmm2/m128,xmm1;' Move packed single-precision floating-point values from xmm1 to xmm2/m128.
	MOVAPS,
	///
	/// 'vmovaps xmm1,xmm2/m128;' Move aligned packed single-precision floatingpoint values from xmm2/mem to xmm1.
	///
	/// 'vmovaps xmm2/m128,xmm1;' Move aligned packed single-precision floatingpoint values from xmm1 to xmm2/mem.
	///
	/// 'vmovaps ymm1,ymm2/m256;' Move aligned packed single-precision floatingpoint values from ymm2/mem to ymm1.
	///
	/// 'vmovaps ymm2/m256,ymm1;' Move aligned packed single-precision floatingpoint values from ymm1 to ymm2/mem.
	VMOVAPS,
// MOVBE--Move Data After Swapping Bytes.
	///
	/// 'movbe r16,m16;' Reverse byte order in m16 and move to r16.
	///
	/// 'movbe r32,m32;' Reverse byte order in m32 and move to r32.
	///
	/// 'movbe r64,m64;' Reverse byte order in m64 and move to r64.
	///
	/// 'movbe m16,r16;' Reverse byte order in r16 and move to m16.
	///
	/// 'movbe m32,r32;' Reverse byte order in r32 and move to m32.
	///
	/// 'movbe m64,r64;' Reverse byte order in r64 and move to m64.
	MOVBE,
// MOVD/MOVQ--Move Doubleword/Move Quadword.
	///
	/// 'movd mm,r/m32;' Move doubleword from r/m32 to mm.
	///
	/// 'movd r/m32,mm;' Move doubleword from mm to r/m32.
	///
	/// 'movd xmm,r/m32;' Move doubleword from r/m32 to xmm.
	///
	/// 'movd r/m32,xmm;' Move doubleword from xmm register to r/m32.
	MOVD,
	///
	/// 'vmovq xmm1,r64/m64;' Move quadword from r/m64 to xmm1.
	///
	/// 'vmovq r64/m64,xmm1;' Move quadword from xmm1 register to r/m64.
	VMOVQ,
	///
	/// 'movq mm,r/m64;' Move quadword from r/m64 to mm.
	///
	/// 'movq r/m64,mm;' Move quadword from mm to r/m64.
	///
	/// 'movq xmm,r/m64;' Move quadword from r/m64 to xmm.
	///
	/// 'movq r/m64,xmm;' Move quadword from xmm register to r/m64.
	MOVQ,
	///
	/// 'vmovd xmm1,r32/m32;' Move doubleword from r/m32 to xmm1.
	///
	/// 'vmovd r32/m32,xmm1;' Move doubleword from xmm1 register to r/m32.
	VMOVD,
// MOVDDUP--Move One Double-FP and Duplicate.
	///
	/// 'vmovddup xmm1,xmm2/m64;' Move double-precision floating-point values from xmm2/mem and duplicate into xmm1.
	///
	/// 'vmovddup ymm1,ymm2/m256;' Move even index double-precision floatingpoint values from ymm2/mem and duplicate each element into ymm1.
	VMOVDDUP,
	///
	/// 'movddup xmm1,xmm2/m64;' Move one double-precision floating-point value from the lower 64-bit operand in xmm2/m64 to xmm1 and duplicate.
	MOVDDUP,
// MOVDQA--Move Aligned Double Quadword.
	///
	/// 'movdqa xmm1,xmm2/m128;' Move aligned double quadword from xmm2/m128 to xmm1.
	///
	/// 'movdqa xmm2/m128,xmm1;' Move aligned double quadword from xmm1 to xmm2/m128.
	MOVDQA,
	///
	/// 'vmovdqa xmm1,xmm2/m128;' Move aligned packed integer values from xmm2/mem to xmm1.
	///
	/// 'vmovdqa xmm2/m128,xmm1;' Move aligned packed integer values from xmm1 to xmm2/mem.
	///
	/// 'vmovdqa ymm1,ymm2/m256;' Move aligned packed integer values from ymm2/mem to ymm1.
	///
	/// 'vmovdqa ymm2/m256,ymm1;' Move aligned packed integer values from ymm1 to ymm2/mem.
	VMOVDQA,
// MOVDQU--Move Unaligned Double Quadword.
	///
	/// 'movdqu xmm1,xmm2/m128;' Move unaligned double quadword from xmm2/m128 to xmm1.
	///
	/// 'movdqu xmm2/m128,xmm1;' Move unaligned double quadword from xmm1 to xmm2/m128.
	MOVDQU,
	///
	/// 'vmovdqu xmm1,xmm2/m128;' Move unaligned packed integer values from xmm2/mem to xmm1.
	///
	/// 'vmovdqu xmm2/m128,xmm1;' Move unaligned packed integer values from xmm1 to xmm2/mem.
	///
	/// 'vmovdqu ymm1,ymm2/m256;' Move unaligned packed integer values from ymm2/mem to ymm1.
	///
	/// 'vmovdqu ymm2/m256,ymm1;' Move unaligned packed integer values from ymm1 to ymm2/mem.
	VMOVDQU,
// MOVDQ2Q--Move Quadword from XMM to MMX Technology Register.
	///
	/// 'movdq2q mm,xmm;' Move low quadword from xmm to mmx register.
	MOVDQ2Q,
// MOVHLPS--Move Packed Single-Precision Floating-Point Values High to Low.
	///
	/// 'vmovhlps xmm1,xmm2,xmm3;' Merge two packed single-precision floatingpoint values from high quadword of xmm3 and low quadword of xmm2.
	VMOVHLPS,
	///
	/// 'movhlps xmm1,xmm2;' Move two packed single-precision floatingpoint values from high quadword of xmm2 to low quadword of xmm1.
	MOVHLPS,
// MOVHPD--Move High Packed Double-Precision Floating-Point Value.
	///
	/// 'vmovhpd xmm2,xmm1,m64;' Merge double-precision floating-point value from m64 and the low quadword of xmm1.
	///
	/// 'vmovhpd m64,xmm1;' Move double-precision floating-point values from high quadword of xmm1 to m64.
	VMOVHPD,
	///
	/// 'movhpd xmm,m64;' Move double-precision floating-point value from m64 to high quadword of xmm.
	///
	/// 'movhpd m64,xmm;' Move double-precision floating-point value from high quadword of xmm to m64.
	MOVHPD,
// MOVHPS--Move High Packed Single-Precision Floating-Point Values.
	///
	/// 'vmovhps xmm2,xmm1,m64;' Merge two packed single-precision floatingpoint values from m64 and the low quadword of xmm1.
	///
	/// 'vmovhps m64,xmm1;' Move two packed single-precision floatingpoint values from high quadword of xmm1to m64.
	VMOVHPS,
	///
	/// 'movhps xmm,m64;' Move two packed single-precision floatingpoint values from m64 to high quadword of xmm.
	///
	/// 'movhps m64,xmm;' Move two packed single-precision floatingpoint values from high quadword of xmm to m64.
	MOVHPS,
// MOVLHPS--Move Packed Single-Precision Floating-Point Values Low to High.
	///
	/// 'movlhps xmm1,xmm2;' Move two packed single-precision floatingpoint values from low quadword of xmm2 to high quadword of xmm1.
	MOVLHPS,
	///
	/// 'vmovlhps xmm1,xmm2,xmm3;' Merge two packed single-precision floatingpoint values from low quadword of xmm3 and low quadword of xmm2.
	VMOVLHPS,
// MOVLPD--Move Low Packed Double-Precision Floating-Point Value.
	///
	/// 'vmovlpd xmm2,xmm1,m64;' Merge double-precision floating-point value from m64 and the high quadword of xmm1.
	///
	/// 'vmovlpd m64,xmm1;' Move double-precision floating-point values from low quadword of xmm1 to m64.
	VMOVLPD,
	///
	/// 'movlpd xmm,m64;' Move double-precision floating-point value from m64 to low quadword of xmm register.
	///
	/// 'movlpd m64,xmm;' Move double-precision floating-point nvalue from low quadword of xmm register to m64.
	MOVLPD,
// MOVLPS--Move Low Packed Single-Precision Floating-Point Values.
	///
	/// 'movlps xmm,m64;' Move two packed single-precision floatingpoint values from m64 to low quadword of xmm.
	///
	/// 'movlps m64,xmm;' Move two packed single-precision floatingpoint values from low quadword of xmm to m64.
	MOVLPS,
	///
	/// 'vmovlps xmm2,xmm1,m64;' Merge two packed single-precision floatingpoint values from m64 and the high quadword of xmm1.
	///
	/// 'vmovlps m64,xmm1;' Move two packed single-precision floatingpoint values from low quadword of xmm1 to m64.
	VMOVLPS,
// MOVMSKPD--Extract Packed Double-Precision Floating-Point Sign Mask.
	///
	/// 'vmovmskpd reg,xmm2;' Extract 2-bit sign mask from xmm2 and store in reg. The upper bits of r32 or r64 are zeroed.
	///
	/// 'vmovmskpd reg,ymm2;' Extract 4-bit sign mask from ymm2 and store in reg. The upper bits of r32 or r64 are zeroed.
	VMOVMSKPD,
	///
	/// 'movmskpd reg,xmm;' Extract 2-bit sign mask from xmm and store in reg. The upper bits of r32 or r64 are filled with zeros.
	MOVMSKPD,
// MOVMSKPS--Extract Packed Single-Precision Floating-Point Sign Mask.
	///
	/// 'vmovmskps reg,xmm2;' Extract 4-bit sign mask from xmm2 and store in reg. The upper bits of r32 or r64 are zeroed.
	///
	/// 'vmovmskps reg,ymm2;' Extract 8-bit sign mask from ymm2 and store in reg. The upper bits of r32 or r64 are zeroed.
	VMOVMSKPS,
	///
	/// 'movmskps reg,xmm;' Extract 4-bit sign mask from xmm and store in reg. The upper bits of r32 or r64 are filled with zeros.
	MOVMSKPS,
// MOVNTDQA--Load Double Quadword Non-Temporal Aligned Hint.
	///
	/// 'movntdqa xmm1,m128;' Move double quadword from m128 to xmm using non-temporal hint if WC memory type.
	MOVNTDQA,
	///
	/// 'vmovntdqa xmm1,m128;' Move double quadword from m128 to xmm using non-temporal hint if WC memory type.
	///
	/// 'vmovntdqa ymm1,m256;' Move 256-bit data from m256 to ymm using non-temporal hint if WC memory type.
	VMOVNTDQA,
// MOVNTDQ--Store Double Quadword Using Non-Temporal Hint.
	///
	/// 'movntdq m128,xmm;' Move double quadword from xmm to m128 using non-temporal hint.
	MOVNTDQ,
	///
	/// 'vmovntdq m128,xmm1;' Move packed integer values in xmm1 to m128 using non-temporal hint.
	///
	/// 'vmovntdq m256,ymm1;' Move packed integer values in ymm1 to m256 using non-temporal hint.
	VMOVNTDQ,
// MOVNTI--Store Doubleword Using Non-Temporal Hint.
	///
	/// 'movnti m32,r32;' Move doubleword from r32 to m32 using nontemporal hint.
	///
	/// 'movnti m64,r64;' Move quadword from r64 to m64 using nontemporal hint.
	MOVNTI,
// MOVNTPD--Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint.
	///
	/// 'movntpd m128,xmm;' Move packed double-precision floating-point values from xmm to m128 using nontemporal hint.
	MOVNTPD,
	///
	/// 'vmovntpd m128,xmm1;' Move packed double-precision values in xmm1 to m128 using non-temporal hint.
	///
	/// 'vmovntpd m256,ymm1;' Move packed double-precision values in ymm1 to m256 using non-temporal hint.
	VMOVNTPD,
// MOVNTPS--Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint.
	///
	/// 'vmovntps m128,xmm1;' Move packed single-precision values xmm1 to mem using non-temporal hint.
	///
	/// 'vmovntps m256,ymm1;' Move packed single-precision values ymm1 to mem using non-temporal hint.
	VMOVNTPS,
	///
	/// 'movntps m128,xmm;' Move packed single-precision floating-point values from xmm to m128 using nontemporal hint.
	MOVNTPS,
// MOVNTQ--Store of Quadword Using Non-Temporal Hint.
	///
	/// 'movntq m64,mm;' Move quadword from mm to m64 using nontemporal hint.
	MOVNTQ,
// MOVQ--Move Quadword.
	///
	/// 'movq mm,mm/m64;' Move quadword from mm/m64 to mm.
	///
	/// 'movq mm/m64,mm;' Move quadword from mm to mm/m64.
	///
	/// 'movq xmm1,xmm2/m64;' Move quadword from xmm2/mem64 to xmm1.
	///
	/// 'movq xmm2/m64,xmm1;' Move quadword from xmm1 to xmm2/mem64.
	MOVQ,
	///
	/// 'vmovq xmm1,xmm2;' Move quadword from xmm2 to xmm1.
	///
	/// 'vmovq xmm1,m64;' Load quadword from m64 to xmm1.
	///
	/// 'vmovq xmm1/m64,xmm2;' Move quadword from xmm2 register to xmm1/m64.
	VMOVQ,
// MOVQ2DQ--Move Quadword from MMX Technology to XMM Register.
	///
	/// 'movq2dq xmm,mm;' Move quadword from mmx to low quadword of xmm.
	MOVQ2DQ,
// MOVS/MOVSB/MOVSW/MOVSD/MOVSQ--Move Data from String to String \.
	///
	/// 'movsw;' For legacy mode, move word from address DS:(E)SI to ES:(E)DI. For 64-bit mode move word at address (R|E)SI to (R|E)DI.
	MOVSW,
	///
	/// 'movsd;' For legacy mode, move dword from address DS:(E)SI to ES:(E)DI. For 64-bit mode move dword from address (R|E)SI to (R|E)DI.
	MOVSD,
	///
	/// 'movs m8,m8;' For legacy mode, Move byte from address DS:(E)SI to ES:(E)DI. For 64-bit mode move byte from address (R|E)SI to (R|E)DI.
	///
	/// 'movs m16,m16;' For legacy mode, move word from address DS:(E)SI to ES:(E)DI. For 64-bit mode move word at address (R|E)SI to (R|E)DI.
	///
	/// 'movs m32,m32;' For legacy mode, move dword from address DS:(E)SI to ES:(E)DI. For 64-bit mode move dword from address (R|E)SI to (R|E)DI.
	///
	/// 'movs m64,m64;' Move qword from address (R|E)SI to (R|E)DI.
	MOVS,
	///
	/// 'movsb;' For legacy mode, Move byte from address DS:(E)SI to ES:(E)DI. For 64-bit mode move byte from address (R|E)SI to (R|E)DI.
	MOVSB,
	///
	/// 'movsq;' Move qword from address (R|E)SI to (R|E)DI.
	MOVSQ,
// MOVSD--Move Scalar Double-Precision Floating-Point Value.
	///
	/// 'vmovsd xmm1,xmm2,xmm3;' Merge scalar double-precision floating-point value from xmm2 and xmm3 to xmm1 register.
	///
	/// 'vmovsd xmm1,m64;' Load scalar double-precision floating-point value from m64 to xmm1 register.
	///
	/// 'vmovsd xmm1,xmm2,xmm3;' Merge scalar double-precision floating-point value from xmm2 and xmm3 registers to xmm1.
	///
	/// 'vmovsd m64,xmm1;' Move scalar double-precision floating-point value from xmm1 register to m64.
	VMOVSD,
	///
	/// 'movsd xmm1,xmm2/m64;' Move scalar double-precision floating-point value from xmm2/m64 to xmm1 register.
	///
	/// 'movsd xmm2/m64,xmm1;' Move scalar double-precision floating-point value from xmm1 register to xmm2/m64.
	MOVSD,
// MOVSHDUP--Move Packed Single-FP High and Duplicate.
	///
	/// 'movshdup xmm1,xmm2/m128;' Move two single-precision floating-point values from the higher 32-bit operand of each qword in xmm2/m128 to xmm1 and duplicate each 32-bit operand to the lower 32-bits of each qword.
	MOVSHDUP,
	///
	/// 'vmovshdup xmm1,xmm2/m128;' Move odd index single-precision floating-point values from xmm2/mem and duplicate each element into xmm1.
	///
	/// 'vmovshdup ymm1,ymm2/m256;' Move odd index single-precision floating-point values from ymm2/mem and duplicate each element into ymm1.
	VMOVSHDUP,
// MOVSLDUP--Move Packed Single-FP Low and Duplicate.
	///
	/// 'vmovsldup xmm1,xmm2/m128;' Move even index single-precision floatingpoint values from xmm2/mem and duplicate each element into xmm1.
	///
	/// 'vmovsldup ymm1,ymm2/m256;' Move even index single-precision floatingpoint values from ymm2/mem and duplicate each element into ymm1.
	VMOVSLDUP,
	///
	/// 'movsldup xmm1,xmm2/m128;' Move two single-precision floating-point values from the lower 32-bit operand of each qword in xmm2/m128 to xmm1 and duplicate each 32-bit operand to the higher 32-bits of each qword.
	MOVSLDUP,
// MOVSS--Move Scalar Single-Precision Floating-Point Values.
	///
	/// 'movss xmm1,xmm2/m32;' Move scalar single-precision floating-point value from xmm2/m32 to xmm1 register.
	///
	/// 'movss xmm2/m32,xmm;' Move scalar single-precision floating-point value from xmm1 register to xmm2/m32.
	MOVSS,
	///
	/// 'vmovss xmm1,xmm2,xmm3;' Merge scalar single-precision floating-point value from xmm2 and xmm3 to xmm1 register.
	///
	/// 'vmovss xmm1,m32;' Load scalar single-precision floating-point value from m32 to xmm1 register.
	///
	/// 'vmovss xmm1,xmm2,xmm3;' Move scalar single-precision floating-point value from xmm2 and xmm3 to xmm1 register.
	///
	/// 'vmovss m32,xmm1;' Move scalar single-precision floating-point value from xmm1 register to m32.
	VMOVSS,
// MOVSX/MOVSXD--Move with Sign-Extension.
	///
	/// 'movsx r16,r/m8;' Move byte to word with sign-extension.
	///
	/// 'movsx r32,r/m8;' Move byte to doubleword with signextension.
	///
	/// 'movsx r64,r/m8*;' Move byte to quadword with sign-extension.
	///
	/// 'movsx r32,r/m16;' Move word to doubleword, with signextension.
	///
	/// 'movsx r64,r/m16;' Move word to quadword with sign-extension.
	MOVSX,
	///
	/// 'movsxd r64,r/m32;' Move doubleword to quadword with signextension.
	MOVSXD,
// MOVUPD--Move Unaligned Packed Double-Precision Floating-Point Values.
	///
	/// 'vmovupd xmm1,xmm2/m128;' Move unaligned packed double-precision floating-point from xmm2/mem to xmm1.
	///
	/// 'vmovupd ymm1,ymm2/m256;' Move unaligned packed double-precision floating-point from ymm2/mem to ymm1.
	///
	/// 'vmovupd xmm2/m128,xmm1;' Move unaligned packed double-precision floating-point from xmm1 to xmm2/mem.
	///
	/// 'vmovupd ymm2/m256,ymm1;' Move unaligned packed double-precision floating-point from ymm1 to ymm2/mem.
	VMOVUPD,
	///
	/// 'movupd xmm1,xmm2/m128;' Move packed double-precision floating-point values from xmm2/m128 to xmm1.
	///
	/// 'movupd xmm2/m128,xmm;' Move packed double-precision floating-point values from xmm1 to xmm2/m128.
	MOVUPD,
// MOVUPS--Move Unaligned Packed Single-Precision Floating-Point Values.
	///
	/// 'movups xmm1,xmm2/m128;' Move packed single-precision floating-point values from xmm2/m128 to xmm1.
	///
	/// 'movups xmm2/m128,xmm1;' Move packed single-precision floating-point values from xmm1 to xmm2/m128.
	MOVUPS,
	///
	/// 'vmovups xmm1,xmm2/m128;' Move unaligned packed single-precision floating-point from xmm2/mem to xmm1.
	///
	/// 'vmovups ymm1,ymm2/m256;' Move unaligned packed single-precision floating-point from ymm2/mem to ymm1.
	///
	/// 'vmovups xmm2/m128,xmm1;' Move unaligned packed single-precision floating-point from xmm1 to xmm2/mem.
	///
	/// 'vmovups ymm2/m256,ymm1;' Move unaligned packed single-precision floating-point from ymm1 to ymm2/mem.
	VMOVUPS,
// MOVZX--Move with Zero-Extend.
	///
	/// 'movzx r16,r/m8;' Move byte to word with zero-extension.
	///
	/// 'movzx r32,r/m8;' Move byte to doubleword, zero-extension.
	///
	/// 'movzx r64,r/m8*;' Move byte to quadword, zero-extension.
	///
	/// 'movzx r32,r/m16;' Move word to doubleword, zero-extension.
	///
	/// 'movzx r64,r/m16;' Move word to quadword, zero-extension.
	MOVZX,
// MPSADBW--Compute Multiple Packed Sums of Absolute Difference.
	///
	/// 'vmpsadbw xmm1,xmm2,xmm3/m128,imm8;' Sums absolute 8-bit integer difference of adjacent groups of 4 byte integers in xmm2 and xmm3/m128 and writes the results in xmm1. Starting offsets within xmm2 and xmm3/m128 are determined by imm8.
	///
	/// 'vmpsadbw ymm1,ymm2,ymm3/m256,imm8;' Sums absolute 8-bit integer difference of adjacent groups of 4 byte integers in xmm2 and ymm3/m128 and writes the results in ymm1. Starting offsets within ymm2 and xmm3/m128 are determined by imm8.
	VMPSADBW,
	///
	/// 'mpsadbw xmm1,xmm2/m128,imm8;' Sums absolute 8-bit integer difference of adjacent groups of 4 byte integers in xmm1 and xmm2/m128 and writes the results in xmm1. Starting offsets within xmm1 and xmm2/m128 are determined by imm8.
	MPSADBW,
// MUL--Unsigned Multiply.
	///
	/// 'mul r/m8*;' Unsigned multiply (AX <-- AL * r/m8).
	///
	/// 'mul r/m8;' Unsigned multiply (AX <-- AL * r/m8).
	///
	/// 'mul r/m16;' Unsigned multiply (DX:AX <-- AX * r/m16).
	///
	/// 'mul r/m32;' Unsigned multiply (EDX:EAX <-- EAX * r/m32).
	///
	/// 'mul r/m64;' Unsigned multiply (RDX:RAX <-- RAX * r/m64).
	MUL,
// MULPD--Multiply Packed Double-Precision Floating-Point Values.
	///
	/// 'vmulpd xmm1,xmm2,xmm3/m128;' Multiply packed double-precision floating-point values from xmm3/mem to xmm2 and stores result in xmm1.
	///
	/// 'vmulpd ymm1,ymm2,ymm3/m256;' Multiply packed double-precision floating-point values from ymm3/mem to ymm2 and stores result in ymm1.
	VMULPD,
	///
	/// 'mulpd xmm1,xmm2/m128;' Multiply packed double-precision floating-point values in xmm2/m128 by xmm1.
	MULPD,
// MULPS--Multiply Packed Single-Precision Floating-Point Values.
	///
	/// 'mulps xmm1,xmm2/m128;' Multiply packed single-precision floating-point values in xmm2/mem by xmm1.
	MULPS,
	///
	/// 'vmulps xmm1,xmm2,xmm3/m128;' Multiply packed single-precision floating-point values from xmm3/mem to xmm2 and stores result in xmm1.
	///
	/// 'vmulps ymm1,ymm2,ymm3/m256;' Multiply packed single-precision floating-point values from ymm3/mem to ymm2 and stores result in ymm1.
	VMULPS,
// MULSD--Multiply Scalar Double-Precision Floating-Point Values.
	///
	/// 'vmulsd xmm1,xmm2,xmm3/m64;' Multiply the low double-precision floatingpoint value in xmm3/mem64 by low double precision floating-point value in xmm2.
	VMULSD,
	///
	/// 'mulsd xmm1,xmm2/m64;' Multiply the low double-precision floatingpoint value in xmm2/mem64 by low doubleprecision floating-point value in xmm1.
	MULSD,
// MULSS--Multiply Scalar Single-Precision Floating-Point Values.
	///
	/// 'vmulss xmm1,xmm2,xmm3/m32;' Multiply the low single-precision floating-point value in xmm3/mem by the low singleprecision floating-point value in xmm2.
	VMULSS,
	///
	/// 'mulss xmm1,xmm2/m32;' Multiply the low single-precision floating-point value in xmm2/mem by the low singleprecision floating-point value in xmm1.
	MULSS,
// MULX--Unsigned Multiply Without Affecting Flags.
	///
	/// 'mulx r32a,r32b,r/m32;' Unsigned multiply of r/m32 with EDX without affecting arithmetic flags.
	///
	/// 'mulx r64a,r64b,r/m64;' Unsigned multiply of r/m64 with RDX without affecting arithmetic flags.
	MULX,
// MWAIT--Monitor Wait.
	///
	/// 'mwait;' A hint that allow the processor to stop instruction execution and enter an implementation-dependent optimized state until occurrence of a class of events.
	MWAIT,
// NEG--Two's Complement Negation.
	///
	/// 'neg r/m8;' Two's complement negate r/m8.
	///
	/// 'neg r/m8*;' Two's complement negate r/m8.
	///
	/// 'neg r/m16;' Two's complement negate r/m16.
	///
	/// 'neg r/m32;' Two's complement negate r/m32.
	///
	/// 'neg r/m64;' Two's complement negate r/m64.
	NEG,
// NOP--No Operation.
	///
	/// 'nop;' One byte no-operation instruction.
	///
	/// 'nop r/m16;' Multi-byte no-operation instruction.
	///
	/// 'nop r/m32;' Multi-byte no-operation instruction.
	NOP,
// NOT--One's Complement Negation.
	///
	/// 'not r/m8;' Reverse each bit of r/m8.
	///
	/// 'not r/m8*;' Reverse each bit of r/m8.
	///
	/// 'not r/m16;' Reverse each bit of r/m16.
	///
	/// 'not r/m32;' Reverse each bit of r/m32.
	///
	/// 'not r/m64;' Reverse each bit of r/m64.
	NOT,
// OR--Logical Inclusive OR.
	///
	/// 'or AL,imm8;' AL OR imm8.
	///
	/// 'or AX,imm16;' AX OR imm16.
	///
	/// 'or EAX,imm32;' EAX OR imm32.
	///
	/// 'or RAX,imm32;' RAX OR imm32 (sign-extended).
	///
	/// 'or r/m8,imm8;' r/m8 OR imm8.
	///
	/// 'or r/m8*,imm8;' r/m8 OR imm8.
	///
	/// 'or r/m16,imm16;' r/m16 OR imm16.
	///
	/// 'or r/m32,imm32;' r/m32 OR imm32.
	///
	/// 'or r/m64,imm32;' r/m64 OR imm32 (sign-extended).
	///
	/// 'or r/m16,imm8;' r/m16 OR imm8 (sign-extended).
	///
	/// 'or r/m32,imm8;' r/m32 OR imm8 (sign-extended).
	///
	/// 'or r/m64,imm8;' r/m64 OR imm8 (sign-extended).
	///
	/// 'or r/m8,r8;' r/m8 OR r8.
	///
	/// 'or r/m8*,r8*;' r/m8 OR r8.
	///
	/// 'or r/m16,r16;' r/m16 OR r16.
	///
	/// 'or r/m32,r32;' r/m32 OR r32.
	///
	/// 'or r/m64,r64;' r/m64 OR r64.
	///
	/// 'or r8,r/m8;' r8 OR r/m8.
	///
	/// 'or r8*,r/m8*;' r8 OR r/m8.
	///
	/// 'or r16,r/m16;' r16 OR r/m16.
	///
	/// 'or r32,r/m32;' r32 OR r/m32.
	///
	/// 'or r64,r/m64;' r64 OR r/m64.
	OR,
// ORPD--Bitwise Logical OR of Double-Precision Floating-Point Values.
	///
	/// 'orpd xmm1,xmm2/m128;' Bitwise OR of xmm2/m128 and xmm1.
	ORPD,
	///
	/// 'vorpd xmm1,xmm2,xmm3/m128;' Return the bitwise logical OR of packed double-precision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vorpd ymm1,ymm2,ymm3/m256;' Return the bitwise logical OR of packed double-precision floating-point values in ymm2 and ymm3/mem.
	VORPD,
// ORPS--Bitwise Logical OR of Single-Precision Floating-Point Values.
	///
	/// 'vorps xmm1,xmm2,xmm3/m128;' Return the bitwise logical OR of packed singleprecision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vorps ymm1,ymm2,ymm3/m256;' Return the bitwise logical OR of packed singleprecision floating-point values in ymm2 and ymm3/mem.
	VORPS,
	///
	/// 'orps xmm1,xmm2/m128;' Bitwise OR of xmm1 and xmm2/m128.
	ORPS,
// OUT--Output to Port.
	///
	/// 'out imm8,AL;' Output byte in AL to I/O port address imm8.
	///
	/// 'out imm8,AX;' Output word in AX to I/O port address imm8.
	///
	/// 'out imm8,EAX;' Output doubleword in EAX to I/O port address imm8.
	///
	/// 'out DX,AL;' Output byte in AL to I/O port address in DX.
	///
	/// 'out DX,AX;' Output word in AX to I/O port address in DX.
	///
	/// 'out DX,EAX;' Output doubleword in EAX to I/O port address in DX.
	OUT,
// OUTS/OUTSB/OUTSW/OUTSD--Output String to Port.
	///
	/// 'outsd;' Output doubleword from memory location specified in DS:(E)SI or RSI to I/O port specified in DX**.
	OUTSD,
	///
	/// 'outsw;' Output word from memory location specified in DS:(E)SI or RSI to I/O port specified in DX**.
	OUTSW,
	///
	/// 'outsb;' Output byte from memory location specified in DS:(E)SI or RSI to I/O port specified in DX**.
	OUTSB,
	///
	/// 'outs DX,m8;' Output byte from memory location specified in DS:(E)SI or RSI to I/O port specified in DX**.
	///
	/// 'outs DX,m16;' Output word from memory location specified in DS:(E)SI or RSI to I/O port specified in DX**.
	///
	/// 'outs DX,m32;' Output doubleword from memory location specified in DS:(E)SI or RSI to I/O port specified in DX**.
	OUTS,
// PABSB/PABSW/PABSD--Packed Absolute Value.
	///
	/// 'pabsw mm1,mm2/m64;' Compute the absolute value of 16-bit integers in mm2/m64 and store UNSIGNED result in mm1.
	///
	/// 'pabsw xmm1,xmm2/m128;' Compute the absolute value of 16-bit integers in xmm2/m128 and store UNSIGNED result in xmm1.
	PABSW,
	///
	/// 'vpabsw xmm1,xmm2/m128;' Compute the absolute value of 16bit integers in xmm2/m128 and store UNSIGNED result in xmm1.
	///
	/// 'vpabsw ymm1,ymm2/m256;' Compute the absolute value of 16-bit integers in ymm2/m256 and store UNSIGNED result in ymm1.
	VPABSW,
	///
	/// 'pabsb mm1,mm2/m64;' Compute the absolute value of bytes in mm2/m64 and store UNSIGNED result in mm1.
	///
	/// 'pabsb xmm1,xmm2/m128;' Compute the absolute value of bytes in xmm2/m128 and store UNSIGNED result in xmm1.
	PABSB,
	///
	/// 'vpabsd xmm1,xmm2/m128;' Compute the absolute value of 32bit integers in xmm2/m128 and store UNSIGNED result in xmm1.
	///
	/// 'vpabsd ymm1,ymm2/m256;' Compute the absolute value of 32-bit integers in ymm2/m256 and store UNSIGNED result in ymm1.
	VPABSD,
	///
	/// 'vpabsb xmm1,xmm2/m128;' Compute the absolute value of bytes in xmm2/m128 and store UNSIGNED result in xmm1.
	///
	/// 'vpabsb ymm1,ymm2/m256;' Compute the absolute value of bytes in ymm2/m256 and store UNSIGNED result in ymm1.
	VPABSB,
	///
	/// 'pabsd mm1,mm2/m64;' Compute the absolute value of 32-bit integers in mm2/m64 and store UNSIGNED result in mm1.
	///
	/// 'pabsd xmm1,xmm2/m128;' Compute the absolute value of 32-bit integers in xmm2/m128 and store UNSIGNED result in xmm1.
	PABSD,
// PACKSSWB/PACKSSDW--Pack with Signed Saturation.
	///
	/// 'packssdw mm1,mm2/m64;' Converts 2 packed signed doubleword integers from mm1 and from mm2/m64 into 4 packed signed word integers in mm1 using signed saturation.
	///
	/// 'packssdw xmm1,xmm2/m128;' Converts 4 packed signed doubleword integers from xmm1 and from xxm2/m128 into 8 packed signed word integers in xxm1 using signed saturation.
	PACKSSDW,
	///
	/// 'packsswb mm1,mm2/m64;' Converts 4 packed signed word integers from mm1 and from mm2/m64 into 8 packed signed byte integers in mm1 using signed saturation.
	///
	/// 'packsswb xmm1,xmm2/m128;' Converts 8 packed signed word integers from xmm1 and from xxm2/m128 into 16 packed signed byte integers in xxm1 using signed saturation.
	PACKSSWB,
	///
	/// 'vpackssdw xmm1,xmm2,xmm3/m128;' Converts 4 packed signed doubleword integers from xmm2 and from xmm3/m128 into 8 packed signed word integers in xmm1 using signed saturation.
	///
	/// 'vpackssdw ymm1,ymm2,ymm3/m256;' Converts 8 packed signed doubleword integers from ymm2 and from ymm3/m256 into 16 packed signed word integers in ymm1using signed saturation.
	VPACKSSDW,
	///
	/// 'vpacksswb xmm1,xmm2,xmm3/m128;' Converts 8 packed signed word integers from xmm2 and from xmm3/m128 into 16 packed signed byte integers in xmm1 using signed saturation.
	///
	/// 'vpacksswb ymm1,ymm2,ymm3/m256;' Converts 16 packed signed word integers from ymm2 and from ymm3/m256 into 32 packed signed byte integers in ymm1 using signed saturation.
	VPACKSSWB,
// PACKUSDW--Pack with Unsigned Saturation.
	///
	/// 'vpackusdw xmm1,xmm2,xmm3/m128;' Convert 4 packed signed doubleword integers from xmm2 and 4 packed signed doubleword integers from xmm3/m128 into 8 packed unsigned word integers in xmm1 using unsigned saturation.
	///
	/// 'vpackusdw ymm1,ymm2,ymm3/m256;' Convert 8 packed signed doubleword integers from ymm2 and 8 packed signed doubleword integers from ymm3/m128 into 16 packed unsigned word integers in ymm1 using unsigned saturation.
	VPACKUSDW,
	///
	/// 'packusdw xmm1,xmm2/m128;' Convert 4 packed signed doubleword integers from xmm1 and 4 packed signed doubleword integers from xmm2/m128 into 8 packed unsigned word integers in xmm1 using unsigned saturation.
	PACKUSDW,
// PACKUSWB--Pack with Unsigned Saturation.
	///
	/// 'vpackuswb xmm1,xmm2,xmm3/m128;' Converts 8 signed word integers from xmm2 and 8 signed word integers from xmm3/m128 into 16 unsigned byte integers in xmm1 using unsigned saturation.
	///
	/// 'vpackuswb ymm1,ymm2,ymm3/m256;' Converts 16 signed word integers from ymm2 and 16signed word integers from ymm3/m256 into 32 unsigned byte integers in ymm1 using unsigned saturation.
	VPACKUSWB,
	///
	/// 'packuswb mm,mm/m64;' Converts 4 signed word integers from mm and 4 signed word integers from mm/m64 into 8 unsigned byte integers in mm using unsigned saturation.
	///
	/// 'packuswb xmm1,xmm2/m128;' Converts 8 signed word integers from xmm1 and 8 signed word integers from xmm2/m128 into 16 unsigned byte integers in xmm1 using unsigned saturation.
	PACKUSWB,
// PADDB/PADDW/PADDD--Add Packed Integers.
	///
	/// 'vpaddw xmm1,xmm2,xmm3/m128;' Add packed word integers from xmm3/m128 and xmm2.
	///
	/// 'vpaddw ymm1,ymm2,ymm3/m256;' Add packed word integers from ymm2, ymm3/m256 and store in ymm1.
	VPADDW,
	///
	/// 'paddd mm,mm/m64;' Add packed doubleword integers from mm/m64 and mm.
	///
	/// 'paddd xmm1,xmm2/m128;' Add packed doubleword integers from xmm2/m128 and xmm1.
	PADDD,
	///
	/// 'paddb mm,mm/m64;' Add packed byte integers from mm/m64 and mm.
	///
	/// 'paddb xmm1,xmm2/m128;' Add packed byte integers from xmm2/m128 and xmm1.
	PADDB,
	///
	/// 'vpaddd xmm1,xmm2,xmm3/m128;' Add packed doubleword integers from xmm3/m128 and xmm2.
	///
	/// 'vpaddd ymm1,ymm2,ymm3/m256;' Add packed doubleword integers from ymm2, ymm3/m256 and store in ymm1.
	VPADDD,
	///
	/// 'vpaddb xmm1,xmm2,xmm3/m128;' Add packed byte integers from xmm3/m128 and xmm2.
	///
	/// 'vpaddb ymm1,ymm2,ymm3/m256;' Add packed byte integers from ymm2, and ymm3/m256 and store in ymm1.
	VPADDB,
	///
	/// 'paddw mm,mm/m64;' Add packed word integers from mm/m64 and mm.
	///
	/// 'paddw xmm1,xmm2/m128;' Add packed word integers from xmm2/m128 and xmm1.
	PADDW,
// PADDQ--Add Packed Quadword Integers.
	///
	/// 'paddq mm1,mm2/m64;' Add quadword integer mm2/m64 to mm1.
	///
	/// 'paddq xmm1,xmm2/m128;' Add packed quadword integers xmm2/m128 to xmm1.
	PADDQ,
	///
	/// 'vpaddq xmm1,xmm2,xmm3/m128;' Add packed quadword integers xmm3/m128 and xmm2.
	///
	/// 'vpaddq ymm1,ymm2,ymm3/m256;' Add packed quadword integers from ymm2, ymm3/m256 and store in ymm1.
	VPADDQ,
// PADDSB/PADDSW--Add Packed Signed Integers with Signed Saturation.
	///
	/// 'paddsb mm,mm/m64;' Add packed signed byte integers from mm/m64 and mm and saturate the results.
	///
	/// 'paddsb xmm1,xmm2/m128;' Add packed signed byte integers from xmm2/m128 and xmm1 saturate the results.
	PADDSB,
	///
	/// 'paddsw mm,mm/m64;' Add packed signed word integers from mm/m64 and mm and saturate the results.
	///
	/// 'paddsw xmm1,xmm2/m128;' Add packed signed word integers from xmm2/m128 and xmm1 and saturate the results.
	PADDSW,
	///
	/// 'vpaddsb xmm1,xmm2,xmm3/m128;' Add packed signed byte integers from xmm3/m128 and xmm2 saturate the results.
	///
	/// 'vpaddsb ymm1,ymm2,ymm3/m256;' Add packed signed byte integers from ymm2, and ymm3/m256 and store the saturated results in ymm1.
	VPADDSB,
	///
	/// 'vpaddsw xmm1,xmm2,xmm3/m128;' Add packed signed word integers from xmm3/m128 and xmm2 and saturate the results.
	///
	/// 'vpaddsw ymm1,ymm2,ymm3/m256;' Add packed signed word integers from ymm2, and ymm3/m256 and store the saturated results in ymm1.
	VPADDSW,
// PADDUSB/PADDUSW--Add Packed Unsigned Integers with Unsigned Saturation.
	///
	/// 'paddusw mm,mm/m64;' Add packed unsigned word integers from mm/m64 and mm and saturate the results.
	///
	/// 'paddusw xmm1,xmm2/m128;' Add packed unsigned word integers from xmm2/m128 to xmm1 and saturate the results.
	PADDUSW,
	///
	/// 'paddusb mm,mm/m64;' Add packed unsigned byte integers from mm/m64 and mm and saturate the results.
	///
	/// 'paddusb xmm1,xmm2/m128;' Add packed unsigned byte integers from xmm2/m128 and xmm1 saturate the results.
	PADDUSB,
	///
	/// 'vpaddusb xmm1,xmm2,xmm3/m128;' Add packed unsigned byte integers from xmm3/m128 to xmm2 and saturate the results.
	///
	/// 'vpaddusb ymm1,ymm2,ymm3/m256;' Add packed unsigned byte integers from ymm2, and ymm3/m256 and store the saturated results in ymm1.
	VPADDUSB,
	///
	/// 'vpaddusw xmm1,xmm2,xmm3/m128;' Add packed unsigned word integers from xmm3/m128 to xmm2 and saturate the results.
	///
	/// 'vpaddusw ymm1,ymm2,ymm3/m256;' Add packed unsigned word integers from ymm2, and ymm3/m256 and store the saturated results in ymm1.
	VPADDUSW,
// PALIGNR--Packed Align Right.
	///
	/// 'vpalignr xmm1,xmm2,xmm3/m128,imm8;' Concatenate xmm2 and xmm3/m128, extract byte aligned result shifted to the right by constant value in imm8 and result is stored in xmm1.
	///
	/// 'vpalignr ymm1,ymm2,ymm3/m256,imm8;' Concatenate pairs of 16 bytes in ymm2 and ymm3/m256 into 32-byte intermediate result, extract byte-aligned, 16-byte result shifted to the right by constant values in imm8 from each intermediate result, and two 16-byte results are stored in ymm1.
	VPALIGNR,
	///
	/// 'palignr mm1,mm2/m64,imm8;' Concatenate destination and source operands, extract byte-aligned result shifted to the right by constant value in imm8 into mm1.
	///
	/// 'palignr xmm1,xmm2/m128,imm8;' Concatenate destination and source operands, extract byte-aligned result shifted to the right by constant value in imm8 into xmm1.
	PALIGNR,
// PAND--Logical AND.
	///
	/// 'vpand xmm1,xmm2,xmm3/m128;' Bitwise AND of xmm3/m128 and xmm.
	///
	/// 'vpand ymm1,ymm2,ymm3/.m256;' Bitwise AND of ymm2, and ymm3/m256 and store result in ymm1.
	VPAND,
	///
	/// 'pand mm,mm/m64;' Bitwise AND mm/m64 and mm.
	///
	/// 'pand xmm1,xmm2/m128;' Bitwise AND of xmm2/m128 and xmm1.
	PAND,
// PANDN--Logical AND NOT.
	///
	/// 'pandn mm,mm/m64;' Bitwise AND NOT of mm/m64 and mm.
	///
	/// 'pandn xmm1,xmm2/m128;' Bitwise AND NOT of xmm2/m128 and xmm1.
	PANDN,
	///
	/// 'vpandn xmm1,xmm2,xmm3/m128;' Bitwise AND NOT of xmm3/m128 and xmm2.
	///
	/// 'vpandn ymm1,ymm2,ymm3/m256;' Bitwise AND NOT of ymm2, and ymm3/m256 and store result in ymm1.
	VPANDN,
// PAUSE--Spin Loop Hint.
	///
	/// 'pause;' Gives hint to processor that improves performance of spin-wait loops.
	PAUSE,
// PAVGB/PAVGW--Average Packed Integers.
	///
	/// 'vpavgw xmm1,xmm2,xmm3/m128;' Average packed unsigned word integers from xmm3/m128 and xmm2 with rounding.
	///
	/// 'vpavgw ymm1,ymm2,ymm3/m256;' Average packed unsigned word integers from ymm2, ymm3/m256 with rounding to ymm1.
	VPAVGW,
	///
	/// 'vpavgb xmm1,xmm2,xmm3/m128;' Average packed unsigned byte integers from xmm3/m128 and xmm2 with rounding.
	///
	/// 'vpavgb ymm1,ymm2,ymm3/m256;' Average packed unsigned byte integers from ymm2, and ymm3/m256 with rounding and store to ymm1.
	VPAVGB,
	///
	/// 'pavgw mm1,mm2/m64;' Average packed unsigned word integers from mm2/m64 and mm1 with rounding.
	///
	/// 'pavgw xmm1,xmm2/m128;' Average packed unsigned word integers from xmm2/m128 and xmm1 with rounding.
	PAVGW,
	///
	/// 'pavgb mm1,mm2/m64;' Average packed unsigned byte integers from mm2/m64 and mm1 with rounding.
	///
	/// 'pavgb xmm1,xmm2/m128;' Average packed unsigned byte integers from xmm2/m128 and xmm1 with rounding.
	PAVGB,
// PBLENDVB--Variable Blend Packed Bytes.
	///
	/// 'vpblendvb xmm1,xmm2,xmm3/m128,xmm4;' Select byte values from xmm2 and xmm3/m128 using mask bits in the specified mask register, xmm4, and store the values into xmm1.
	///
	/// 'vpblendvb ymm1,ymm2,ymm3/m256,ymm4;' Select byte values from ymm2 and ymm3/m256 from mask specified in the high values into ymm1.
	VPBLENDVB,
	///
	/// 'pblendvb xmm1,xmm2/m128,<XMM0>;' Select byte values from xmm1 and xmm2/m128 from mask specified in the high values into xmm1.
	PBLENDVB,
// PBLENDW--Blend Packed Words.
	///
	/// 'pblendw xmm1,xmm2/m128,imm8;' Select words from xmm1 and xmm2/m128 from mask specified in imm8 and store the values into xmm1.
	PBLENDW,
	///
	/// 'vpblendw xmm1,xmm2,xmm3/m128,imm8;' Select words from xmm2 and xmm3/m128 from mask specified in imm8 and store the values into xmm1.
	///
	/// 'vpblendw ymm1,ymm2,ymm3/m256,imm8;' Select words from ymm2 and ymm3/m256 from mask specified in imm8 and store the values into ymm1.
	VPBLENDW,
// PCLMULQDQ--Carry-Less Multiplication Quadword.
	///
	/// 'pclmulqdq xmm1,xmm2/m128,imm8;' Carry-less multiplication of one quadword of xmm1 by one quadword of xmm2/m128, stores the 128-bit result in xmm1. The immediate is used to determine which quadwords of xmm1 and xmm2/m128 should be used.
	PCLMULQDQ,
	///
	/// 'vpclmulqdq xmm1,xmm2,xmm3/m128,imm8;' Carry-less multiplication of one quadword of xmm2 by one quadword of xmm3/m128, stores the 128-bit result in xmm1. The immediate is used to determine which quadwords of xmm2 and xmm3/m128 should be used.
	VPCLMULQDQ,
// PCMPEQB/PCMPEQW/PCMPEQD--Compare Packed Data for Equal.
	///
	/// 'pcmpeqw mm,mm/m64;' Compare packed words in mm/m64 and mm for equality.
	///
	/// 'pcmpeqw xmm1,xmm2/m128;' Compare packed words in xmm2/m128 and xmm1 for equality.
	PCMPEQW,
	///
	/// 'vpcmpeqb xmm1,xmm2,xmm3/m128;' Compare packed bytes in xmm3/m128 and xmm2 for equality.
	///
	/// 'vpcmpeqb ymm1,ymm2,ymm3 /m256;' Compare packed bytes in ymm3/m256 and ymm2 for equality.
	VPCMPEQB,
	///
	/// 'pcmpeqd mm,mm/m64;' Compare packed doublewords in mm/m64 and mm for equality.
	///
	/// 'pcmpeqd xmm1,xmm2/m128;' Compare packed doublewords in xmm2/m128 and xmm1 for equality.
	PCMPEQD,
	///
	/// 'vpcmpeqw xmm1,xmm2,xmm3/m128;' Compare packed words in xmm3/m128 and xmm2 for equality.
	///
	/// 'vpcmpeqw ymm1,ymm2,ymm3 /m256;' Compare packed words in ymm3/m256 and ymm2 for equality.
	VPCMPEQW,
	///
	/// 'pcmpeqb mm,mm/m64;' Compare packed bytes in mm/m64 and mm for equality.
	///
	/// 'pcmpeqb xmm1,xmm2/m128;' Compare packed bytes in xmm2/m128 and xmm1 for equality.
	PCMPEQB,
	///
	/// 'vpcmpeqd xmm1,xmm2,xmm3/m128;' Compare packed doublewords in xmm3/m128 and xmm2 for equality.
	///
	/// 'vpcmpeqd ymm1,ymm2,ymm3 /m256;' Compare packed doublewords in ymm3/m256 and ymm2 for equality.
	VPCMPEQD,
// PCMPEQQ--Compare Packed Qword Data for Equal.
	///
	/// 'pcmpeqq xmm1,xmm2/m128;' Compare packed qwords in xmm2/m128 and xmm1 for equality.
	PCMPEQQ,
	///
	/// 'vpcmpeqq xmm1,xmm2,xmm3/m128;' Compare packed quadwords in xmm3/m128 and xmm2 for equality.
	///
	/// 'vpcmpeqq ymm1,ymm2,ymm3 /m256;' Compare packed quadwords in ymm3/m256 and ymm2 for equality.
	VPCMPEQQ,
// PCMPESTRI--Packed Compare Explicit Length Strings, Return Index.
	///
	/// 'vpcmpestri xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with explicit lengths, generating an index, and storing the result in ECX.
	VPCMPESTRI,
	///
	/// 'pcmpestri xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with explicit lengths, generating an index, and storing the result in ECX.
	PCMPESTRI,
// PCMPESTRM--Packed Compare Explicit Length Strings, Return Mask.
	///
	/// 'pcmpestrm xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with explicit lengths, generating a mask, and storing the result in XMM0.
	PCMPESTRM,
	///
	/// 'vpcmpestrm xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with explicit lengths, generating a mask, and storing the result in XMM0.
	VPCMPESTRM,
// PCMPGTB/PCMPGTW/PCMPGTD--Compare Packed Signed Integers for Greater Than.
	///
	/// 'pcmpgtw mm,mm/m64;' Compare packed signed word integers in mm and mm/m64 for greater than.
	///
	/// 'pcmpgtw xmm1,xmm2/m128;' Compare packed signed word integers in xmm1 and xmm2/m128 for greater than.
	PCMPGTW,
	///
	/// 'pcmpgtb mm,mm/m64;' Compare packed signed byte integers in mm and mm/m64 for greater than.
	///
	/// 'pcmpgtb xmm1,xmm2/m128;' Compare packed signed byte integers in xmm1 and xmm2/m128 for greater than.
	PCMPGTB,
	///
	/// 'vpcmpgtd xmm1,xmm2,xmm3/m128;' Compare packed signed doubleword integers in xmm2 and xmm3/m128 for greater than.
	///
	/// 'vpcmpgtd ymm1,ymm2,ymm3/m256;' Compare packed signed doubleword integers in ymm2 and ymm3/m256 for greater than.
	VPCMPGTD,
	///
	/// 'vpcmpgtw xmm1,xmm2,xmm3/m128;' Compare packed signed word integers in xmm2 and xmm3/m128 for greater than.
	///
	/// 'vpcmpgtw ymm1,ymm2,ymm3/m256;' Compare packed signed word integers in ymm2 and ymm3/m256 for greater than.
	VPCMPGTW,
	///
	/// 'pcmpgtd mm,mm/m64;' Compare packed signed doubleword integers in mm and mm/m64 for greater than.
	///
	/// 'pcmpgtd xmm1,xmm2/m128;' Compare packed signed doubleword integers in xmm1 and xmm2/m128 for greater than.
	PCMPGTD,
	///
	/// 'vpcmpgtb xmm1,xmm2,xmm3/m128;' Compare packed signed byte integers in xmm2 and xmm3/m128 for greater than.
	///
	/// 'vpcmpgtb ymm1,ymm2,ymm3/m256;' Compare packed signed byte integers in ymm2 and ymm3/m256 for greater than.
	VPCMPGTB,
// PCMPGTQ--Compare Packed Data for Greater Than.
	///
	/// 'vpcmpgtq xmm1,xmm2,xmm3/m128;' Compare packed signed qwords in xmm2 and xmm3/m128 for greater than.
	///
	/// 'vpcmpgtq ymm1,ymm2,ymm3/m256;' Compare packed signed qwords in ymm2 and ymm3/m256 for greater than.
	VPCMPGTQ,
	///
	/// 'pcmpgtq xmm1,xmm2/m128;' Compare packed signed qwords in xmm2/m128 and xmm1 for greater than.
	PCMPGTQ,
// PCMPISTRI--Packed Compare Implicit Length Strings, Return Index.
	///
	/// 'pcmpistri xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with implicit lengths, generating an index, and storing the result in ECX.
	PCMPISTRI,
	///
	/// 'vpcmpistri xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with implicit lengths, generating an index, and storing the result in ECX.
	VPCMPISTRI,
// PCMPISTRM--Packed Compare Implicit Length Strings, Return Mask.
	///
	/// 'pcmpistrm xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with implicit lengths, generating a mask, and storing the result in XMM0.
	PCMPISTRM,
	///
	/// 'vpcmpistrm xmm1,xmm2/m128,imm8;' Perform a packed comparison of string data with implicit lengths, generating a Mask, and storing the result in XMM0.
	VPCMPISTRM,
// PDEP--Parallel Bits Deposit.
	///
	/// 'pdep r32a,r32b,r/m32;' Parallel deposit of bits from r32b using mask in r/m32, result is written to r32a.
	///
	/// 'pdep r64a,r64b,r/m64;' Parallel deposit of bits from r64b using mask in r/m64, result is written to r64a.
	PDEP,
// PEXT--Parallel Bits Extract.
	///
	/// 'pext r32a,r32b,r/m32;' Parallel extract of bits from r32b using mask in r/m32, result is written to r32a.
	///
	/// 'pext r64a,r64b,r/m64;' Parallel extract of bits from r64b using mask in r/m64, result is written to r64a.
	PEXT,
// PEXTRB/PEXTRD/PEXTRQ--Extract Byte/Dword/Qword.
	///
	/// 'pextrd r/m32,xmm2,imm8;' Extract a dword integer value from xmm2 at the source dword offset specified by imm8 into r/m32.
	PEXTRD,
	///
	/// 'vpextrq r64/m64,xmm2,imm8;' Extract a qword integer value from xmm2 at the source dword offset specified by imm8 into r64/m64.
	VPEXTRQ,
	///
	/// 'pextrb reg/m8,xmm2,imm8;' Extract a byte integer value from xmm2 at the source byte offset specified by imm8 into reg or m8. The upper bits of r32 or r64 are zeroed.
	PEXTRB,
	///
	/// 'pextrq r/m64,xmm2,imm8;' Extract a qword integer value from xmm2 at the source qword offset specified by imm8 into r/m64.
	PEXTRQ,
	///
	/// 'vpextrd r32/m32,xmm2,imm8;' Extract a dword integer value from xmm2 at the source dword offset specified by imm8 into r32/m32.
	VPEXTRD,
	///
	/// 'vpextrb reg/m8,xmm2,imm8;' Extract a byte integer value from xmm2 at the source byte offset specified by imm8 into reg or m8. The upper bits of r64/r32 is filled with zeros.
	VPEXTRB,
// PEXTRW--Extract Word.
	///
	/// 'pextrw reg,mm,imm8;' Extract the word specified by imm8 from mm and move it to reg, bits 15-0. The upper bits of r32 or r64 is zeroed.
	///
	/// 'pextrw reg,xmm,imm8;' Extract the word specified by imm8 from xmm and move it to reg, bits 15-0. The upper bits of r32 or r64 is zeroed.
	///
	/// 'pextrw reg/m16,xmm,imm8;' Extract the word specified by imm8 from xmm and copy it to lowest 16 bits of reg or m16. Zero-extend the result in the destination, r32 or r64.
	PEXTRW,
	///
	/// 'vpextrw reg,xmm1,imm8;' Extract the word specified by imm8 from xmm1 and move it to reg, bits 15:0. Zeroextend the result. The upper bits of r64/r32 is filled with zeros.
	///
	/// 'vpextrw reg/m16,xmm2,imm8;' Extract a word integer value from xmm2 at the source word offset specified by imm8 into reg or m16. The upper bits of r64/r32 is filled with zeros.
	VPEXTRW,
// PHADDW/PHADDD--Packed Horizontal Add.
	///
	/// 'phaddd mm1,mm2/m64;' Add 32-bit integers horizontally, pack to mm1.
	///
	/// 'phaddd xmm1,xmm2/m128;' Add 32-bit integers horizontally, pack to xmm1.
	PHADDD,
	///
	/// 'vphaddd xmm1,xmm2,xmm3/m128;' Add 32-bit integers horizontally, pack to xmm1.
	///
	/// 'vphaddd ymm1,ymm2,ymm3/m256;' Add 32-bit signed integers horizontally, pack to ymm1.
	VPHADDD,
	///
	/// 'phaddw mm1,mm2/m64;' Add 16-bit integers horizontally, pack to mm1.
	///
	/// 'phaddw xmm1,xmm2/m128;' Add 16-bit integers horizontally, pack to xmm1.
	PHADDW,
	///
	/// 'vphaddw xmm1,xmm2,xmm3/m128;' Add 16-bit integers horizontally, pack to xmm1.
	///
	/// 'vphaddw ymm1,ymm2,ymm3/m256;' Add 16-bit signed integers horizontally, pack to ymm1.
	VPHADDW,
// PHADDSW--Packed Horizontal Add and Saturate.
	///
	/// 'phaddsw mm1,mm2/m64;' Add 16-bit signed integers horizontally, pack saturated integers to mm1.
	///
	/// 'phaddsw xmm1,xmm2/m128;' Add 16-bit signed integers horizontally, pack saturated integers to xmm1.
	PHADDSW,
	///
	/// 'vphaddsw xmm1,xmm2,xmm3/m128;' Add 16-bit signed integers horizontally, pack saturated integers to xmm1.
	///
	/// 'vphaddsw ymm1,ymm2,ymm3/m256;' Add 16-bit signed integers horizontally, pack saturated integers to ymm1.
	VPHADDSW,
// PHMINPOSUW--Packed Horizontal Word Minimum.
	///
	/// 'vphminposuw xmm1,xmm2/m128;' Find the minimum unsigned word in xmm2/m128 and place its value in the low word of xmm1 and its index in the secondlowest word of xmm1.
	VPHMINPOSUW,
	///
	/// 'phminposuw xmm1,xmm2/m128;' Find the minimum unsigned word in xmm2/m128 and place its value in the low word of xmm1 and its index in the secondlowest word of xmm1.
	PHMINPOSUW,
// PHSUBW/PHSUBD--Packed Horizontal Subtract.
	///
	/// 'vphsubd xmm1,xmm2,xmm3/m128;' Subtract 32-bit signed integers horizontally, pack to xmm1.
	///
	/// 'vphsubd ymm1,ymm2,ymm3/m256;' Subtract 32-bit signed integers horizontally, pack to ymm1.
	VPHSUBD,
	///
	/// 'phsubd mm1,mm2/m64;' Subtract 32-bit signed integers horizontally, pack to mm1.
	///
	/// 'phsubd xmm1,xmm2/m128;' Subtract 32-bit signed integers horizontally, pack to xmm1.
	PHSUBD,
	///
	/// 'vphsubw xmm1,xmm2,xmm3/m128;' Subtract 16-bit signed integers horizontally, pack to xmm1.
	///
	/// 'vphsubw ymm1,ymm2,ymm3/m256;' Subtract 16-bit signed integers horizontally, pack to ymm1.
	VPHSUBW,
	///
	/// 'phsubw mm1,mm2/m64;' Subtract 16-bit signed integers horizontally, pack to mm1.
	///
	/// 'phsubw xmm1,xmm2/m128;' Subtract 16-bit signed integers horizontally, pack to xmm1.
	PHSUBW,
// PHSUBSW--Packed Horizontal Subtract and Saturate.
	///
	/// 'vphsubsw xmm1,xmm2,xmm3/m128;' Subtract 16-bit signed integer horizontally, pack saturated integers to xmm1.
	///
	/// 'vphsubsw ymm1,ymm2,ymm3/m256;' Subtract 16-bit signed integer horizontally, pack saturated integers to ymm1.
	VPHSUBSW,
	///
	/// 'phsubsw mm1,mm2/m64;' Subtract 16-bit signed integer horizontally, pack saturated integers to mm1.
	///
	/// 'phsubsw xmm1,xmm2/m128;' Subtract 16-bit signed integer horizontally, pack saturated integers to xmm1.
	PHSUBSW,
// PINSRB/PINSRD/PINSRQ--Insert Byte/Dword/Qword.
	///
	/// 'pinsrq xmm1,r/m64,imm8;' Insert a qword integer value from r/m64 into the xmm1 at the destination element specified by imm8.
	PINSRQ,
	///
	/// 'pinsrb xmm1,r32/m8,imm8;' Insert a byte integer value from r32/m8 into xmm1 at the destination element in xmm1 specified by imm8.
	PINSRB,
	///
	/// 'pinsrd xmm1,r/m32,imm8;' Insert a dword integer value from r/m32 into the xmm1 at the destination element specified by imm8.
	PINSRD,
	///
	/// 'vpinsrb xmm1,xmm2,r32/m8,imm8;' Merge a byte integer value from r32/m8 and rest from xmm2 into xmm1 at the byte offset in imm8.
	VPINSRB,
	///
	/// 'vpinsrd xmm1,xmm2,r/m32,imm8;' Insert a dword integer value from r32/m32 and rest from xmm2 into xmm1 at the dword offset in imm8.
	VPINSRD,
// PINSRW--Insert Word.
	///
	/// 'pinsrw mm,r32/m16,imm8;' Insert the low word from r32 or from m16 into mm at the word position specified by imm8.
	///
	/// 'pinsrw xmm,r32/m16,imm8;' Move the low word of r32 or from m16 into xmm at the word position specified by imm8.
	PINSRW,
	///
	/// 'vpinsrw xmm1,xmm2,r32/m16,imm8;' Insert a word integer value from r32/m16 and rest from xmm2 into xmm1 at the word offset in imm8.
	VPINSRW,
// PMADDUBSW--Multiply and Add Packed Signed and Unsigned Bytes.
	///
	/// 'pmaddubsw mm1,mm2/m64;' Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to mm1.
	///
	/// 'pmaddubsw xmm1,xmm2/m128;' Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to xmm1.
	PMADDUBSW,
	///
	/// 'vpmaddubsw xmm1,xmm2,xmm3/m128;' Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to xmm1.
	///
	/// 'vpmaddubsw ymm1,ymm2,ymm3/m256;' Multiply signed and unsigned bytes, add horizontal pair of signed words, pack saturated signed-words to ymm1.
	VPMADDUBSW,
// PMADDWD--Multiply and Add Packed Integers.
	///
	/// 'pmaddwd mm,mm/m64;' Multiply the packed words in mm by the packed words in mm/m64, add adjacent doubleword results, and store in mm.
	///
	/// 'pmaddwd xmm1,xmm2/m128;' Multiply the packed word integers in xmm1 by the packed word integers in xmm2/m128, add adjacent doubleword results, and store in xmm1.
	PMADDWD,
	///
	/// 'vpmaddwd xmm1,xmm2,xmm3/m128;' Multiply the packed word integers in xmm2 by the packed word integers in xmm3/m128, add adjacent doubleword results, and store in xmm1.
	///
	/// 'vpmaddwd ymm1,ymm2,ymm3/m256;' Multiply the packed word integers in ymm2 by the packed word integers in ymm3/m256, add adjacent doubleword results, and store in ymm1.
	VPMADDWD,
// PMAXSB--Maximum of Packed Signed Byte Integers.
	///
	/// 'vpmaxsb xmm1,xmm2,xmm3/m128;' Compare packed signed byte integers in xmm2 and xmm3/m128 and store packed maximum values in xmm1.
	///
	/// 'vpmaxsb ymm1,ymm2,ymm3/m256;' Compare packed signed byte integers in ymm2 and ymm3/m128 and store packed maximum values in ymm1.
	VPMAXSB,
	///
	/// 'pmaxsb xmm1,xmm2/m128;' Compare packed signed byte integers in xmm1 and xmm2/m128 and store packed maximum values in xmm1.
	PMAXSB,
// PMAXSD--Maximum of Packed Signed Dword Integers.
	///
	/// 'pmaxsd xmm1,xmm2/m128;' Compare packed signed dword integers in xmm1 and xmm2/m128 and store packed maximum values in xmm1.
	PMAXSD,
	///
	/// 'vpmaxsd xmm1,xmm2,xmm3/m128;' Compare packed signed dword integers in xmm2 and xmm3/m128 and store packed maximum values in xmm1.
	///
	/// 'vpmaxsd ymm1,ymm2,ymm3/m256;' Compare packed signed dword integers in ymm2 and ymm3/m128 and store packed maximum values in ymm1.
	VPMAXSD,
// PMAXSW--Maximum of Packed Signed Word Integers.
	///
	/// 'pmaxsw mm1,mm2/m64;' Compare signed word integers in mm2/m64 and mm1 and return maximum values.
	///
	/// 'pmaxsw xmm1,xmm2/m128;' Compare signed word integers in xmm2/m128 and xmm1 and return maximum values.
	PMAXSW,
	///
	/// 'vpmaxsw xmm1,xmm2,xmm3/m128;' Compare packed signed word integers in xmm3/m128 and xmm2 and store packed maximum values in xmm1.
	///
	/// 'vpmaxsw ymm1,ymm2,ymm3/m256;' Compare packed signed word integers in ymm3/m128 and ymm2 and store packed maximum values in ymm1.
	VPMAXSW,
// PMAXUB--Maximum of Packed Unsigned Byte Integers.
	///
	/// 'pmaxub mm1,mm2/m64;' Compare unsigned byte integers in mm2/m64 and mm1 and returns maximum values.
	///
	/// 'pmaxub xmm1,xmm2/m128;' Compare unsigned byte integers in xmm2/m128 and xmm1 and returns maximum values.
	PMAXUB,
	///
	/// 'vpmaxub xmm1,xmm2,xmm3/m128;' Compare packed unsigned byte integers in xmm2 and xmm3/m128 and store packed maximum values in xmm1.
	///
	/// 'vpmaxub ymm1,ymm2,ymm3/m256;' Compare packed unsigned byte integers in ymm2 and ymm3/m256 and store packed maximum values in ymm1.
	VPMAXUB,
// PMAXUD--Maximum of Packed Unsigned Dword Integers.
	///
	/// 'pmaxud xmm1,xmm2/m128;' Compare packed unsigned dword integers in xmm1 and xmm2/m128 and store packed maximum values in xmm1.
	PMAXUD,
	///
	/// 'vpmaxud xmm1,xmm2,xmm3/m128;' Compare packed unsigned dword integers in xmm2 and xmm3/m128 and store packed maximum values in xmm1.
	///
	/// 'vpmaxud ymm1,ymm2,ymm3/m256;' Compare packed unsigned dword integers in ymm2 and ymm3/m256 and store packed maximum values in ymm1.
	VPMAXUD,
// PMAXUW--Maximum of Packed Word Integers.
	///
	/// 'vpmaxuw xmm1,xmm2,xmm3/m128;' Compare packed unsigned word integers in xmm3/m128 and xmm2 and store maximum packed values in xmm1.
	///
	/// 'vpmaxuw ymm1,ymm2,ymm3/m256;' Compare packed unsigned word integers in ymm3/m256 and ymm2 and store maximum packed values in ymm1.
	VPMAXUW,
	///
	/// 'pmaxuw xmm1,xmm2/m128;' Compare packed unsigned word integers in xmm1 and xmm2/m128 and store packed maximum values in xmm1.
	PMAXUW,
// PMINSB--Minimum of Packed Signed Byte Integers.
	///
	/// 'pminsb xmm1,xmm2/m128;' Compare packed signed byte integers in xmm1 and xmm2/m128 and store packed minimum values in xmm1.
	PMINSB,
	///
	/// 'vpminsb xmm1,xmm2,xmm3/m128;' Compare packed signed byte integers in xmm2 and xmm3/m128 and store packed minimum values in xmm1.
	///
	/// 'vpminsb ymm1,ymm2,ymm3/m256;' Compare packed signed byte integers in ymm2 and ymm3/m256 and store packed minimum values in ymm1.
	VPMINSB,
// PMINSD--Minimum of Packed Dword Integers.
	///
	/// 'vpminsd xmm1,xmm2,xmm3/m128;' Compare packed signed dword integers in xmm2 and xmm3/m128 and store packed minimum values in xmm1.
	///
	/// 'vpminsd ymm1,ymm2,ymm3/m256;' Compare packed signed dword integers in ymm2 and ymm3/m128 and store packed minimum values in ymm1.
	VPMINSD,
	///
	/// 'pminsd xmm1,xmm2/m128;' Compare packed signed dword integers in xmm1 and xmm2/m128 and store packed minimum values in xmm1.
	PMINSD,
// PMINSW--Minimum of Packed Signed Word Integers.
	///
	/// 'vpminsw xmm1,xmm2,xmm3/m128;' Compare packed signed word integers in xmm3/m128 and xmm2 and return packed minimum values in xmm1.
	///
	/// 'vpminsw ymm1,ymm2,ymm3/m256;' Compare packed signed word integers in ymm3/m256 and ymm2 and return packed minimum values in ymm1.
	VPMINSW,
	///
	/// 'pminsw mm1,mm2/m64;' Compare signed word integers in mm2/m64 and mm1 and return minimum values.
	///
	/// 'pminsw xmm1,xmm2/m128;' Compare signed word integers in xmm2/m128 and xmm1 and return minimum values.
	PMINSW,
// PMINUB--Minimum of Packed Unsigned Byte Integers.
	///
	/// 'vpminub xmm1,xmm2,xmm3/m128;' Compare packed unsigned byte integers in xmm2 and xmm3/m128 and store packed minimum values in xmm1.
	///
	/// 'vpminub ymm1,ymm2,ymm3/m256;' Compare packed unsigned byte integers in ymm2 and ymm3/m256 and store packed minimum values in ymm1.
	VPMINUB,
	///
	/// 'pminub mm1,mm2/m64;' Compare unsigned byte integers in mm2/m64 and mm1 and returns minimum values.
	///
	/// 'pminub xmm1,xmm2/m128;' Compare unsigned byte integers in xmm2/m128 and xmm1 and returns minimum values.
	PMINUB,
// PMINUD--Minimum of Packed Dword Integers.
	///
	/// 'pminud xmm1,xmm2/m128;' Compare packed unsigned dword integers in xmm1 and xmm2/m128 and store packed minimum values in xmm1.
	PMINUD,
	///
	/// 'vpminud xmm1,xmm2,xmm3/m128;' Compare packed unsigned dword integers in xmm2 and xmm3/m128 and store packed minimum values in xmm1.
	///
	/// 'vpminud ymm1,ymm2,ymm3/m256;' Compare packed unsigned dword integers in ymm2 and ymm3/m256 and store packed minimum values in ymm1.
	VPMINUD,
// PMINUW--Minimum of Packed Word Integers.
	///
	/// 'vpminuw xmm1,xmm2,xmm3/m128;' Compare packed unsigned word integers in xmm3/m128 and xmm2 and return packed minimum values in xmm1.
	///
	/// 'vpminuw ymm1,ymm2,ymm3/m256;' Compare packed unsigned word integers in ymm3/m256 and ymm2 and return packed minimum values in ymm1.
	VPMINUW,
	///
	/// 'pminuw xmm1,xmm2/m128;' Compare packed unsigned word integers in xmm1 and xmm2/m128 and store packed minimum values in xmm1.
	PMINUW,
// PMOVMSKB--Move Byte Mask.
	///
	/// 'pmovmskb reg,mm;' Move a byte mask of mm to reg. The upper bits of r32 or r64 are zeroed.
	///
	/// 'pmovmskb reg,xmm;' Move a byte mask of xmm to reg. The upper bits of r32 or r64 are zeroed.
	PMOVMSKB,
	///
	/// 'vpmovmskb reg,xmm1;' Move a byte mask of xmm1 to reg. The upper bits of r32 or r64 are filled with zeros.
	///
	/// 'vpmovmskb reg,ymm1;' Move a 32-bit mask of ymm1 to reg. The upper bits of r64 are filled with zeros.
	VPMOVMSKB,
// PMOVSX--Packed Move with Sign Extend.
	///
	/// 'pmovsxwd xmm1,xmm2/m64;' Sign extend 4 packed signed 16-bit integers in the low 8 bytes of xmm2/m64 to 4 packed signed 32-bit integers in xmm1.
	PMOVSXWD,
	///
	/// 'pmovsxbw xmm1,xmm2/m64;' Sign extend 8 packed signed 8-bit integers in the low 8 bytes of xmm2/m64 to 8 packed signed 16-bit integers in xmm1.
	PMOVSXBW,
	///
	/// 'vpmovsxwq xmm1,xmm2/m32;' Sign extend 2 packed 16-bit integers in the low 4 bytes of xmm2/m32 to 2 packed 64-bit integers in xmm1.
	///
	/// 'vpmovsxwq ymm1,xmm2/m64;' Sign extend 4 packed 16-bit integers in the low 8 bytes of xmm2/m64 to 4 packed 64-bit integers in ymm1.
	VPMOVSXWQ,
	///
	/// 'vpmovsxbd xmm1,xmm2/m32;' Sign extend 4 packed 8-bit integers in the low 4 bytes of xmm2/m32 to 4 packed 32-bit integers in xmm1.
	///
	/// 'vpmovsxbd ymm1,xmm2/m64;' Sign extend 8 packed 8-bit integers in the low 8 bytes of xmm2/m64 to 8 packed 32-bit integers in ymm1.
	VPMOVSXBD,
	///
	/// 'vpmovsxbw xmm1,xmm2/m64;' Sign extend 8 packed 8-bit integers in the low 8 bytes of xmm2/m64 to 8 packed 16-bit integers in xmm1.
	///
	/// 'vpmovsxbw ymm1,xmm2/m128;' Sign extend 16 packed 8-bit integers in xmm2/m128 to 16 packed 16-bit integers in ymm1.
	VPMOVSXBW,
	///
	/// 'pmovsxwq xmm1,xmm2/m32;' Sign extend 2 packed signed 16-bit integers in the low 4 bytes of xmm2/m32 to 2 packed signed 64-bit integers in xmm1.
	PMOVSXWQ,
	///
	/// 'pmovsxbq xmm1,xmm2/m16;' Sign extend 2 packed signed 8-bit integers in the low 2 bytes of xmm2/m16 to 2 packed signed 64-bit integers in xmm1.
	PMOVSXBQ,
	///
	/// 'pmovsxdq xmm1,xmm2/m64;' Sign extend 2 packed signed 32-bit integers in the low 8 bytes of xmm2/m64 to 2 packed signed 64-bit integers in xmm1.
	PMOVSXDQ,
	///
	/// 'pmovsxbd xmm1,xmm2/m32;' Sign extend 4 packed signed 8-bit integers in the low 4 bytes of xmm2/m32 to 4 packed signed 32-bit integers in xmm1.
	PMOVSXBD,
	///
	/// 'vpmovsxwd xmm1,xmm2/m64;' Sign extend 4 packed 16-bit integers in the low 8 bytes of xmm2/m64 to 4 packed 32-bit integers in xmm1.
	///
	/// 'vpmovsxwd ymm1,xmm2/m128;' Sign extend 8 packed 16-bit integers in the low 16 bytes of xmm2/m128 to 8 packed 32.
	VPMOVSXWD,
	///
	/// 'vpmovsxbq xmm1,xmm2/m16;' Sign extend 2 packed 8-bit integers in the low 2 bytes of xmm2/m16 to 2 packed 64-bit integers in xmm1.
	///
	/// 'vpmovsxbq ymm1,xmm2/m32;' Sign extend 4 packed 8-bit integers in the low 4 bytes of xmm2/m32 to 4 packed 64-bit integers in ymm1.
	VPMOVSXBQ,
	///
	/// 'vpmovsxdq xmm1,xmm2/m64;' Sign extend 2 packed 32-bit integers in the low 8 bytes of xmm2/m64 to 2 packed 64-bit integers in xmm1.
	///
	/// 'vpmovsxdq ymm1,xmm2/m128;' Sign extend 4 packed 32-bit integers in the low 16 bytes of xmm2/m128 to 4 packed 64.
	VPMOVSXDQ,
// PMOVZX--Packed Move with Zero Extend.
	///
	/// 'vpmovzxwd xmm1,xmm2/m64;' Zero extend 4 packed 16-bit integers in the low 8 bytes of xmm2/m64 to 4 packed 32-bit integers in xmm1.
	///
	/// 'vpmovzxwd ymm1,xmm2/m128;' Zero extend 8 packed 16-bit integers in the low 16 bytes of xmm2/m128 to 8 packed 32.
	VPMOVZXWD,
	///
	/// 'pmovzxbd xmm1,xmm2/m32;' Zero extend 4 packed 8-bit integers in the low 4 bytes of xmm2/m32 to 4 packed 32-bit integers in xmm1.
	PMOVZXBD,
	///
	/// 'pmovzxbq xmm1,xmm2/m16;' Zero extend 2 packed 8-bit integers in the low 2 bytes of xmm2/m16 to 2 packed 64-bit integers in xmm1.
	PMOVZXBQ,
	///
	/// 'vpmovzxbd xmm1,xmm2/m32;' Zero extend 4 packed 8-bit integers in the low 4 bytes of xmm2/m32 to 4 packed 32-bit integers in xmm1.
	///
	/// 'vpmovzxbd ymm1,xmm2/m64;' Zero extend 8 packed 8-bit integers in the low 8 bytes of xmm2/m64 to 8 packed 32-bit integers in ymm1.
	VPMOVZXBD,
	///
	/// 'pmovzxwd xmm1,xmm2/m64;' Zero extend 4 packed 16-bit integers in the low 8 bytes of xmm2/m64 to 4 packed 32-bit integers in xmm1.
	PMOVZXWD,
	///
	/// 'vpmovzxdq xmm1,xmm2/m64;' Zero extend 2 packed 32-bit integers in the low 8 bytes of xmm2/m64 to 2 packed 64-bit integers in xmm1.
	///
	/// 'vpmovzxdq ymm1,xmm2/m128;' Zero extend 4 packed 32-bit integers in the low 16 bytes of xmm2/m128 to 4 packed 64.
	VPMOVZXDQ,
	///
	/// 'pmovzxwq xmm1,xmm2/m32;' Zero extend 2 packed 16-bit integers in the low 4 bytes of xmm2/m32 to 2 packed 64-bit integers in xmm1.
	PMOVZXWQ,
	///
	/// 'vpmovzxwq xmm1,xmm2/m32;' Zero extend 2 packed 16-bit integers in the low 4 bytes of xmm2/m32 to 2 packed 64-bit integers in xmm1.
	///
	/// 'vpmovzxwq ymm1,xmm2/m64;' Zero extend 4 packed 16-bit integers in the low 8 bytes of xmm2/m64 to 4 packed 64-bit integers in xmm1.
	VPMOVZXWQ,
	///
	/// 'pmovzxbw xmm1,xmm2/m64;' Zero extend 8 packed 8-bit integers in the low 8 bytes of xmm2/m64 to 8 packed 16-bit integers in xmm1.
	PMOVZXBW,
	///
	/// 'pmovzxdq xmm1,xmm2/m64;' Zero extend 2 packed 32-bit integers in the low 8 bytes of xmm2/m64 to 2 packed 64-bit integers in xmm1.
	PMOVZXDQ,
	///
	/// 'vpmovzxbq xmm1,xmm2/m16;' Zero extend 2 packed 8-bit integers in the low 2 bytes of xmm2/m16 to 2 packed 64-bit integers in xmm1.
	///
	/// 'vpmovzxbq ymm1,xmm2/m32;' Zero extend 4 packed 8-bit integers in the low 4 bytes of xmm2/m32 to 4 packed 64-bit integers in ymm1.
	VPMOVZXBQ,
	///
	/// 'vpmovzxbw xmm1,xmm2/m64;' Zero extend 8 packed 8-bit integers in the low 8 bytes of xmm2/m64 to 8 packed 16-bit integers in xmm1.
	///
	/// 'vpmovzxbw ymm1,xmm2/m128;' Zero extend 16 packed 8-bit integers in the low 16 bytes of xmm2/m128 to 16 packed 16-bit integers in ymm1.
	VPMOVZXBW,
// PMULDQ--Multiply Packed Signed Dword Integers.
	///
	/// 'pmuldq xmm1,xmm2/m128;' Multiply the packed signed dword integers in xmm1 and xmm2/m128 and store the quadword product in xmm1.
	PMULDQ,
	///
	/// 'vpmuldq xmm1,xmm2,xmm3/m128;' Multiply packed signed doubleword integers in xmm2 by packed signed doubleword integers in xmm3/m128, and store the quadword results in xmm1.
	///
	/// 'vpmuldq ymm1,ymm2,ymm3/m256;' Multiply packed signed doubleword integers in ymm2 by packed signed doubleword integers in ymm3/m256, and store the quadword results in ymm1.
	VPMULDQ,
// PMULHRSW--Packed Multiply High with Round and Scale.
	///
	/// 'vpmulhrsw xmm1,xmm2,xmm3/m128;' Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to xmm1.
	///
	/// 'vpmulhrsw ymm1,ymm2,ymm3/m256;' Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to ymm1.
	VPMULHRSW,
	///
	/// 'pmulhrsw mm1,mm2/m64;' Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to mm1.
	///
	/// 'pmulhrsw xmm1,xmm2/m128;' Multiply 16-bit signed words, scale and round signed doublewords, pack high 16 bits to xmm1.
	PMULHRSW,
// PMULHUW--Multiply Packed Unsigned Integers and Store High Result.
	///
	/// 'vpmulhuw xmm1,xmm2,xmm3/m128;' Multiply the packed unsigned word integers in xmm2 and xmm3/m128, and store the high 16 bits of the results in xmm1.
	///
	/// 'vpmulhuw ymm1,ymm2,ymm3/m256;' Multiply the packed unsigned word integers in ymm2 and ymm3/m256, and store the high 16 bits of the results in ymm1.
	VPMULHUW,
	///
	/// 'pmulhuw mm1,mm2/m64;' Multiply the packed unsigned word integers in mm1 register and mm2/m64, and store the high 16 bits of the results in mm1.
	///
	/// 'pmulhuw xmm1,xmm2/m128;' Multiply the packed unsigned word integers in xmm1 and xmm2/m128, and store the high 16 bits of the results in xmm1.
	PMULHUW,
// PMULHW--Multiply Packed Signed Integers and Store High Result.
	///
	/// 'pmulhw mm,mm/m64;' Multiply the packed signed word integers in mm1 register and mm2/m64, and store the high 16 bits of the results in mm1.
	///
	/// 'pmulhw xmm1,xmm2/m128;' Multiply the packed signed word integers in xmm1 and xmm2/m128, and store the high 16 bits of the results in xmm1.
	PMULHW,
	///
	/// 'vpmulhw xmm1,xmm2,xmm3/m128;' Multiply the packed signed word integers in xmm2 and xmm3/m128, and store the high 16 bits of the results in xmm1.
	///
	/// 'vpmulhw ymm1,ymm2,ymm3/m256;' Multiply the packed signed word integers in ymm2 and ymm3/m256, and store the high 16 bits of the results in ymm1.
	VPMULHW,
// PMULLD--Multiply Packed Signed Dword Integers and Store Low Result.
	///
	/// 'pmulld xmm1,xmm2/m128;' Multiply the packed dword signed integers in xmm1 and xmm2/m128 and store the low 32 bits of each product in xmm1.
	PMULLD,
	///
	/// 'vpmulld xmm1,xmm2,xmm3/m128;' Multiply the packed dword signed integers in xmm2 and xmm3/m128 and store the low 32 bits of each product in xmm1.
	///
	/// 'vpmulld ymm1,ymm2,ymm3/m256;' Multiply the packed dword signed integers in ymm2 and ymm3/m256 and store the low 32 bits of each product in ymm1.
	VPMULLD,
// PMULLW--Multiply Packed Signed Integers and Store Low Result.
	///
	/// 'pmullw mm,mm/m64;' Multiply the packed signed word integers in mm1 register and mm2/m64, and store the low 16 bits of the results in mm1.
	///
	/// 'pmullw xmm1,xmm2/m128;' Multiply the packed signed word integers in xmm1 and xmm2/m128, and store the low 16 bits of the results in xmm1.
	PMULLW,
	///
	/// 'vpmullw xmm1,xmm2,xmm3/m128;' Multiply the packed dword signed integers in xmm2 and xmm3/m128 and store the low 32 bits of each product in xmm1.
	///
	/// 'vpmullw ymm1,ymm2,ymm3/m256;' Multiply the packed signed word integers in ymm2 and ymm3/m256, and store the low 16 bits of the results in ymm1.
	VPMULLW,
// PMULUDQ--Multiply Packed Unsigned Doubleword Integers.
	///
	/// 'vpmuludq xmm1,xmm2,xmm3/m128;' Multiply packed unsigned doubleword integers in xmm2 by packed unsigned doubleword integers in xmm3/m128, and store the quadword results in xmm1.
	///
	/// 'vpmuludq ymm1,ymm2,ymm3/m256;' Multiply packed unsigned doubleword integers in ymm2 by packed unsigned doubleword integers in ymm3/m256, and store the quadword results in ymm1.
	VPMULUDQ,
	///
	/// 'pmuludq mm1,mm2/m64;' Multiply unsigned doubleword integer in mm1 by unsigned doubleword integer in mm2/m64, and store the quadword result in mm1.
	///
	/// 'pmuludq xmm1,xmm2/m128;' Multiply packed unsigned doubleword integers in xmm1 by packed unsigned doubleword integers in xmm2/m128, and store the quadword results in xmm1.
	PMULUDQ,
// POP--Pop a Value from the Stack.
	///
	/// 'pop r/m16;' Pop top of stack into m16; increment stack pointer.
	///
	/// 'pop r/m32;' Pop top of stack into m32; increment stack pointer.
	///
	/// 'pop r/m64;' Pop top of stack into m64; increment stack pointer. Cannot encode 32-bit operand size.
	///
	/// 'pop r16;' Pop top of stack into r16; increment stack pointer.
	///
	/// 'pop r32;' Pop top of stack into r32; increment stack pointer.
	///
	/// 'pop r64;' Pop top of stack into r64; increment stack pointer. Cannot encode 32-bit operand size.
	///
	/// 'pop DS;' Pop top of stack into DS; increment stack pointer.
	///
	/// 'pop ES;' Pop top of stack into ES; increment stack pointer.
	///
	/// 'pop SS;' Pop top of stack into SS; increment stack pointer.
	///
	/// 'pop FS;' Pop top of stack into FS; increment stack pointer by 16 bits.
	///
	/// 'pop FS;' Pop top of stack into FS; increment stack pointer by 32 bits.
	///
	/// 'pop FS;' Pop top of stack into FS; increment stack pointer by 64 bits.
	///
	/// 'pop GS;' Pop top of stack into GS; increment stack pointer by 16 bits.
	///
	/// 'pop GS;' Pop top of stack into GS; increment stack pointer by 32 bits.
	///
	/// 'pop GS;' Pop top of stack into GS; increment stack pointer by 64 bits.
	POP,
// POPA/POPAD--Pop All General-Purpose Registers.
	///
	/// 'popad;' Pop EDI, ESI, EBP, EBX, EDX, ECX, and EAX.
	POPAD,
	///
	/// 'popa;' Pop DI, SI, BP, BX, DX, CX, and AX.
	POPA,
// POPCNT--Return the Count of Number of Bits Set to 1.
	///
	/// 'popcnt r16,r/m16;' POPCNT on r/m16.
	///
	/// 'popcnt r32,r/m32;' POPCNT on r/m32.
	///
	/// 'popcnt r64,r/m64;' POPCNT on r/m64.
	POPCNT,
// POPF/POPFD/POPFQ--Pop Stack into EFLAGS Register.
	///
	/// 'popfq;' Pop top of stack and zero-extend into RFLAGS.
	POPFQ,
	///
	/// 'popf;' Pop top of stack into lower 16 bits of EFLAGS.
	POPF,
	///
	/// 'popfd;' Pop top of stack into EFLAGS.
	POPFD,
// POR--Bitwise Logical OR.
	///
	/// 'por mm,mm/m64;' Bitwise OR of mm/m64 and mm.
	///
	/// 'por xmm1,xmm2/m128;' Bitwise OR of xmm2/m128 and xmm1.
	POR,
	///
	/// 'vpor xmm1,xmm2,xmm3/m128;' Bitwise OR of xmm2/m128 and xmm3.
	///
	/// 'vpor ymm1,ymm2,ymm3/m256;' Bitwise OR of ymm2/m256 and ymm3.
	VPOR,
// PREFETCHh--Prefetch Data Into Caches.
	///
	/// 'prefetcht1 m8;' Move data from m8 closer to the processor using T1 hint.
	PREFETCHT1,
	///
	/// 'prefetcht0 m8;' Move data from m8 closer to the processor using T0 hint.
	PREFETCHT0,
	///
	/// 'prefetchnta m8;' Move data from m8 closer to the processor using NTA hint.
	PREFETCHNTA,
	///
	/// 'prefetcht2 m8;' Move data from m8 closer to the processor using T2 hint.
	PREFETCHT2,
// PREFETCHW--Prefetch Data into Caches in Anticipation of a Write.
	///
	/// 'prefetchw m8;' Move data from m8 closer to the processor in anticipation of a write.
	PREFETCHW,
// PREFETCHWT1--Prefetch Vector Data Into Caches with Intent to Write and T1 Hint.
	///
	/// 'prefetchwt1 m8;' Move data from m8 closer to the processor using T1 hint with intent to write.
	PREFETCHWT1,
// PSADBW--Compute Sum of Absolute Differences.
	///
	/// 'psadbw mm1,mm2/m64;' Computes the absolute differences of the packed unsigned byte integers from mm2 /m64 and mm1; differences are then summed to produce an unsigned word integer result.
	///
	/// 'psadbw xmm1,xmm2/m128;' Computes the absolute differences of the packed unsigned byte integers from xmm2 /m128 and xmm1; the 8 low differences and 8 high differences are then summed separately to produce two unsigned word integer results.
	PSADBW,
	///
	/// 'vpsadbw xmm1,xmm2,xmm3/m128;' Computes the absolute differences of the packed unsigned byte integers from xmm3 /m128 and xmm2; the 8 low differences and 8 high differences are then summed separately to produce two unsigned word integer results.
	///
	/// 'vpsadbw ymm1,ymm2,ymm3/m256;' Computes the absolute differences of the packed unsigned byte integers from ymm3 /m256 and ymm2; then each consecutive 8 differences are summed separately to produce four unsigned word integer results.
	VPSADBW,
// PSHUFB--Packed Shuffle Bytes.
	///
	/// 'vpshufb xmm1,xmm2,xmm3/m128;' Shuffle bytes in xmm2 according to contents of xmm3/m128.
	///
	/// 'vpshufb ymm1,ymm2,ymm3/m256;' Shuffle bytes in ymm2 according to contents of ymm3/m256.
	VPSHUFB,
	///
	/// 'pshufb mm1,mm2/m64;' Shuffle bytes in mm1 according to contents of mm2/m64.
	///
	/// 'pshufb xmm1,xmm2/m128;' Shuffle bytes in xmm1 according to contents of xmm2/m128.
	PSHUFB,
// PSHUFD--Shuffle Packed Doublewords.
	///
	/// 'pshufd xmm1,xmm2/m128,imm8;' Shuffle the doublewords in xmm2/m128 based on the encoding in imm8 and store the result in xmm1.
	PSHUFD,
	///
	/// 'vpshufd xmm1,xmm2/m128,imm8;' Shuffle the doublewords in xmm2/m128 based on the encoding in imm8 and store the result in xmm1.
	///
	/// 'vpshufd ymm1,ymm2/m256,imm8;' Shuffle the doublewords in ymm2/m256 based on the encoding in imm8 and store the result in ymm1.
	VPSHUFD,
// PSHUFHW--Shuffle Packed High Words.
	///
	/// 'vpshufhw xmm1,xmm2/m128,imm8;' Shuffle the high words in xmm2/m128 based on the encoding in imm8 and store the result in xmm1.
	///
	/// 'vpshufhw ymm1,ymm2/m256,imm8;' Shuffle the high words in ymm2/m256 based on the encoding in imm8 and store the result in ymm1.
	VPSHUFHW,
	///
	/// 'pshufhw xmm1,xmm2/m128,imm8;' Shuffle the high words in xmm2/m128 based on the encoding in imm8 and store the result in xmm1.
	PSHUFHW,
// PSHUFLW--Shuffle Packed Low Words.
	///
	/// 'vpshuflw xmm1,xmm2/m128,imm8;' Shuffle the low words in xmm2/m128 based on the encoding in imm8 and store the result in xmm1.
	///
	/// 'vpshuflw ymm1,ymm2/m256,imm8;' Shuffle the low words in ymm2/m256 based on the encoding in imm8 and store the result in ymm1.
	VPSHUFLW,
	///
	/// 'pshuflw xmm1,xmm2/m128,imm8;' Shuffle the low words in xmm2/m128 based on the encoding in imm8 and store the result in xmm1.
	PSHUFLW,
// PSHUFW--Shuffle Packed Words.
	///
	/// 'pshufw mm1,mm2/m64,imm8;' Shuffle the words in mm2/m64 based on the encoding in imm8 and store the result in mm1.
	PSHUFW,
// PSIGNB/PSIGNW/PSIGND--Packed SIGN.
	///
	/// 'vpsignb xmm1,xmm2,xmm3/m128;' Negate/zero/preserve packed byte integers in xmm2 depending on the corresponding sign in xmm3/m128.
	///
	/// 'vpsignb ymm1,ymm2,ymm3/m256;' Negate packed byte integers in ymm2 if the corresponding sign in ymm3/m256 is less than zero.
	VPSIGNB,
	///
	/// 'vpsignw xmm1,xmm2,xmm3/m128;' Negate/zero/preserve packed word integers in xmm2 depending on the corresponding sign in xmm3/m128.
	///
	/// 'vpsignw ymm1,ymm2,ymm3/m256;' Negate packed 16-bit integers in ymm2 if the corresponding sign in ymm3/m256 is less than zero.
	VPSIGNW,
	///
	/// 'psignw mm1,mm2/m64;' Negate/zero/preserve packed word integers in mm1 depending on the corresponding sign in mm2/m128.
	///
	/// 'psignw xmm1,xmm2/m128;' Negate/zero/preserve packed word integers in xmm1 depending on the corresponding sign in xmm2/m128.
	PSIGNW,
	///
	/// 'psignb mm1,mm2/m64;' Negate/zero/preserve packed byte integers in mm1 depending on the corresponding sign in mm2/m64.
	///
	/// 'psignb xmm1,xmm2/m128;' Negate/zero/preserve packed byte integers in xmm1 depending on the corresponding sign in xmm2/m128.
	PSIGNB,
	///
	/// 'vpsignd xmm1,xmm2,xmm3/m128;' Negate/zero/preserve packed doubleword integers in xmm2 depending on the corresponding sign in xmm3/m128.
	///
	/// 'vpsignd ymm1,ymm2,ymm3/m256;' Negate packed doubleword integers in ymm2 if the corresponding sign in ymm3/m256 is less than zero.
	VPSIGND,
	///
	/// 'psignd mm1,mm2/m64;' Negate/zero/preserve packed doubleword integers in mm1 depending on the corresponding sign in mm2/m128.
	///
	/// 'psignd xmm1,xmm2/m128;' Negate/zero/preserve packed doubleword integers in xmm1 depending on the corresponding sign in xmm2/m128.
	PSIGND,
// PSLLDQ--Shift Double Quadword Left Logical.
	///
	/// 'vpslldq xmm1,xmm2,imm8;' Shift xmm2 left by imm8 bytes while shifting in 0s and store result in xmm1.
	///
	/// 'vpslldq ymm1,ymm2,imm8;' Shift ymm2 left by imm8 bytes while shifting in 0s and store result in ymm1.
	VPSLLDQ,
	///
	/// 'pslldq xmm1,imm8;' Shift xmm1 left by imm8 bytes while shifting in 0s.
	PSLLDQ,
// PSLLW/PSLLD/PSLLQ--Shift Packed Data Left Logical.
	///
	/// 'pslld mm,mm/m64;' Shift doublewords in mm left by mm/m64 while shifting in 0s.
	///
	/// 'pslld xmm1,xmm2/m128;' Shift doublewords in xmm1 left by xmm2/m128 while shifting in 0s.
	///
	/// 'pslld mm,imm8;' Shift doublewords in mm left by imm8 while shifting in 0s.
	///
	/// 'pslld xmm1,imm8;' Shift doublewords in xmm1 left by imm8 while shifting in 0s.
	PSLLD,
	///
	/// 'vpslld xmm1,xmm2,xmm3/m128;' Shift doublewords in xmm2 left by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpslld xmm1,xmm2,imm8;' Shift doublewords in xmm2 left by imm8 while shifting in 0s.
	///
	/// 'vpslld ymm1,ymm2,xmm3/m128;' Shift doublewords in ymm2 left by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpslld ymm1,ymm2,imm8;' Shift doublewords in ymm2 left by imm8 while shifting in 0s.
	VPSLLD,
	///
	/// 'vpsllq xmm1,xmm2,xmm3/m128;' Shift quadwords in xmm2 left by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsllq xmm1,xmm2,imm8;' Shift quadwords in xmm2 left by imm8 while shifting in 0s.
	///
	/// 'vpsllq ymm1,ymm2,xmm3/m128;' Shift quadwords in ymm2 left by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsllq ymm1,ymm2,imm8;' Shift quadwords in ymm2 left by imm8 while shifting in 0s.
	VPSLLQ,
	///
	/// 'vpsllw xmm1,xmm2,xmm3/m128;' Shift words in xmm2 left by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsllw xmm1,xmm2,imm8;' Shift words in xmm2 left by imm8 while shifting in 0s.
	///
	/// 'vpsllw ymm1,ymm2,xmm3/m128;' Shift words in ymm2 left by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsllw ymm1,ymm2,imm8;' Shift words in ymm2 left by imm8 while shifting in 0s.
	VPSLLW,
	///
	/// 'psllw mm,mm/m64;' Shift words in mm left mm/m64 while shifting in 0s.
	///
	/// 'psllw xmm1,xmm2/m128;' Shift words in xmm1 left by xmm2/m128 while shifting in 0s.
	///
	/// 'psllw mm1,imm8;' Shift words in mm left by imm8 while shifting in 0s.
	///
	/// 'psllw xmm1,imm8;' Shift words in xmm1 left by imm8 while shifting in 0s.
	PSLLW,
	///
	/// 'psllq mm,mm/m64;' Shift quadword in mm left by mm/m64 while shifting in 0s.
	///
	/// 'psllq xmm1,xmm2/m128;' Shift quadwords in xmm1 left by xmm2/m128 while shifting in 0s.
	///
	/// 'psllq mm,imm8;' Shift quadword in mm left by imm8 while shifting in 0s.
	///
	/// 'psllq xmm1,imm8;' Shift quadwords in xmm1 left by imm8 while shifting in 0s.
	PSLLQ,
// PSRAW/PSRAD--Shift Packed Data Right Arithmetic.
	///
	/// 'vpsrad xmm1,xmm2,xmm3/m128;' Shift doublewords in xmm2 right by amount specified in xmm3/m128 while shifting in sign bits.
	///
	/// 'vpsrad xmm1,xmm2,imm8;' Shift doublewords in xmm2 right by imm8 while shifting in sign bits.
	///
	/// 'vpsrad ymm1,ymm2,xmm3/m128;' Shift doublewords in ymm2 right by amount specified in xmm3/m128 while shifting in sign bits.
	///
	/// 'vpsrad ymm1,ymm2,imm8;' Shift doublewords in ymm2 right by imm8 while shifting in sign bits.
	VPSRAD,
	///
	/// 'vpsraw xmm1,xmm2,xmm3/m128;' Shift words in xmm2 right by amount specified in xmm3/m128 while shifting in sign bits.
	///
	/// 'vpsraw xmm1,xmm2,imm8;' Shift words in xmm2 right by imm8 while shifting in sign bits.
	///
	/// 'vpsraw ymm1,ymm2,xmm3/m128;' Shift words in ymm2 right by amount specified in xmm3/m128 while shifting in sign bits.
	///
	/// 'vpsraw ymm1,ymm2,imm8;' Shift words in ymm2 right by imm8 while shifting in sign bits.
	VPSRAW,
	///
	/// 'psraw mm,mm/m64;' Shift words in mm right by mm/m64 while shifting in sign bits.
	///
	/// 'psraw xmm1,xmm2/m128;' Shift words in xmm1 right by xmm2/m128 while shifting in sign bits.
	///
	/// 'psraw mm,imm8;' Shift words in mm right by imm8 while shifting in sign bits.
	///
	/// 'psraw xmm1,imm8;' Shift words in xmm1 right by imm8 while shifting in sign bits.
	PSRAW,
	///
	/// 'psrad mm,mm/m64;' Shift doublewords in mm right by mm/m64 while shifting in sign bits.
	///
	/// 'psrad xmm1,xmm2/m128;' Shift doubleword in xmm1 right by xmm2 /m128 while shifting in sign bits.
	///
	/// 'psrad mm,imm8;' Shift doublewords in mm right by imm8 while shifting in sign bits.
	///
	/// 'psrad xmm1,imm8;' Shift doublewords in xmm1 right by imm8 while shifting in sign bits.
	PSRAD,
// PSRLDQ--Shift Double Quadword Right Logical.
	///
	/// 'vpsrldq xmm1,xmm2,imm8;' Shift xmm2 right by imm8 bytes while shifting in 0s.
	///
	/// 'vpsrldq ymm1,ymm2,imm8;' Shift ymm1 right by imm8 bytes while shifting in 0s.
	VPSRLDQ,
	///
	/// 'psrldq xmm1,imm8;' Shift xmm1 right by imm8 while shifting in 0s.
	PSRLDQ,
// PSRLW/PSRLD/PSRLQ--Shift Packed Data Right Logical.
	///
	/// 'psrlq mm,mm/m64;' Shift mm right by amount specified in mm/m64 while shifting in 0s.
	///
	/// 'psrlq xmm1,xmm2/m128;' Shift quadwords in xmm1 right by amount specified in xmm2/m128 while shifting in 0s.
	///
	/// 'psrlq mm,imm8;' Shift mm right by imm8 while shifting in 0s.
	///
	/// 'psrlq xmm1,imm8;' Shift quadwords in xmm1 right by imm8 while shifting in 0s.
	PSRLQ,
	///
	/// 'psrld mm,mm/m64;' Shift doublewords in mm right by amount specified in mm/m64 while shifting in 0s.
	///
	/// 'psrld xmm1,xmm2/m128;' Shift doublewords in xmm1 right by amount specified in xmm2 /m128 while shifting in 0s.
	///
	/// 'psrld mm,imm8;' Shift doublewords in mm right by imm8 while shifting in 0s.
	///
	/// 'psrld xmm1,imm8;' Shift doublewords in xmm1 right by imm8 while shifting in 0s.
	PSRLD,
	///
	/// 'vpsrlw xmm1,xmm2,xmm3/m128;' Shift words in xmm2 right by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrlw xmm1,xmm2,imm8;' Shift words in xmm2 right by imm8 while shifting in 0s.
	///
	/// 'vpsrlw ymm1,ymm2,xmm3/m128;' Shift words in ymm2 right by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrlw ymm1,ymm2,imm8;' Shift words in ymm2 right by imm8 while shifting in 0s.
	VPSRLW,
	///
	/// 'psrlw mm,mm/m64;' Shift words in mm right by amount specified in mm/m64 while shifting in 0s.
	///
	/// 'psrlw xmm1,xmm2/m128;' Shift words in xmm1 right by amount specified in xmm2/m128 while shifting in 0s.
	///
	/// 'psrlw mm,imm8;' Shift words in mm right by imm8 while shifting in 0s.
	///
	/// 'psrlw xmm1,imm8;' Shift words in xmm1 right by imm8 while shifting in 0s.
	PSRLW,
	///
	/// 'vpsrld xmm1,xmm2,xmm3/m128;' Shift doublewords in xmm2 right by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrld xmm1,xmm2,imm8;' Shift doublewords in xmm2 right by imm8 while shifting in 0s.
	///
	/// 'vpsrld ymm1,ymm2,xmm3/m128;' Shift doublewords in ymm2 right by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrld ymm1,ymm2,imm8;' Shift doublewords in ymm2 right by imm8 while shifting in 0s.
	VPSRLD,
	///
	/// 'vpsrlq xmm1,xmm2,xmm3/m128;' Shift quadwords in xmm2 right by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrlq xmm1,xmm2,imm8;' Shift quadwords in xmm2 right by imm8 while shifting in 0s.
	///
	/// 'vpsrlq ymm1,ymm2,xmm3/m128;' Shift quadwords in ymm2 right by amount specified in xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrlq ymm1,ymm2,imm8;' Shift quadwords in ymm2 right by imm8 while shifting in 0s.
	VPSRLQ,
// PSUBB/PSUBW/PSUBD--Subtract Packed Integers.
	///
	/// 'psubb mm,mm/m64;' Subtract packed byte integers in mm/m64 from packed byte integers in mm.
	///
	/// 'psubb xmm1,xmm2/m128;' Subtract packed byte integers in xmm2/m128 from packed byte integers in xmm1.
	PSUBB,
	///
	/// 'vpsubd xmm1,xmm2,xmm3/m128;' Subtract packed doubleword integers in xmm3/m128 from xmm2.
	///
	/// 'vpsubd ymm1,ymm2,ymm3/m256;' Subtract packed doubleword integers in ymm3/m256 from ymm2.
	VPSUBD,
	///
	/// 'psubw mm,mm/m64;' Subtract packed word integers in mm/m64 from packed word integers in mm.
	///
	/// 'psubw xmm1,xmm2/m128;' Subtract packed word integers in xmm2/m128 from packed word integers in xmm1.
	PSUBW,
	///
	/// 'psubd mm,mm/m64;' Subtract packed doubleword integers in mm/m64 from packed doubleword integers in mm.
	///
	/// 'psubd xmm1,xmm2/m128;' Subtract packed doubleword integers in xmm2/mem128 from packed doubleword integers in xmm1.
	PSUBD,
	///
	/// 'vpsubw xmm1,xmm2,xmm3/m128;' Subtract packed word integers in xmm3/m128 from xmm2.
	///
	/// 'vpsubw ymm1,ymm2,ymm3/m256;' Subtract packed word integers in ymm3/m256 from ymm2.
	VPSUBW,
	///
	/// 'vpsubb xmm1,xmm2,xmm3/m128;' Subtract packed byte integers in xmm3/m128 from xmm2.
	///
	/// 'vpsubb ymm1,ymm2,ymm3/m256;' Subtract packed byte integers in ymm3/m256 from ymm2.
	VPSUBB,
// PSUBQ--Subtract Packed Quadword Integers.
	///
	/// 'psubq mm1,mm2/m64;' Subtract quadword integer in mm1 from mm2 /m64.
	///
	/// 'psubq xmm1,xmm2/m128;' Subtract packed quadword integers in xmm1 from xmm2 /m128.
	PSUBQ,
	///
	/// 'vpsubq xmm1,xmm2,xmm3/m128;' Subtract packed quadword integers in xmm3/m128 from xmm2.
	///
	/// 'vpsubq ymm1,ymm2,ymm3/m256;' Subtract packed quadword integers in ymm3/m256 from ymm2.
	VPSUBQ,
// PSUBSB/PSUBSW--Subtract Packed Signed Integers with Signed Saturation.
	///
	/// 'psubsb mm,mm/m64;' Subtract signed packed bytes in mm/m64 from signed packed bytes in mm and saturate results.
	///
	/// 'psubsb xmm1,xmm2/m128;' Subtract packed signed byte integers in xmm2/m128 from packed signed byte integers in xmm1 and saturate results.
	PSUBSB,
	///
	/// 'vpsubsb xmm1,xmm2,xmm3/m128;' Subtract packed signed byte integers in xmm3/m128 from packed signed byte integers in xmm2 and saturate results.
	///
	/// 'vpsubsb ymm1,ymm2,ymm3/m256;' Subtract packed signed byte integers in ymm3/m256 from packed signed byte integers in ymm2 and saturate results.
	VPSUBSB,
	///
	/// 'psubsw mm,mm/m64;' Subtract signed packed words in mm/m64 from signed packed words in mm and saturate results.
	///
	/// 'psubsw xmm1,xmm2/m128;' Subtract packed signed word integers in xmm2/m128 from packed signed word integers in xmm1 and saturate results.
	PSUBSW,
	///
	/// 'vpsubsw xmm1,xmm2,xmm3/m128;' Subtract packed signed word integers in xmm3/m128 from packed signed word integers in xmm2 and saturate results.
	///
	/// 'vpsubsw ymm1,ymm2,ymm3/m256;' Subtract packed signed word integers in ymm3/m256 from packed signed word integers in ymm2 and saturate results.
	VPSUBSW,
// PSUBUSB/PSUBUSW--Subtract Packed Unsigned Integers with Unsigned Saturation.
	///
	/// 'vpsubusb xmm1,xmm2,xmm3/m128;' Subtract packed unsigned byte integers in xmm3/m128 from packed unsigned byte integers in xmm2 and saturate result.
	///
	/// 'vpsubusb ymm1,ymm2,ymm3/m256;' Subtract packed unsigned byte integers in ymm3/m256 from packed unsigned byte integers in ymm2 and saturate result.
	VPSUBUSB,
	///
	/// 'psubusb mm,mm/m64;' Subtract unsigned packed bytes in mm/m64 from unsigned packed bytes in mm and saturate result.
	///
	/// 'psubusb xmm1,xmm2/m128;' Subtract packed unsigned byte integers in xmm2/m128 from packed unsigned byte integers in xmm1 and saturate result.
	PSUBUSB,
	///
	/// 'vpsubusw xmm1,xmm2,xmm3/m128;' Subtract packed unsigned word integers in xmm3/m128 from packed unsigned word integers in xmm2 and saturate result.
	///
	/// 'vpsubusw ymm1,ymm2,ymm3/m256;' Subtract packed unsigned word integers in ymm3/m256 from packed unsigned word integers in ymm2 and saturate result.
	VPSUBUSW,
	///
	/// 'psubusw mm,mm/m64;' Subtract unsigned packed words in mm/m64 from unsigned packed words in mm and saturate result.
	///
	/// 'psubusw xmm1,xmm2/m128;' Subtract packed unsigned word integers in xmm2/m128 from packed unsigned word integers in xmm1 and saturate result.
	PSUBUSW,
// PTEST--Logical Compare.
	///
	/// 'ptest xmm1,xmm2/m128;' Set ZF if xmm2/m128 AND xmm1 result is all 0s. Set CF if xmm2/m128 AND NOT xmm1 result is all 0s.
	PTEST,
	///
	/// 'vptest xmm1,xmm2/m128;' Set ZF and CF depending on bitwise AND and ANDN of sources.
	///
	/// 'vptest ymm1,ymm2/m256;' Set ZF and CF depending on bitwise AND and ANDN of sources.
	VPTEST,
// PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ--Unpack High Data.
	///
	/// 'punpckhdq mm,mm/m64;' Unpack and interleave high-order doublewords from mm and mm/m64 into mm.
	///
	/// 'punpckhdq xmm1,xmm2/m128;' Unpack and interleave high-order doublewords from xmm1 and xmm2/m128 into xmm1.
	PUNPCKHDQ,
	///
	/// 'vpunpckhdq xmm1,xmm2,xmm3/m128;' Interleave high-order doublewords from xmm2 and xmm3/m128 into xmm1.
	///
	/// 'vpunpckhdq ymm1,ymm2,ymm3/m256;' Interleave high-order doublewords from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKHDQ,
	///
	/// 'vpunpckhqdq xmm1,xmm2,xmm3/m128;' Interleave high-order quadword from xmm2 and xmm3/m128 into xmm1 register.
	///
	/// 'vpunpckhqdq ymm1,ymm2,ymm3/m256;' Interleave high-order quadword from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKHQDQ,
	///
	/// 'punpckhqdq xmm1,xmm2/m128;' Unpack and interleave high-order quadwords from xmm1 and xmm2/m128 into xmm1.
	PUNPCKHQDQ,
	///
	/// 'vpunpckhbw xmm1,xmm2,xmm3/m128;' Interleave high-order bytes from xmm2 and xmm3/m128 into xmm1.
	///
	/// 'vpunpckhbw ymm1,ymm2,ymm3/m256;' Interleave high-order bytes from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKHBW,
	///
	/// 'punpckhbw mm,mm/m64;' Unpack and interleave high-order bytes from mm and mm/m64 into mm.
	///
	/// 'punpckhbw xmm1,xmm2/m128;' Unpack and interleave high-order bytes from xmm1 and xmm2/m128 into xmm1.
	PUNPCKHBW,
	///
	/// 'punpckhwd mm,mm/m64;' Unpack and interleave high-order words from mm and mm/m64 into mm.
	///
	/// 'punpckhwd xmm1,xmm2/m128;' Unpack and interleave high-order words from xmm1 and xmm2/m128 into xmm1.
	PUNPCKHWD,
	///
	/// 'vpunpckhwd xmm1,xmm2,xmm3/m128;' Interleave high-order words from xmm2 and xmm3/m128 into xmm1.
	///
	/// 'vpunpckhwd ymm1,ymm2,ymm3/m256;' Interleave high-order words from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKHWD,
// PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ--Unpack Low Data.
	///
	/// 'vpunpcklwd xmm1,xmm2,xmm3/m128;' Interleave low-order words from xmm2 and xmm3/m128 into xmm1.
	///
	/// 'vpunpcklwd ymm1,ymm2,ymm3/m256;' Interleave low-order words from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKLWD,
	///
	/// 'punpcklwd mm,mm/m32;' Interleave low-order words from mm and mm/m32 into mm.
	///
	/// 'punpcklwd xmm1,xmm2/m128;' Interleave low-order words from xmm1 and xmm2/m128 into xmm1.
	PUNPCKLWD,
	///
	/// 'vpunpckldq xmm1,xmm2,xmm3/m128;' Interleave low-order doublewords from xmm2 and xmm3/m128 into xmm1.
	///
	/// 'vpunpckldq ymm1,ymm2,ymm3/m256;' Interleave low-order doublewords from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKLDQ,
	///
	/// 'punpckldq mm,mm/m32;' Interleave low-order doublewords from mm and mm/m32 into mm.
	///
	/// 'punpckldq xmm1,xmm2/m128;' Interleave low-order doublewords from xmm1 and xmm2/m128 into xmm1.
	PUNPCKLDQ,
	///
	/// 'vpunpcklbw xmm1,xmm2,xmm3/m128;' Interleave low-order bytes from xmm2 and xmm3/m128 into xmm1.
	///
	/// 'vpunpcklbw ymm1,ymm2,ymm3/m256;' Interleave low-order bytes from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKLBW,
	///
	/// 'punpcklbw mm,mm/m32;' Interleave low-order bytes from mm and mm/m32 into mm.
	///
	/// 'punpcklbw xmm1,xmm2/m128;' Interleave low-order bytes from xmm1 and xmm2/m128 into xmm1.
	PUNPCKLBW,
	///
	/// 'vpunpcklqdq xmm1,xmm2,xmm3/m128;' Interleave low-order quadword from xmm2 and xmm3/m128 into xmm1 register.
	///
	/// 'vpunpcklqdq ymm1,ymm2,ymm3/m256;' Interleave low-order quadword from ymm2 and ymm3/m256 into ymm1 register.
	VPUNPCKLQDQ,
	///
	/// 'punpcklqdq xmm1,xmm2/m128;' Interleave low-order quadword from xmm1 and xmm2/m128 into xmm1 register.
	PUNPCKLQDQ,
// PUSH--Push Word, Doubleword or Quadword Onto the Stack.
	///
	/// 'push r/m16;' Push r/m16.
	///
	/// 'push r/m32;' Push r/m32.
	///
	/// 'push r/m64;' Push r/m64.
	///
	/// 'push r16;' Push r16.
	///
	/// 'push r32;' Push r32.
	///
	/// 'push r64;' Push r64.
	///
	/// 'push imm8;' Push imm8.
	///
	/// 'push imm16;' Push imm16.
	///
	/// 'push imm32;' Push imm32.
	///
	/// 'push CS;' Push CS.
	///
	/// 'push SS;' Push SS.
	///
	/// 'push DS;' Push DS.
	///
	/// 'push ES;' Push ES.
	///
	/// 'push FS;' Push FS.
	///
	/// 'push GS;' Push GS.
	PUSH,
// PUSHA/PUSHAD--Push All General-Purpose Registers.
	///
	/// 'pushad;' Push EAX, ECX, EDX, EBX, original ESP, EBP, ESI, and EDI.
	PUSHAD,
	///
	/// 'pusha;' Push AX, CX, DX, BX, original SP, BP, SI, and DI.
	PUSHA,
// PUSHF/PUSHFD--Push EFLAGS Register onto the Stack.
	///
	/// 'pushf;' Push lower 16 bits of EFLAGS.
	PUSHF,
	///
	/// 'pushfq;' Push RFLAGS.
	PUSHFQ,
	///
	/// 'pushfd;' Push EFLAGS.
	PUSHFD,
// PXOR--Logical Exclusive OR.
	///
	/// 'pxor mm,mm/m64;' Bitwise XOR of mm/m64 and mm.
	///
	/// 'pxor xmm1,xmm2/m128;' Bitwise XOR of xmm2/m128 and xmm1.
	PXOR,
	///
	/// 'vpxor xmm1,xmm2,xmm3/m128;' Bitwise XOR of xmm3/m128 and xmm2.
	///
	/// 'vpxor ymm1,ymm2,ymm3/m256;' Bitwise XOR of ymm3/m256 and ymm2.
	VPXOR,
// RCL/RCR/ROL/ROR---Rotate.
	///
	/// 'rcl r/m8,1;' Rotate 9 bits (CF, r/m8) left once.
	///
	/// 'rcl r/m8*,1;' Rotate 9 bits (CF, r/m8) left once.
	///
	/// 'rcl r/m8,CL;' Rotate 9 bits (CF, r/m8) left CL times.
	///
	/// 'rcl r/m8*,CL;' Rotate 9 bits (CF, r/m8) left CL times.
	///
	/// 'rcl r/m8,imm8;' Rotate 9 bits (CF, r/m8) left imm8 times.
	///
	/// 'rcl r/m8*,imm8;' Rotate 9 bits (CF, r/m8) left imm8 times.
	///
	/// 'rcl r/m16,1;' Rotate 17 bits (CF, r/m16) left once.
	///
	/// 'rcl r/m16,CL;' Rotate 17 bits (CF, r/m16) left CL times.
	///
	/// 'rcl r/m16,imm8;' Rotate 17 bits (CF, r/m16) left imm8 times.
	///
	/// 'rcl r/m32,1;' Rotate 33 bits (CF, r/m32) left once.
	///
	/// 'rcl r/m64,1;' Rotate 65 bits (CF, r/m64) left once. Uses a 6.
	///
	/// 'rcl r/m32,CL;' Rotate 33 bits (CF, r/m32) left CL times.
	///
	/// 'rcl r/m64,CL;' Rotate 65 bits (CF, r/m64) left CL times. Uses a 6 bit count.
	///
	/// 'rcl r/m32,imm8;' Rotate 33 bits (CF, r/m32) left imm8 times.
	///
	/// 'rcl r/m64,imm8;' Rotate 65 bits (CF, r/m64) left imm8 times. Uses a 6 bit count.
	RCL,
	///
	/// 'ror r/m8,1;' Rotate 8 bits r/m8 right once.
	///
	/// 'ror r/m8*,1;' Rotate 8 bits r/m8 right once.
	///
	/// 'ror r/m8,CL;' Rotate 8 bits r/m8 right CL times.
	///
	/// 'ror r/m8*,CL;' Rotate 8 bits r/m8 right CL times.
	///
	/// 'ror r/m8,imm8;' Rotate 8 bits r/m16 right imm8 times.
	///
	/// 'ror r/m8*,imm8;' Rotate 8 bits r/m16 right imm8 times.
	///
	/// 'ror r/m16,1;' Rotate 16 bits r/m16 right once.
	///
	/// 'ror r/m16,CL;' Rotate 16 bits r/m16 right CL times.
	///
	/// 'ror r/m16,imm8;' Rotate 16 bits r/m16 right imm8 times.
	///
	/// 'ror r/m32,1;' Rotate 32 bits r/m32 right once.
	///
	/// 'ror r/m64,1;' Rotate 64 bits r/m64 right once. Uses a 6 bit count.
	///
	/// 'ror r/m32,CL;' Rotate 32 bits r/m32 right CL times.
	///
	/// 'ror r/m64,CL;' Rotate 64 bits r/m64 right CL times. Uses a 6.
	///
	/// 'ror r/m32,imm8;' Rotate 32 bits r/m32 right imm8 times.
	///
	/// 'ror r/m64,imm8;' Rotate 64 bits r/m64 right imm8 times. Uses a 6 bit count.
	ROR,
	///
	/// 'rcr r/m8,1;' Rotate 9 bits (CF, r/m8) right once.
	///
	/// 'rcr r/m8*,1;' Rotate 9 bits (CF, r/m8) right once.
	///
	/// 'rcr r/m8,CL;' Rotate 9 bits (CF, r/m8) right CL times.
	///
	/// 'rcr r/m8*,CL;' Rotate 9 bits (CF, r/m8) right CL times.
	///
	/// 'rcr r/m8,imm8;' Rotate 9 bits (CF, r/m8) right imm8 times.
	///
	/// 'rcr r/m8*,imm8;' Rotate 9 bits (CF, r/m8) right imm8 times.
	///
	/// 'rcr r/m16,1;' Rotate 17 bits (CF, r/m16) right once.
	///
	/// 'rcr r/m16,CL;' Rotate 17 bits (CF, r/m16) right CL times.
	///
	/// 'rcr r/m16,imm8;' Rotate 17 bits (CF, r/m16) right imm8 times.
	///
	/// 'rcr r/m32,1;' Rotate 33 bits (CF, r/m32) right once. Uses a 6.
	///
	/// 'rcr r/m64,1;' Rotate 65 bits (CF, r/m64) right once. Uses a 6.
	///
	/// 'rcr r/m32,CL;' Rotate 33 bits (CF, r/m32) right CL times.
	///
	/// 'rcr r/m64,CL;' Rotate 65 bits (CF, r/m64) right CL times. Uses a 6 bit count.
	///
	/// 'rcr r/m32,imm8;' Rotate 33 bits (CF, r/m32) right imm8 times.
	///
	/// 'rcr r/m64,imm8;' Rotate 65 bits (CF, r/m64) right imm8 times. Uses a 6 bit count.
	RCR,
	///
	/// 'rol r/m8,1;' Rotate 8 bits r/m8 left once.
	///
	/// 'rol r/m8*,1;' Rotate 8 bits r/m8 left once.
	///
	/// 'rol r/m8,CL;' Rotate 8 bits r/m8 left CL times.
	///
	/// 'rol r/m8*,CL;' Rotate 8 bits r/m8 left CL times.
	///
	/// 'rol r/m8,imm8;' Rotate 8 bits r/m8 left imm8 times.
	///
	/// 'rol r/m8*,imm8;' Rotate 8 bits r/m8 left imm8 times.
	///
	/// 'rol r/m16,1;' Rotate 16 bits r/m16 left once.
	///
	/// 'rol r/m16,CL;' Rotate 16 bits r/m16 left CL times.
	///
	/// 'rol r/m16,imm8;' Rotate 16 bits r/m16 left imm8 times.
	///
	/// 'rol r/m32,1;' Rotate 32 bits r/m32 left once.
	///
	/// 'rol r/m64,1;' Rotate 64 bits r/m64 left once. Uses a 6 bit count.
	///
	/// 'rol r/m32,CL;' Rotate 32 bits r/m32 left CL times.
	///
	/// 'rol r/m64,CL;' Rotate 64 bits r/m64 left CL times. Uses a 6.
	///
	/// 'rol r/m32,imm8;' Rotate 32 bits r/m32 left imm8 times.
	///
	/// 'rol r/m64,imm8;' Rotate 64 bits r/m64 left imm8 times. Uses a 6 bit count.
	ROL,
// RCPPS--Compute Reciprocals of Packed Single-Precision Floating-Point Values.
	///
	/// 'vrcpps xmm1,xmm2/m128;' Computes the approximate reciprocals of packed single-precision values in xmm2/mem and stores the results in xmm1.
	///
	/// 'vrcpps ymm1,ymm2/m256;' Computes the approximate reciprocals of packed single-precision values in ymm2/mem and stores the results in ymm1.
	VRCPPS,
	///
	/// 'rcpps xmm1,xmm2/m128;' Computes the approximate reciprocals of the packed single-precision floating-point values in xmm2/m128 and stores the results in xmm1.
	RCPPS,
// RCPSS--Compute Reciprocal of Scalar Single-Precision Floating-Point Values.
	///
	/// 'vrcpss xmm1,xmm2,xmm3/m32;' Computes the approximate reciprocal of the scalar single-precision floating-point value in xmm3/m32 and stores the result in xmm1. Also, upper single precision floating-point values (bits[127:32]) from xmm2 are copied to xmm1[127:32].
	VRCPSS,
	///
	/// 'rcpss xmm1,xmm2/m32;' Computes the approximate reciprocal of the scalar single-precision floating-point value in xmm2/m32 and stores the result in xmm1.
	RCPSS,
// RDFSBASE/RDGSBASE--Read FS/GS Segment Base.
// RDMSR--Read from Model Specific Register.
	///
	/// 'rdmsr;' Read MSR specified by ECX into EDX:EAX.
	RDMSR,
// RDPKRU--Read Protection Key Rights for User Pages.
	///
	/// 'rdpkru;' Reads PKRU into EAX.
	RDPKRU,
// RDPMC--Read Performance-Monitoring Counters.
	///
	/// 'rdpmc;' Read performance-monitoring counter specified by ECX into EDX:EAX.
	RDPMC,
// RDRAND--Read Random Number.
	///
	/// 'rdrand r16;' Read a 16-bit random number and store in the destination register.
	///
	/// 'rdrand r32;' Read a 32-bit random number and store in the destination register.
	RDRAND,
// RDSEED--Read Random SEED.
	///
	/// 'rdseed r16;' Read a 16-bit NIST SP800-90B & C compliant random value and store in the destination register.
	///
	/// 'rdseed r32;' Read a 32-bit NIST SP800-90B & C compliant random value and store in the destination register.
	RDSEED,
// RDTSC--Read Time-Stamp Counter.
	///
	/// 'rdtsc;' Read time-stamp counter into EDX:EAX.
	RDTSC,
// RDTSCP--Read Time-Stamp Counter and Processor ID.
	///
	/// 'rdtscp;' Read 64-bit time-stamp counter and 32-bit IA32_TSC_AUX value into EDX:EAX and ECX.
	RDTSCP,
// REP/REPE/REPZ/REPNE/REPNZ--Repeat String Operation Prefix.
	///
	/// 'rep INS m8,DX;' Input (E)CX bytes from port DX into ES:[(E)DI].
	///
	/// 'rep INS m8,DX;' Input RCX bytes from port DX into [RDI].
	///
	/// 'rep INS m16,DX;' Input (E)CX words from port DX into ES:[(E)DI.].
	///
	/// 'rep INS m32,DX;' Input (E)CX doublewords from port DX into ES:[(E)DI].
	///
	/// 'rep INS r/m32,DX;' Input RCX default size from port DX into [RDI].
	///
	/// 'rep MOVS m8,m8;' Move (E)CX bytes from DS:[(E)SI] to ES:[(E)DI].
	///
	/// 'rep MOVS m8,m8;' Move RCX bytes from [RSI] to [RDI].
	///
	/// 'rep MOVS m16,m16;' Move (E)CX words from DS:[(E)SI] to ES:[(E)DI].
	///
	/// 'rep MOVS m32,m32;' Move (E)CX doublewords from DS:[(E)SI] to ES:[(E)DI].
	///
	/// 'rep MOVS m64,m64;' Move RCX quadwords from [RSI] to [RDI].
	///
	/// 'rep OUTS DX,r/m8;' Output (E)CX bytes from DS:[(E)SI] to port DX.
	///
	/// 'rep OUTS DX,r/m8*;' Output RCX bytes from [RSI] to port DX.
	///
	/// 'rep OUTS DX,r/m16;' Output (E)CX words from DS:[(E)SI] to port DX.
	///
	/// 'rep OUTS DX,r/m32;' Output (E)CX doublewords from DS:[(E)SI] to port DX.
	///
	/// 'rep OUTS DX,r/m32;' Output RCX default size from [RSI] to port DX.
	///
	/// 'rep LODS AL;' Load (E)CX bytes from DS:[(E)SI] to AL.
	///
	/// 'rep LODS AL;' Load RCX bytes from [RSI] to AL.
	///
	/// 'rep LODS AX;' Load (E)CX words from DS:[(E)SI] to AX.
	///
	/// 'rep LODS EAX;' Load (E)CX doublewords from DS:[(E)SI] to EAX.
	///
	/// 'rep LODS RAX;' Load RCX quadwords from [RSI] to RAX.
	///
	/// 'rep STOS m8;' Fill (E)CX bytes at ES:[(E)DI] with AL.
	///
	/// 'rep STOS m8;' Fill RCX bytes at [RDI] with AL.
	///
	/// 'rep STOS m16;' Fill (E)CX words at ES:[(E)DI] with AX.
	///
	/// 'rep STOS m32;' Fill (E)CX doublewords at ES:[(E)DI] with EAX.
	///
	/// 'rep STOS m64;' Fill RCX quadwords at [RDI] with RAX.
	REP,
	///
	/// 'repne CMPS m8,m8;' Find matching bytes in ES:[(E)DI] and DS:[(E)SI].
	///
	/// 'repne CMPS m8,m8;' Find matching bytes in [RDI] and [RSI].
	///
	/// 'repne CMPS m16,m16;' Find matching words in ES:[(E)DI] and DS:[(E)SI].
	///
	/// 'repne CMPS m32,m32;' Find matching doublewords in ES:[(E)DI] and DS:[(E)SI].
	///
	/// 'repne CMPS m64,m64;' Find matching doublewords in [RDI] and [RSI].
	///
	/// 'repne SCAS m8;' Find AL, starting at ES:[(E)DI].
	///
	/// 'repne SCAS m8;' Find AL, starting at [RDI].
	///
	/// 'repne SCAS m16;' Find AX, starting at ES:[(E)DI].
	///
	/// 'repne SCAS m32;' Find EAX, starting at ES:[(E)DI].
	///
	/// 'repne SCAS m64;' Find RAX, starting at [RDI].
	REPNE,
	///
	/// 'repe CMPS m8,m8;' Find nonmatching bytes in ES:[(E)DI] and DS:[(E)SI].
	///
	/// 'repe CMPS m8,m8;' Find non-matching bytes in [RDI] and [RSI].
	///
	/// 'repe CMPS m16,m16;' Find nonmatching words in ES:[(E)DI] and DS:[(E)SI].
	///
	/// 'repe CMPS m32,m32;' Find nonmatching doublewords in ES:[(E)DI] and DS:[(E)SI].
	///
	/// 'repe CMPS m64,m64;' Find non-matching quadwords in [RDI] and [RSI].
	///
	/// 'repe SCAS m8;' Find non-AL byte starting at ES:[(E)DI].
	///
	/// 'repe SCAS m8;' Find non-AL byte starting at [RDI].
	///
	/// 'repe SCAS m16;' Find non-AX word starting at ES:[(E)DI].
	///
	/// 'repe SCAS m32;' Find non-EAX doubleword starting at ES:[(E)DI].
	///
	/// 'repe SCAS m64;' Find non-RAX quadword starting at [RDI].
	REPE,
// RET--Return from Procedure.
	///
	/// 'ret;' Near return to calling procedure.
	///
	/// 'ret;' Far return to calling procedure.
	///
	/// 'ret imm16;' Near return to calling procedure and pop imm16 bytes from stack.
	///
	/// 'ret imm16;' Far return to calling procedure and pop imm16 bytes from stack.
	RET,
// RORX--Rotate Right Logical Without Affecting Flags.
	///
	/// 'rorx r32,r/m32,imm8;' Rotate 32-bit r/m32 right imm8 times without affecting arithmetic flags.
	///
	/// 'rorx r64,r/m64,imm8;' Rotate 64-bit r/m64 right imm8 times without affecting arithmetic flags.
	RORX,
// ROUNDPD--Round Packed Double Precision Floating-Point Values.
	///
	/// 'vroundpd xmm1,xmm2/m128,imm8;' Round packed double-precision floating-point values in xmm2/m128 and place the result in xmm1. The rounding mode is determined by imm8.
	///
	/// 'vroundpd ymm1,ymm2/m256,imm8;' Round packed double-precision floating-point values in ymm2/m256 and place the result in ymm1. The rounding mode is determined by imm8.
	VROUNDPD,
	///
	/// 'roundpd xmm1,xmm2/m128,imm8;' Round packed double precision floating-point values in xmm2/m128 and place the result in xmm1. The rounding mode is determined by imm8.
	ROUNDPD,
// ROUNDPS--Round Packed Single Precision Floating-Point Values.
	///
	/// 'roundps xmm1,xmm2/m128,imm8;' Round packed single precision floating-point values in xmm2/m128 and place the result in xmm1. The rounding mode is determined by imm8.
	ROUNDPS,
	///
	/// 'vroundps xmm1,xmm2/m128,imm8;' Round packed single-precision floating-point values in xmm2/m128 and place the result in xmm1. The rounding mode is determined by imm8.
	///
	/// 'vroundps ymm1,ymm2/m256,imm8;' Round packed single-precision floating-point values in ymm2/m256 and place the result in ymm1. The rounding mode is determined by imm8.
	VROUNDPS,
// ROUNDSD--Round Scalar Double Precision Floating-Point Values.
	///
	/// 'vroundsd xmm1,xmm2,xmm3/m64,imm8;' Round the low packed double precision floating-point value in xmm3/m64 and place the result in xmm1. The rounding mode is determined by imm8. Upper packed double precision floating-point value (bits[127:64]) from xmm2 is copied to xmm1[127:64].
	VROUNDSD,
	///
	/// 'roundsd xmm1,xmm2/m64,imm8;' Round the low packed double precision floating-point value in xmm2/m64 and place the result in xmm1. The rounding mode is determined by imm8.
	ROUNDSD,
// ROUNDSS--Round Scalar Single Precision Floating-Point Values.
	///
	/// 'vroundss xmm1,xmm2,xmm3/m32,imm8;' Round the low packed single precision floating-point value in xmm3/m32 and place the result in xmm1. The rounding mode is determined by imm8. Also, upper packed single precision floating-point values (bits[127:32]) from xmm2 are copied to xmm1[127:32].
	VROUNDSS,
	///
	/// 'roundss xmm1,xmm2/m32,imm8;' Round the low packed single precision floating-point value in xmm2/m32 and place the result in xmm1. The rounding mode is determined by imm8.
	ROUNDSS,
// RSM--Resume from System Management Mode.
	///
	/// 'rsm;' Resume operation of interrupted program.
	RSM,
// RSQRTPS--Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values.
	///
	/// 'vrsqrtps xmm1,xmm2/m128;' Computes the approximate reciprocals of the square roots of packed single-precision values in xmm2/mem and stores the results in xmm1.
	///
	/// 'vrsqrtps ymm1,ymm2/m256;' Computes the approximate reciprocals of the square roots of packed single-precision values in ymm2/mem and stores the results in ymm1.
	VRSQRTPS,
	///
	/// 'rsqrtps xmm1,xmm2/m128;' Computes the approximate reciprocals of the square roots of the packed single-precision floating-point values in xmm2/m128 and stores the results in xmm1.
	RSQRTPS,
// RSQRTSS--Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value.
	///
	/// 'vrsqrtss xmm1,xmm2,xmm3/m32;' Computes the approximate reciprocal of the square root of the low single precision floating-point value in xmm3/m32 and stores the results in xmm1. Also, upper single precision floating-point values (bits[127:32]) from xmm2 are copied to xmm1[127:32].
	VRSQRTSS,
	///
	/// 'rsqrtss xmm1,xmm2/m32;' Computes the approximate reciprocal of the square root of the low single-precision floating-point value in xmm2/m32 and stores the results in xmm1.
	RSQRTSS,
// SAHF--Store AH into Flags.
	///
	/// 'sahf;' Loads SF, ZF, AF, PF, and CF from AH into EFLAGS register.
	SAHF,
// SAL/SAR/SHL/SHR--Shift.
	///
	/// 'sar r/m8,1;' Signed divide* r/m8 by 2, once.
	///
	/// 'sar r/m8**,1;' Signed divide* r/m8 by 2, once.
	///
	/// 'sar r/m8,CL;' Signed divide* r/m8 by 2, CL times.
	///
	/// 'sar r/m8**,CL;' Signed divide* r/m8 by 2, CL times.
	///
	/// 'sar r/m8,imm8;' Signed divide* r/m8 by 2, imm8 time.
	///
	/// 'sar r/m8**,imm8;' Signed divide* r/m8 by 2, imm8 times.
	///
	/// 'sar r/m16,1;' Signed divide* r/m16 by 2, once.
	///
	/// 'sar r/m16,CL;' Signed divide* r/m16 by 2, CL times.
	///
	/// 'sar r/m16,imm8;' Signed divide* r/m16 by 2, imm8 times.
	///
	/// 'sar r/m32,1;' Signed divide* r/m32 by 2, once.
	///
	/// 'sar r/m64,1;' Signed divide* r/m64 by 2, once.
	///
	/// 'sar r/m32,CL;' Signed divide* r/m32 by 2, CL times.
	///
	/// 'sar r/m64,CL;' Signed divide* r/m64 by 2, CL times.
	///
	/// 'sar r/m32,imm8;' Signed divide* r/m32 by 2, imm8 times.
	///
	/// 'sar r/m64,imm8;' Signed divide* r/m64 by 2, imm8 times.
	SAR,
	///
	/// 'sal r/m8,1;' Multiply r/m8 by 2, once.
	///
	/// 'sal r/m8**,1;' Multiply r/m8 by 2, once.
	///
	/// 'sal r/m8,CL;' Multiply r/m8 by 2, CL times.
	///
	/// 'sal r/m8**,CL;' Multiply r/m8 by 2, CL times.
	///
	/// 'sal r/m8,imm8;' Multiply r/m8 by 2, imm8 times.
	///
	/// 'sal r/m8**,imm8;' Multiply r/m8 by 2, imm8 times.
	///
	/// 'sal r/m16,1;' Multiply r/m16 by 2, once.
	///
	/// 'sal r/m16,CL;' Multiply r/m16 by 2, CL times.
	///
	/// 'sal r/m16,imm8;' Multiply r/m16 by 2, imm8 times.
	///
	/// 'sal r/m32,1;' Multiply r/m32 by 2, once.
	///
	/// 'sal r/m64,1;' Multiply r/m64 by 2, once.
	///
	/// 'sal r/m32,CL;' Multiply r/m32 by 2, CL times.
	///
	/// 'sal r/m64,CL;' Multiply r/m64 by 2, CL times.
	///
	/// 'sal r/m32,imm8;' Multiply r/m32 by 2, imm8 times.
	///
	/// 'sal r/m64,imm8;' Multiply r/m64 by 2, imm8 times.
	SAL,
	///
	/// 'shl r/m8,1;' Multiply r/m8 by 2, once.
	///
	/// 'shl r/m8**,1;' Multiply r/m8 by 2, once.
	///
	/// 'shl r/m8,CL;' Multiply r/m8 by 2, CL times.
	///
	/// 'shl r/m8**,CL;' Multiply r/m8 by 2, CL times.
	///
	/// 'shl r/m8,imm8;' Multiply r/m8 by 2, imm8 times.
	///
	/// 'shl r/m8**,imm8;' Multiply r/m8 by 2, imm8 times.
	///
	/// 'shl r/m16,1;' Multiply r/m16 by 2, once.
	///
	/// 'shl r/m16,CL;' Multiply r/m16 by 2, CL times.
	///
	/// 'shl r/m16,imm8;' Multiply r/m16 by 2, imm8 times.
	///
	/// 'shl r/m32,1;' Multiply r/m32 by 2, once.
	///
	/// 'shl r/m64,1;' Multiply r/m64 by 2, once.
	///
	/// 'shl r/m32,CL;' Multiply r/m32 by 2, CL times.
	///
	/// 'shl r/m64,CL;' Multiply r/m64 by 2, CL times.
	///
	/// 'shl r/m32,imm8;' Multiply r/m32 by 2, imm8 times.
	///
	/// 'shl r/m64,imm8;' Multiply r/m64 by 2, imm8 times.
	SHL,
	///
	/// 'shr r/m8,1;' Unsigned divide r/m8 by 2, once.
	///
	/// 'shr r/m8**,1;' Unsigned divide r/m8 by 2, once.
	///
	/// 'shr r/m8,CL;' Unsigned divide r/m8 by 2, CL times.
	///
	/// 'shr r/m8**,CL;' Unsigned divide r/m8 by 2, CL times.
	///
	/// 'shr r/m8,imm8;' Unsigned divide r/m8 by 2, imm8 times.
	///
	/// 'shr r/m8**,imm8;' Unsigned divide r/m8 by 2, imm8 times.
	///
	/// 'shr r/m16,1;' Unsigned divide r/m16 by 2, once.
	///
	/// 'shr r/m16,CL;' Unsigned divide r/m16 by 2, CL times.
	///
	/// 'shr r/m16,imm8;' Unsigned divide r/m16 by 2, imm8 times.
	///
	/// 'shr r/m32,1;' Unsigned divide r/m32 by 2, once.
	///
	/// 'shr r/m64,1;' Unsigned divide r/m64 by 2, once.
	///
	/// 'shr r/m32,CL;' Unsigned divide r/m32 by 2, CL times.
	///
	/// 'shr r/m64,CL;' Unsigned divide r/m64 by 2, CL times.
	///
	/// 'shr r/m32,imm8;' Unsigned divide r/m32 by 2, imm8 times.
	///
	/// 'shr r/m64,imm8;' Unsigned divide r/m64 by 2, imm8 times.
	SHR,
// SARX/SHLX/SHRX--Shift Without Affecting Flags.
	///
	/// 'sarx r32a,r/m32,r32b;' Shift r/m32 arithmetically right with count specified in r32b.
	///
	/// 'sarx r64a,r/m64,r64b;' Shift r/m64 arithmetically right with count specified in r64b.
	SARX,
	///
	/// 'shrx r32a,r/m32,r32b;' Shift r/m32 logically right with count specified in r32b.
	///
	/// 'shrx r64a,r/m64,r64b;' Shift r/m64 logically right with count specified in r64b.
	SHRX,
	///
	/// 'shlx r32a,r/m32,r32b;' Shift r/m32 logically left with count specified in r32b.
	///
	/// 'shlx r64a,r/m64,r64b;' Shift r/m64 logically left with count specified in r64b.
	SHLX,
// SBB--Integer Subtraction with Borrow.
	///
	/// 'sbb AL,imm8;' Subtract with borrow imm8 from AL.
	///
	/// 'sbb AX,imm16;' Subtract with borrow imm16 from AX.
	///
	/// 'sbb EAX,imm32;' Subtract with borrow imm32 from EAX.
	///
	/// 'sbb RAX,imm32;' Subtract with borrow sign-extended imm.32 to 64-bits from RAX.
	///
	/// 'sbb r/m8,imm8;' Subtract with borrow imm8 from r/m8.
	///
	/// 'sbb r/m8*,imm8;' Subtract with borrow imm8 from r/m8.
	///
	/// 'sbb r/m16,imm16;' Subtract with borrow imm16 from r/m16.
	///
	/// 'sbb r/m32,imm32;' Subtract with borrow imm32 from r/m32.
	///
	/// 'sbb r/m64,imm32;' Subtract with borrow sign-extended imm32 to 64-bits from r/m64.
	///
	/// 'sbb r/m16,imm8;' Subtract with borrow sign-extended imm8 from r/m16.
	///
	/// 'sbb r/m32,imm8;' Subtract with borrow sign-extended imm8 from r/m32.
	///
	/// 'sbb r/m64,imm8;' Subtract with borrow sign-extended imm8 from r/m64.
	///
	/// 'sbb r/m8,r8;' Subtract with borrow r8 from r/m8.
	///
	/// 'sbb r/m8*,r8;' Subtract with borrow r8 from r/m8.
	///
	/// 'sbb r/m16,r16;' Subtract with borrow r16 from r/m16.
	///
	/// 'sbb r/m32,r32;' Subtract with borrow r32 from r/m32.
	///
	/// 'sbb r/m64,r64;' Subtract with borrow r64 from r/m64.
	///
	/// 'sbb r8,r/m8;' Subtract with borrow r/m8 from r8.
	///
	/// 'sbb r8*,r/m8*;' Subtract with borrow r/m8 from r8.
	///
	/// 'sbb r16,r/m16;' Subtract with borrow r/m16 from r16.
	///
	/// 'sbb r32,r/m32;' Subtract with borrow r/m32 from r32.
	///
	/// 'sbb r64,r/m64;' Subtract with borrow r/m64 from r64.
	SBB,
// SCAS/SCASB/SCASW/SCASD--Scan String.
	///
	/// 'scasb;' Compare AL with byte at ES:(E)DI or RDI then set status flags.*.
	SCASB,
	///
	/// 'scasw;' Compare AX with word at ES:(E)DI or RDI then set status flags.*.
	SCASW,
	///
	/// 'scasq;' Compare RAX with quadword at RDI or EDI then set status flags.
	SCASQ,
	///
	/// 'scas m8;' Compare AL with byte at ES:(E)DI or RDI, then set status flags.*.
	///
	/// 'scas m16;' Compare AX with word at ES:(E)DI or RDI, then set status flags.*.
	///
	/// 'scas m32;' Compare EAX with doubleword at ES(E)DI or RDI then set status flags.*.
	///
	/// 'scas m64;' Compare RAX with quadword at RDI or EDI then set status flags.
	SCAS,
	///
	/// 'scasd;' Compare EAX with doubleword at ES:(E)DI or RDI then set status flags.*.
	SCASD,
// SETcc--Set Byte on Condition.
	///
	/// 'setl r/m8;' Set byte if less (SF != OF).
	///
	/// 'setl r/m8*;' Set byte if less (SF != OF).
	SETL,
	///
	/// 'setz r/m8;' Set byte if zero (ZF=1).
	///
	/// 'setz r/m8*;' Set byte if zero (ZF=1).
	SETZ,
	///
	/// 'setp r/m8;' Set byte if parity (PF=1).
	///
	/// 'setp r/m8*;' Set byte if parity (PF=1).
	SETP,
	///
	/// 'setne r/m8;' Set byte if not equal (ZF=0).
	///
	/// 'setne r/m8*;' Set byte if not equal (ZF=0).
	SETNE,
	///
	/// 'setnl r/m8;' Set byte if not less (SF=OF).
	///
	/// 'setnl r/m8*;' Set byte if not less (SF=OF).
	SETNL,
	///
	/// 'setng r/m8;' Set byte if not greater (ZF=1 or SF != OF).
	///
	/// 'setng r/m8*;' Set byte if not greater (ZF=1 or SF != OF).
	SETNG,
	///
	/// 'setg r/m8;' Set byte if greater (ZF=0 and SF=OF).
	///
	/// 'setg r/m8*;' Set byte if greater (ZF=0 and SF=OF).
	SETG,
	///
	/// 'setle r/m8;' Set byte if less or equal (ZF=1 or SF != OF).
	///
	/// 'setle r/m8*;' Set byte if less or equal (ZF=1 or SF != OF).
	SETLE,
	///
	/// 'setge r/m8;' Set byte if greater or equal (SF=OF).
	///
	/// 'setge r/m8*;' Set byte if greater or equal (SF=OF).
	SETGE,
	///
	/// 'setnae r/m8;' Set byte if not above or equal (CF=1).
	///
	/// 'setnae r/m8*;' Set byte if not above or equal (CF=1).
	SETNAE,
	///
	/// 'setnc r/m8;' Set byte if not carry (CF=0).
	///
	/// 'setnc r/m8*;' Set byte if not carry (CF=0).
	SETNC,
	///
	/// 'setnge r/m8;' Set byte if not greater or equal (SF != OF).
	///
	/// 'setnge r/m8*;' Set byte if not greater or equal (SF != OF).
	SETNGE,
	///
	/// 'setns r/m8;' Set byte if not sign (SF=0).
	///
	/// 'setns r/m8*;' Set byte if not sign (SF=0).
	SETNS,
	///
	/// 'seto r/m8;' Set byte if overflow (OF=1).
	///
	/// 'seto r/m8*;' Set byte if overflow (OF=1).
	SETO,
	///
	/// 'setnz r/m8;' Set byte if not zero (ZF=0).
	///
	/// 'setnz r/m8*;' Set byte if not zero (ZF=0).
	SETNZ,
	///
	/// 'setnb r/m8;' Set byte if not below (CF=0).
	///
	/// 'setnb r/m8*;' Set byte if not below (CF=0).
	SETNB,
	///
	/// 'setnbe r/m8;' Set byte if not below or equal (CF=0 and ZF=0).
	///
	/// 'setnbe r/m8*;' Set byte if not below or equal (CF=0 and ZF=0).
	SETNBE,
	///
	/// 'setc r/m8;' Set byte if carry (CF=1).
	///
	/// 'setc r/m8*;' Set byte if carry (CF=1).
	SETC,
	///
	/// 'setnp r/m8;' Set byte if not parity (PF=0).
	///
	/// 'setnp r/m8*;' Set byte if not parity (PF=0).
	SETNP,
	///
	/// 'setpo r/m8;' Set byte if parity odd (PF=0).
	///
	/// 'setpo r/m8*;' Set byte if parity odd (PF=0).
	SETPO,
	///
	/// 'sets r/m8;' Set byte if sign (SF=1).
	///
	/// 'sets r/m8*;' Set byte if sign (SF=1).
	SETS,
	///
	/// 'setbe r/m8;' Set byte if below or equal (CF=1 or ZF=1).
	///
	/// 'setbe r/m8*;' Set byte if below or equal (CF=1 or ZF=1).
	SETBE,
	///
	/// 'setnle r/m8;' Set byte if not less or equal (ZF=0 and SF=OF).
	///
	/// 'setnle r/m8*;' Set byte if not less or equal (ZF=0 and SF=OF).
	SETNLE,
	///
	/// 'sete r/m8;' Set byte if equal (ZF=1).
	///
	/// 'sete r/m8*;' Set byte if equal (ZF=1).
	SETE,
	///
	/// 'setpe r/m8;' Set byte if parity even (PF=1).
	///
	/// 'setpe r/m8*;' Set byte if parity even (PF=1).
	SETPE,
	///
	/// 'setna r/m8;' Set byte if not above (CF=1 or ZF=1).
	///
	/// 'setna r/m8*;' Set byte if not above (CF=1 or ZF=1).
	SETNA,
	///
	/// 'seta r/m8;' Set byte if above (CF=0 and ZF=0).
	///
	/// 'seta r/m8*;' Set byte if above (CF=0 and ZF=0).
	SETA,
	///
	/// 'setae r/m8;' Set byte if above or equal (CF=0).
	///
	/// 'setae r/m8*;' Set byte if above or equal (CF=0).
	SETAE,
	///
	/// 'setno r/m8;' Set byte if not overflow (OF=0).
	///
	/// 'setno r/m8*;' Set byte if not overflow (OF=0).
	SETNO,
	///
	/// 'setb r/m8;' Set byte if below (CF=1).
	///
	/// 'setb r/m8*;' Set byte if below (CF=1).
	SETB,
// SFENCE--Store Fence.
	///
	/// 'sfence;' Serializes store operations.
	SFENCE,
// SGDT--Store Global Descriptor Table Register.
	///
	/// 'sgdt m;' Store GDTR to m.
	SGDT,
// SHLD--Double Precision Shift Left.
	///
	/// 'shld r/m16,r16,imm8;' Shift r/m16 to left imm8 places while shifting bits from r16 in from the right.
	///
	/// 'shld r/m16,r16,CL;' Shift r/m16 to left CL places while shifting bits from r16 in from the right.
	///
	/// 'shld r/m32,r32,imm8;' Shift r/m32 to left imm8 places while shifting bits from r32 in from the right.
	///
	/// 'shld r/m64,r64,imm8;' Shift r/m64 to left imm8 places while shifting bits from r64 in from the right.
	///
	/// 'shld r/m32,r32,CL;' Shift r/m32 to left CL places while shifting bits from r32 in from the right.
	///
	/// 'shld r/m64,r64,CL;' Shift r/m64 to left CL places while shifting bits from r64 in from the right.
	SHLD,
// SHRD--Double Precision Shift Right.
	///
	/// 'shrd r/m16,r16,imm8;' Shift r/m16 to right imm8 places while shifting bits from r16 in from the left.
	///
	/// 'shrd r/m16,r16,CL;' Shift r/m16 to right CL places while shifting bits from r16 in from the left.
	///
	/// 'shrd r/m32,r32,imm8;' Shift r/m32 to right imm8 places while shifting bits from r32 in from the left.
	///
	/// 'shrd r/m64,r64,imm8;' Shift r/m64 to right imm8 places while shifting bits from r64 in from the left.
	///
	/// 'shrd r/m32,r32,CL;' Shift r/m32 to right CL places while shifting bits from r32 in from the left.
	///
	/// 'shrd r/m64,r64,CL;' Shift r/m64 to right CL places while shifting bits from r64 in from the left.
	SHRD,
// SHUFPD--Shuffle Packed Double-Precision Floating-Point Values.
	///
	/// 'vshufpd xmm1,xmm2,xmm3/m128,imm8;' Shuffle Packed double-precision floatingpoint values selected by imm8 from xmm2 and xmm3/mem.
	///
	/// 'vshufpd ymm1,ymm2,ymm3/m256,imm8;' Shuffle Packed double-precision floatingpoint values selected by imm8 from ymm2 and ymm3/mem.
	VSHUFPD,
	///
	/// 'shufpd xmm1,xmm2/m128,imm8;' Shuffle packed double-precision floatingpoint values selected by imm8 from xmm1 and xmm2/m128 to xmm1.
	SHUFPD,
// SHUFPS--Shuffle Packed Single-Precision Floating-Point Values.
	///
	/// 'shufps xmm1,xmm2/m128,imm8;' Shuffle packed single-precision floating-point values selected by imm8 from xmm1 and xmm1/m128 to xmm1.
	SHUFPS,
	///
	/// 'vshufps xmm1,xmm2,xmm3/m128,imm8;' Shuffle Packed single-precision floating-point values selected by imm8 from xmm2 and xmm3/mem.
	///
	/// 'vshufps ymm1,ymm2,ymm3/m256,imm8;' Shuffle Packed single-precision floating-point values selected by imm8 from ymm2 and ymm3/mem.
	VSHUFPS,
// SIDT--Store Interrupt Descriptor Table Register.
	///
	/// 'sidt m;' Store IDTR to m.
	SIDT,
// SLDT--Store Local Descriptor Table Register.
	///
	/// 'sldt r/m16;' Stores segment selector from LDTR in r/m16.
	///
	/// 'sldt r64/m16;' Stores segment selector from LDTR in r64/m16.
	SLDT,
// SMSW--Store Machine Status Word.
	///
	/// 'smsw r/m16;' Store machine status word to r/m16.
	///
	/// 'smsw r32/m16;' Store machine status word in low-order 16 bits of r32/m16; high-order 16 bits of r32 are undefined.
	///
	/// 'smsw r64/m16;' Store machine status word in low-order 16 bits of r64/m16; high-order 16 bits of r32 are undefined.
	SMSW,
// SQRTPD--Compute Square Roots of Packed Double-Precision Floating-Point Values.
	///
	/// 'sqrtpd xmm1,xmm2/m128;' Computes square roots of the packed doubleprecision floating-point values in xmm2/m128 and stores the results in xmm1.
	SQRTPD,
	///
	/// 'vsqrtpd xmm1,xmm2/m128;' Computes Square Roots of the packed doubleprecision floating-point values in xmm2/m128 and stores the result in xmm1.
	///
	/// 'vsqrtpd ymm1,ymm2/m256;' Computes Square Roots of the packed doubleprecision floating-point values in ymm2/m256 and stores the result in ymm1.
	VSQRTPD,
// SQRTPS--Compute Square Roots of Packed Single-Precision Floating-Point Values.
	///
	/// 'sqrtps xmm1,xmm2/m128;' Computes square roots of the packed singleprecision floating-point values in xmm2/m128 and stores the results in xmm1.
	SQRTPS,
	///
	/// 'vsqrtps xmm1,xmm2/m128;' Computes Square Roots of the packed singleprecision floating-point values in xmm2/m128 and stores the result in xmm1.
	///
	/// 'vsqrtps ymm1,ymm2/m256;' Computes Square Roots of the packed singleprecision floating-point values in ymm2/m256 and stores the result in ymm1.
	VSQRTPS,
// SQRTSD--Compute Square Root of Scalar Double-Precision Floating-Point Value.
	///
	/// 'sqrtsd xmm1,xmm2/m64;' Computes square root of the low doubleprecision floating-point value in xmm2/m64 and stores the results in xmm1.
	SQRTSD,
	///
	/// 'vsqrtsd xmm1,xmm2,xmm3/m64;' Computes square root of the low doubleprecision floating point value in xmm3/m64 and stores the results in xmm2. Also, upper double precision floating-point value (bits[127:64]) from xmm2 are copied to xmm1[127:64].
	VSQRTSD,
// SQRTSS--Compute Square Root of Scalar Single-Precision Floating-Point Value.
	///
	/// 'vsqrtss xmm1,xmm2,xmm3/m32;' Computes square root of the low singleprecision floating-point value in xmm3/m32 and stores the results in xmm1. Also, upper single precision floating-point values (bits[127:32]) from xmm2 are copied to xmm1[127:32].
	VSQRTSS,
	///
	/// 'sqrtss xmm1,xmm2/m32;' Computes square root of the low singleprecision floating-point value in xmm2/m32 and stores the results in xmm1.
	SQRTSS,
// STAC--Set AC Flag in EFLAGS Register.
	///
	/// 'stac;' Set the AC flag in the EFLAGS register.
	STAC,
// STC--Set Carry Flag.
	///
	/// 'stc;' Set CF flag.
	STC,
// STD--Set Direction Flag.
	///
	/// 'std;' Set DF flag.
	STD,
// STI--Set Interrupt Flag.
	///
	/// 'sti;' Set interrupt flag; external, maskable interrupts enabled at the end of the next instruction.
	STI,
// STMXCSR--Store MXCSR Register State.
	///
	/// 'stmxcsr m32;' Store contents of MXCSR register to m32.
	STMXCSR,
	///
	/// 'vstmxcsr m32;' Store contents of MXCSR register to m32.
	VSTMXCSR,
// STOS/STOSB/STOSW/STOSD/STOSQ--Store String.
	///
	/// 'stosb;' For legacy mode, store AL at address ES:(E)DI; For 64-bit mode store AL at address RDI or EDI.
	STOSB,
	///
	/// 'stos m8;' For legacy mode, store AL at address ES:(E)DI; For 64-bit mode store AL at address RDI or EDI.
	///
	/// 'stos m16;' For legacy mode, store AX at address ES:(E)DI; For 64-bit mode store AX at address RDI or EDI.
	///
	/// 'stos m32;' For legacy mode, store EAX at address ES:(E)DI; For 64-bit mode store EAX at address RDI or EDI.
	///
	/// 'stos m64;' Store RAX at address RDI or EDI.
	STOS,
	///
	/// 'stosd;' For legacy mode, store EAX at address ES:(E)DI; For 64-bit mode store EAX at address RDI or EDI.
	STOSD,
	///
	/// 'stosw;' For legacy mode, store AX at address ES:(E)DI; For 64-bit mode store AX at address RDI or EDI.
	STOSW,
	///
	/// 'stosq;' Store RAX at address RDI or EDI.
	STOSQ,
// STR--Store Task Register.
	///
	/// 'str r/m16;' Stores segment selector from TR in r/m16.
	STR,
// SUB--Subtract.
	///
	/// 'sub AL,imm8;' Subtract imm8 from AL.
	///
	/// 'sub AX,imm16;' Subtract imm16 from AX.
	///
	/// 'sub EAX,imm32;' Subtract imm32 from EAX.
	///
	/// 'sub RAX,imm32;' Subtract imm32 sign-extended to 64-bits from RAX.
	///
	/// 'sub r/m8,imm8;' Subtract imm8 from r/m8.
	///
	/// 'sub r/m8*,imm8;' Subtract imm8 from r/m8.
	///
	/// 'sub r/m16,imm16;' Subtract imm16 from r/m16.
	///
	/// 'sub r/m32,imm32;' Subtract imm32 from r/m32.
	///
	/// 'sub r/m64,imm32;' Subtract imm32 sign-extended to 64-bits from r/m64.
	///
	/// 'sub r/m16,imm8;' Subtract sign-extended imm8 from r/m16.
	///
	/// 'sub r/m32,imm8;' Subtract sign-extended imm8 from r/m32.
	///
	/// 'sub r/m64,imm8;' Subtract sign-extended imm8 from r/m64.
	///
	/// 'sub r/m8,r8;' Subtract r8 from r/m8.
	///
	/// 'sub r/m8*,r8*;' Subtract r8 from r/m8.
	///
	/// 'sub r/m16,r16;' Subtract r16 from r/m16.
	///
	/// 'sub r/m32,r32;' Subtract r32 from r/m32.
	///
	/// 'sub r/m64,r64;' Subtract r64 from r/m64.
	///
	/// 'sub r8,r/m8;' Subtract r/m8 from r8.
	///
	/// 'sub r8*,r/m8*;' Subtract r/m8 from r8.
	///
	/// 'sub r16,r/m16;' Subtract r/m16 from r16.
	///
	/// 'sub r32,r/m32;' Subtract r/m32 from r32.
	///
	/// 'sub r64,r/m64;' Subtract r/m64 from r64.
	SUB,
// SUBPD--Subtract Packed Double-Precision Floating-Point Values.
	///
	/// 'vsubpd xmm1,xmm2,xmm3/m128;' Subtract packed double-precision floatingpoint values in xmm3/mem from xmm2 and stores result in xmm1.
	///
	/// 'vsubpd ymm1,ymm2,ymm3/m256;' Subtract packed double-precision floatingpoint values in ymm3/mem from ymm2 and stores result in ymm1.
	VSUBPD,
	///
	/// 'subpd xmm1,xmm2/m128;' Subtract packed double-precision floatingpoint values in xmm2/m128 from xmm1.
	SUBPD,
// SUBPS--Subtract Packed Single-Precision Floating-Point Values.
	///
	/// 'subps xmm1 xmm2/m128;' Subtract packed single-precision floating-point values in xmm2/mem from xmm1.
	SUBPS,
	///
	/// 'vsubps xmm1,xmm2,xmm3/m128;' Subtract packed single-precision floating-point values in xmm3/mem from xmm2 and stores result in xmm1.
	///
	/// 'vsubps ymm1,ymm2,ymm3/m256;' Subtract packed single-precision floating-point values in ymm3/mem from ymm2 and stores result in ymm1.
	VSUBPS,
// SUBSD--Subtract Scalar Double-Precision Floating-Point Values.
	///
	/// 'subsd xmm1,xmm2/m64;' Subtracts the low double-precision floatingpoint values in xmm2/mem64 from xmm1.
	SUBSD,
	///
	/// 'vsubsd xmm1,xmm2,xmm3/m64;' Subtract the low double-precision floatingpoint value in xmm3/mem from xmm2 and store the result in xmm1.
	VSUBSD,
// SUBSS--Subtract Scalar Single-Precision Floating-Point Values.
	///
	/// 'subss xmm1,xmm2/m32;' Subtract the lower single-precision floatingpoint values in xmm2/m32 from xmm1.
	SUBSS,
	///
	/// 'vsubss xmm1,xmm2,xmm3/m32;' Subtract the low single-precision floatingpoint value in xmm3/mem from xmm2 and store the result in xmm1.
	VSUBSS,
// SWAPGS--Swap GS Base Register.
// SYSCALL--Fast System Call.
// SYSENTER--Fast System Call.
	///
	/// 'sysenter;' Fast call to privilege level 0 system procedures.
	SYSENTER,
// SYSEXIT--Fast Return from Fast System Call.
	///
	/// 'sysexit;' Fast return to privilege level 3 user code.
	///
	/// 'sysexit;' Fast return to 64-bit mode privilege level 3 user code.
	SYSEXIT,
// SYSRET--Return From Fast System Call.
// TEST--Logical Compare.
	///
	/// 'test AL,imm8;' AND imm8 with AL; set SF, ZF, PF according to result.
	///
	/// 'test AX,imm16;' AND imm16 with AX; set SF, ZF, PF according to result.
	///
	/// 'test EAX,imm32;' AND imm32 with EAX; set SF, ZF, PF according to result.
	///
	/// 'test RAX,imm32;' AND imm32 sign-extended to 64-bits with RAX; set SF, ZF, PF according to result.
	///
	/// 'test r/m8,imm8;' AND imm8 with r/m8; set SF, ZF, PF according to result.
	///
	/// 'test r/m8*,imm8;' AND imm8 with r/m8; set SF, ZF, PF according to result.
	///
	/// 'test r/m16,imm16;' AND imm16 with r/m16; set SF, ZF, PF according to result.
	///
	/// 'test r/m32,imm32;' AND imm32 with r/m32; set SF, ZF, PF according to result.
	///
	/// 'test r/m64,imm32;' AND imm32 sign-extended to 64-bits with r/m64; set SF, ZF, PF according to result.
	///
	/// 'test r/m8,r8;' AND r8 with r/m8; set SF, ZF, PF according to result.
	///
	/// 'test r/m8*,r8*;' AND r8 with r/m8; set SF, ZF, PF according to result.
	///
	/// 'test r/m16,r16;' AND r16 with r/m16; set SF, ZF, PF according to result.
	///
	/// 'test r/m32,r32;' AND r32 with r/m32; set SF, ZF, PF according to result.
	///
	/// 'test r/m64,r64;' AND r64 with r/m64; set SF, ZF, PF according to result.
	TEST,
// TZCNT--Count the Number of Trailing Zero Bits.
	///
	/// 'tzcnt r16,r/m16;' Count the number of trailing zero bits in r/m16, return result in r16.
	///
	/// 'tzcnt r32,r/m32;' Count the number of trailing zero bits in r/m32, return result in r32.
	///
	/// 'tzcnt r64,r/m64;' Count the number of trailing zero bits in r/m64, return result in r64.
	TZCNT,
// UCOMISD--Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS.
	///
	/// 'ucomisd xmm1,xmm2/m64;' Compares (unordered) the low doubleprecision floating-point values in xmm1 and xmm2/m64 and set the EFLAGS accordingly.
	UCOMISD,
	///
	/// 'vucomisd xmm1,xmm2/m64;' Compare low double precision floating-point values in xmm1 and xmm2/mem64 and set the EFLAGS flags accordingly.
	VUCOMISD,
// UCOMISS--Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS.
	///
	/// 'vucomiss xmm1,xmm2/m32;' Compare low single precision floating-point values in xmm1 and xmm2/mem32 and set the EFLAGS flags accordingly.
	VUCOMISS,
	///
	/// 'ucomiss xmm1,xmm2/m32;' Compare lower single-precision floating-point value in xmm1 register with lower singleprecision floating-point value in xmm2/mem and set the status flags accordingly.
	UCOMISS,
// UD2--Undefined Instruction.
	///
	/// 'ud2;' Raise invalid opcode exception.
	UD2,
// UNPCKHPD--Unpack and Interleave High Packed Double-Precision Floating-Point Values.
	///
	/// 'vunpckhpd xmm1,xmm2,xmm3/m128;' Unpacks and Interleaves double precision floating-point values from high quadwords of xmm2 and xmm3/m128.
	///
	/// 'vunpckhpd ymm1,ymm2,ymm3/m256;' Unpacks and Interleaves double precision floating-point values from high quadwords of ymm2 and ymm3/m256.
	VUNPCKHPD,
	///
	/// 'unpckhpd xmm1,xmm2/m128;' Unpacks and Interleaves double-precision floating-point values from high quadwords of xmm1 and xmm2/m128.
	UNPCKHPD,
// UNPCKHPS--Unpack and Interleave High Packed Single-Precision Floating-Point Values.
	///
	/// 'vunpckhps xmm1,xmm2,xmm3/m128;' Unpacks and Interleaves single-precision floating-point values from high quadwords of xmm2 and xmm3/m128.
	///
	/// 'vunpckhps ymm1,ymm2,ymm3/m256;' Unpacks and Interleaves single-precision floating-point values from high quadwords of ymm2 and ymm3/m256.
	VUNPCKHPS,
	///
	/// 'unpckhps xmm1,xmm2/m128;' Unpacks and Interleaves single-precision floating-point values from high quadwords of xmm1 and xmm2/mem into xmm1.
	UNPCKHPS,
// UNPCKLPD--Unpack and Interleave Low Packed Double-Precision Floating-Point Values.
	///
	/// 'vunpcklpd xmm1,xmm2,xmm3/m128;' Unpacks and Interleaves double precision floating-point values low high quadwords of xmm2 and xmm3/m128.
	///
	/// 'vunpcklpd ymm1,ymm2,ymm3/m256;' Unpacks and Interleaves double precision floating-point values low high quadwords of ymm2 and ymm3/m256.
	VUNPCKLPD,
	///
	/// 'unpcklpd xmm1,xmm2/m128;' Unpacks and Interleaves double-precision floating-point values from low quadwords of xmm1 and xmm2/m128.
	UNPCKLPD,
// UNPCKLPS--Unpack and Interleave Low Packed Single-Precision Floating-Point Values.
	///
	/// 'unpcklps xmm1,xmm2/m128;' Unpacks and Interleaves single-precision floating-point values from low quadwords of xmm1 and xmm2/mem into xmm1.
	UNPCKLPS,
	///
	/// 'vunpcklps xmm1,xmm2,xmm3/m128;' Unpacks and Interleaves single-precision floating-point values from low quadwords of xmm2 and xmm3/m128.
	///
	/// 'vunpcklps ymm1,ymm2,ymm3/m256;' Unpacks and Interleaves single-precision floating-point values from low quadwords of ymm2 and ymm3/m256.
	VUNPCKLPS,
// VBROADCAST--Broadcast Floating-Point Data.
	///
	/// 'vbroadcastss xmm1,m32;' Broadcast single-precision floating-point element in mem to four locations in xmm1.
	///
	/// 'vbroadcastss ymm1,m32;' Broadcast single-precision floating-point element in mem to eight locations in ymm1.
	///
	/// 'vbroadcastss xmm1,xmm2;' Broadcast the low single-precision floatingpoint element in the source operand to four locations in xmm1.
	///
	/// 'vbroadcastss ymm1,xmm2;' Broadcast low single-precision floating-point element in the source operand to eight locations in ymm1.
	VBROADCASTSS,
	///
	/// 'vbroadcastf128 ymm1,m128;' Broadcast 128 bits of floating-point data in mem to low and high 128-bits in ymm1.
	VBROADCASTF128,
	///
	/// 'vbroadcastsd ymm1,m64;' Broadcast double-precision floating-point element in mem to four locations in ymm1.
	///
	/// 'vbroadcastsd ymm1,xmm2;' Broadcast low double-precision floating-point element in the source operand to four locations in ymm1.
	VBROADCASTSD,
// VCVTPH2PS--Convert 16-bit FP Values to Single-Precision FP Values.
	///
	/// 'vcvtph2ps ymm1,xmm2/m128;' Convert eight packed half precision (16-bit) floating-point values in xmm2/m128 to packed single-precision floating-point value in ymm1.
	///
	/// 'vcvtph2ps xmm1,xmm2/m64;' Convert four packed half precision (16-bit) floating-point values in xmm2/m64 to packed single-precision floating-point value in xmm1.
	VCVTPH2PS,
// VCVTPS2PH--Convert Single-Precision FP value to 16-bit FP value.
	///
	/// 'vcvtps2ph xmm1/m128,ymm2,imm8;' Convert eight packed single-precision floating-point value in ymm2 to packed half-precision (16-bit) floating-point value in xmm1/mem. Imm8 provides rounding controls.
	///
	/// 'vcvtps2ph xmm1/m64,xmm2,imm8;' Convert four packed single-precision floating-point value in xmm2 to packed halfprecision (16-bit) floating-point value in xmm1/mem. Imm8 provides rounding controls.
	VCVTPS2PH,
// VERR/VERW--Verify a Segment for Reading or Writing.
	///
	/// 'verw r/m16;' Set ZF=1 if segment specified with r/m16 can be written.
	VERW,
	///
	/// 'verr r/m16;' Set ZF=1 if segment specified with r/m16 can be read.
	VERR,
// VEXTRACTF128--Extract Packed Floating-Point Values.
	///
	/// 'vextractf128 xmm1/m128,ymm2,imm8;' Extract 128 bits of packed floating-point values from ymm2 and store results in xmm1/mem.
	VEXTRACTF128,
// VEXTRACTI128--Extract packed Integer Values.
	///
	/// 'vextracti128 xmm1/m128,ymm2,imm8;' Extract 128 bits of integer data from ymm2 and store results in xmm1/mem.
	VEXTRACTI128,
// VFMADD132PD/VFMADD213PD/VFMADD231PD--Fused Multiply-Add of Packed Double-Precision Floating-Point Values.
	///
	/// 'vfmadd132pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm2/mem, add to xmm1 and put result in xmm0.
	///
	/// 'vfmadd132pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm2/mem, add to ymm1 and put result in ymm0.
	VFMADD132PD,
	///
	/// 'vfmadd231pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm1 and xmm2/mem, add to xmm0 and put result in xmm0.
	///
	/// 'vfmadd231pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm1 and ymm2/mem, add to ymm0 and put result in ymm0.
	VFMADD231PD,
	///
	/// 'vfmadd213pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm1, add to xmm2/mem and put result in xmm0.
	///
	/// 'vfmadd213pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm1, add to ymm2/mem and put result in ymm0.
	VFMADD213PD,
// VFMADD132PS/VFMADD213PS/VFMADD231PS--Fused Multiply-Add of Packed Single-Precision Floating-Point Values.
	///
	/// 'vfmadd213ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm1, add to xmm2/mem and put result in xmm0.
	///
	/// 'vfmadd213ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm1, add to ymm2/mem and put result in ymm0.
	VFMADD213PS,
	///
	/// 'vfmadd231ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm1 and xmm2/mem, add to xmm0 and put result in xmm0.
	///
	/// 'vfmadd231ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm1 and ymm2/mem, add to ymm0 and put result in ymm0.
	VFMADD231PS,
	///
	/// 'vfmadd132ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm2/mem, add to xmm1 and put result in xmm0.
	///
	/// 'vfmadd132ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm2/mem, add to ymm1 and put result in ymm0.
	VFMADD132PS,
// VFMADD132SD/VFMADD213SD/VFMADD231SD--Fused Multiply-Add of Scalar Double-Precision Floating-Point Values.
	///
	/// 'vfmadd132sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm2/mem, add to xmm1 and put result in xmm0.
	VFMADD132SD,
	///
	/// 'vfmadd231sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm1 and xmm2/mem, add to xmm0 and put result in xmm0.
	VFMADD231SD,
	///
	/// 'vfmadd213sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm1, add to xmm2/mem and put result in xmm0.
	VFMADD213SD,
// VFMADD132SS/VFMADD213SS/VFMADD231SS--Fused Multiply-Add of Scalar Single-Precision Floating-Point Values.
	///
	/// 'vfmadd132ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm2/mem, add to xmm1 and put result in xmm0.
	VFMADD132SS,
	///
	/// 'vfmadd213ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm1, add to xmm2/mem and put result in xmm0.
	VFMADD213SS,
	///
	/// 'vfmadd231ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm1 and xmm2/mem, add to xmm0 and put result in xmm0.
	VFMADD231SS,
// VFMADDSUB132PD/VFMADDSUB213PD/VFMADDSUB231PD--Fused Multiply-Alternating Add/Subtract of Packed Double-Precision Floating-Point Values.
	///
	/// 'vfmaddsub132pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm2/mem, add/subtract elements in xmm1 and put result in xmm0.
	///
	/// 'vfmaddsub132pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm2/mem, add/subtract elements in ymm1 and put result in ymm0.
	VFMADDSUB132PD,
	///
	/// 'vfmaddsub231pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm1 and xmm2/mem, add/subtract elements in xmm0 and put result in xmm0.
	///
	/// 'vfmaddsub231pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm1 and ymm2/mem, add/subtract elements in ymm0 and put result in ymm0.
	VFMADDSUB231PD,
	///
	/// 'vfmaddsub213pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm1, add/subtract elements in xmm2/mem and put result in xmm0.
	///
	/// 'vfmaddsub213pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm1, add/subtract elements in ymm2/mem and put result in ymm0.
	VFMADDSUB213PD,
// VFMADDSUB132PS/VFMADDSUB213PS/VFMADDSUB231PS--Fused Multiply-Alternating Add/Subtract of Packed Single-Precision Floating-Point Values.
	///
	/// 'vfmaddsub132ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm2/mem, add/subtract elements in xmm1 and put result in xmm0.
	///
	/// 'vfmaddsub132ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm2/mem, add/subtract elements in ymm1 and put result in ymm0.
	VFMADDSUB132PS,
	///
	/// 'vfmaddsub213ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm1, add/subtract elements in xmm2/mem and put result in xmm0.
	///
	/// 'vfmaddsub213ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm1, add/subtract elements in ymm2/mem and put result in ymm0.
	VFMADDSUB213PS,
	///
	/// 'vfmaddsub231ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm1 and xmm2/mem, add/subtract elements in xmm0 and put result in xmm0.
	///
	/// 'vfmaddsub231ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm1 and ymm2/mem, add/subtract elements in ymm0 and put result in ymm0.
	VFMADDSUB231PS,
// VFMSUBADD132PD/VFMSUBADD213PD/VFMSUBADD231PD--Fused Multiply-Alternating Subtract/Add of Packed Double-Precision Floating-Point Values.
	///
	/// 'vfmsubadd213pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm1, subtract/add elements in xmm2/mem and put result in xmm0.
	///
	/// 'vfmsubadd213pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm1, subtract/add elements in ymm2/mem and put result in ymm0.
	VFMSUBADD213PD,
	///
	/// 'vfmsubadd231pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm1 and xmm2/mem, subtract/add elements in xmm0 and put result in xmm0.
	///
	/// 'vfmsubadd231pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm1 and ymm2/mem, subtract/add elements in ymm0 and put result in ymm0.
	VFMSUBADD231PD,
	///
	/// 'vfmsubadd132pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm2/mem, subtract/add elements in xmm1 and put result in xmm0.
	///
	/// 'vfmsubadd132pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm2/mem, subtract/add elements in ymm1 and put result in ymm0.
	VFMSUBADD132PD,
// VFMSUBADD132PS/VFMSUBADD213PS/VFMSUBADD231PS--Fused Multiply-Alternating Subtract/Add of Packed Single-Precision Floating-Point Values.
	///
	/// 'vfmsubadd132ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm2/mem, subtract/add elements in xmm1 and put result in xmm0.
	///
	/// 'vfmsubadd132ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm2/mem, subtract/add elements in ymm1 and put result in ymm0.
	VFMSUBADD132PS,
	///
	/// 'vfmsubadd213ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm1, subtract/add elements in xmm2/mem and put result in xmm0.
	///
	/// 'vfmsubadd213ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm1, subtract/add elements in ymm2/mem and put result in ymm0.
	VFMSUBADD213PS,
	///
	/// 'vfmsubadd231ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm1 and xmm2/mem, subtract/add elements in xmm0 and put result in xmm0.
	///
	/// 'vfmsubadd231ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm1 and ymm2/mem, subtract/add elements in ymm0 and put result in ymm0.
	VFMSUBADD231PS,
// VFMSUB132PD/VFMSUB213PD/VFMSUB231PD--Fused Multiply-Subtract of Packed Double-Precision Floating-Point Values.
	///
	/// 'vfmsub231pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm1 and xmm2/mem, subtract xmm0 and put result in xmm0.
	///
	/// 'vfmsub231pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm1 and ymm2/mem, subtract ymm0 and put result in ymm0.
	VFMSUB231PD,
	///
	/// 'vfmsub213pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm1, subtract xmm2/mem and put result in xmm0.
	///
	/// 'vfmsub213pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm1, subtract ymm2/mem and put result in ymm0.
	VFMSUB213PD,
	///
	/// 'vfmsub132pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm2/mem, subtract xmm1 and put result in xmm0.
	///
	/// 'vfmsub132pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm2/mem, subtract ymm1 and put result in ymm0.
	VFMSUB132PD,
// VFMSUB132PS/VFMSUB213PS/VFMSUB231PS--Fused Multiply-Subtract of Packed Single-Precision Floating-Point Values.
	///
	/// 'vfmsub132ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm2/mem, subtract xmm1 and put result in xmm0.
	///
	/// 'vfmsub132ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm2/mem, subtract ymm1 and put result in ymm0.
	VFMSUB132PS,
	///
	/// 'vfmsub213ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm1, subtract xmm2/mem and put result in xmm0.
	///
	/// 'vfmsub213ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm1, subtract ymm2/mem and put result in ymm0.
	VFMSUB213PS,
	///
	/// 'vfmsub231ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm1 and xmm2/mem, subtract xmm0 and put result in xmm0.
	///
	/// 'vfmsub231ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm1 and ymm2/mem, subtract ymm0 and put result in ymm0.
	VFMSUB231PS,
// VFMSUB132SD/VFMSUB213SD/VFMSUB231SD--Fused Multiply-Subtract of Scalar Double-Precision Floating-Point Values.
	///
	/// 'vfmsub231sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm1 and xmm2/mem, subtract xmm0 and put result in xmm0.
	VFMSUB231SD,
	///
	/// 'vfmsub132sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm2/mem, subtract xmm1 and put result in xmm0.
	VFMSUB132SD,
	///
	/// 'vfmsub213sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm1, subtract xmm2/mem and put result in xmm0.
	VFMSUB213SD,
// VFMSUB132SS/VFMSUB213SS/VFMSUB231SS--Fused Multiply-Subtract of Scalar Single-Precision Floating-Point Values.
	///
	/// 'vfmsub132ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm2/mem, subtract xmm1 and put result in xmm0.
	VFMSUB132SS,
	///
	/// 'vfmsub231ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm1 and xmm2/mem, subtract xmm0 and put result in xmm0.
	VFMSUB231SS,
	///
	/// 'vfmsub213ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm1, subtract xmm2/mem and put result in xmm0.
	VFMSUB213SS,
// VFNMADD132PD/VFNMADD213PD/VFNMADD231PD--Fused Negative Multiply-Add of Packed Double-Precision Floating-Point Values.
	///
	/// 'vfnmadd231pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm1 and xmm2/mem, negate the multiplication result and add to xmm0 and put result in xmm0.
	///
	/// 'vfnmadd231pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm1 and ymm2/mem, negate the multiplication result and add to ymm0 and put result in ymm0.
	VFNMADD231PD,
	///
	/// 'vfnmadd213pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm1, negate the multiplication result and add to xmm2/mem and put result in xmm0.
	///
	/// 'vfnmadd213pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm1, negate the multiplication result and add to ymm2/mem and put result in ymm0.
	VFNMADD213PD,
	///
	/// 'vfnmadd132pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm2/mem, negate the multiplication result and add to xmm1 and put result in xmm0.
	///
	/// 'vfnmadd132pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm2/mem, negate the multiplication result and add to ymm1 and put result in ymm0.
	VFNMADD132PD,
// VFNMADD132PS/VFNMADD213PS/VFNMADD231PS--Fused Negative Multiply-Add of Packed Single-Precision Floating-Point Values.
	///
	/// 'vfnmadd213ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm1, negate the multiplication result and add to xmm2/mem and put result in xmm0.
	///
	/// 'vfnmadd213ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm1, negate the multiplication result and add to ymm2/mem and put result in ymm0.
	VFNMADD213PS,
	///
	/// 'vfnmadd132ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm2/mem, negate the multiplication result and add to xmm1 and put result in xmm0.
	///
	/// 'vfnmadd132ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm2/mem, negate the multiplication result and add to ymm1 and put result in ymm0.
	VFNMADD132PS,
	///
	/// 'vfnmadd231ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm1 and xmm2/mem, negate the multiplication result and add to xmm0 and put result in xmm0.
	///
	/// 'vfnmadd231ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm1 and ymm2/mem, negate the multiplication result and add to ymm0 and put result in ymm0.
	VFNMADD231PS,
// VFNMADD132SD/VFNMADD213SD/VFNMADD231SD--Fused Negative Multiply-Add of Scalar Double-Precision Floating-Point Values.
	///
	/// 'vfnmadd132sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm2/mem, negate the multiplication result and add to xmm1 and put result in xmm0.
	VFNMADD132SD,
	///
	/// 'vfnmadd213sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm1, negate the multiplication result and add to xmm2/mem and put result in xmm0.
	VFNMADD213SD,
	///
	/// 'vfnmadd231sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm1 and xmm2/mem, negate the multiplication result and add to xmm0 and put result in xmm0.
	VFNMADD231SD,
// VFNMADD132SS/VFNMADD213SS/VFNMADD231SS--Fused Negative Multiply-Add of Scalar Single-Precision Floating-Point Values.
	///
	/// 'vfnmadd132ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm2/mem, negate the multiplication result and add to xmm1 and put result in xmm0.
	VFNMADD132SS,
	///
	/// 'vfnmadd213ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm1, negate the multiplication result and add to xmm2/mem and put result in xmm0.
	VFNMADD213SS,
	///
	/// 'vfnmadd231ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm1 and xmm2/mem, negate the multiplication result and add to xmm0 and put result in xmm0.
	VFNMADD231SS,
// VFNMSUB132PD/VFNMSUB213PD/VFNMSUB231PD--Fused Negative Multiply-Subtract of Packed Double-Precision Floating-Point Values.
	///
	/// 'vfnmsub132pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm2/mem, negate the multiplication result and subtract xmm1 and put result in xmm0.
	///
	/// 'vfnmsub132pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm2/mem, negate the multiplication result and subtract ymm1 and put result in ymm0.
	VFNMSUB132PD,
	///
	/// 'vfnmsub213pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm0 and xmm1, negate the multiplication result and subtract xmm2/mem and put result in xmm0.
	///
	/// 'vfnmsub213pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm0 and ymm1, negate the multiplication result and subtract ymm2/mem and put result in ymm0.
	VFNMSUB213PD,
	///
	/// 'vfnmsub231pd xmm0,xmm1,xmm2/m128;' Multiply packed double-precision floating-point values from xmm1 and xmm2/mem, negate the multiplication result and subtract xmm0 and put result in xmm0.
	///
	/// 'vfnmsub231pd ymm0,ymm1,ymm2/m256;' Multiply packed double-precision floating-point values from ymm1 and ymm2/mem, negate the multiplication result and subtract ymm0 and put result in ymm0.
	VFNMSUB231PD,
// VFNMSUB132PS/VFNMSUB213PS/VFNMSUB231PS--Fused Negative Multiply-Subtract of Packed Single-Precision Floating-Point Values.
	///
	/// 'vfnmsub132ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm2/mem, negate the multiplication result and subtract xmm1 and put result in xmm0.
	///
	/// 'vfnmsub132ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm2/mem, negate the multiplication result and subtract ymm1 and put result in ymm0.
	VFNMSUB132PS,
	///
	/// 'vfnmsub213ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm0 and xmm1, negate the multiplication result and subtract xmm2/mem and put result in xmm0.
	///
	/// 'vfnmsub213ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm0 and ymm1, negate the multiplication result and subtract ymm2/mem and put result in ymm0.
	VFNMSUB213PS,
	///
	/// 'vfnmsub231ps xmm0,xmm1,xmm2/m128;' Multiply packed single-precision floating-point values from xmm1 and xmm2/mem, negate the multiplication result and subtract xmm0 and put result in xmm0.
	///
	/// 'vfnmsub231ps ymm0,ymm1,ymm2/m256;' Multiply packed single-precision floating-point values from ymm1 and ymm2/mem, negate the multiplication result and subtract ymm0 and put result in ymm0.
	VFNMSUB231PS,
// VFNMSUB132SD/VFNMSUB213SD/VFNMSUB231SD--Fused Negative Multiply-Subtract of Scalar Double-Precision Floating-Point Values.
	///
	/// 'vfnmsub231sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm1 and xmm2/mem, negate the multiplication result and subtract xmm0 and put result in xmm0.
	VFNMSUB231SD,
	///
	/// 'vfnmsub132sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm2/mem, negate the multiplication result and subtract xmm1 and put result in xmm0.
	VFNMSUB132SD,
	///
	/// 'vfnmsub213sd xmm0,xmm1,xmm2/m64;' Multiply scalar double-precision floating-point value from xmm0 and xmm1, negate the multiplication result and subtract xmm2/mem and put result in xmm0.
	VFNMSUB213SD,
// VFNMSUB132SS/VFNMSUB213SS/VFNMSUB231SS--Fused Negative Multiply-Subtract of Scalar Single-Precision Floating-Point Values.
	///
	/// 'vfnmsub132ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm2/mem, negate the multiplication result and subtract xmm1 and put result in xmm0.
	VFNMSUB132SS,
	///
	/// 'vfnmsub213ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm0 and xmm1, negate the multiplication result and subtract xmm2/mem and put result in xmm0.
	VFNMSUB213SS,
	///
	/// 'vfnmsub231ss xmm0,xmm1,xmm2/m32;' Multiply scalar single-precision floating-point value from xmm1 and xmm2/mem, negate the multiplication result and subtract xmm0 and put result in xmm0.
	VFNMSUB231SS,
// VGATHERDPD/VGATHERQPD--Gather Packed DP FP Values Using Signed Dword/Qword Indices.
	///
	/// 'vgatherqpd xmm1,vm64x,xmm2;' Using qword indices specified in vm64x, gather double-precision FP values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vgatherqpd ymm1,vm64y,ymm2;' Using qword indices specified in vm64y, gather double-precision FP values from memory conditioned on mask specified by ymm2. Conditionally gathered elements are merged into ymm1.
	VGATHERQPD,
	///
	/// 'vgatherdpd xmm1,vm32x,xmm2;' Using dword indices specified in vm32x, gather double-precision FP values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vgatherdpd ymm1,vm32x,ymm2;' Using dword indices specified in vm32x, gather double-precision FP values from memory conditioned on mask specified by ymm2. Conditionally gathered elements are merged into ymm1.
	VGATHERDPD,
// VGATHERDPS/VGATHERQPS--Gather Packed SP FP values Using Signed Dword/Qword Indices.
	///
	/// 'vgatherqps xmm1,vm64x,xmm2;' Using qword indices specified in vm64x, gather single-precision FP values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vgatherqps xmm1,vm64y,xmm2;' Using qword indices specified in vm64y, gather single-precision FP values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	VGATHERQPS,
	///
	/// 'vgatherdps xmm1,vm32x,xmm2;' Using dword indices specified in vm32x, gather single-precision FP values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vgatherdps ymm1,vm32y,ymm2;' Using dword indices specified in vm32y, gather single-precision FP values from memory conditioned on mask specified by ymm2. Conditionally gathered elements are merged into ymm1.
	VGATHERDPS,
// VPGATHERDD/VPGATHERQD--Gather Packed Dword Values Using Signed Dword/Qword Indices.
	///
	/// 'vpgatherqd xmm1,vm64x,xmm2;' Using qword indices specified in vm64x, gather dword values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vpgatherqd xmm1,vm64y,xmm2;' Using qword indices specified in vm64y, gather dword values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	VPGATHERQD,
	///
	/// 'vpgatherdd xmm1,vm32x,xmm2;' Using dword indices specified in vm32x, gather dword values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vpgatherdd ymm1,vm32y,ymm2;' Using dword indices specified in vm32y, gather dword from memory conditioned on mask specified by ymm2. Conditionally gathered elements are merged into ymm1.
	VPGATHERDD,
// VPGATHERDQ/VPGATHERQQ--Gather Packed Qword Values Using Signed Dword/Qword Indices.
	///
	/// 'vpgatherdq xmm1,vm32x,xmm2;' Using dword indices specified in vm32x, gather qword values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vpgatherdq ymm1,vm32x,ymm2;' Using dword indices specified in vm32x, gather qword values from memory conditioned on mask specified by ymm2. Conditionally gathered elements are merged into ymm1.
	VPGATHERDQ,
	///
	/// 'vpgatherqq xmm1,vm64x,xmm2;' Using qword indices specified in vm64x, gather qword values from memory conditioned on mask specified by xmm2. Conditionally gathered elements are merged into xmm1.
	///
	/// 'vpgatherqq ymm1,vm64y,ymm2;' Using qword indices specified in vm64y, gather qword values from memory conditioned on mask specified by ymm2. Conditionally gathered elements are merged into ymm1.
	VPGATHERQQ,
// VINSERTF128--Insert Packed Floating-Point Values.
	///
	/// 'vinsertf128 ymm1,ymm2,xmm3/m128,imm8;' Insert a single precision floating-point value selected by imm8 from xmm3/m128 into ymm2 at the specified destination element specified by imm8 and zero out destination elements in ymm1 as indicated in imm8.
	VINSERTF128,
// VINSERTI128--Insert Packed Integer Values.
	///
	/// 'vinserti128 ymm1,ymm2,xmm3/m128,imm8;' Insert 128-bits of integer data from xmm3/mem and the remaining values from ymm2 into ymm1.
	VINSERTI128,
// VMASKMOV--Conditional SIMD Packed Loads and Stores.
	///
	/// 'vmaskmovps xmm1,xmm2,m128;' Conditionally load packed single-precision values from m128 using mask in xmm2 and store in xmm1.
	///
	/// 'vmaskmovps ymm1,ymm2,m256;' Conditionally load packed single-precision values from m256 using mask in ymm2 and store in ymm1.
	///
	/// 'vmaskmovps m128,xmm1,xmm2;' Conditionally store packed single-precision values from xmm2 using mask in xmm1.
	///
	/// 'vmaskmovps m256,ymm1,ymm2;' Conditionally store packed single-precision values from ymm2 using mask in ymm1.
	VMASKMOVPS,
	///
	/// 'vmaskmovpd xmm1,xmm2,m128;' Conditionally load packed double-precision values from m128 using mask in xmm2 and store in xmm1.
	///
	/// 'vmaskmovpd ymm1,ymm2,m256;' Conditionally load packed double-precision values from m256 using mask in ymm2 and store in ymm1.
	///
	/// 'vmaskmovpd m128,xmm1,xmm2;' Conditionally store packed double-precision values from xmm2 using mask in xmm1.
	///
	/// 'vmaskmovpd m256,ymm1,ymm2;' Conditionally store packed double-precision values from ymm2 using mask in ymm1.
	VMASKMOVPD,
// VPBLENDD--Blend Packed Dwords.
	///
	/// 'vpblendd xmm1,xmm2,xmm3/m128,imm8;' Select dwords from xmm2 and xmm3/m128 from mask specified in imm8 and store the values into xmm1.
	///
	/// 'vpblendd ymm1,ymm2,ymm3/m256,imm8;' Select dwords from ymm2 and ymm3/m256 from mask specified in imm8 and store the values into ymm1.
	VPBLENDD,
// VPBROADCAST--Broadcast Integer Data.
	///
	/// 'vpbroadcastw xmm1,xmm2/m16;' Broadcast a word integer in the source operand to eight locations in xmm1.
	///
	/// 'vpbroadcastw ymm1,xmm2/m16;' Broadcast a word integer in the source operand to sixteen locations in ymm1.
	VPBROADCASTW,
	///
	/// 'vpbroadcastb xmm1,xmm2/m8;' Broadcast a byte integer in the source operand to sixteen locations in xmm1.
	///
	/// 'vpbroadcastb ymm1,xmm2/m8;' Broadcast a byte integer in the source operand to thirtytwo locations in ymm1.
	VPBROADCASTB,
	///
	/// 'vpbroadcastq xmm1,xmm2/m64;' Broadcast a qword element in mem to two locations in xmm1.
	///
	/// 'vpbroadcastq ymm1,xmm2/m64;' Broadcast a qword element in mem to four locations in ymm1.
	VPBROADCASTQ,
	///
	/// 'vbroadcasti128 ymm1,m128;' Broadcast 128 bits of integer data in mem to low and high 128-bits in ymm1.
	VBROADCASTI128,
	///
	/// 'vpbroadcastd xmm1,xmm2/m32;' Broadcast a dword integer in the source operand to four locations in xmm1.
	///
	/// 'vpbroadcastd ymm1,xmm2/m32;' Broadcast a dword integer in the source operand to eight locations in ymm1.
	VPBROADCASTD,
// VPERMD--Full Doublewords Element Permutation.
	///
	/// 'vpermd ymm1,ymm2,ymm3/m256;' Permute doublewords in ymm3/m256 using indexes in ymm2 and store the result in ymm1.
	VPERMD,
// VPERMPD--Permute Double-Precision Floating-Point Elements.
	///
	/// 'vpermpd ymm1,ymm2/m256,imm8;' Permute double-precision floating-point elements in ymm2/m256 using indexes in imm8 and store the result in ymm1.
	VPERMPD,
// VPERMPS--Permute Single-Precision Floating-Point Elements.
	///
	/// 'vpermps ymm1,ymm2,ymm3/m256;' Permute single-precision floating-point elements in ymm3/m256 using indexes in ymm2 and store the result in ymm1.
	VPERMPS,
// VPERMQ--Qwords Element Permutation.
	///
	/// 'vpermq ymm1,ymm2/m256,imm8;' Permute qwords in ymm2/m256 using indexes in imm8 and store the result in ymm1.
	VPERMQ,
// VPERM2I128--Permute Integer Values.
	///
	/// 'vperm2i128 ymm1,ymm2,ymm3/m256,imm8;' Permute 128-bit integer data in ymm2 and ymm3/mem using controls from imm8 and store result in ymm1.
	VPERM2I128,
// VPERMILPD--Permute Double-Precision Floating-Point Values.
	///
	/// 'vpermilpd xmm1,xmm2,xmm3/m128;' Permute double-precision floating-point values in xmm2 using controls from xmm3/mem and store result in xmm1.
	///
	/// 'vpermilpd ymm1,ymm2,ymm3/m256;' Permute double-precision floating-point values in ymm2 using controls from ymm3/mem and store result in ymm1.
	///
	/// 'vpermilpd xmm1,xmm2/m128,imm8;' Permute double-precision floating-point values in xmm2/mem using controls from imm8.
	///
	/// 'vpermilpd ymm1,ymm2/m256,imm8;' Permute double-precision floating-point values in ymm2/mem using controls from imm8.
	VPERMILPD,
// VPERMILPS--Permute Single-Precision Floating-Point Values.
	///
	/// 'vpermilps xmm1,xmm2,xmm3/m128;' Permute single-precision floating-point values in xmm2 using controls from xmm3/mem and store result in xmm1.
	///
	/// 'vpermilps xmm1,xmm2/m128,imm8;' Permute single-precision floating-point values in xmm2/mem using controls from imm8 and store result in xmm1.
	///
	/// 'vpermilps ymm1,ymm2,ymm3/m256;' Permute single-precision floating-point values in ymm2 using controls from ymm3/mem and store result in ymm1.
	///
	/// 'vpermilps ymm1,ymm2/m256,imm8;' Permute single-precision floating-point values in ymm2/mem using controls from imm8 and store result in ymm1.
	VPERMILPS,
// VPERM2F128--Permute Floating-Point Values.
	///
	/// 'vperm2f128 ymm1,ymm2,ymm3/m256,imm8;' Permute 128-bit floating-point fields in ymm2 and ymm3/mem using controls from imm8 and store result in ymm1.
	VPERM2F128,
// VPMASKMOV--Conditional SIMD Integer Packed Loads and Stores.
	///
	/// 'vpmaskmovd xmm1,xmm2,m128;' Conditionally load dword values from m128 using mask in xmm2 and store in xmm1.
	///
	/// 'vpmaskmovd ymm1,ymm2,m256;' Conditionally load dword values from m256 using mask in ymm2 and store in ymm1.
	///
	/// 'vpmaskmovd m128,xmm1,xmm2;' Conditionally store dword values from xmm2 using mask in xmm1.
	///
	/// 'vpmaskmovd m256,ymm1,ymm2;' Conditionally store dword values from ymm2 using mask in ymm1.
	VPMASKMOVD,
	///
	/// 'vpmaskmovq xmm1,xmm2,m128;' Conditionally load qword values from m128 using mask in xmm2 and store in xmm1.
	///
	/// 'vpmaskmovq ymm1,ymm2,m256;' Conditionally load qword values from m256 using mask in ymm2 and store in ymm1.
	///
	/// 'vpmaskmovq m128,xmm1,xmm2;' Conditionally store qword values from xmm2 using mask in xmm1.
	///
	/// 'vpmaskmovq m256,ymm1,ymm2;' Conditionally store qword values from ymm2 using mask in ymm1.
	VPMASKMOVQ,
// VPSLLVD/VPSLLVQ--Variable Bit Shift Left Logical.
	///
	/// 'vpsllvq xmm1,xmm2,xmm3/m128;' Shift bits in quadwords in xmm2 left by amount specified in the corresponding element of xmm3/m128 while shifting in 0s.
	///
	/// 'vpsllvq ymm1,ymm2,ymm3/m256;' Shift bits in quadwords in ymm2 left by amount specified in the corresponding element of ymm3/m256 while shifting in 0s.
	VPSLLVQ,
	///
	/// 'vpsllvd xmm1,xmm2,xmm3/m128;' Shift bits in doublewords in xmm2 left by amount specified in the corresponding element of xmm3/m128 while shifting in 0s.
	///
	/// 'vpsllvd ymm1,ymm2,ymm3/m256;' Shift bits in doublewords in ymm2 left by amount specified in the corresponding element of ymm3/m256 while shifting in 0s.
	VPSLLVD,
// VPSRAVD--Variable Bit Shift Right Arithmetic.
	///
	/// 'vpsravd xmm1,xmm2,xmm3/m128;' Shift bits in doublewords in xmm2 right by amount specified in the corresponding element of xmm3/m128 while shifting in the sign bits.
	///
	/// 'vpsravd ymm1,ymm2,ymm3/m256;' Shift bits in doublewords in ymm2 right by amount specified in the corresponding element of ymm3/m256 while shifting in the sign bits.
	VPSRAVD,
// VPSRLVD/VPSRLVQ--Variable Bit Shift Right Logical.
	///
	/// 'vpsrlvd xmm1,xmm2,xmm3/m128;' Shift bits in doublewords in xmm2 right by amount specified in the corresponding element of xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrlvd ymm1,ymm2,ymm3/m256;' Shift bits in doublewords in ymm2 right by amount specified in the corresponding element of ymm3/m256 while shifting in 0s.
	VPSRLVD,
	///
	/// 'vpsrlvq xmm1,xmm2,xmm3/m128;' Shift bits in quadwords in xmm2 right by amount specified in the corresponding element of xmm3/m128 while shifting in 0s.
	///
	/// 'vpsrlvq ymm1,ymm2,ymm3/m256;' Shift bits in quadwords in ymm2 right by amount specified in the corresponding element of ymm3/m256 while shifting in 0s.
	VPSRLVQ,
// VTESTPD/VTESTPS--Packed Bit Test.
	///
	/// 'vtestpd xmm1,xmm2/m128;' Set ZF and CF depending on sign bit AND and ANDN of packed double-precision floating-point sources.
	///
	/// 'vtestpd ymm1,ymm2/m256;' Set ZF and CF depending on sign bit AND and ANDN of packed double-precision floating-point sources.
	VTESTPD,
	///
	/// 'vtestps xmm1,xmm2/m128;' Set ZF and CF depending on sign bit AND and ANDN of packed single-precision floating-point sources.
	///
	/// 'vtestps ymm1,ymm2/m256;' Set ZF and CF depending on sign bit AND and ANDN of packed single-precision floating-point sources.
	VTESTPS,
// VZEROALL--Zero All YMM Registers.
	///
	/// 'vzeroall;' Zero all YMM registers.
	VZEROALL,
// VZEROUPPER--Zero Upper Bits of YMM Registers.
	///
	/// 'vzeroupper;' Zero upper 128 bits of all YMM registers.
	VZEROUPPER,
// WAIT/FWAIT--Wait.
	///
	/// 'fwait;' Check pending unmasked floating-point exceptions.
	FWAIT,
	///
	/// 'wait;' Check pending unmasked floating-point exceptions.
	WAIT,
// WBINVD--Write Back and Invalidate Cache.
	///
	/// 'wbinvd;' Write back and flush Internal caches; initiate writing-back and flushing of external caches.
	WBINVD,
// WRFSBASE/WRGSBASE--Write FS/GS Segment Base.
// WRMSR--Write to Model Specific Register.
	///
	/// 'wrmsr;' Write the value in EDX:EAX to MSR specified by ECX.
	WRMSR,
// WRPKRU--Write Data to User Page Key Register.
	///
	/// 'wrpkru;' Writes EAX into PKRU.
	WRPKRU,
// XACQUIRE/XRELEASE--Hardware Lock Elision Prefix Hints.
	///
	/// 'xacquire;' A hint used with an 'XACQUIRE-enabled' instruction to start lock elision on the instruction memory operand address.
	XACQUIRE,
	///
	/// 'xrelease;' A hint used with an 'XRELEASE-enabled' instruction to end lock elision on the instruction memory operand address.
	XRELEASE,
// XABORT--Transactional Abort.
	///
	/// 'xabort imm8;' Causes an RTM abort if in RTM execution.
	XABORT,
// XADD--Exchange and Add.
	///
	/// 'xadd r/m8,r8;' Exchange r8 and r/m8; load sum into r/m8.
	///
	/// 'xadd r/m8*,r8*;' Exchange r8 and r/m8; load sum into r/m8.
	///
	/// 'xadd r/m16,r16;' Exchange r16 and r/m16; load sum into r/m16.
	///
	/// 'xadd r/m32,r32;' Exchange r32 and r/m32; load sum into r/m32.
	///
	/// 'xadd r/m64,r64;' Exchange r64 and r/m64; load sum into r/m64.
	XADD,
// XBEGIN--Transactional Begin.
	///
	/// 'xbegin rel16;' Specifies the start of an RTM region. Provides a 16-bit relative offset to compute the address of the fallback instruction address at which execution resumes following an RTM abort.
	///
	/// 'xbegin rel32;' Specifies the start of an RTM region. Provides a 32-bit relative offset to compute the address of the fallback instruction address at which execution resumes following an RTM abort.
	XBEGIN,
// XCHG--Exchange Register/Memory with Register.
	///
	/// 'xchg AX,r16;' Exchange r16 with AX.
	///
	/// 'xchg r16,AX;' Exchange AX with r16.
	///
	/// 'xchg EAX,r32;' Exchange r32 with EAX.
	///
	/// 'xchg RAX,r64;' Exchange r64 with RAX.
	///
	/// 'xchg r32,EAX;' Exchange EAX with r32.
	///
	/// 'xchg r64,RAX;' Exchange RAX with r64.
	///
	/// 'xchg r/m8,r8;' Exchange r8 (byte register) with byte from r/m8.
	///
	/// 'xchg r/m8*,r8*;' Exchange r8 (byte register) with byte from r/m8.
	///
	/// 'xchg r8,r/m8;' Exchange byte from r/m8 with r8 (byte register).
	///
	/// 'xchg r8*,r/m8*;' Exchange byte from r/m8 with r8 (byte register).
	///
	/// 'xchg r/m16,r16;' Exchange r16 with word from r/m16.
	///
	/// 'xchg r16,r/m16;' Exchange word from r/m16 with r16.
	///
	/// 'xchg r/m32,r32;' Exchange r32 with doubleword from r/m32.
	///
	/// 'xchg r/m64,r64;' Exchange r64 with quadword from r/m64.
	///
	/// 'xchg r32,r/m32;' Exchange doubleword from r/m32 with r32.
	///
	/// 'xchg r64,r/m64;' Exchange quadword from r/m64 with r64.
	XCHG,
// XEND--Transactional End.
	///
	/// 'xend;' Specifies the end of an RTM code region.
	XEND,
// XGETBV--Get Value of Extended Control Register.
	///
	/// 'xgetbv;' Reads an XCR specified by ECX into EDX:EAX.
	XGETBV,
// XLAT/XLATB--Table Look-up Translation.
	///
	/// 'xlat m8;' Set AL to memory byte DS:[(E)BX + unsigned AL].
	XLAT,
	///
	/// 'xlatb;' Set AL to memory byte DS:[(E)BX + unsigned AL].
	///
	/// 'xlatb;' Set AL to memory byte [RBX + unsigned AL].
	XLATB,
// XOR--Logical Exclusive OR.
	///
	/// 'xor AL,imm8;' AL XOR imm8.
	///
	/// 'xor AX,imm16;' AX XOR imm16.
	///
	/// 'xor EAX,imm32;' EAX XOR imm32.
	///
	/// 'xor RAX,imm32;' RAX XOR imm32 (sign-extended).
	///
	/// 'xor r/m8,imm8;' r/m8 XOR imm8.
	///
	/// 'xor r/m8*,imm8;' r/m8 XOR imm8.
	///
	/// 'xor r/m16,imm16;' r/m16 XOR imm16.
	///
	/// 'xor r/m32,imm32;' r/m32 XOR imm32.
	///
	/// 'xor r/m64,imm32;' r/m64 XOR imm32 (sign-extended).
	///
	/// 'xor r/m16,imm8;' r/m16 XOR imm8 (sign-extended).
	///
	/// 'xor r/m32,imm8;' r/m32 XOR imm8 (sign-extended).
	///
	/// 'xor r/m64,imm8;' r/m64 XOR imm8 (sign-extended).
	///
	/// 'xor r/m8,r8;' r/m8 XOR r8.
	///
	/// 'xor r/m8*,r8*;' r/m8 XOR r8.
	///
	/// 'xor r/m16,r16;' r/m16 XOR r16.
	///
	/// 'xor r/m32,r32;' r/m32 XOR r32.
	///
	/// 'xor r/m64,r64;' r/m64 XOR r64.
	///
	/// 'xor r8,r/m8;' r8 XOR r/m8.
	///
	/// 'xor r8*,r/m8*;' r8 XOR r/m8.
	///
	/// 'xor r16,r/m16;' r16 XOR r/m16.
	///
	/// 'xor r32,r/m32;' r32 XOR r/m32.
	///
	/// 'xor r64,r/m64;' r64 XOR r/m64.
	XOR,
// XORPD--Bitwise Logical XOR for Double-Precision Floating-Point Values.
	///
	/// 'xorpd xmm1,xmm2/m128;' Bitwise exclusive-OR of xmm2/m128 and xmm1.
	XORPD,
	///
	/// 'vxorpd xmm1,xmm2,xmm3/m128;' Return the bitwise logical XOR of packed double-precision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vxorpd ymm1,ymm2,ymm3/m256;' Return the bitwise logical XOR of packed double-precision floating-point values in ymm2 and ymm3/mem.
	VXORPD,
// XORPS--Bitwise Logical XOR for Single-Precision Floating-Point Values.
	///
	/// 'xorps xmm1,xmm2/m128;' Bitwise exclusive-OR of xmm2/m128 and xmm1.
	XORPS,
	///
	/// 'vxorps xmm1,xmm2,xmm3/m128;' Return the bitwise logical XOR of packed singleprecision floating-point values in xmm2 and xmm3/mem.
	///
	/// 'vxorps ymm1,ymm2,ymm3/m256;' Return the bitwise logical XOR of packed singleprecision floating-point values in ymm2 and ymm3/mem.
	VXORPS,
// XRSTOR--Restore Processor Extended States.
	///
	/// 'xrstor mem;' Restore state components specified by EDX:EAX from mem.
	XRSTOR,
	///
	/// 'xrstor64 mem;' Restore state components specified by EDX:EAX from mem.
	XRSTOR64,
// XRSTORS--Restore Processor Extended States Supervisor.
	///
	/// 'xrstors mem;' Restore state components specified by EDX:EAX from mem.
	XRSTORS,
	///
	/// 'xrstors64 mem;' Restore state components specified by EDX:EAX from mem.
	XRSTORS64,
// XSAVE--Save Processor Extended States.
	///
	/// 'xsave mem;' Save state components specified by EDX:EAX to mem.
	XSAVE,
	///
	/// 'xsave64 mem;' Save state components specified by EDX:EAX to mem.
	XSAVE64,
// XSAVEC--Save Processor Extended States with Compaction.
	///
	/// 'xsavec mem;' Save state components specified by EDX:EAX to mem with compaction.
	XSAVEC,
	///
	/// 'xsavec64 mem;' Save state components specified by EDX:EAX to mem with compaction.
	XSAVEC64,
// XSAVEOPT--Save Processor Extended States Optimized.
	///
	/// 'xsaveopt64 mem;' Save state components specified by EDX:EAX to mem, optimizing if possible.
	XSAVEOPT64,
	///
	/// 'xsaveopt mem;' Save state components specified by EDX:EAX to mem, optimizing if possible.
	XSAVEOPT,
// XSAVES--Save Processor Extended States Supervisor.
	///
	/// 'xsaves mem;' Save state components specified by EDX:EAX to mem with compaction, optimizing if possible.
	XSAVES,
	///
	/// 'xsaves64 mem;' Save state components specified by EDX:EAX to mem with compaction, optimizing if possible.
	XSAVES64,
// XSETBV--Set Extended Control Register.
	///
	/// 'xsetbv;' Write the value in EDX:EAX to the XCR specified by ECX.
	XSETBV,
// XTEST--Test If In Transactional Execution.
	///
	/// 'xtest;' Test if executing in a transactional region.
	XTEST,
}

