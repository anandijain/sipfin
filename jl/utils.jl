module Utils
using Plots, Dates
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra, DelimitedFiles, Base
using LightGraphs, GraphPlot, SimpleWeightedGraphs, MetaGraphs, GraphRecipes

# https://assets.bwbx.io/s3/mediaservices/superelastic/data.json
norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
norm_mat(m::AbstractMatrix) = hcat(map(a->(a .- mean(a)) ./ std(a), eachcol(m))...)
norm_df(df::AbstractDataFrame) = DataFrame(norm_mat(Matrix(df)), names(df))

cdf(df::AbstractDataFrame) = DataFrame(cor(Matrix(df)), names(df))
cor_df(df::AbstractDataFrame) = DataFrame(cor(Matrix(df)), names(df))
# apply_df(df::AbstractDataFrame) = DataFrame(cor(Matrix(df)), names(df))

## todo function that composes 
function corr_5(dfs)
    m = join(rand(dfs, 5)..., on = :t)
    mat = Matrix(m)[:, 2:5:end]
    labs = names(m)[2:5:end]    
    corrplot(mat, label = labs)
end

function sa_yf()
    # we're assuming we just ran finox w cargo run,
    now = Dates.today()
    yr = string(year(now))
    month = string(Dates.month(now))
    day = string(Dates.day(now))
    # fns = Glob.glob("*:US_$(yr)_$(month)_$(day).csv", "./data/")
    sa = CSV.read("./ref_data/sa.csv")
    sa_slugs = uppercase.(unique(dropmissing(sa, :slug).slug))
    # reg = r"data\/(.*):"
    # matches = match.(reg, fns)
    # yf_slugs = map(x -> x.captures[1], matches)
    yf_slugs = readlines("./ref_data/tickers.txt")
    intersect_ticks = intersect(sa_slugs, yf_slugs)
    println(intersect_ticks)
    str_arr_to_txt("./ref_data/intersect_sa_yf.txt", intersect_ticks)
end

function df_col_to_txt(df::AbstractDataFrame, s::Symbol, fn::String)
    open(fn, "w") do io
        writedlm(io, df[:, s], "\n")
    end
end

function str_arr_to_txt(fn::String, arr::Array{String,1})
    open(fn, "w") do io
        writedlm(io, arr, "\n")
    end
end


function sec13f_fix()
    for fn in Glob.glob("./ref_data/rentec/*.csv")
        header = Symbol.(["nameOfIssuer",
        "titleOfClass",
        "cusip",
        "value",
        "sshPrnamt",
        "sshPrnamtType",
        "investmentDiscretion",
        "otherManager",
        "Sole",
        "Shared",
        "None"])
        df = DataFrame(permutedims(Matrix(CSV.read(fn, header = false))), header)
        print(df)
        CSV.write(fn, df)
    end
end


# sa_yf()
# sec13f_fix()

function change_sep(fn) 
    df = CSV.read("$fn.txt", delim = "|")
    rename!(x->Symbol(replace(string(x), " " => "_")), df)
    CSV.write("$fn.csv", df)
    return df
end

mavg(vs,n) = [sum(@view vs[i:(i + n - 1)]) / n for i in 1:(length(vs) - (n - 1))]
fib(n) = ([1 1 ; 1 0]^n)[1, 1]

function plot_fibs(arr::AbstractArray, range = 5:10)
    plot([mavg(arr, fib(i)) for i in range])
end
    
diff(df) = df[2:end, :] .- df[1:end - 1, :]

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
            val < threshold ? add_edge!(g, i, j, val) : continue
        end
    end
    g
end

function edges_df_to_meta_graph(df::AbstractDataFrame)::MetaGraph
    @assert size(df, 1) == size(df, 2) # assert square
    dim = size(df, 1)
    g = MetaGraph(dim)
    for i in 1:dim
        for j in 1:dim
            val = df[i, j]
            add_edge!(g, i, j)
            set_prop!(g, Edge(i, j), :weight, val)
        end
    end
    g
end


df_to_cor_graph(df::AbstractDataFrame, threshold::Float64 = 0.8) = edges_df_to_graph(cor_df_to_edge_weights(cor_df(df)), threshold)
df_to_meta_cor_graph(df::AbstractDataFrame) = edges_df_to_meta_graph(cor_df_to_edge_weights(cor_df(df)))


function get_dfs(glob_pat) 
    fns = glob(glob_pat)
    print(fns)
    df_dict = Dict(zip(map(x->split(x, "_")[1], fns), CSV.read.(fns)))
    collect(values(df_dict))
end


function quick(dfs = get_dfs(), glob_pat::String = "./data/*7d*.csv", re::Regex = r"(c_*)")::AbstractDataFrame #, join_n::Integer = 200, filt_n::Integer = 5000)::AbstractDataFrame
    ts = map(df->df[:, :t], dfs)
    cols = re_cols(dfs, re)
    catted = hcat.(ts, cols)
    # filt = filter(x->size(x)[1] > filt_n, catted) # history size 5000 or greater
    join(catted..., on = :x1)
end

function df_from_str(s::String)
       fn = glob("./data/$(s)_yf7d*.csv")[1]
       df = CSV.read(fn)

       end

end