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

using Statistics
function parse_ndaq_df(df)
  df = dropmissing(df)
  # df.last_sale_price = map(x-> parse(Float32, x[2:end]), df.last_sale_price)
  df.net_change = parse.(Float64, df.net_change)
  # df.percentage_change = map(x-> parse(Float32, x[1:end-1]), df.percentage_change)
  sort!(df, [:percentage_change, :last_sale_price], rev=true)
  return df 
end

function simple_value(df) 
    s2 = mean(var.(eachcol(df)[3:end-1]))
    μ = mean(mean.(eachcol(df)[3:end-1]))
    δ = df.c[end] - df.c[1]
    μ + δ - s2
end

# AAPL knapsack DP example
my = ["TWO", "NRZ", "RWT"]
cur_df = parse_ndaq_df(CSV.read("./finox/nasdaq_new8.csv"))
sort!(cur_df, :market_cap, rev=true)
prices = cur_df[in.(cur_df.symbol, Ref(my)), :last_sale_price]
capacity = Int(round(sum(prices), digits=1) * 10)


get_fns(tick_list) = [glob("./data/$(t)_yf7d*.csv")[1] for t in tick_list]
fns = get_fns(my)
mys = [dropmissing(CSV.read(fn)) for fn in fns]
vals = simple_value.(mys)
est_val = sum(vals) * 10
diff = est_val - capacity


include("./jl/utils.jl")
using .Utils

# dfs = dropmissing.(Utils.get_dfs("./data/*_yf7d*.csv"))

ks = unique(cur_df.symbol)#[1:100]
to_val_fns = get_fns(ks)
to_val = dropmissing.(CSV.read.(to_val_fns))
filt_to_val = filter(x-> size(x, 1) > 300, to_val)
vals =round.(simple_value.(filt_to_val), digits=1) * 10

evals =  convert.(Int, vals)
to_prices = cur_df[in.(cur_df.symbol, Ref(ks)), [:symbol, :last_sale_price]]

evaldf = hcat(to_prices, evals)
evaldf.last_sale_price = convert.(Int, round.(evaldf.last_sale_price, digits=1)*10)
evaldf.x1 = convert.(Int, round.(evaldf.x1, digits=1)*10)

outlook = sum(evaldf.last_sale_price .- evaldf.x1)
@assert outlook == 1 # dollar long
evaldf

function knapsack(df, W)
  w = df.last_sale_price
  v = df.x1
  n = size(df, 1)
  m = zeros(n, W)
  for i in 2:n
    for j in 1:W
      if w[i] ≥ j
        m[i, j] = m[i-1, j]
      else
        println("$(df[i, :])")
        
        m[i, j] = max(m[i-1, j], m[i-1, j-w[i]] + v[i])
      end
    end
  end
  return m
end

sol = knapsack(evaldf, 1000)


function garbo(df) 
  df.last_price = parse.(Float64, replace.(replace.(df.last_price, "\$"=>""), ","=>""))
  df.shares_traded = parse.(Int, replace.(df.shares_traded, ","=>""))
  df.shares_held =  parse.(Int, replace.(df.shares_held, ","=>""))
  dtfmt = "m/d/y"
  df.last_date = Date.(df.last_date, dtfmt)
end