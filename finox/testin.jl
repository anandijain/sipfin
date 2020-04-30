include("Utils.jl")
using .Utils
using Glob
using LightGraphs, GraphPlot, SimpleWeightedGraphs, MetaGraphs, GraphRecipes

# rename!(x -> Symbol(replace(string(x), " "=>"_")), df)
reg = r"data\/(.*):"



glob_pat = "./data/*H*.csv"
re = r"(c_*|v_*)"
filt_n = 1000
cols = [Symbol("$x") for x in readlines("./ref_data/50_biggest.txt")]

glob_pats = ["./data/$(t)_H*.csv" for t in  map(x -> split(String(x), "_")[end], cols)]
big50_fns = vcat(glob.(glob_pats)...)

dfs = Utils.get_dfs(big50_fns)
join_n = 47
joined = Utils.quick(dfs, glob_pat, re, join_n, filt_n)
g = Utils.df_to_meta_cor_graph(joined[:, 2:end])

p = graphplot(g,
  edge_width = (s, d, w)->get_prop(g, Edge(s, d), :weight),
  names = map(x->"$(x[1])_$(x[2])", enumerate(names(joined))),
  node_weights = vols, 
  size = (1600, 1600),
  curves = false,
  fontsize=4,
  dpi=300,
  )

# savefig(p, "close_cors2.png")
