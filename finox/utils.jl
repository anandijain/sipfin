using Plots, CSV, DataFrames, Glob, Dates
using Statistics, StatsBase, Dates

fns = Glob.glob("*yf_cur.csv*", "./finox/data/")
dfs = CSV.read.(fns)


norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
norm_mat(m::AbstractMatrix) = hcat(map(a -> (a .- mean(a)) ./ std(a), eachcol(m))...)
norm_df(df::AbstractDataFrame) = DataFrame(norm_mat(Matrix(df)), names(df))


df = CSV.read("../intraday_inner.csv")

pv = df[:, 3:end]
df = df[:, 2:end]
ndf = norm_df(pv)
# df = dfs[1]
dtfmt = DateFormat("Y-m-d H:M:S+H:S")
ndf.date_time = Date.(ndf.date_time, dtfmt)

colnames = names(ndf)
colnames = names(df)[2:end]
ndf.date_time

plot(ndf.date_time, [df.AAPL_price, df.AAPL_volume])

df = join(dfs..., on=:t, makeunique=true)
df = CSV.read("history_merged.csv")

for cn in colnames
    display(plot(df.date_time, df[:, cn], label=cn))
    print(cn)
end

