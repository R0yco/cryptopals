# cbc padding oracle

## background

Given an oracle which outputs ciphertexts, and a function which decrypts the ciphertexts and outputs whether the plaintexts padding is valid, we can exfiltrate the whole plaintext.

## the how

We modify a valid ciphertext from the oracle. 
By changing the Nth byte in the block before the byte we want to discover, we control what byte is being XOR'ed into the byte.

We can go through 1-255 and when the decryption function tells us the encryption is valid, we can do this:

S- the secret byte we want to discover
B- the value we brute forced.

### first case

S ^ B = 0x1

S = B ^ 0x1 

we got the last byte!

### the rest

since we now know the value of the last plaintext byte, we can manipulate the byte in the previous block to change it to anything.

so to find the second to last byte, we change the last one to 0x2 using this technique, and brute force it.

### edge case

if the second to last byte is set to 0x02, we might brute force and the value that causes the last byte to be 0x02, and then it will be a valid padding (last 2  bytes of message are 0x02), and then the rest will fail, because we haven't actually found the first bytes value.

to overcome this, when we think we found the first byte (padding is correct), we change the previous byte (doesn't matter to what) and check again. now, if the "valid padding" was 0x2,0x2, then it will not be valid again.







