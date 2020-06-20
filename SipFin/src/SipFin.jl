module SipFin

using CSV, DataFrames, Glob, DelimitedFiles
using Statistics, StatsBase, Dates, LinearAlgebra
#=


=#

"given array of file names, with fn as key"
df_dict(fns::Array{Any, 1})::Dict{Any, DataFrame} = Dict(zip(fns, dropmissing.(CSV.read.(fns))))

function df_dict(path::String)::Dict{String, DataFrame} 
    fns = readdir(path, join=true)
    Dict(zip(fns, dropmissing.(CSV.read.(fns))))
end 

function normed_svals(a)
    Fs = svd.(a)
    map(x -> x.S ./ x.S[1], Fs)
end

function sv_df(p)
    fns = readdir(p, join = true)
    d = df_dict(fns)
    nums = map(x -> Matrix{Float64}(x[:, 4:(end - 1)]), values(d))
    vs = hcat(normed_svals(nums)...)'
    DataFrame(hcat(collect(keys(d)), vs))
end

"lowercase and replace spaces to underscore for columns of `DataFrame`"
fix_colnames(df) =
    rename(df, map(x -> replace(lowercase(string(x)), " " => "_"), names(df)))

"write an array to a txt file, defaults to newline as delimiter"
function str_arr_to_txt(fn, arr; delim="\n")
    open(fn, "w") do io
        writedlm(io, arr, "\n")
    end
end

"get tickers_stocks.txt from ftp://nasdaqlisted.txt, given fn with no ext"
function nasdaq_fix(fn)
    df = filter(
        x -> x.etf .== "N",
        dropmissing(fix_colnames(CSV.read("$fn.txt", delim = "|"))),
    )
    str_arr_to_txt("./ref_data/tickers_stocks.txt", df.symbol)
end

# used in script to quickly get all fred 
# prob wanna do it in all rust tho
# having bash + rust + julia is jank af
function fred_fix(fn; col::Symbol = :symbol) 

end 

export fix_colnames, df_dict, sv_df

end
