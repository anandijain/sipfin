include("utils.jl")
using .Utils
using Plots, Dates
using CSV, DataFrames, Glob
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


# lildiff
using DifferentialEquations, Flux, Optim, DiffEqFlux, Zygote, Diff

df = CSV.read("./data/AAPL_HIST_2020_4_27.csv")
df = dropmissing(df)
d = df[1:1000, [:t, :c_AAPL]]
t, x = d[:, 1], d[: ,2]
x ./maximum(x)

function simple_growth(du,u,α,t)
  x = u
  du = α*x 
end
u0 = x[1]
t = convert(Array{Float32}, t / t[end])
tspan = (t[1], t[end])  

f(u,p,t) = 0.1u
tspan = (0.0,1.0)
prob = ODEProblem(f,u0,tspan, saveat=t)

sol = solve(prob)

plot(sol)
plot!(t, x) 

function parse_ndaq_df(df)
  df = dropmissing(df)
  df.last_sale_price = map(x-> parse(Float32, x[2:end]), df.last_sale_price)
  df.net_change = parse.(Float32, df.net_change)
  df.percentage_change = map(x-> parse(Float32, x[1:end-1]), df.percentage_change)
  return df 
end
