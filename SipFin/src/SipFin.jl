module SipFin

using CSV, DataFrames, Glob, DelimitedFiles
using Statistics, StatsBase, Dates, LinearAlgebra
#=


=#

df_dict(fns) = Dict(zip(fns, dropmissing.(CSV.read.(fns))))

vals_dict(d) = Dict(zip(keys(d), map(x -> begin
    F = svd(x)
    return F.S ./ maximum(F.S)
end, values(d))))

svdf(vs::Dict) = DataFrame(hcat(collect(keys(vs)), hcat(collect(values(vs))...)'))

nums_dict(d) = Dict(zip(keys(d), map(x -> Matrix(x[:, 3:(end - 1)]), values(d))))

quik(p) = svdf(vals_dict(nums_dict(df_dict(readdir(p, join = true)))))

fix_colnames(df) = rename(df, map(x->replace(lowercase(string(x)), " "=>"_"), names(df)))

function str_arr_to_txt(fn, arr)
    open(fn, "w") do io
        writedlm(io, arr, "\n")
    end
end

function nasdaq_fix(fn)
    df = CSV.read("$fn.txt", delim = "|")
	df = dropmissing(fix_colnames(df))
	df = filter(x-> x.etf .== "N", df)
	str_arr_to_txt("./ref_data/tickers_stocks.txt", df.symbol)
end


export fix_colnames, df_dict, svdf, vals_dict, quik 

end
