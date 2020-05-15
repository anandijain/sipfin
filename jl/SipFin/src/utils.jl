module Utils
using Plots, Dates
using CSV, DataFrames, Glob
using Statistics, StatsBase, Dates, LinearAlgebra, DelimitedFiles, Base




function summarize_rt(df::DataFrame)::DataFrame
    spreads = by(df, :symbol, xmax = :x => maximum, xmin = :x => minimum)
    amts = by(df, :symbol, :amt => sum)
    nrows = by(df, :symbol, nrows = nrow)
    delts = sort(vcat(map(x -> sum(abs.(x.x[2:end] .- x.x[1:end-1])), gdf)...), :x1)
    summary = join([amts, spreads, nrows, delts]..., on=:symbol, makeunique=true)
    # by(df, :symbol, describe)
    sort!(volvals, (:x1, :v), rev=(true, false))
    by(df, :symbol, tdelt = :t=> x-> maximum(x) - minimum(x))

end



end