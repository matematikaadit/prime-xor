.PHONY: png clean pic

png: prime.png

prime-xor: prime-xor.rs
	rustc -O prime-xor.rs

prime.pbm: prime-xor
	./prime-xor 600 > prime.pbm

prime.png: prime.pbm
	convert prime.pbm prime.png

pic: prime.pbm

clean:
	rm -f prime-xor prime.pbm

