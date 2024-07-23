#import "@preview/diagraph:0.2.5": *
#set page(width: auto, height: auto, margin: 0cm);
 $ Delta^(2)_(0) delta^(1)_(0) delta^(1)_(0) w^(2)_(00) w^(2)_(00) x_0 x_1 $ #raw-render(```
digraph {
graph[splines=line] 
node[shape=circle]  
edge [arrowhead=none] rankdir=LR 
 subgraph cluster_0{
	x_1 [style=filled, color=red]
	x_0 [style=filled, color=red]
}
subgraph cluster_1{
	"n^(1)_1" 
	"n^(1)_0" [style=filled, color=green]
}
subgraph cluster_2{
	"n^(2)_0" [style=filled, color=blue]
}
subgraph cluster_1{
	"x_1" -> "n^(1)_1" 
	"x_0" -> "n^(1)_1" 
	"x_1" -> "n^(1)_0" [color=purple]
	"x_0" -> "n^(1)_0" [color=purple]
}
subgraph cluster_2{
	"n^(1)_1" -> "n^(2)_0" 
	"n^(1)_0" -> "n^(2)_0" [color=red]
}
 }```)
