$ N: NN "Layercount" $
$ M_i: NN "Neurons per layer" $
$ theta_(j k)^i: RR "Layer, Left, Right" $
$ phi: RR -> RR "activation function" $

Single layer:
$ f(theta, x) = phi(sum_(i)^(M_1) x_i theta^1_(i 1)) $
Double layer:
$ n^ell_i (theta, x) "output of neuron in layer" l "and index" i $
$ n^0_i (theta, x) = x_i $
$ n^ell_j (theta, x) = phi(sum_(i)^(M_(ell - 1)) n^(ell - 1)_i (theta, x) theta^(ell - 1)_(i j)) $ 

$ delta^ell_i (theta, x) = phi'(sum_(i)^(M_(ell - 1)) n^(ell - 1)_i (theta, x) theta^(ell - 1)_(i j)) $
$ (partial n^ell_j (theta, x)) / (partial w) = delta^ell_i (theta, x)  $ 

#pagebreak()