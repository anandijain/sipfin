using Plots
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra
using DelimitedFiles 
using Base
using LightGraphs, GraphPlot, SimpleWeightedGraphs
using GraphRecipes


# rename!(x -> Symbol(replace(string(x), " "=>"_")), df)
reg = r"data\/(.*):"

# thanks seth
to_prune(g, n) = findall(LightGraphs.weights(g) .> n)
to_prune_nodes(g, n::Integer) = findall((g) .> n)

cor_df(df::AbstractDataFrame)::AbstractDataFrame = DataFrame(cor(Matrix(df)), names(df))
re_cols(dfs::Array{DataFrame,1}, re::Regex) = map(df->df[:, re], dfs)

function cor_df_to_edge_weights(df::AbstractDataFrame, ret_df::Bool = true)::Union{AbstractDataFrame,Matrix}
    @assert size(df, 1) == size(df, 2)
    dim = size(df, 1)
    if ret_df
        DataFrame(ones(dim, dim) - Matrix(df), names(df))
    else
        ones(dim, dim) - Matrix(df)
    end
end

function edges_df_to_graph(df::AbstractDataFrame, threshold = 0.6)::SimpleWeightedGraph
    @assert size(df, 1) == size(df, 2) # assert square
    dim = size(df, 1)
    g = SimpleWeightedGraph(dim)
    # zero weights do nothing thankfully
    for i in 1:dim
        for j in 1:dim
            val = df[i, j]
            val < threshold ? add_edge!(g, i, j, df[i, j]) : continue
        end
    end
    g
end

df_to_cor_graph(df::AbstractDataFrame, threshold::Float64 = 0.8) = edges_df_to_graph(cor_df_to_edge_weights(cor_df(df)), threshold)


function get_dfs() 
    fns = glob(glob_pat)
    df_dict = Dict(zip(map(x->split(x, "_")[1], fns), CSV.read.(fns)))
    collect(values(df_dict))
end

function quick(glob_pat::String = "./data/*7d*.csv", re::Regex = r"(c_*)", join_n::Integer = 200, filt_n::Integer = 5000)::AbstractDataFrame
    dfs = get_dfs()
    ts = map(df->df[:, :t], dfs)
    cls = re_cols(dfs, re)
    tcls = hcat.(ts, cls)
    filt = filter(x->size(x)[1] > filt_n, tcls) # history size 5000 or greater
    join(filt[end - join_n:end]..., on = :x1)
end

glob_pat = "./data/*7d*.csv"
re = r"(v_*)"
re2 = r"(c_*)"
join_n = 200
filt_n = 2000

joined = quick(glob_pat, re, join_n, filt_n)
joinedc = quick(glob_pat, re2, join_n, filt_n)

desc = describe(joined)
descc = describe(joinedc)
cols = desc[desc.nmissing .== 1, :variable]
colsc = descc[descc.nmissing .== 1, :variable]
ss = dropmissing(joined[:, cols])
avg_vols = mean.(eachcol(ss))
scaled = avg_vols / minimum(avg_vols)
ssc = dropmissing(joinedc[:, colsc])

g = df_to_cor_graph(ssc, 2.)

p = graphplot(g, curves = false, names = map(x->"$(x[1])_$(x[2])", enumerate(cols)), fontsize = 3, edge)

p = graphplot(g, curves = false, names = map(x->"$(x[1])_$(x[2])", enumerate(cols)), fontsize = 3, nodesize=scaled)
savefig(p, "close_cors2.png")