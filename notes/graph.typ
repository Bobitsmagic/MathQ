#import "@preview/diagraph:0.2.5": *

#raw-render(```
  digraph {
    node[shape=circle]
    rankdir=LR

subgraph cluster_0{
        x_0 [style=filled, color=red]
        x_1 [style=filled, color=red]
}
subgraph cluster_1{
        "n^(1)_0" [style=filled, color=green]
        "n^(1)_1" [style=filled, color=green]
}
subgraph cluster_2{
        "n^(2)_0" [style=filled, color=blue]
        "n^(2)_1"
}
subgraph cluster_3 {
        "n^(3)_0" [style=filled, color=green]
        "n^(3)_1"
}
subgraph cluster_1 {
        "x_0" -> "n^(1)_0" [arrowhead=none]
        "x_1" -> "n^(1)_0" [arrowhead=none]
        "x_0" -> "n^(1)_1" [arrowhead=none]
        "x_1" -> "n^(1)_1" [arrowhead=none]
}
subgraph cluster_2{
        "n^(1)_0" -> "n^(2)_0" [style=filled, color=red, arrowhead=none]
        "n^(1)_1" -> "n^(2)_0" [style=filled, color=red, arrowhead=none]
        "n^(1)_0" -> "n^(2)_1" [arrowhead=none]
        "n^(1)_1" -> "n^(2)_1" [arrowhead=none]
}
subgraph cluster_3{
        "n^(2)_0" -> "n^(3)_0" [style=filled, color=red, arrowhead=none]
        "n^(2)_1" -> "n^(3)_0" [arrowhead=none]
        "n^(2)_0" -> "n^(3)_1" [arrowhead=none]
        "n^(2)_1" -> "n^(3)_1" [arrowhead=none]
}
  }
  ```
)