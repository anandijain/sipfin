module SipFin

using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra
#=


=#

df_dict(fns) = Dict(zip(fns, dropmissing.(CSV.read.(fns))))

vals_dict(d) = Dict(zip(keys(d), map(x -> begin
    F = svd(x)
    return F.S ./ maximum(F.S)
end, values(d))))

svdf(vs::Dict) = DataFrame(hcat(collect(keys(vs)), hcat(collect(values(vs))...)'))

toz(d::Dict) = Dict(map(
    x -> x.symbol[1] => hcat(zscore.(eachcol(x[:, 3:(end - 1)]))...),
    collect(values(d)),
))

nums_dict(d) = Dict(zip(keys(d), map(x -> Matrix(x[:, 3:(end - 1)]), values(d))))

quikz(p) = svdf(vals_dict(nums_dict(df_dict(readdir(p, join = true)))))

export df_dict, svdf, vals_dict, quik, toz

end
