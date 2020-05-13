module Utils
using Plots, Dates
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra, DelimitedFiles, Base

# https://assets.bwbx.io/s3/mediaservices/superelastic/data.json
norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
norm_mat(m::AbstractMatrix) = hcat(map(a->(a .- mean(a)) ./ std(a), eachcol(m))...)
norm_df(df::AbstractDataFrame) = DataFrame(norm_mat(Matrix(df)), names(df))
cor_df(df::AbstractDataFrame) = DataFrame(cor(Matrix(df)), names(df))

mavg(vs,n) = [sum(@view vs[i:(i + n - 1)]) / n for i in 1:(length(vs) - (n - 1))]
fib(n) = ([1 1 ; 1 0]^n)[1, 1]
    
diff(df) = df[2:end, :] .- df[1:end - 1, :]
cor_df(df::AbstractDataFrame)::AbstractDataFrame = DataFrame(cor(Matrix(df)), names(df))
re_cols(dfs::Array{DataFrame,1}, re::Regex) = map(df->df[:, re], dfs)

# used w nasdaq_o2
# takes a dataframe dictionary and returns the len of each df value
sizes(d::Dict{String,DataFrame})::DataFrame = sort(DataFrame(ticker = collect(keys(d)), nrows = map(x->x[1], size.(collect(values(d))))), :nrows, rev = true)
sizes(dfs::Array{DataFrame,1})::DataFrame = sort(DataFrame(ticker = map(x->names(x)[2], dfs), nrows = map(x->size(x)[1], dfs)), :nrows, rev = true)

feichanghao(glob_pat)::DataFrame = vcat(collect(values(add_tickers(df_dict(glob_pat))))...)

# df[!, :shares_traded] = parse.(Int, map(x-> replace(x, ","=>""), df.shares_traded))


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

function change_sep(fn) 
    df = CSV.read("$fn.txt", delim = "|")
    rename!(x->Symbol(replace(string(x), " " => "_")), df)
    CSV.write("$fn.csv", df)
    return df
end


function plot_fibs(arr::AbstractArray, range = 5:10)
    plot([mavg(arr, fib(i)) for i in range])
end


function df_dict(glob_pat)::Dict{String,DataFrame}
    fns = glob(glob_pat)
    Dict(zip(map(x->split(x, "_")[1], fns), CSV.read.(fns)))
end

function add_tickers(d::Dict{String,DataFrame})::Dict{String,DataFrame}
    for (k, v) in d
        v[!, :symbol] .= k
    end
    d
end


# function lilmetrics()


function get_dfs(glob_pat)::Array{DataFrame,1}
    CSV.read.(glob(glob_pat))
end


function quick(dfs = get_dfs("./data/*7d*.csv"), re::Regex = r"(c_*)")::AbstractDataFrame # , join_n::Integer = 200, filt_n::Integer = 5000)::AbstractDataFrame
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

function cat_quotes()
    


end