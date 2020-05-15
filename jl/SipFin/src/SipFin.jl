module SipFin

using Plots, Dates
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra, DelimitedFiles, Base
#=


=#
norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
norm_mat(m::AbstractMatrix) = hcat(norm_arr.(eachcol(m))...)
# todo, ignore string columns
norm_df(df::AbstractDataFrame) = DataFrame(norm_mat(Matrix(df)), names(df))
cor_df(df::AbstractDataFrame) = DataFrame(cor(Matrix(df)), names(df))

mavg(vec,n) = [sum(@view vec[i:(i + n - 1)]) / n for i in 1:(length(vec) - (n - 1))]
fib(n) = ([1 1 ; 1 0]^n)[1, 1]
    
# diff(df) = df[2:end, :] .- df[1:end - 1, :]
diff_arr(arr::Array) = sum(abs.(arr[2:end] .- arr[1:end-1]))

cor_df(df::AbstractDataFrame)::AbstractDataFrame = DataFrame(cor(Matrix(df)), names(df))
re_cols(dfs::Array{DataFrame,1}, re::Regex) = map(df->df[:, re], dfs)

# used w nasdaq_o2
# takes a dataframe dictionary and returns the len of each df value
sizes(d::Dict{String,DataFrame})::DataFrame = sort(DataFrame(ticker = collect(keys(d)), nrows = map(x->x[1], size.(collect(values(d))))), :nrows, rev = true)
sizes(dfs::Array{DataFrame,1})::DataFrame = sort(DataFrame(ticker = map(x->names(x)[2], dfs), nrows = map(x->size(x)[1], dfs)), :nrows, rev = true)

feichanghao(glob_pat)::DataFrame = vcat(collect(values(add_tickers(df_dict(glob_pat))))...)

rep_rm(s::String, rmstr::String)::String = replace(s, rmstr =>"")
to_num(s::String)::Float64 = parse(Float64, rep_rm(s, ",")) 

# obviously dangerous, TODO: FIX
usd_to_float(s::String)::Float64 = parse(Float64, rep_rm(rep_rm(s, "\$"), ","))
usd_col_to_float(df::DataFrame, col::Symbol)::Array{Float64, 1} = usd_to_float.(df[:, col])


dir_to_dfs() = vcat(CSV.read.(readdir())...)

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

function garbo_info(p::Array{String, 1})
    df = vcat(CSV.read.(readdir())...)
    df.last_sale_price = usd_col_to_float(df, :last_sale_price)
    rn = df[occursin.("May 14", df.last_trade_timestamp), :]
    groupby(sort(rn[in.(rn.symbol, Ref(p)), :], :symbol), :symbol)

end

function parse_rt(df::DataFrame)::DataFrame
    df[!, :x] = usd_to_float.(df.x)
    df[!, :v] = to_num.(df.v)
    df[!, :amt] = df[:, :x] .* df[:, :v]
    df
    # df = sort(df, :amt, rev=true)
end

# used to clean the insiders data
function parse_insiders(df) 
  df.last_price = usd_col_to_float(df, :last_price)
  df.shares_traded = parse.(Int, replace.(df.shares_traded, ","=>""))
  df.shares_held =  parse.(Int, replace.(df.shares_held, ","=>""))
  dtfmt = "m/d/y"
  df.last_date = Date.(df.last_date, dtfmt)
end


function summarize_rt(df::DataFrame)::DataFrame
    spreads = by(df, :symbol, xmax = :x => maximum, xmin = :x => minimum)
    amts = by(df, :symbol, :amt => sum)
    nrows = by(df, :symbol, nrows = nrow)
    delts = sort(vcat(map(x -> diff_arr(x.x), gdf)...), :x1)
    summary = join([amts, spreads, nrows, delts]..., on=:symbol, makeunique=true)
    # by(df, :symbol, describe)
    # @. sort(by(df, :v, nrow), [:x1, :v], rev=(true, false))
    # by(df, :symbol, tdelt = :t=> x-> maximum(x) - minimum(x))

end

end # module
