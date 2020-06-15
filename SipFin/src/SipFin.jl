module SipFin

using CSV, DataFrames, Glob, DelimitedFiles
using Statistics, StatsBase, Dates, LinearAlgebra
#=


=#

df_dict(fns) = Dict(zip(fns, dropmissing.(CSV.read.(fns))))

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

fix_colnames(df) =
    rename(df, map(x -> replace(lowercase(string(x)), " " => "_"), names(df)))

function str_arr_to_txt(fn, arr)
    open(fn, "w") do io
        writedlm(io, arr, "\n")
    end
end

function nasdaq_fix(fn)
    df = filter(
        x -> x.etf .== "N",
        dropmissing(fix_colnames(CSV.read("$fn.txt", delim = "|"))),
    )
    str_arr_to_txt("./ref_data/tickers_stocks.txt", df.symbol)
end


export fix_colnames, df_dict, sv_df

end
