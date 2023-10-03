//now we have a certain group I call it G, the computing in G is mutiplucation mod p
//p is a large modulo number
//alpha and beta are two generators in a subgroup G'. So their order are both ord(G')
//q is the ord(G')
//password as x, random number as k
//y1 = alpha ^ x, beta ^ x (mod p)
//prover send y1 and y2 to verifier, and verifier store them
//when prover wants to login , he sends r1 = alpha ^ k and r2 = beta ^ k to verifier
//verifier store r1 and r2, and sends a random number c to prover
//prover calculates s = c - k * x (mod q)

struct ZKP {
    alpha,beta,p,q
    
}