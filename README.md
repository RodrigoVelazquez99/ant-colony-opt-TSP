# ant-colony-opt-TSP
Implementación de la metaheurística Ant Colony Optimization para el problema TSP.

Ejemplar tomado de : [Solving TSPs](http://www.math.uwaterloo.ca/tsp/world/djlog.html)


### Dependencias 
* Rustc 1.46.0
* Cargo 1.46.0

### Ejecución 
``` bash
cargo run 

Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/ant-colony-opt-tsp`

Optimización por colonia de hormigas


 <<<<<<<<<<<<<<<<<<<<< MEJOR TOUR >>>>>>>>>>>>>>>>>>>>> 

[Ciudad 10 -> Ciudad 1] : pheromone : 0.0052530556
[Ciudad 1 -> Ciudad 2] : pheromone : 0.009298536
[Ciudad 2 -> Ciudad 3] : pheromone : 0.009243906
[Ciudad 3 -> Ciudad 4] : pheromone : 0.009191314
[Ciudad 4 -> Ciudad 5] : pheromone : 0.009047874
[Ciudad 5 -> Ciudad 6] : pheromone : 0.00909041
[Ciudad 6 -> Ciudad 7] : pheromone : 0.009232535
[Ciudad 7 -> Ciudad 8] : pheromone : 0.009159989
[Ciudad 8 -> Ciudad 9] : pheromone : 0.009107744
[Ciudad 9 -> Ciudad 11] : pheromone : 0.0052650236
[Ciudad 11 -> Ciudad 12] : pheromone : 0.009177357
[Ciudad 12 -> Ciudad 13] : pheromone : 0.009142051
[Ciudad 13 -> Ciudad 14] : pheromone : 0.009061508
[Ciudad 14 -> Ciudad 15] : pheromone : 0.009062967
[Ciudad 15 -> Ciudad 16] : pheromone : 0.009190165
[Ciudad 16 -> Ciudad 17] : pheromone : 0.009133292
[Ciudad 17 -> Ciudad 18] : pheromone : 0.009022912
[Ciudad 18 -> Ciudad 19] : pheromone : 0.009057119
[Ciudad 19 -> Ciudad 20] : pheromone : 0.009123791
[Ciudad 20 -> Ciudad 21] : pheromone : 0.009093607
[Ciudad 21 -> Ciudad 22] : pheromone : 0.00903796
[Ciudad 22 -> Ciudad 23] : pheromone : 0.009121724
[Ciudad 23 -> Ciudad 24] : pheromone : 0.009161319
[Ciudad 24 -> Ciudad 25] : pheromone : 0.009207351
[Ciudad 25 -> Ciudad 26] : pheromone : 0.0092383865
[Ciudad 26 -> Ciudad 27] : pheromone : 0.009110381
[Ciudad 27 -> Ciudad 28] : pheromone : 0.009012713
[Ciudad 28 -> Ciudad 29] : pheromone : 0.009075159
[Ciudad 29 -> Ciudad 30] : pheromone : 0.009058258
[Ciudad 30 -> Ciudad 32] : pheromone : 0.005191833
[Ciudad 32 -> Ciudad 31] : pheromone : 0.005134445
[Ciudad 31 -> Ciudad 33] : pheromone : 0.005237988
[Ciudad 33 -> Ciudad 34] : pheromone : 0.009017165
[Ciudad 34 -> Ciudad 35] : pheromone : 0.009009677
[Ciudad 35 -> Ciudad 36] : pheromone : 0.009100728
[Ciudad 36 -> Ciudad 38] : pheromone : 0.005522787
[Ciudad 38 -> Ciudad 37] : pheromone : 0.0054820147
[Ciudad 37 -> Ciudad 10] : pheromone : 0.005134445

Costo (distancia) : 13843.822 

Tiempo que tomó encontrarlo : 4.892764592s 

```
